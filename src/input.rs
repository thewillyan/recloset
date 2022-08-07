use std::process;
use crate::menu::{ Menu, Act, Runner};
use crate::closet::{ Kind, Sex, Size, Target, Rgb };
use std::io::{ self, Write };

pub enum ErrType {
    Recover,
    Abort,
    Fatal(u8)
}

pub struct InputErr {
    pub class: ErrType,
    pub msg: String,
}

impl InputErr {
    pub fn user_abort() -> InputErr {
        InputErr {
            class: ErrType::Abort,
            msg: String::from("Aborted by the user."),
        }
    }

    pub fn wrong(msg: &str) -> InputErr {
        InputErr {
            class: ErrType::Recover,
            msg: String::from(msg)
        }
    }

    // returns true if is possible to recover, false otherwise.
    pub fn handle(err: &InputErr) -> bool {
        match err.class {
            ErrType::Recover => {
                eprintln!("{}", &err.msg);
                true
            },
            ErrType::Abort => {
                eprintln!("{}", &err.msg);
                false
            },
            ErrType::Fatal(ext_code) => {
                eprintln!("FATAL error: {}", &err.msg);
                process::exit(ext_code as i32)
            }
        }
    }

    pub fn until_ok<F,T>(func: F) -> Option<T>
        where F: Fn() -> Result<T, InputErr>
    {
        loop {
            match func() {
                Ok(var) => break Some(var),
                Err(err) => {
                    let try_again = Self::handle(&err);
                    if try_again { continue; }
                    else { break None; }
                }
            }
        }
    }

    pub fn log_until_ok<F, T>(mut log: T, fill: F) -> Result<T, (T, InputErr)>
        where F: Fn(&mut T) -> Option<InputErr>
    {
        loop {
            match fill(&mut log) {
                None => break Ok(log),
                Some(err) => {
                    let try_again = Self::handle(&err);
                    if try_again { continue; }
                    else { break Err((log, err)); }
                }
            }
        }
    }
}

pub fn read_not_empty(msg: &str) -> String {
    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input!");

        let input = input.trim();
        if input.is_empty() {
            eprintln!("Invalid input!");
        } else {
            break String::from(input);
        }
    }
}

pub fn menu_from_vec(label: &str, vec: &[&str]) -> Menu<usize> {
    vec.iter().enumerate().fold(Menu::new(label), |mut menu, (index, item)| {
        menu.add_action(Act::new(item, index));
        menu
    })
}

pub fn kind() -> Result<Kind, InputErr> {
    let menu = menu_from_vec("kind menu", &["Chest", "Leg", "Foot", "Exit"]);
    let sel_index = Runner::new(menu).run("Select a type: ").unwrap();

    match sel_index {
        0 => Ok(Kind::Chest),
        1 => Ok(Kind::Leg),
        2 => Ok(Kind::Foot),
        _ => Err(InputErr::user_abort())
    }
}

pub fn sex() -> Result<Sex, InputErr> {
    let menu = menu_from_vec("sex menu", &["Male", "Female", "Unissex", "Exit"]);
    let sel_index = Runner::new(menu).run("Select a sex: ").unwrap();

    match sel_index {
        0 => Ok(Sex::Male),
        1 => Ok(Sex::Female),
        2 => Ok(Sex::Unissex),
        _ => Err(InputErr::user_abort())
    }
}

pub fn size() -> Result<Size, InputErr> {
    let menu = menu_from_vec("size menu", &["XS", "S", "M", "L", "XL", "Exit"]);
    let sel_index = Runner::new(menu).run("Select a size: ").unwrap();

    match sel_index {
        0 => Ok(Size::XS),
        1 => Ok(Size::S),
        2 => Ok(Size::M),
        3 => Ok(Size::L),
        4 => Ok(Size::XL),
        _ => Err(InputErr::user_abort())
    }
}

pub fn target(price: u64) -> Result<Target, InputErr> {
    let menu = menu_from_vec("target menu", &["Donation", "Sale", "Keep", "Exit"]);
    let sel_index = Runner::new(menu).run("Select a target: ").unwrap();

    match sel_index {
        0 => Ok(Target::Donation),
        1 => Ok(Target::Sale(price)),
        2 => Ok(Target::Keep),
        _ => Err(InputErr::user_abort())
    }
}

pub fn price() -> Result<u64, InputErr> {
    let price = read_not_empty("Enter a price: $");

    if price.to_lowercase() == "exit" {
        return Err(InputErr::user_abort());
    }

    match price.parse::<f64>() {
        Ok(value) => Ok((value * 100.0) as u64),
        Err(_) => Err(InputErr::wrong("Invalid price!")),
    }
}

pub fn color() -> Result<Rgb, InputErr> {
    let input = read_not_empty("Enter a color: ").to_lowercase();

    if input == "exit" {
        return Err(InputErr::user_abort());
    }

    let color = if &input[0..1] == "#" {
        Rgb::try_from_hex(&input[1..])
    } else {
        None
    };

    color.ok_or(InputErr::wrong("Invalid color! Help: valid colors are \
            hex colors."))
}

pub fn style_name() -> Result<String, InputErr> {
    let input = read_not_empty("Enter a style name: ").to_lowercase();

    if input == "exit" {
        return Err(InputErr::user_abort());
    }

    Ok(input)
}
