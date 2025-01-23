#![cfg(feature = "bracketed-paste")]
use crossterm::cursor;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};
use std::io::{stdout};


pub trait<T> MenuObject<T> {
    fn print(&self);
    fn get_text(&self) -> &str;
    fn get_value(&self) -> &[(&str, T)];
}

#[derive(Clone, Debug)]
struct Lable {
    text: String
}

impl Lable {
    pub fn new(text: &str) -> Self {
        Self {text: text.to_string()}
    }
}

#[derive(Clone, Debug)]
struct Button {
    text: String,
    selected: bool,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            selected: false,
        }
    }
}

#[derive(Clone, Debug)]
struct RadioButton {
    text: String,
    options: Vec<String>,
    selected: Vec<bool>,
    count: usize,
    horizontal: bool,
}

impl RadioButton {
    pub fn new(text: &str, options: &[String], horizontal: bool) -> Self {
        let count = options.len();
        Self {
            text: text.to_string(),
            options: options.collect(),
            selected: vec![false; count],
            count,
            horizontal,
        }
    }
}

#[derive(Clone, Debug)]
struct Selection {
    text: String,
    options: Vec<String>,
    selected: Vec<bool>,
    count: usize,
    horizontal: bool,
}

impl Selection {
    pub fn new(text: &str, options: &[String], horizontal: bool) -> Self {
        let count = options.len();
        Self {
            text: text.to_string(),
            options: options.collect(),
            selected: vec![false; count],
            count,
            horizontal,
        }
    }
}

struct TextBox {
    text: String,
    input_text: String,
    defualt_text: String,
}

impl TextBox {
    pub fn new(text: &str, defualt_text: &str) -> Self {
        Self {
            text: text.to_string,
            input_text: String::new(),
            default_text: default_text.to_string(),
        }
    }
}