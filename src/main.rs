use recloset::storage::FileData;
use std::fs::{ self, File };
use std::process;
use std::io::Write;

const DATAFILE: &str = "data.toml";
const DATAPATH: &str = concat!(env!("HOME"), "/.config/recloset");

fn main() {
    let filedir = format!("{}/{}", DATAPATH, DATAFILE);
    let file_content = match fs::read_to_string(&filedir) {
        Ok(content) => content,
        Err(_) => {
            fs::create_dir_all(DATAPATH).unwrap();
            File::create(&filedir).unwrap();
            String::from("")
        },
    };

    let fdata = match FileData::from(&file_content) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}:\n  {}", filedir, err);
            process::exit(1);
        }
    };

    let mut data = match fdata.to_data() {
        Ok(data) => data,
        Err(msg) => {
            eprintln!("{}:\n  Data error: {}", filedir, msg);
            process::exit(1);
        }
    };

    recloset::run(&mut data);

    let mut file = match File::create(&filedir) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error while opening {} for writing: {}", &filedir, err);
            process::exit(1);
        }
    };
    file.write_all(data.to_toml().as_bytes()).unwrap();
}
