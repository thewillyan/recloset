use std::rc::Rc;
use std::io::{ self, Write };

pub struct Menu<T: Clone> {
    label: String,
    items: Vec<Item<T>>
}

impl<T: Clone> Menu<T> {
    pub fn new(label: &str) -> Menu<T> {
        Menu { label: String::from(label), items: Vec::new() }
    }

    pub fn add_submenu(&mut self, menu: Menu<T>) {
        self.items.push(Item::Submenu(Rc::new(menu)));
    }

    pub fn add_action(&mut self, act: Act<T>) {
        self.items.push(Item::Action(act));
    }

    pub fn select(&self, id: usize) -> Result<&Item<T>, &'static str> {
        self.items.get(id).ok_or("Invalid option!")
    }

    pub fn display(&self) {
        self.items.iter().enumerate().for_each(|(index, item)| {
            println!("[{}] {}", index, item.label());
        })
    }
}

pub struct Act<T: Clone> {
    label: String,
    value: T
}

impl<T: Clone> Act<T> {
    pub fn new(label: &str, value: T) -> Act<T> {
        Act { label: String::from(label), value }
    }
}

pub enum Item<T: Clone> {
    Submenu(Rc<Menu<T>>),
    Action(Act<T>)
}

impl<T: Clone> Item<T> {
    pub fn label(&self) -> &str {
        match self {
            Self::Submenu(sub) => &sub.label,
            Self::Action(act) => &act.label
        }
    }
}

pub struct Runner<T: Clone> {
    path: Vec<Rc<Menu<T>>>
}

impl<T: Clone> Runner<T> {
    pub fn new(menu: Menu<T>) -> Runner<T> {
        Runner { path: vec![Rc::new(menu)] }
    }

    pub fn req_sel<'a>(
        menu: &'a Menu<T>,
        prompt: &str
    ) -> Result<&'a Item<T>, &'static str> 
    {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            return Err("Failed to read the input!");
        }

        let menu_id: usize = match input.trim().parse() {
            Ok(id) => id,
            Err(_) => return Err("Invalid number!")
        };

        menu.select(menu_id)
    }

    pub fn select(&mut self, menu: Rc<Menu<T>>) {
        self.path.push(menu);
    }

    pub fn back(&mut self) -> Result<(), &'static str> {
        if self.path.len() == 1 {
            Err("Can't back from the root menu!")
        } else {
            self.path.pop();
            Ok(())
        }
    }

    pub fn run(&mut self, prompt: &str) -> Option<T> {
        let curr_menu = Rc::clone(self.path.last().unwrap());
        curr_menu.display();

        let selected = loop {
            match Self::req_sel(&curr_menu, prompt) {
                Ok(item) => break item,
                Err(msg) => println!("Error: {}", msg)
            }
        };

        match selected {
            Item::Action(act) => Some(act.value.clone()),
            Item::Submenu(sub) => {
                self.select(Rc::clone(sub));
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_items() {
        let mut sub = Menu::new("Submenu");
        sub.add_action(Act::new("Back", 1));

        let mut root = Menu::new("root");
        root.add_submenu(sub);
        root.add_action(Act::new("Quit", 0));

        assert!(root.items.len() == 2);
    }

    #[test]
    fn reject_empty_path() {
        let mut root = Menu::new("root");
        root.add_action(Act::new("Quit", 0));

        let mut runner = Runner::new(root);
        assert!(runner.back().is_err());
    }
}
