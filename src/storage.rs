//const FILE: &str = "data.toml";
use std::collections::HashMap;
use std::fmt::Display;

pub struct ParseError {
    pub line: u32,
    pub msg: String,
}

impl ParseError {
    pub fn new(line: u32, msg: String) -> ParseError {
        ParseError { line , msg }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on line {}: {}", self.line, &self.msg)
    }
}

pub struct FileData {
    pub clth_chunks: Vec<DataChunk>,
    pub outfit_chunks: Vec<DataChunk>,
}

impl FileData {
    pub fn new() -> FileData {
        FileData {
            clth_chunks: Vec::new(),
            outfit_chunks: Vec::new()
        }
    }

    pub fn from(text: &str) -> Result<FileData, ParseError> {
        let mut fdata = FileData::new();
        let chunks = match into_chunks(text) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        for chunk in chunks {
            match chunk.header {
                DataHeader::Clth => fdata.clth_chunks.push(chunk),
                DataHeader::Outfit => fdata.outfit_chunks.push(chunk),
            }
        }
        Ok(fdata)
    }
}

pub enum DataHeader {
    Clth,
    Outfit
}

pub enum Value {
    Text(String),
    Num(i32)
}

pub struct DataChunk{
    pub header: DataHeader,
    pub fields: HashMap<String, Value>,
}

pub fn parse(chunk: &str) -> Result<DataChunk, String> {
    let mut lines = chunk.lines();
    let header = lines
        .next()
        .expect("Expected a header, found: nothing.")
        .trim();

    let header = match header {
        "[clth]" => DataHeader::Clth,
        "[outfit]" => DataHeader::Outfit,
        _ => return Err(format!("'{}' is a invalid header.", header))
    };

    let mut fields = HashMap::new();
    for line in lines {
        let (key, value) = match line.split_once('=') {
            Some((left, right)) => (left.trim(), right.trim()),
            None => return Err(format!("Syntax error: '{}'", line))
        };

        let value = if value.starts_with("\"") && value.ends_with("\"") {
            let end = value.len() - 1; 
            Value::Text(String::from(&value[1..end]))
        } else {
            let num: i32 = match value.parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Syntax error: '{}'", value))
            };
            Value::Num(num)
        };
        fields.insert(String::from(key), value); }

    Ok(DataChunk { header, fields })
}

pub fn into_chunks(text: &str) -> Result<Vec<DataChunk>, ParseError> { 
    let mut chunks: Vec<DataChunk> = Vec::new();
    let mut chunk: Option<String> = None;
    let mut header_line = 0;

    for (index, line) in text.lines().enumerate() {
        if !line.trim().is_empty() {
            if chunk.is_none() {
                header_line = index as u32 + 1;
            }
            let value = chunk.get_or_insert(String::new());
            value.push_str(line);
            value.push('\n');
        } else if chunk.is_some() {
            let value = chunk.take().unwrap(); 
            match parse(&value) {
                Ok(data) => chunks.push(data),
                Err(msg) => return Err(ParseError::new(header_line, msg)),
            }
        }
    }

    if let Some(value) = chunk.take() {
        match parse(&value) {
            Ok(data) => chunks.push(data),
            Err(msg) => return Err(ParseError::new(header_line, msg)),
        }
    }
    Ok(chunks)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const TEXT1: &str = "[clth]\n id   = 2\nkind = \"chest\"";
    const TEXT2: &str = "[outfit]\nchest = 1\nleg = 3\n";

    #[test]
    pub fn text_to_datachunk() {
        let result = match super::parse(TEXT1) {
            Ok(value) => value,
            Err(msg) => panic!("{}", msg),
        };

        assert!(matches!(result.header, DataHeader::Clth));

        let id = match &*result.fields.get("id").expect("Id not found.") {
            Value::Num(v) => v,
            Value::Text(_) => panic!("Id interpreted as string!"),
        };
        assert_eq!(2, *id);

        let kind = match &*result.fields.get("kind").expect("Kind not found.") {
            Value::Text(v) => v,
            Value::Num(_) => panic!("Kind interpreted as number!"),
        };
        assert_eq!("chest", kind);
    }

    #[test]
    pub fn create_datachunks_from_text() {
        let txt = format!("{}\n\n\n\n{}", TEXT1, TEXT2);
        let file_data = match FileData::from(&txt) {
            Ok(value) => value,
            Err(msg) => panic!("{}", msg),
        };
        assert_eq!(1, file_data.clth_chunks.len());
        assert_eq!(1, file_data.outfit_chunks.len());
    }
}
