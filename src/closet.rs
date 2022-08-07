use chrono::NaiveDate;
use std::fmt;
use std::rc::{Rc, Weak};
use std::collections::HashMap;

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
}

impl fmt::Display for Clth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields = Vec::with_capacity(8);
        fields.push(format!("Id: {}", self.id));
        fields.push(format!("Kind: {:?}", self.kind));
        fields.push(format!("Sex: {:?}", self.sex));
        fields.push(format!("Size: {:?}", self.size));
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

#[derive(Debug, Clone, PartialEq)]
pub enum Sex {
    Male,
    Female,
    Unissex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    XS,
    S,
    M,
    L,
    XL,
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
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Sale(u64),
    Donation,
    Keep,
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
    pub count: u32,
}

impl Style {
    pub fn new(name: &str) -> Style {
        Style {
            name: String::from(name),
            count: 0,
        }
    }
}

pub struct Clothes {
    pub list: Vec<Rc<Clth>>,
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

    pub fn from_rcs(list: Vec<Rc<Clth>>) -> Clothes {
        Clothes { list }
    }

    pub fn add(&mut self, clth: Clth) {
        if let Some(_) = self.get(clth.id) {
            panic!("A clothing with the same id already exists in 'Clothes'");
        }
        self.list.push(Rc::new(clth));
    }

    pub fn request_id(&self) -> u32 {
        let ids: Vec<u32> = self.list.iter().map(|clth| clth.id).collect();
        let mut new_id = 0;
        while ids.contains(&new_id) {
            new_id += 1
        }
        new_id
    }

    pub fn get(&self, id: u32) -> Option<&Rc<Clth>> {
        for clth in self.list.iter() {
            if clth.id == id {
                return Some(clth);
            }
        }
        None
    }

    pub fn filter_by_kind(&self, kind: Kind) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.kind == kind {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Self::from_rcs(filtered)
    }

    pub fn filter_by_sex(&self, sex: Sex) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.sex == sex {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Self::from_rcs(filtered)
    }

    pub fn filter_by_size(&self, size: Size) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.size == size {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Self::from_rcs(filtered)
    }

    pub fn filter_by_color(&self, color: Rgb) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.color == color {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Self::from_rcs(filtered)
    }

    pub fn filter_by_style(&self, name: &str) -> Clothes {
        let filtered = self
            .list
            .iter()
            .fold(Vec::new(), |mut acc, clth| {
                if clth.style.name == name {
                    acc.push(Rc::clone(clth));
                }
                acc
            });
        Self::from_rcs(filtered)
    }

    pub fn map_by_target(&self) -> HashMap<&str, Clothes> {
        let mut keep = Vec::new();
        let mut donation = Vec::new();
        let mut sale = Vec::new();

        for clth in self.list.iter() {
            match clth.target {
                Target::Keep => keep.push(Rc::clone(clth)),
                Target::Donation => donation.push(Rc::clone(clth)),
                Target::Sale(_) => sale.push(Rc::clone(clth)),
            }
        }

        let mut map = HashMap::new();
        map.insert("keep", Self::from_rcs(keep));
        map.insert("donation", Self::from_rcs(donation));
        map.insert("sale", Self::from_rcs(sale));
        map
    }

    pub fn map_by_kind(&self) -> HashMap<&str, Clothes> {
        let mut chest = Vec::new();
        let mut legs = Vec::new();
        let mut foot = Vec::new();

        for clth in self.list.iter() {
            match clth.kind {
                Kind::Chest => chest.push(Rc::clone(clth)),
                Kind::Leg => legs.push(Rc::clone(clth)),
                Kind::Foot => foot.push(Rc::clone(clth)),
            }
        }

        let mut map = HashMap::new();
        map.insert("chest", Self::from_rcs(chest));
        map.insert("leg", Self::from_rcs(legs));
        map.insert("foot", Self::from_rcs(foot));
        map
    }

}

impl fmt::Display for Clothes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clothes = if self.list.is_empty() {
            String::from("No clothes to display!")
        } else {
            self.list
                .iter()
                .map(|clth| clth.to_string())
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

pub struct ClthSet {
    upper: Weak<Clth>,
    lower: Weak<Clth>,
    foot: Weak<Clth>,
}

impl ClthSet {
    pub fn new(
        upper: Weak<Clth>,
        lower: Weak<Clth>,
        foot: Weak<Clth>,
    ) -> Result<ClthSet, &'static str> {
        let Clth {
            style: up_stl,
            kind: up_kind,
            ..
        } = &*upper.upgrade().unwrap();

        let Clth {
            style: low_stl,
            kind: low_kind,
            ..
        } = &*lower.upgrade().unwrap();

        let Clth {
            style: foot_stl,
            kind: foot_kind,
            ..
        } = &*foot.upgrade().unwrap();

        if !(up_stl.name == low_stl.name && low_stl.name == foot_stl.name) {
            return Err("The clothes of a clothing set must have the same style.");
        }

        if let (Kind::Chest, Kind::Leg, Kind::Foot) = (&up_kind, &low_kind, &foot_kind) {
            Ok(ClthSet { upper, lower, foot })
        } else {
            Err("Invalid clothing set!")
        }
    }

    pub fn upper(&self) -> Weak<Clth> {
        Weak::clone(&self.upper)
    }

    pub fn lower(&self) -> Weak<Clth> {
        Weak::clone(&self.lower)
    }

    pub fn foot(&self) -> Weak<Clth> {
        Weak::clone(&self.foot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use std::rc::Rc;

    #[test]
    fn reject_invalid_clthset() {
        let clth1 = Rc::new(Clth::new(
            0,
            Kind::Chest,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::new(Style::new("style1")),
        ));

        let clth2 = Rc::new(Clth::new(
            0,
            Kind::Leg,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::new(Style::new("style2")),
        ));

        let clth3 = Rc::new(Clth::new(
            0,
            Kind::Foot,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::new(Style::new("style3")),
        ));

        let set1 = ClthSet::new(
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth1),
        );

        let set2 = ClthSet::new(
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth2),
            Rc::downgrade(&clth3),
        );

        assert!(set1.is_err() && set2.is_err());
    }

    #[test]
    fn accept_valid_clthset() {
        let style = Rc::new(Style::new("style"));
        let clth1 = Rc::new(Clth::new(
            0,
            Kind::Chest,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::clone(&style),
        ));

        let clth2 = Rc::new(Clth::new(
            0,
            Kind::Leg,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            Rc::clone(&style),
        ));

        let clth3 = Rc::new(Clth::new(
            0,
            Kind::Foot,
            Sex::Male,
            Size::M,
            Rgb(0, 0, 0),
            Target::Keep,
            Local::today().naive_local(),
            style,
        ));

        assert!(ClthSet::new(
            Rc::downgrade(&clth1),
            Rc::downgrade(&clth2),
            Rc::downgrade(&clth3)
        )
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
}