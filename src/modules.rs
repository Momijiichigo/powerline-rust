use crate::powerline::Powerline;

mod cmd;
mod cwd;
mod exit_code;
mod git;
mod host;
#[cfg(target_family = "unix")]
mod readonly;
#[cfg(target_family = "unix")]
mod user;
mod venv;
mod padding;
#[cfg(feature = "time")]
mod time;

pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use exit_code::{ExitCode, ExitCodeScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
#[cfg(target_family = "unix")]
pub use readonly::{ReadOnly, ReadOnlyScheme};
#[cfg(feature = "time")]
pub use time::{Time, TimeScheme};
#[cfg(target_family = "unix")]
pub use user::{User, UserScheme};
pub use venv::{VirtualEnv, VirtualEnvScheme};
pub use padding::{PaddingBlock, NextLineBlock, PaddingScheme};

pub trait Module {
    fn append_segments(&mut self, powerline: &mut Powerline);
}
