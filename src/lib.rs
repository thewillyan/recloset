pub mod closet;
pub mod menu;
pub mod input;
pub mod storage;

// external
use std::rc::Rc;
use chrono::Local;

// intern
use closet::*;
use menu::{Act, Menu, Runner};
use input::InputErr;

pub struct Data {
    pub clothes: Clothes,
    pub styles: Styles,
    pub outfits: Outfits,
    pub cache: TmpCache,
}

impl Data {
    pub fn new() -> Data {
        Data {
            clothes: Clothes::new(),
            styles: Styles::new(),
            outfits: Outfits::new(),
            cache: TmpCache::new(),
        }
    }

    pub fn to_toml(&self) -> String {
        format!("{}\n\n{}", self.clothes.to_toml(), self.outfits.to_toml())
    }
}

pub struct TmpCache {
    pub clth: Option<ClthBuffer>,
    pub outfit: Option<OutfitBuffer>
}

impl TmpCache {
    pub fn new() -> TmpCache {
        TmpCache {
            clth: None,
            outfit: None
        }
    }
}

pub fn fill_clth_buffer(cache: &mut ClthBuffer) -> Result<(), InputErr> {
    if let None = cache.kind {
        cache.kind = Some(input::kind()?);
    }

    if let None = cache.sex {
        cache.sex = Some(input::sex()?);
    }

    if let None = cache.size {
        cache.size = Some(input::size()?);
    }

    if let None = cache.color {
        cache.color = Some(input::color()?);
    }

    if let None = cache.price {
        cache.price = Some(input::price()?);
    }
    
    if let None = cache.target {
        cache.target = Some(input::target(cache.price.unwrap())?);
    }
    Ok(())
}

pub fn fill_outfit_buffer(cache: &mut OutfitBuffer, clothes: &Clothes)
    -> Result<(), InputErr>
{
    let map = clothes.map_by_kind();
    let chests = map.get("chest").unwrap();
    let leggings = map.get("leg").unwrap();
    let footwears = map.get("foot").unwrap();

    let separator = ">-<".repeat(10);

    if let None = cache.chest {
        println!("{}", chests);
        cache.chest = Some(Rc::downgrade(&input::select_clth(chests)?));
        println!("{}", separator);
    }

    if let None = cache.leg {
        println!("{}", leggings);
        cache.leg = Some(Rc::downgrade(&input::select_clth(leggings)?));
        println!("{}", separator);
    }

    if let None = cache.foot {
        println!("{}", footwears);
        cache.foot = Some(Rc::downgrade(&input::select_clth(footwears)?));
    }
    Ok(())
}

pub fn user_add_clth(data: &mut Data) {
    let buffer = match data.cache.clth.take() {
        Some(buffer) => {
            let use_cache = InputErr::until_ok(|| {
                input::confirm("Restore last session")
            }).unwrap();

            if use_cache { buffer } else { ClthBuffer::new() }
        },
        None => ClthBuffer::new(),
    };

    let result = InputErr::log_until_ok(buffer, fill_clth_buffer);
    match result {
        Ok(buffer) => {
            let stl_name = InputErr::until_ok(input::style_name);
            if let None = stl_name { return; }
            let stl = data.styles.get_or_add(&stl_name.unwrap());

            let id = data.clothes.request_id();
            let date = Local::today().naive_local();
            let new_clth = buffer.to_clth(id, date, stl);
            data.clothes.add(new_clth);
            println!("Clothing has been added.\n");
        },
        Err((buffer, _)) => data.cache.clth = Some(buffer),
    }
}

pub fn user_rm_clth(data: &mut Data) {
    println!("{}", data.clothes);
    let clth = match InputErr::until_ok(|| input::select_clth(&data.clothes)) {
        Some(value) => value,
        None => return,
    };
    data.clothes.remove(clth.borrow().id).unwrap();
}

pub fn user_update_clth(data: &mut Data) {
    println!("{}\n", data.clothes);
    let clth = InputErr::until_ok(|| input::select_clth(&data.clothes));
    if let None = clth { return ;}
    let field = InputErr::until_ok(input::select_clth_field);
    if let None = field { return ;}

    match field.unwrap().as_str() {
        "color" => {
            let color = InputErr::until_ok(input::color);
            if let None = color { return ;}
            clth.unwrap().borrow_mut().color = color.unwrap();
        },
        "kind" => {
            let clth = clth.unwrap();

            if Rc::weak_count(&clth) != 0 {
                eprintln!("Can't update the kind of a clothing that is in \
                some outfit.");
                return;
            }

            let kind = InputErr::until_ok(input::kind);
            if let None = kind { return ;}
            clth.borrow_mut().kind = kind.unwrap();
        },
        "size" => {
            let size = InputErr::until_ok(input::size);
            if let None = size { return ;}
            clth.unwrap().borrow_mut().size = size.unwrap();
        },
        "sex" => {
            let sex = InputErr::until_ok(input::sex);
            if let None = sex { return ;}
            clth.unwrap().borrow_mut().sex = sex.unwrap();
        },
        "target" => {
            let price = match InputErr::until_ok(input::price) {
                Some(num) => num,
                None => return,
            };

            let target = InputErr::until_ok(|| input::target(price));
            if let None = target { return ;}
            clth.unwrap().borrow_mut().target = target.unwrap();
        },
        "style" => {
            let clth = clth.unwrap();

            if Rc::weak_count(&clth) != 0 {
                eprintln!("Can't update the style of a clothing that is in \
                some outfit.");
                return;
            }

            let stl_name = match InputErr::until_ok(input::style_name) {
                Some(name) => name,
                None => return,
            };

            clth.borrow_mut().style = data.styles.get_or_add(&stl_name);
        },
        value => panic!("Expecting a clothing field, found: '{}'.", value)
    }
}

pub fn user_add_outfit(data: &mut Data) {
    let cache = match data.cache.outfit.take() {
        Some(value) => {
            let use_cache = InputErr::until_ok(|| {
                input::confirm("Restore last session")
            }).unwrap();

            if use_cache { value } else { OutfitBuffer::new() }
        },
        None => OutfitBuffer::new()
    };

    let cache_res = InputErr::log_until_ok(cache, |log| {
        fill_outfit_buffer(log, &data.clothes)
    });

    let cache = match cache_res {
        Ok(buffer) => buffer,
        Err((buffer, _)) => {
            data.cache.outfit = Some(buffer);
            return;
        },
    };

    let outfit = cache.to_outfit(data.outfits.request_id());

    let result = match outfit {
        Ok(set) => data.outfits.add(set),
        Err(msg) => {
            eprintln!("Error while creating outfit: {}", msg);
            return ;
        },
    };

    match result {
        Some(err) => eprintln!("Error while adding outfit: {}", err),
        None => println!("Oufit has been added.\n")
    }
}

pub fn user_rm_outfit(data: &mut Data) {
    let outfit = match InputErr::until_ok(|| input::select_outfit(&data.outfits)) {
        Some(outfit) => outfit,
        None => return,
    };

    data.outfits.remove(outfit.id).unwrap();
}

#[derive(Clone)]
pub enum Event {
    AddClth,
    RemoveClth,
    ListClths,
    UpdateClth,
    AddOutfit,
    RemoveOutfit,
    ListOutfits,
    Back,
    Quit,
}

pub fn run(data: &mut Data) {
    let mut clth_menu = Menu::new("Clothes");
    clth_menu.add_action(Act::new("Add clothing", Event::AddClth));
    clth_menu.add_action(Act::new("Remove clothing", Event::RemoveClth));
    clth_menu.add_action(Act::new("Update clothing", Event::UpdateClth));
    clth_menu.add_action(Act::new("List clothes", Event::ListClths));
    clth_menu.add_action(Act::new("Back", Event::Back));

    let mut outfit_menu = Menu::new("Outfits");
    outfit_menu.add_action(Act::new("Add outfit", Event::AddOutfit));
    outfit_menu.add_action(Act::new("Remove outfit", Event::RemoveOutfit));
    outfit_menu.add_action(Act::new("List outfits", Event::ListOutfits));
    outfit_menu.add_action(Act::new("Back", Event::Back));

    let mut menu = Menu::new("root");
    menu.add_submenu(clth_menu);
    menu.add_submenu(outfit_menu);
    menu.add_action(Act::new("Quit", Event::Quit));

    let mut runner = Runner::new(menu);

    loop {
        if let Some(act) = runner.run("> ") {
            match act {
                Event::AddClth => user_add_clth(data),
                Event::RemoveClth => {
                    user_rm_clth(data);
                    data.outfits.clean();
                },
                Event::ListClths => println!("{}\n", &data.clothes),
                Event::UpdateClth => user_update_clth(data),
                Event::AddOutfit => user_add_outfit(data),
                Event::RemoveOutfit => user_rm_outfit(data),
                Event::ListOutfits => println!("{}\n", &data.outfits),
                Event::Back => runner.back().unwrap(),
                Event::Quit => break,
            }
        }
    }
}
