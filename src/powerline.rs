use std::fmt::{self, Display, Write};

use crate::modules::Module;
use crate::terminal::*;

#[derive(Clone)]
pub struct Style {
    pub fg: FgColor,
    pub bg: Option<BgColor>,
    pub sep: Option<char>,
    pub sep_fg: FgColor,
}

impl Style {
    pub fn simple(fg: Color, bg: Option<Color>) -> Style {
        let bg = bg.map(|c| c.into());
        Style { 
            fg: fg.into(),
            bg,
            sep: Some('\u{E0B0}'),
            sep_fg: bg.unwrap_or(Color(0).into()).transpose()
        }
    }

    pub fn special(fg: Color, bg: Option<Color>, sep: Option<char>, sep_fg: Color) -> Style {
        let bg = bg.map(|c| c.into());
        Style { fg: fg.into(), bg, sep, sep_fg: sep_fg.into() }
    }
}

pub struct Powerline {
    buffer: String,
    last_style: Option<Style>,
}

impl Powerline {
    pub fn new() -> Powerline {
        Powerline { buffer: String::with_capacity(512), last_style: None }
    }

    #[inline(always)]
    fn write_segment<D: Display>(&mut self, seg: D, style: Style, spaces: bool) {
        // write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, next.bg, seg.sep_col, seg.sep)?;

        let _ = if let Some(Style { sep_fg, sep, .. }) = self.last_style {
            (
                if let Some(bg) = style.bg {
                    write!(self.buffer, "{}{}", bg, sep_fg)
                } else {
                    write!(self.buffer, "{}{}", ResetBG, sep_fg)
                }
            ).and_then(|_| {
                if let Some(sep) = sep {
                    write!(self.buffer, "{}", sep)
                } else {
                    Ok(())
                }
            })
        } else {
            if let Some(bg) = style.bg {
                write!(self.buffer, "{}", bg)
            } else {
                write!(self.buffer, "{}", ResetBG)
            }
        };

        if self.last_style.as_ref().map(|s| s.sep_fg) != Some(style.fg) {
            let _ = write!(self.buffer, "{}", style.fg);
        }

        let _ = if spaces { write!(self.buffer, " {} ", seg) } else { write!(self.buffer, "{}", seg) };

        self.last_style = Some(style)
    }

    pub fn add_segment<D: Display>(&mut self, seg: D, style: Style) {
        self.write_segment(seg, style, true)
    }

    pub fn add_short_segment<D: Display>(&mut self, seg: D, style: Style) {
        self.write_segment(seg, style, false)
    }

    pub fn add_module<M: Module>(&mut self, mut module: M) {
        module.append_segments(self)
    }

    pub fn last_style_mut(&mut self) -> Option<&mut Style> {
        self.last_style.as_mut()
    }
}

impl fmt::Display for Powerline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.last_style {
            Some(Style { sep_fg, sep, .. }) => {
                write!(f, "{}{}{}", self.buffer, Reset, sep_fg)?;
                if let Some(sep) = sep {
                    write!(f, "{}{}", sep, Reset)
                } else {
                    write!(f, "{}", Reset)
                }
            },
            None => Ok(()),
        }
    }
}
