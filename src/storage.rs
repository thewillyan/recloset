use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use chrono::NaiveDate;

use crate::Data;
use crate::closet::{ Kind, Sex, Size, Rgb, Target};
use crate::closet::{ Clth, Clothes, Outfits, Outfit, Styles };

#[derive(Debug)]
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

    pub fn extract_clths(&self) -> Result<(Clothes, Styles), &'static str> {
        let mut clothes = Clothes::new();
        let mut styles = Styles::new();

        for chunk in &self.clth_chunks {
            let id = match chunk.fields.get("id") {
                Some(Value::Num(num)) => *num as u32,
                Some(_) => return Err("'id' is not a text field."),
                None => return Err("Missing 'id' field."),
            };

            let kind = match chunk.fields.get("kind") {
                Some(Value::Text(value)) => Kind::from_str(value),
                Some(_) => return Err("'kind' is not a numerical field."),
                None => return Err("Missing 'kind' field."),
            };

            let kind = match kind {
                Ok(value) => value,
                Err(msg) => return Err(msg),
            };

            let sex = match chunk.fields.get("sex") {
                Some(Value::Text(value)) => Sex::from_str(value),
                Some(_) => return Err("'sex' is not a numerical field."),
                None => return Err("Missing 'sex' field."),
            };

            let sex = match sex {
                Ok(value) => value,
                Err(msg) => return Err(msg),
            };

            let size = match chunk.fields.get("size") {
                Some(Value::Text(value)) => Size::from_str(value),
                Some(_) => return Err("'size' is not a numerical field."),
                None => return Err("Missing 'size' field."),
            };

            let size = match size {
                Ok(value) => value,
                Err(msg) => return Err(msg),
            };

            let color = match chunk.fields.get("color") {
                Some(Value::Text(value)) => Rgb::try_from_hex(value),
                Some(_) => return Err("'color' is not a numerical field."),
                None => return Err("Missing 'color' field."),
            };

            let color = match color {
                Some(color) => color,
                None => return Err("Invalid color."),
            };

            let target = match chunk.fields.get("target") {
                Some(Value::Text(value)) => Target::from_str(value),
                Some(_) => return Err("'target' is not a numerical field."),
                None => return Err("Missing 'target' field."),
            };

            let target = match target {
                Ok(value) => value,
                Err(msg) => return Err(msg) ,
            };

            let purchase_date = match chunk.fields.get("purchase_date") {
                Some(Value::Text(value)) => {
                    NaiveDate::parse_from_str(value, "%Y-%m-%d")
                },
                Some(_) => return Err("'purchase_date' is not a numerical field."),
                None => return Err("Missing 'purchase_date' field."),
            };

            let purchase_date = match purchase_date {
                Ok(date) => date,
                Err(_) => return Err("Invalid date."),
            };

            let stl_name = match chunk.fields.get("style") {
                Some(Value::Text(name)) => name,
                Some(_) => return Err("'style' is not a numerical field."),
                None => return Err("Missing 'style' field."),
            };
            let style = styles.get_or_add(&stl_name);

            clothes.add(
                Clth::new(id, kind, sex, size, color, target, purchase_date, style)
            );
        }
        Ok((clothes, styles))
    }

    pub fn extract_outfits(&self, clothes: &Clothes) -> Result<Outfits, &'static str> {
        let mut outfits = Outfits::new();

        for chunk in &self.outfit_chunks {
            let chest_id = match chunk.fields.get("chest") {
                Some(Value::Num(num)) => *num as u32,
                Some(_) => return Err("'chest' is not a text field."),
                None => return Err("Missing 'chest' field."),
            };
            let chest = match clothes.get(chest_id) {
                Some(rc) => Rc::downgrade(rc),
                None => return Err("Invalid 'chest' id."),
            };

            let leg_id = match chunk.fields.get("leg") {
                Some(Value::Num(num)) => *num as u32,
                Some(_) => return Err("'leg' is not a text field."),
                None => return Err("Missing 'leg' field."),
            };
            let leg = match clothes.get(leg_id) {
                Some(rc) => Rc::downgrade(rc),
                None => return Err("Invalid 'leg' id."),
            };

            let foot_id = match chunk.fields.get("foot") {
                Some(Value::Num(num)) => *num as u32,
                Some(_) => return Err("'foot' is not a text field."),
                None => return Err("Missing 'foot' field."),
            };
            let foot = match clothes.get(foot_id) {
                Some(rc) => Rc::downgrade(rc),
                None => return Err("Invalid 'foot' id."),
            };

            let outfit = match Outfit::new(outfits.request_id(), chest, leg, foot) {
                Ok(value) => value,
                Err(msg) => return Err(msg),
            };
            outfits.add(outfit);
        }
        Ok(outfits)
    }

    pub fn to_data(&self) -> Result<Data, &'static str> {
        let (clothes, styles) = match self.extract_clths() {
            Ok(values) => values,
            Err(msg) => return Err(msg),
        };

        let outfits = match self.extract_outfits(&clothes) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };

        Ok(Data { clothes, styles, outfits, ..Data::new() })
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

    const CLTH1: &str = "
        [clth]
        id = 0
        kind = \"chest\"
        sex = \"male\"
        size = \"L\"
        color = \"FF00EE\"
        target= \"Sale for $20.75\"
        purchase_date = \"2022-08-15\"
        style = \"summer\"
    ";
    const CLTH2: &str = "
        [clth]
        id = 1
        kind = \"foot\"
        sex = \"male\"
        size = \"M\"
        color = \"FF00EE\"
        target= \"Sale for $20.75\"
        purchase_date = \"2022-08-15\"
        style = \"summer\"
    ";
    const CLTH3: &str = "
        [clth]
        id = 2
        kind = \"leg\"
        sex = \"male\"
        size = \"M\"
        color = \"FF00EE\"
        target= \"Sale for $20.75\"
        purchase_date = \"2022-08-15\"
        style = \"summer\"
    ";

    const OUTFIT: &str = "
        [outfit]
        chest = 0
        leg = 2
        foot = 1
    ";

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

    #[test]
    pub fn create_clths() {
        let text = [CLTH1, CLTH2, CLTH3].join("\n\n");
        let fdata = FileData::from(&text).unwrap();
        assert_eq!(3, fdata.clth_chunks.len());
        assert_eq!(0, fdata.outfit_chunks.len());

        let (clths, stls) = fdata.extract_clths().unwrap();
        assert!(
            clths.get(0).is_some() && clths.get(1).is_some() &&
            clths.get(2).is_some());
        assert!(stls.get("summer").is_some());
    }

    #[test]
    pub fn create_outfits() {
        let text = [CLTH1, CLTH2, CLTH3, OUTFIT].join("\n\n");
        let fdata = FileData::from(&text).unwrap();

        let (clths, _) = fdata.extract_clths().unwrap();
        let outfits = fdata.extract_outfits(&clths).unwrap();
        assert_eq!(vec![ [0,2,1] ], outfits.to_id_matrix());
    }
}
