
#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::modules::*;
use powerline::themes::gruvbox_theme::GruvboxTheme;

fn main() {
    let mut prompt = powerline::Powerline::new();

    #[cfg(feature = "time")]
    prompt.add_module(Time::<GruvboxTheme>::with_time_format("%H:%M:%S"))?;

    prompt.add_module(PaddingBlock::<GruvboxTheme>::new("", Some('\u{e0b3}')));
    // prompt.add_module(VirtualEnv::<GruvboxTheme>::new());
    // prompt.add_module(Host::<GruvboxTheme>::new());
    prompt.add_module(Cwd::<GruvboxTheme>::new(30, 4, 15, false));
    prompt.add_module(Git::<GruvboxTheme>::new());
    prompt.add_module(Cmd::<GruvboxTheme>::new());
    
    // prompt.add_module(VirtualEnv::<GruvboxTheme>::new())?;
    // prompt.add_module(ExitCode::<GruvboxTheme>::new())?;

    prompt.add_module(NextLineBlock::<GruvboxTheme>::new("\u{e0b0}\u{e0b1}", Some(' ')));

    println!("{prompt}");
}
