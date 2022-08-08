pub mod closet;
pub mod menu;
pub mod input;

// external
use std::rc::Rc;
use chrono::{Local, NaiveDate};

// intern
use closet::*;
use menu::{Act, Menu, Runner};
use input::InputErr;

pub struct Data {
    pub clothes: Clothes,
    pub styles: Styles,
    pub clth_sets: Vec<ClthSet>,
    pub cache: TmpCache,
}

impl Data {
    pub fn new() -> Data {
        Data {
            clothes: Clothes::new(),
            styles: Styles::new(),
            clth_sets: Vec::new(),
            cache: TmpCache::new(),
        }
    }
}

pub struct TmpCache {
    pub clth: Option<ClthBuffer>
}

impl TmpCache {
    pub fn new() -> TmpCache {
        TmpCache {
            clth: None,
        }
    }
}

pub struct ClthBuffer {
    pub kind: Option<Kind>,
    pub sex: Option<Sex>,
    pub size: Option<Size>,
    pub color: Option<Rgb>,
    pub price: Option<u64>,
    pub target: Option<Target>,
}

impl ClthBuffer {
    pub fn new() -> ClthBuffer {
        ClthBuffer {
            kind: None,
            sex: None,
            size: None,
            color: None,
            price: None,
            target: None,
        }
    }

    pub fn to_clth(self, id: u32, date: NaiveDate, style: Rc<Style>) -> Clth {
        Clth::new(
            id,
            self.kind.expect("Missing 'kind' field on buffer."),
            self.sex.expect("Missing 'sex' field on buffer."),
            self.size.expect("Missing 'size' field on buffer."),
            self.color.expect("Missing 'color' field on buffer."),
            self.target.expect("Missing 'target' field on buffer."),
            date,
            style
        )
    }
}

pub fn fill_clth_buffer(cache: &mut ClthBuffer) -> Option<InputErr> {
    if let None = cache.kind {
        match input::kind() {
            Ok(kind) => cache.kind = Some(kind),
            Err(err) => return Some(err)
        }
    }

    if let None = cache.sex {
        match input::sex() {
            Ok(sex) => cache.sex = Some(sex),
            Err(err) => return Some(err)
        }
    }

    if let None = cache.size {
        match input::size() {
            Ok(size) => cache.size = Some(size),
            Err(err) => return Some(err)
        }
    }

    if let None = cache.color {
        match input::color() {
            Ok(color) => cache.color = Some(color),
            Err(err) => return Some(err)
        }
    }

    if let None = cache.price {
        match input::price() {
            Ok(price) => cache.price = Some(price),
            Err(err) => return Some(err)
        }
    }
    
    if let None = cache.target {
        match input::target(cache.price.unwrap()) {
            Ok(target) => cache.target = Some(target),
            Err(err) => return Some(err)
        }
    }

    None
}

pub fn user_add_clth(data: &mut Data) {
    let result = InputErr::log_until_ok(ClthBuffer::new(), fill_clth_buffer);
    match result {
        Ok(buffer) => {
            let stl_name = InputErr::until_ok(input::style_name);
            if let None = stl_name { return; }
            let stl = data.styles.get_or_add(&stl_name.unwrap());

            let id = data.clothes.request_id();
            let date = Local::today().naive_local();
            let new_clth = buffer.to_clth(id, date, stl);
            data.clothes.add(new_clth);
        },
        Err((buffer, _)) => data.cache.clth = Some(buffer),
    }
}

pub fn user_update_clth(data: &mut Data) {
    println!("{}", data.clothes);
    let clth = InputErr::until_ok(|| input::select_clth(&data.clothes));
    if let None = clth { return ;}
    let field = InputErr::until_ok(input::select_clth_field);
    if let None = field { return ;}

    match field.unwrap().as_str() {
        "color" => {
            let color = InputErr::until_ok(input::color);
            if let None = color { return ;}
            clth.unwrap().borrow_mut().color = color.unwrap()
        },
        "kind" => {
            let kind = InputErr::until_ok(input::kind);
            if let None = kind { return ;}
            clth.unwrap().borrow_mut().kind = kind.unwrap()
        },
        "size" => {
            let size = InputErr::until_ok(input::size);
            if let None = size { return ;}
            clth.unwrap().borrow_mut().size = size.unwrap()
        },
        "sex" => {
            let sex = InputErr::until_ok(input::sex);
            if let None = sex { return ;}
            clth.unwrap().borrow_mut().sex = sex.unwrap()
        },
        "target" => {
            let price = match InputErr::until_ok(input::price) {
                Some(num) => num,
                None => return,
            };

            let target = InputErr::until_ok(|| input::target(price));
            if let None = target { return ;}
            clth.unwrap().borrow_mut().target = target.unwrap()
        },
        "style" => {
            let stl_name = match InputErr::until_ok(input::style_name) {
                Some(name) => name,
                None => return,
            };

            clth.unwrap().borrow_mut().style = data.styles.get_or_add(&stl_name)
        },
        value => panic!("Expecting a clothing field, found: '{}'.", value)
    }
}

#[derive(Clone)]
pub enum Event {
    AddClth,
    ListClths,
    UpdateClth,
    Back,
    Quit,
}

pub fn run(mut data: Data) {
    let mut clth_menu = Menu::new("Clothing options");
    clth_menu.add_action(Act::new("Add clothing", Event::AddClth));
    clth_menu.add_action(Act::new("Update clothing", Event::UpdateClth));
    clth_menu.add_action(Act::new("List clothes", Event::ListClths));

    clth_menu.add_action(Act::new("Back", Event::Back));

    let mut menu = Menu::new("root");
    menu.add_submenu(clth_menu);
    menu.add_action(Act::new("Quit", Event::Quit));

    let mut runner = Runner::new(menu);

    loop {
        if let Some(act) = runner.run("> ") {
            match act {
                Event::AddClth => user_add_clth(&mut data),
                Event::ListClths => println!("{}", data.clothes),
                Event::UpdateClth => user_update_clth(&mut data),
                Event::Back => runner.back().unwrap(),
                Event::Quit => break,
            }
        }
    }
}
