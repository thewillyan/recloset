use chrono::NaiveDate;
use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

pub type ErrMsg = &'static str;

pub struct Clth {
    pub id: u32,
    pub kind: Kind,
    pub sex: Sex,
    pub size: Size,
    pub color: Rgb,
    pub target: Target,
    pub purchase_date: NaiveDate,
    pub style: Rc<Style>,
}

impl Clth {
    pub fn new(
        id: u32,
        kind: Kind,
        sex: Sex,
        size: Size,
        color: Rgb,
        target: Target,
        purchase_date: NaiveDate,
        style: Rc<Style>,
    ) -> Clth {
        Clth {
            id,
            kind,
            sex,
            size,
            color,
            target,
            purchase_date,
            style,
        }
    }

    pub fn to_toml(&self) -> String {
        let mut result = String::from("[clth]\n");
        result.push_str(&format!("id = {}\n", self.id));
        result.push_str(&format!("kind = \"{}\"\n", self.kind));
        result.push_str(&format!("sex = \"{}\"\n", self.sex));
        result.push_str(&format!("size = \"{}\"\n", self.size));
        result.push_str(&format!("color = \"{}\"\n", self.color.to_hex()));
        result.push_str(&format!("target = \"{}\"\n", self.target));
        result.push_str(&format!("purchase_date = \"{}\"\n", self.purchase_date));
        result.push_str(&format!("style = \"{}\"", self.style.name));
        result
    }
}

impl fmt::Display for Clth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields = Vec::with_capacity(8);
        fields.push(format!("Id: {}", self.id));
        fields.push(format!("Kind: {}", self.kind));
        fields.push(format!("Sex: {}", self.sex));
        fields.push(format!("Size: {}", self.size));
        fields.push(format!("Color: {}", self.color));
        fields.push(format!("Target: {}", self.target));
        fields.push(format!("Purchase date: {}", self.purchase_date));
        fields.push(format!("Style: {}", &self.style.name));
        write!(f, "{}", fields.join("\n"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Chest,
    Leg,
    Foot,
}

impl Kind {
    pub fn from_str(value: &str) -> Result<Kind, ErrMsg> {
        let kind = match value.to_lowercase().as_str() {
            "chest" => Kind::Chest,
            "leg" => Kind::Leg,
            "foot" => Kind::Foot,
            _ => return Err("Invalid kind.")
        };
        Ok(kind)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sex {
    Male,
    Female,
    Unissex,
}

impl Sex {
    pub fn from_str(value: &str) -> Result<Sex, ErrMsg> {
        let sex = match value.to_lowercase().as_str() {
            "male" => Sex::Male,
            "female" => Sex::Female,
            "unissex" => Sex::Unissex,
            _ => return Err("Invalid sex.")
        };
        Ok(sex)
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    XS,
    S,
    M,
    L,
    XL,
}

impl Size {
    pub fn from_str(value: &str) -> Result<Size, ErrMsg> {
        let size = match value.to_lowercase().as_str() {
            "xs" => Size::XS,
            "s" => Size::S,
            "m" => Size::M,
            "l" => Size::L,
            "xl" => Size::XL,
            _ => return Err("Invalid size.")
        };
        Ok(size)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb(red, green, blue)
    }

    pub fn try_from_hex(hex: &str) -> Option<Rgb> {
        if hex.len() != 6 {
            return None;
        }

        let mut bytes = Vec::with_capacity(3);
        for i in (0..6).step_by(2) {
            match u8::from_str_radix(&hex[i..i + 2], 16) {
                Ok(value) => bytes.push(value),
                _ => return None,
            };
        }

        Some(Rgb::new(bytes[0], bytes[1], bytes[2]))
    }
    
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.to_hex())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Sale(u64),
    Donation,
    Keep,
}

impl Target {
    pub fn from_str(value: &str) -> Result<Target, ErrMsg> {
        let value = value.to_lowercase();
        let target = match value.split_once('$') {
            Some(("sale for ", price)) => {
                let price: f64 = match price.parse(){
                    Ok(num) => num,
                    Err(_) => return Err("Invalid price."),
                };
                let cents = (price * 100.0).trunc() as u64;
                Target::Sale(cents)
            },
            None if value == "donation" => Target::Donation,
            None if value == "keep" => Target::Keep,
            _ => return Err("Invalid target.")
        };
        Ok(target)
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::Sale(price) = self {
            write!(f, "Sale for ${:.2}", *price as f64 / 100.0)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

pub struct Style {
    pub name: String,
}

impl Style {
    pub fn new(name: &str) -> Style {
        Style {
            name: String::from(name)
        }
    }
}

pub struct Clothes {
    pub list: Vec<Rc<RefCell<Clth>>>,
}

impl Clothes {
    pub fn new() -> Clothes {
        Clothes { list: Vec::new() }
    }

    pub fn from(clothes: Vec<Clth>) -> Clothes {
        clothes.into_iter().fold(Clothes::new(), |mut acc, clth| {
            acc.add(clth);
            acc
        })
    }

    pub fn add(&mut self, clth: Clth) -> Option<ErrMsg> {
        if let Some(_) = self.get(clth.id) {
            return Some("A clothing with the same id already exists in 'Clothes'");
        }
        self.list.push(Rc::new(RefCell::new(clth)));
        None
    }

    pub fn remove(&mut self, id: u32) -> Result<Rc<RefCell<Clth>>, &'static str> {
        let index = match self.list
            .iter()
            .map(|el| el.borrow().id)
            .collect::<Vec<_>>()
            .binary_search(&id) {
                Ok(index) => index,
                Err(_) => return Err("Clothing not found."),
            };

        Ok(self.list.swap_remove(index))
    }

    pub fn request_id(&self) -> u32 {
        let ids: Vec<u32> = self.list.iter().map(|clth| clth.borrow().id).collect();
        let mut new_id = 0;
        while ids.contains(&new_id) {
            new_id += 1
        }
        new_id
    }

    pub fn get(&self, id: u32) -> Option<&Rc<RefCell<Clth>>> {
        for clth in self.list.iter() {
            if clth.borrow().id == id {
                return Some(clth);
            }
        }
        None
    }

    pub fn filter_by_color(&self, color: Rgb) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.borrow().color == color {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Clothes { list: filtered }
    }

    pub fn filter_by_style(&self, name: &str) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.borrow().style.name == name {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Clothes { list: filtered }
    }

    pub fn map_by_target(&self) -> HashMap<&str, Clothes> {
        let mut keep = Vec::new();
        let mut donation = Vec::new();
        let mut sale = Vec::new();

        for clth in self.list.iter() {
            match clth.borrow().target {
                Target::Keep => keep.push(Rc::clone(clth)),
                Target::Donation => donation.push(Rc::clone(clth)),
                Target::Sale(_) => sale.push(Rc::clone(clth)),
            }
        }

        let mut map = HashMap::new();
        map.insert("keep", Clothes { list: keep });
        map.insert("donation", Clothes { list: donation });
        map.insert("sale", Clothes { list: sale});
        map
    }

    pub fn map_by_kind(&self) -> HashMap<&str, Clothes> {
        let mut chest = Vec::new();
        let mut legs = Vec::new();
        let mut foot = Vec::new();

        for clth in self.list.iter() {
            match clth.borrow().kind {
                Kind::Chest => chest.push(Rc::clone(clth)),
                Kind::Leg => legs.push(Rc::clone(clth)),
                Kind::Foot => foot.push(Rc::clone(clth)),
            }
        }

        let mut map = HashMap::new();
        map.insert("chest", Clothes { list: chest });
        map.insert("leg", Clothes { list: legs });
        map.insert("foot", Clothes { list: foot });
        map
    }

    pub fn to_toml(&self) -> String {
        self.list
            .iter()
            .map(|clth| clth.borrow().to_toml() )
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl fmt::Display for Clothes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clothes = if self.list.is_empty() {
            String::from("No clothes to display!")
        } else {
            self.list
                .iter()
                .map(|clth| clth.borrow().to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
        };
        write!(f, "{}", clothes)
    }
}

pub struct Styles {
    list: Vec<Rc<Style>>,
}

impl Styles {
    pub fn new() -> Styles {
        Styles { list: Vec::new() }
    }

    pub fn add(&mut self, style: Style) {
        self.list.push(Rc::new(style));
    }

    pub fn get(&self, name: &str) -> Option<&Rc<Style>> {
        for style in self.list.iter() {
            if &style.name == name {
                return Some(style);
            }
        }
        None
    }

    pub fn get_or_add(&mut self, name: &str) -> Rc<Style> {
        let stl = match self.get(name) {
            Some(value) => value,
            None => {
                self.add(Style::new(name));
                self.list.last().unwrap()
            }
        };
        Rc::clone(stl)
    }
}

pub struct Outfit {
    pub id: u32,
    pub chest: Weak<RefCell<Clth>>,
    pub leg: Weak<RefCell<Clth>>,
    pub foot: Weak<RefCell<Clth>>,
}

impl Outfit {
    pub fn new(
        id: u32,
        chest: Weak<RefCell<Clth>>,
        leg: Weak<RefCell<Clth>>,
        foot: Weak<RefCell<Clth>>,
    ) -> Result<Outfit, &'static str> {
        let up = chest.upgrade().unwrap();
        let up_stl = &up.borrow().style.name;
        let up_kind = &up.borrow().kind;

        let low = leg.upgrade().unwrap();
        let low_stl = &low.borrow().style.name;
        let low_kind = &low.borrow().kind;

        let ft = foot.upgrade().unwrap();
        let foot_stl = &ft.borrow().style.name;
        let foot_kind = &ft.borrow().kind;

        if !(up_stl == low_stl && low_stl == foot_stl) {
            return Err("The clothes of a clothing set must have the same style.");
        }

        if let (Kind::Chest, Kind::Leg, Kind::Foot) = (&up_kind, &low_kind, &foot_kind) {
            Ok(Outfit { id, chest, leg, foot })
        } else {
            Err("Invalid clothing set!")
        }
    }

    pub fn to_clothes(&self) -> Clothes {
        let list = vec![
            self.chest.upgrade().unwrap(),
            self.leg.upgrade().unwrap(),
            self.foot.upgrade().unwrap(),
        ];
        Clothes { list }
    }

    pub fn to_id_arr(&self) -> [u32; 3] {
        [
            self.chest.upgrade().unwrap().borrow().id,
            self.leg.upgrade().unwrap().borrow().id,
            self.foot.upgrade().unwrap().borrow().id,
        ]
    }

    pub fn is_valid(&self) -> bool {
        self.chest.upgrade().is_some() && self.leg.upgrade().is_some() &&
            self.foot.upgrade().is_some()
    }

    pub fn to_toml(&self) -> String {
        let [chest, leg, foot] = self.to_id_arr();
        let mut result = String::from("[outfit]\n");
        result.push_str(&format!("chest = {}\n", chest));
        result.push_str(&format!("leg = {}\n", leg));
        result.push_str(&format!("foot = {}", foot));
        result
    }
}

impl fmt::Display for Outfit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = format!("[ Outfit {} ]", self.id);

        let body = [
            self.chest.upgrade().unwrap().borrow().to_string(),
            self.leg.upgrade().unwrap().borrow().to_string(),
            self.foot.upgrade().unwrap().borrow().to_string(),
        ].join("\n\n");

        write!(f, "{}\n{}", title, body)
    }
}


pub struct Outfits {
    pub list: Vec<Outfit>
}

impl Outfits {
    pub fn new() -> Outfits {
        Outfits { list: Vec::new() }
    }

    pub fn add(&mut self, outfit: Outfit) -> Option<ErrMsg> {
        if self.to_id_matrix().contains(&outfit.to_id_arr()) {
            return Some("This outfit already exists!");
        }
        self.list.push(outfit);
        None
    }

    pub fn remove(&mut self, id: u32) -> Result<Outfit, &'static str> {
        let index = match self.list
            .iter()
            .map(|el| el.id)
            .collect::<Vec<_>>()
            .binary_search(&id) {
                Ok(index) => index,
                Err(_) => return Err("Clothing not found."),
            };

        Ok(self.list.swap_remove(index))
    }

    pub fn to_id_matrix(&self) -> Vec<[u32; 3]> {
        self.list
            .iter()
            .map(|set| set.to_id_arr())
            .collect::<Vec<_>>()
    }

    pub fn get(&self, id: u32) -> Option<&Outfit> {
        for outfit in self.list.iter() {
            if outfit.id == id {
                return Some(outfit);
            }
        }
        None
    }

    pub fn request_id(&self) -> u32 {
        let ids: Vec<u32> = self.list.iter().map(|outfit| outfit.id).collect();
        let mut new_id = 0;
        while ids.contains(&new_id) {
            new_id += 1
        }
        new_id
    }

    pub fn clean(&mut self) {
        let black_list: Vec<_> = self.list
            .iter()
            .filter(|el| !el.is_valid())
            .map(|el| el.id)
            .collect();

        black_list.iter().for_each(|id| { self.remove(*id).unwrap(); });
    }
    
    pub fn to_toml(&self) -> String {
        self.list
            .iter()
            .map(|item| item.to_toml())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl fmt::Display for Outfits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.list.is_empty() {
            write!(f, "No outfits to display!")
        } else {
            write!(f, "{}", self.list
                .iter()
                .map(|set| set.to_string())
                .collect::<Vec<_>>()
                .join("\n"))
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

pub struct OutfitBuffer {
    pub chest: Option<Weak<RefCell<Clth>>>,
    pub leg: Option<Weak<RefCell<Clth>>>,
    pub foot: Option<Weak<RefCell<Clth>>>
}

impl OutfitBuffer {
    pub fn new() -> OutfitBuffer {
        OutfitBuffer {
            chest: None,
            leg: None,
            foot: None
        }
    }

    pub fn to_outfit(self, id: u32) -> Result<Outfit, ErrMsg> {
        let chest = match self.chest {
            Some(value) => value,
            None => return Err("Missing 'chest' field on buffer.")
        };

        let leg = match self.leg {
            Some(value) => value,
            None => return Err("Missing 'leg' field on buffer.")
        };

        let foot = match self.foot {
            Some(value) => value,
            None => return Err("Missing 'foot' field on buffer.")
        };
        Outfit::new(id, chest, leg, foot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use std::rc::Rc;

    #[test]
    fn reject_invalid_clthset() {
        let clth1 = Rc::new(RefCell::new(Clth::new(
            0,
            Kind::Chest,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::new(Style::new("style1")),
        )));

        let clth2 = Rc::new(RefCell::new(Clth::new(
            0,
            Kind::Leg,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::new(Style::new("style2")),
        )));

        let clth3 = Rc::new(RefCell::new(Clth::new(
            0,
            Kind::Foot,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::new(Style::new("style3")),
        )));

        let set1 = Outfit::new(
            0,
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth1),
        );

        let set2 = Outfit::new(
            1,
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth2),
            Rc::downgrade(&clth3),
        );

        assert!(set1.is_err() && set2.is_err());
    }

    #[test]
    fn accept_valid_clthset() {
        let style = Rc::new(Style::new("style"));
        let clth1 = Rc::new(RefCell::new(Clth::new(
            0,
            Kind::Chest,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::clone(&style),
        )));

        let clth2 = Rc::new(RefCell::new(Clth::new(
            0,
            Kind::Leg,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::clone(&style),
        )));

        let clth3 = Rc::new(RefCell::new(Clth::new(
            0,
            Kind::Foot,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            style,
        )));

        assert!(Outfit::new(
                0,
                Rc::downgrade(&clth1),
                Rc::downgrade(&clth2),
                Rc::downgrade(&clth3))
        .is_ok());
    }

    #[test]
    pub fn create_color_by_hex() {
        let color = Rgb::try_from_hex("FFFFFF").unwrap();
        assert_eq!(color.0, 255);
        assert_eq!(color.1, 255);
        assert_eq!(color.2, 255);
    }

    #[test]
    pub fn reject_invalid_color() {
        assert!(Rgb::try_from_hex("FFFFFZ").is_none());
    }

    #[test]
    pub fn color_to_hex() {
        assert_eq!("FFFFFF", Rgb(255,255,255).to_hex());
    }

    #[test]
    pub fn kind_from_str() {
        assert!(
            matches!(Kind::from_str("Chest").unwrap(), Kind::Chest)
        );
        assert!(
            matches!(Kind::from_str("Leg").unwrap(), Kind::Leg)
        );
        assert!(
            matches!(Kind::from_str("Foot").unwrap(), Kind::Foot)
        );
    }

    #[test]
    pub fn sex_from_str() {
        assert!(
            matches!(Sex::from_str("Male").unwrap(), Sex::Male)
        );
        assert!(
            matches!(Sex::from_str("Female").unwrap(), Sex::Female)
        );
        assert!(
            matches!(Sex::from_str("Unissex").unwrap(), Sex::Unissex)
        );
    }

    #[test]
    pub fn size_from_str() {
        assert!(Size::from_str("invalid").is_err());
        assert!(
            matches!(Size::from_str("xs").unwrap(), Size::XS)
        );
        assert!(
            matches!(Size::from_str("s").unwrap(), Size::S)
        );
        assert!(
            matches!(Size::from_str("m").unwrap(), Size::M)
        );
        assert!(
            matches!(Size::from_str("l").unwrap(), Size::L)
        );
        assert!(
            matches!(Size::from_str("xl").unwrap(), Size::XL)
        );
    }

    #[test]
    pub fn target_from_str() {
        assert!(Target::from_str("invalid").is_err());
        assert!(
            matches!(Target::from_str("Sale for $10.00").unwrap(), Target::Sale(1000))
        );
        assert!(
            matches!(Target::from_str("Donation").unwrap(), Target::Donation) 
        );
        assert!(
            matches!(Target::from_str("Keep").unwrap(), Target::Keep) 
        );
    }
}
