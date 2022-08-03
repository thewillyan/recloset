pub mod closet;
pub mod menu;

use chrono::Local;
use closet::*;
use menu::{Act, Menu, Runner};
use std::io::{self, Write};
use std::rc::Rc;

pub struct Data {
    pub clothes: Vec<Rc<Clth>>,
    pub styles: Vec<Rc<Style>>
}

impl Data {
    pub fn new() -> Data {
        Data {
            clothes: Vec::new(),
            styles: Vec::new()
        }
    }

    pub fn add_clth(&mut self, clth: Clth) {
        self.clothes.push(Rc::new(clth));
    }

    pub fn print_clothes(&self) {
        let clothes_data = if self.clothes.is_empty() {
            String::from("No clothes to list!")
        } else {
            self.clothes
                .iter()
                .map(|clth| clth.to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
        };
        println!("{}", clothes_data);
    }

    pub fn request_id(&self) -> u32 {
        let ids: Vec<u32> = self.clothes.iter().map(|clth| clth.id).collect();
        let mut new_id = 0;
        while ids.contains(&new_id) {
            new_id += 1
        }
        new_id
    }

    pub fn get_clth(&self, id: u32) -> Option<Rc<Clth>> {
        for clth in self.clothes.iter() {
            if clth.id == id { return Some(Rc::clone(clth)) }
        }
        None
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(Rc::new(style));
    }

    pub fn get_style(&self, name: &str) -> Option<Rc<Style>> {
        for style in self.styles.iter() {
            if &style.name == name { return Some(Rc::clone(style)) }
        }
        None
    }

    pub fn get_style_or_add(&mut self, name: &str) -> Rc<Style> {
        match self.get_style(name) {
            Some(style) => style,
            None => {
                let style = Style::new(name);
                self.add_style(style);
                Rc::clone(self.styles.last().unwrap())
            }
        }
    }
}

#[derive(Clone)]
enum Action {
    AddClth,
    ListClths,
    Back,
    Quit,
}

pub fn run(mut data: Data) {
    let mut clth_menu = Menu::new("Clothing options");
    clth_menu.add_action(Act::new("Add clothing", Action::AddClth));
    clth_menu.add_action(Act::new("List clothes", Action::ListClths));
    clth_menu.add_action(Act::new("Back", Action::Back));

    let mut menu = Menu::new("root");
    menu.add_submenu(clth_menu);
    menu.add_action(Act::new("Quit", Action::Quit));

    let mut runner = Runner::new(menu);

    loop {
        if let Some(act) = runner.run("> ") {
            match act {
                Action::AddClth => {
                    let new_clth = clth_input(&mut data);
                    data.add_clth(new_clth);
                }
                Action::ListClths => data.print_clothes(),
                Action::Back => runner.back().unwrap(),
                Action::Quit => break,
            }
        }
    }
}

fn clth_input(data: &mut Data) -> Clth {
    let name = read_not_empty("Enter a style name: ").to_lowercase();
    Clth::new(
        data.request_id(),
        kind_input(),
        sex_input(),
        size_input(),
        color_input(),
        target_input(),
        Local::today().naive_local(),
        data.get_style_or_add(&name)
    )
}

fn kind_input() -> Kind {
    let mut kind_menu = Menu::new("kind");
    kind_menu.add_action(Act::new("Chest clothing", Kind::Chest));
    kind_menu.add_action(Act::new("Leg clothing", Kind::Leg));
    kind_menu.add_action(Act::new("Footwear", Kind::Foot));

    let mut runner = Runner::new(kind_menu);
    runner.run("Select a clothing kind: ").unwrap()
}

fn sex_input() -> Sex {
    let mut sex_menu = Menu::new("sex");
    sex_menu.add_action(Act::new("Male", Sex::Male));
    sex_menu.add_action(Act::new("Female", Sex::Female));
    sex_menu.add_action(Act::new("Unissex", Sex::Unissex));

    let mut runner = Runner::new(sex_menu);
    runner.run("Select a sex: ").unwrap()
}

fn size_input() -> Size {
    let mut size_menu = Menu::new("size");
    size_menu.add_action(Act::new("XS", Size::XS));
    size_menu.add_action(Act::new("S", Size::S));
    size_menu.add_action(Act::new("M", Size::M));
    size_menu.add_action(Act::new("L", Size::L));
    size_menu.add_action(Act::new("XL", Size::XL));

    let mut runner = Runner::new(size_menu);
    runner.run("Select a size: ").unwrap()
}

fn color_input() -> Rgb {
    loop {
        let input = read_not_empty("Enter a color: ");
        
        let color = if &input[0..1] == "#" {
            Rgb::try_from_hex(&input[1..])
        } else {
            None
        };

        match color {
            Some(color) => break color,
            None => {
                eprintln!("Invalid color! Help: valid colors are hex colors.")
            }
        }
    }
}

fn target_input() -> Target {
    let mut tar_menu = Menu::new("target");
    tar_menu.add_action(Act::new("Donation", Target::Donation));
    tar_menu.add_action(Act::new("Sale", Target::Sale(price_input())));
    tar_menu.add_action(Act::new("Keep", Target::Keep));

    let mut runner = Runner::new(tar_menu);
    runner.run("Select a target: ").unwrap()
}

fn price_input() -> u64 {
    loop {
        let price = read_not_empty("Enter a price: $");

        match price.trim().parse::<f64>() {
            Ok(value) => break (value * 100.0) as u64,
            Err(_) => eprintln!("Invalid price!"),
        }
    }
}

fn read_not_empty(msg: &str) -> String {
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
