// Expelliarmus! Sectusempra! PRETTIFY!

use std::{
    env,
    io::{self, IsTerminal},
};

pub struct Theme {
    pub green: &'static str,
    pub red: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub mauve: &'static str,
    pub dim: &'static str,
    pub reset: &'static str,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            green: "\x1b[38;2;166;227;161m",
            red: "\x1b[38;2;243;139;168m",
            yellow: "\x1b[38;2;249;226;175m",
            blue: "\x1b[38;2;137;180;250m",
            mauve: "\x1b[38;2;203;166;247m",
            dim: "\x1b[38;2;166;173;200m",
            reset: "\x1b[0m",
        }
    }
}

pub fn get_theme() -> Theme {
    if io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none() {
        Theme::default()
    } else {
        Theme {
            green: "",
            red: "",
            yellow: "",
            blue: "",
            mauve: "",
            dim: "",
            reset: "",
        }
    }
}
