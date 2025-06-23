use crate::modules::*;
use crate::Color;

#[derive(Copy, Clone)]
pub struct GruvboxTheme;

impl CmdScheme for GruvboxTheme {
    const CMD_FAILED_BG: Color = Color(125);
    const CMD_FAILED_FG: Color = Color(15);
    const CMD_PASSED_BG: Color = Color(236);
    const CMD_PASSED_FG: Color = Color(15);
}

impl PaddingScheme for GruvboxTheme {
    const BLOCK_BG: Option<Color> = Some(Color(250)); // Some(Color(130));
    const BLOCK_FG: Color = Color(250);
    const NEXT_LINE_BG: Option<Color> = None;
}

impl CwdScheme for GruvboxTheme {
    const CWD_FG: Color = Color(254);
    const HOME_BG: Color = Color(30); //--
    const HOME_FG: Color = Color(15);
    const PATH_BG: Color = Color(237);
    const PATH_FG: Color = Color(250);
    const SEPARATOR_FG: Color = Color(244);
}

impl ExitCodeScheme for GruvboxTheme {
    const EXIT_CODE_BG: Color = Color(125);
    const EXIT_CODE_FG: Color = Color(15);
}

#[cfg(target_family = "unix")]
impl UserScheme for GruvboxTheme {
    const USERNAME_BG: Color = Color(240);
    const USERNAME_FG: Color = Color(250);
    const USERNAME_ROOT_BG: Color = Color(124);
}

impl HostScheme for GruvboxTheme {
    const HOSTNAME_BG: Color = Color(238);
    const HOSTNAME_FG: Color = Color(250);
}

#[cfg(target_family = "unix")]
impl ReadOnlyScheme for GruvboxTheme {
    const READONLY_BG: Color = Color(124);
    const READONLY_FG: Color = Color(254);
}

#[cfg(feature = "time")]
impl TimeScheme for GruvboxTheme {
    const TIME_BG: Color = Color(238);
    const TIME_FG: Color = Color(250);
}

impl GitScheme for GruvboxTheme {
    const GIT_AHEAD_BG: Color = Color(240);
    const GIT_AHEAD_FG: Color = Color(250);
    const GIT_BEHIND_BG: Color = Color(240);
    const GIT_BEHIND_FG: Color = Color(250);
    const GIT_CONFLICTED_BG: Color = Color(9);
    const GIT_CONFLICTED_FG: Color = Color(15);
    const GIT_NOTSTAGED_BG: Color = Color(130);
    const GIT_NOTSTAGED_FG: Color = Color(15);
    const GIT_REPO_CLEAN_BG: Color = Color(148);
    const GIT_REPO_CLEAN_FG: Color = Color(0);
    const GIT_REPO_DIRTY_BG: Color = Color(125); //--
    const GIT_REPO_DIRTY_FG: Color = Color(15);
    const GIT_STAGED_BG: Color = Color(22);
    const GIT_STAGED_FG: Color = Color(15);
    const GIT_UNTRACKED_BG: Color = Color(52);
    const GIT_UNTRACKED_FG: Color = Color(15);
}

impl VirtualEnvScheme for GruvboxTheme {
    const PYVENV_BG: Color = Color(35);
    const PYVENV_FG: Color = Color(0);
}
