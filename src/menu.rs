#![cfg(feature = "bracketed-paste")]
use crossterm::cursor;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use crossterm::style::{Color, Print};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};
use items::{Lable, MenuItem};
use std::collections::HashMap;
use std::default;
use std::io::stdout;

mod items;

pub struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self { items }
    }
}

impl std::fmt::Display for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
