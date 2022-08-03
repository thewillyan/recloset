use chrono::NaiveDate;
use std::fmt;
use std::rc::{Rc, Weak};

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
    ) -> Clth
    {
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
        if let (Kind::Chest, Kind::Leg, Kind::Foot) = (
            &upper.upgrade().unwrap().kind,
            &lower.upgrade().unwrap().kind,
            &foot.upgrade().unwrap().kind,
        ) {
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

#[derive(Debug, Clone)]
pub enum Kind {
    Chest,
    Leg,
    Foot,
}

#[derive(Debug, Clone)]
pub enum Sex {
    Male,
    Female,
    Unissex,
}

#[derive(Debug, Clone)]
pub enum Size {
    XS,
    S,
    M,
    L,
    XL,
}

#[derive(Debug, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb(red, green, blue)
    }

    pub fn try_from_hex(hex: &str) -> Option<Rgb> {
        if hex.len() != 6 { return None }

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

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use std::rc::Rc;

    #[test]
    fn reject_invalid_clthset() {
        let style = Rc::new(Style::new("style"));
        let clth = Rc::new(Clth::new(
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
            Rc::downgrade(&clth),
            Rc::downgrade(&clth),
            Rc::downgrade(&clth)
        )
        .is_err());
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
