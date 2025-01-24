use core::num;
use std::{collections::HashMap, default};

fn get_tabs(number: u32) -> String {
    let mut tab = String::new();
    for _ in 0..number {
        tab.push('\t');
    };
    tab
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuType {
    Lable,
    Button,
    RadioButton,
    Selection,
    TextBox,
}

#[derive(Clone, Debug)]
pub enum MenuItem {
    Lable(Lable),
    Button(Button),
    RadioButton(RadioButton),
    Selection(Selection),
    TextBox(TextBox),
}

pub trait MenuObject {
    fn get_text(&self) -> &str;
    fn get_type(&self) -> &MenuType;
}

pub trait Value<T, C> {
    fn get(&self, name: &str) -> Option<&T>;
    fn change(&mut self, name: &str, value: C) -> Option<()>;
    fn get_options(&self) -> &[String];
}

pub trait FormattedString {
    fn format(&self, number_of_tab: u32) -> String;
}

#[derive(Clone, Debug)]
pub struct Lable {
    text: String,
    menu_type: MenuType,
}

impl Lable {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            menu_type: MenuType::Lable,
        }
    }
}

impl MenuObject for Lable {
    fn get_text(&self) -> &str {
        &self.text[..]
    }

    fn get_type(&self) -> &MenuType {
        &self.menu_type
    }
}

impl FormattedString for Lable {
    fn format(&self, _number_of_tab: u32) -> String {
        self.get_text().to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    text: String,
    options_text: Vec<String>,
    options: HashMap<String, bool>,
    menu_type: MenuType,
}

impl Button {
    pub fn new(text: &str, button_text: &str) -> Self {
        let mut options = HashMap::new();
        options.insert(button_text.to_string(), false);

        Self {
            text: text.to_string(),
            options_text: vec![button_text.to_string()],
            options,
            menu_type: MenuType::Button,
        }
    }
}

impl Value<bool, bool> for Button {
    fn get(&self, name: &str) -> Option<&bool> {
        self.options.get(name)
    }

    fn change(&mut self, name: &str, value: bool) -> Option<()> {
        match self.options.get_mut(name) {
            Some(v) => {
                *v = value;
                Some(())
            }
            None => None,
        }
    }

    fn get_options(&self) -> &[String] {
        &self.options_text
    }
}

impl MenuObject for Button {
    fn get_text(&self) -> &str {
        &self.text[..]
    }

    fn get_type(&self) -> &MenuType {
        &self.menu_type
    }
}

impl FormattedString for Button {
    fn format(&self, number_of_tab: u32) -> String {
        let mut output = String::new();

        output += self.get_text();
        output += &get_tabs(number_of_tab);

        for name in self.get_options() {
            if *self.get(name).unwrap() {
                output += &format!("[{}] ", name);
            } else {
                output += &format!(" {}  ", name);
            }
        }
        output
    }
}

#[derive(Clone, Debug)]
pub struct RadioButton {
    text: String,
    options_text: Vec<String>,
    options: HashMap<String, bool>,
    menu_type: MenuType,
}

impl RadioButton {
    pub fn new(text: &str, options_text: &[String]) -> Self {
        let mut options = HashMap::new();
        for text in options_text {
            options.insert(text.clone(), false);
        }
        let options_text = options_text.to_vec();

        Self {
            text: text.to_string(),
            options_text,
            options,
            menu_type: MenuType::RadioButton,
        }
    }
}

impl MenuObject for RadioButton {
    fn get_text(&self) -> &str {
        &self.text[..]
    }

    fn get_type(&self) -> &MenuType {
        &self.menu_type
    }
}

impl Value<bool, bool> for RadioButton {
    fn get(&self, name: &str) -> Option<&bool> {
        self.options.get(name)
    }

    fn change(&mut self, name: &str, value: bool) -> Option<()> {
        match self.options.get_mut(name) {
            Some(v) => {
                *v = value;
                Some(())
            }
            None => None,
        }
    }

    fn get_options(&self) -> &[String] {
        &self.options_text
    }
}

impl FormattedString for RadioButton {
    fn format(&self, number_of_tab: u32) -> String {
        let mut output = String::new();

        output += self.get_text();
        output += &get_tabs(number_of_tab);

        for name in self.get_options() {
            if *self.get(name).unwrap() {
                output += &format!("[{}] ", name);
            } else {
                output += &format!(" {}  ", name);
            }
        }
        output
    }
}

#[derive(Clone, Debug)]
pub struct Selection {
    text: String,
    options_text: Vec<String>,
    options: HashMap<String, bool>,
    menu_type: MenuType,
}

impl Selection {
    pub fn new(text: &str, options_text: &[String]) -> Self {
        let mut options = HashMap::new();
        for text in options_text {
            options.insert(text.clone(), false);
        }
        let options_text = options_text.to_vec();

        Self {
            text: text.to_string(),
            options_text,
            options,
            menu_type: MenuType::Selection,
        }
    }
}

impl MenuObject for Selection {
    fn get_text(&self) -> &str {
        &self.text[..]
    }

    fn get_type(&self) -> &MenuType {
        &self.menu_type
    }
}

impl Value<bool, bool> for Selection {
    fn get(&self, name: &str) -> Option<&bool>{
        self.options.get(name)
    }

    fn change(&mut self, name: &str, value: bool) -> Option<()> {
        match self.options.get_mut(name) {
            Some(v) => {
                *v = value;
                Some(())
            }
            None => None,
        }
    }

    fn get_options(&self) -> &[String] {
        &self.options_text
    }
}

impl FormattedString for Selection {
    fn format(&self, number_of_tab: u32) -> String {
        let mut output = String::new();

        output += self.get_text();
        output += &get_tabs(number_of_tab);

        for name in self.get_options() {
            if *self.get(name).unwrap() {
                output += &format!("[{}] ", name);
            } else {
                output += &format!(" {}  ", name);
            }
        }
        output
    }
}

#[derive(Clone, Debug)]
pub struct TextBox {
    text: String,
    input: String,
    options_text: Vec<String>,
    default_input: String,
    menu_type: MenuType,
}

impl TextBox {
    pub fn new(text: &str, option_text: &str, default_input: &str) -> Self {
        Self {
            text: text.to_string(),
            input: String::new(),
            options_text: vec![option_text.to_string()],
            default_input: default_input.to_string(),
            menu_type: MenuType::TextBox,
        }
    }
}

impl MenuObject for TextBox {
    fn get_text(&self) -> &str {
        &self.text[..]
    }

    fn get_type(&self) -> &MenuType {
        &self.menu_type
    }
}

impl Value<String, &str> for TextBox {
    fn get(&self, name: &str) -> Option<&String> {
        if name == self.options_text[0] {
            if self.input.len() == 0 {
                Some(&self.default_input)
            } else {
                Some(&self.input)
            }
        } else {
            None
        }
    }

    fn change(&mut self, name: &str, value: &str) -> Option<()> {
        self.input = value.to_string();
        Some(())
    }

    fn get_options(&self) -> &[String] {
        &self.options_text
    }
}

impl FormattedString for TextBox {
    fn format(&self, number_of_tab: u32) -> String {
        let input = if self.input.len() == 0 {
            &self.default_input
        } else {
            &self.input
        };
        format!("{}{}{}", self.get_text(), get_tabs(number_of_tab), input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const NUMBER_OF_TAB: u32 = 5;

    #[test]
    fn lable_test() {
        let text = "I am a Lable";
        let lable = Lable::new(text);
        assert_eq!(lable.get_type(), &MenuType::Lable);
        assert_eq!(lable.get_text(), text);

        assert_eq!(lable.format(NUMBER_OF_TAB), text);
    }

    #[test]
    fn button_test() {
        let text = "I am a Button";
        let button_text = "Enter";
        let mut button = Button::new(text, button_text);

        assert_eq!(button.get_type(), &MenuType::Button);
        assert_eq!(button.get_text(), text);

        assert_eq!(button.get(button_text).unwrap(), &false);
        button.change(button_text, true).unwrap();
        assert_eq!(button.get(button_text).unwrap(), &true);

        assert_eq!(button.format(NUMBER_OF_TAB), "I am a Button:\t\t\t\t\t[ Enter ]".to_string())
    }

    #[test]
    fn selection_text() {
        let text = "I am a Selection";
        let options_text = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut selection = Selection::new(text, &options_text);

        assert_eq!(selection.get_type(), &MenuType::Selection);
        assert_eq!(selection.get_text(), text);

        assert_eq!(selection.format(NUMBER_OF_TAB), "I am a Selection:\t\t\t\t\t  a  ,   b  ,   c  ");

        assert_eq!(selection.get("a"), Some(&false));
        assert_eq!(selection.get("b"), Some(&false));
        assert_eq!(selection.get("c"), Some(&false));

        selection.change("a", true).unwrap();
        selection.change("b", true).unwrap();
        selection.change("c", true).unwrap();

        assert_eq!(selection.get("a"), Some(&true));
        assert_eq!(selection.get("b"), Some(&true));
        assert_eq!(selection.get("c"), Some(&true));

        assert_eq!(selection.format(NUMBER_OF_TAB), "I am a Selection:\t\t\t\t\t[ a ] [ b ] [ c ]");


        assert_eq!(selection.get("d"), None);
    }

    #[test]
    fn radio_button_test() {
        let text = "I am a Selection";
        let options_text = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut radio = RadioButton::new(text, &options_text);

        assert_eq!(radio.get_type(), &MenuType::RadioButton);
        assert_eq!(radio.get_text(), text);

        assert_eq!(radio.get("a"), Some(&false));
        assert_eq!(radio.get("b"), Some(&false));
        assert_eq!(radio.get("c"), Some(&false));

        radio.change("a", true).unwrap();
        radio.change("b", true).unwrap();
        radio.change("c", true).unwrap();

        assert_eq!(radio.get("a"), Some(&true));
        assert_eq!(radio.get("b"), Some(&true));
        assert_eq!(radio.get("c"), Some(&true));

        assert_eq!(radio.get("d"), None);
    }

    #[test]
    fn text_box_test() {
        let text = "I am a TextBox";
        let options_text = "test1";
        let default_text = "shamokwok";
        
        let mut text_box = TextBox::new(text, &options_text, default_text);

        assert_eq!(text_box.get_type(), &MenuType::TextBox);
        assert_eq!(text_box.get(options_text), Some(&default_text.to_string()));

        text_box.change(&default_text, "kiana");
        assert_eq!(text_box.get(options_text), Some(&"kiana".to_string()));

    }
}
