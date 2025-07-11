use std::marker::PhantomData;
use std::{env, path};

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Cwd<S: CwdScheme> {
    max_length: usize,
    wanted_seg_num: usize,
    max_seg_length: usize,
    resolve_symlinks: bool,
    scheme: PhantomData<S>,
}

pub trait CwdScheme {
    const CWD_FG: Color;
    const PATH_FG: Color;
    const PATH_BG: Color;
    const HOME_FG: Color;
    const HOME_BG: Color;
    const SEPARATOR_FG: Color;
    const CWD_HOME_SYMBOL: &'static str = "~";
}

impl<S: CwdScheme> Cwd<S> {
    pub fn new(
        max_length: usize,
        wanted_seg_num: usize,
        max_seg_length: usize,
        resolve_symlinks: bool,
    ) -> Cwd<S> {
        Cwd {
            max_length,
            wanted_seg_num,
            resolve_symlinks,
            max_seg_length,
            scheme: PhantomData,
        }
    }
}

macro_rules! append_cwd_segments {
    ($powerline:ident, $iter:expr) => {
        for val in $iter {
            $powerline.add_segment(
                val,
                Style::special(
                    S::PATH_FG,
                    Some(S::PATH_BG),
                    '\u{E0B1}',
                    S::SEPARATOR_FG,
                ),
            );
        }
    };
    ($powerline:ident, $iter:expr, $len:expr) => {
        for val in $iter {
            if val.len() > $len {
                $powerline.add_segment(
                    format!("{}\u{ea7c}", &val[..$len]),
                    Style::special(
                        S::PATH_FG,
                        Some(S::PATH_BG),
                        Some('\u{E0B1}'),
                        S::SEPARATOR_FG,
                    ),
                );
            } else {
                $powerline.add_segment(
                    val,
                    Style::special(
                        S::PATH_FG,
                        Some(S::PATH_BG),
                        Some('\u{E0B1}'),
                        S::SEPARATOR_FG,
                    ),
                );
            }
        }
    };
}

impl<S: CwdScheme> Module for Cwd<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        #[cfg(target_family = "unix")]
        let current_dir = if self.resolve_symlinks {
            env::current_dir().unwrap()
        } else {
            path::PathBuf::from(env::var("PWD").unwrap())
        };
        #[cfg(target_family = "unix")]
        let mut cwd = current_dir.to_str().unwrap();

        // println!("{pwd}");
        #[cfg(target_family = "windows")]
        let mut cwd: &str = &get_msys_path(&env::var("PWD").unwrap());

        // println!("{cwd}");

        if cwd == "/" {
            return powerline
                .add_segment('/', Style::simple(S::PATH_FG, Some(S::PATH_BG)));
        }

        if let Ok(home_str) = env::var("HOME") {
            #[cfg(target_family = "windows")]
            let home_str = get_msys_path(&home_str);
            // println!("{home_str}");

            if cwd.starts_with(&home_str) {
                powerline.add_segment(
                    S::CWD_HOME_SYMBOL,
                    Style::simple(S::HOME_FG, Some(S::HOME_BG)),
                );
                cwd = &cwd[home_str.len()..]
            }
        }

        let depth = cwd.matches('/').count();

        if (cwd.len() > self.max_length as usize)
            && (depth > self.wanted_seg_num)
        {
            let left = 1; // self.wanted_seg_num / 2;
            let right = self.wanted_seg_num - left;

            let start = cwd.split('/').skip(1).take(left);
            let end = cwd.split('/').skip(depth - right + 1);

            append_cwd_segments!(powerline, start, self.max_seg_length);
            powerline.add_segment(
                '\u{2026}',
                Style::special(
                    S::PATH_FG,
                    Some(S::PATH_BG),
                    Some('\u{E0B1}'),
                    S::SEPARATOR_FG,
                ),
            );
            append_cwd_segments!(powerline, end, self.max_seg_length);
        } else {
            append_cwd_segments!(
                powerline,
                cwd.split('/').skip(1),
                self.max_seg_length
            );
        };

        if let Some(style) = powerline.last_style_mut() {
            style.sep = Some('\u{E0B0}');
            style.sep_fg = style.bg.unwrap_or(Color(0).into()).transpose();
        }
    }
}

#[cfg(target_family = "windows")]
fn get_msys_path(path_str: &str) -> String {
    path::PathBuf::from(
        path_str.replace("C:\\msys64", "").replace("C:/msys64", "").replace("C:", "/c").replace("\\", "/"),
    )
    .to_str()
    .unwrap()
    .into()
}
