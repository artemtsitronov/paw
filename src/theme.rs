// Expelliarmus! Sectusempra! PRETTIFY! (hp reference)

use std::{
    env,
    io::{self, IsTerminal},
};

#[allow(dead_code)]
pub struct Theme {
    pub green: &'static str,
    pub red: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub mauve: &'static str,
    pub dim: &'static str,
    pub reset: &'static str,
    pub frame: &'static str,
    pub chip_ok: &'static str,
    pub chip_err: &'static str,
    pub chip_dim: &'static str,
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
            frame: "\x1b[38;2;88;91;112m",
            chip_ok: "\x1b[48;2;166;227;161m\x1b[38;2;30;30;46m",
            chip_err: "\x1b[48;2;243;139;168m\x1b[38;2;30;30;46m",
            chip_dim: "\x1b[48;2;69;71;90m\x1b[38;2;205;214;244m",
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
            frame: "",
            chip_ok: "",
            chip_err: "",
            chip_dim: "",
        }
    }
}
