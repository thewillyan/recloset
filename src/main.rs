use recloset::storage::FileData;
use std::fs::{ self, File };
use std::process;

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

    let data = match fdata.to_data() {
        Ok(data) => data,
        Err(msg) => {
            eprintln!("{}:\n  Data error: {}", filedir, msg);
            process::exit(1);
        }
    };

    recloset::run(data);
}
