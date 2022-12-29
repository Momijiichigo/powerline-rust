use std::marker::PhantomData;

use super::Module;
use crate::{Color, Powerline, Style};

pub trait PaddingScheme {
    const BLOCK_BG: Option<Color>;
    const BLOCK_FG: Color;
}

pub struct PaddingBlock<S: PaddingScheme> {
    scheme: PhantomData<S>,
    content: String,
    separator: Option<char>,
}

pub struct NextLineBlock<S: PaddingScheme> {
    scheme: PhantomData<S>,
    content: String,
    separator: Option<char>,
}

impl<S: PaddingScheme> PaddingBlock<S> {
    pub fn new(content: &str, separator: Option<char>) -> PaddingBlock<S> {
        PaddingBlock {
            scheme: PhantomData,
            content: content.to_string(),
            separator,
        }
    }
}

impl<S: PaddingScheme> NextLineBlock<S> {
    pub fn new(content: &str, separator: Option<char>) -> NextLineBlock<S> {
        NextLineBlock {
            scheme: PhantomData,
            content: content.to_string(),
            separator,
        }
    }
}

impl<S: PaddingScheme> Module for PaddingBlock<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        powerline.add_short_segment(&self.content, Style::special(
                S::BLOCK_FG,
                S::BLOCK_BG,
                self.separator,
                S::BLOCK_BG.unwrap_or(Color(0)),
            )
        );
    }
}

impl<S: PaddingScheme> Module for NextLineBlock<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        powerline.add_short_segment('\n', Style::special(
                S::BLOCK_FG,
                None,
                None,
                S::BLOCK_BG.unwrap_or(Color(0)),
        ));
        powerline.add_short_segment(&self.content, Style::special(
                S::BLOCK_FG,
                S::BLOCK_BG,
                self.separator,
                S::BLOCK_BG.unwrap_or(Color(0)),
            )
        );
    }
}
