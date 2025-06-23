
#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::modules;
use powerline::themes::gruvbox_theme::GruvboxTheme;

fn main() {
    let mut prompt = powerline::Powerline::new();

    #[cfg(feature = "time")]
    prompt.add_module(Time::<GruvboxTheme>::with_time_format("%H:%M:%S"))?;

    prompt.add_module(modules::PaddingBlock::<GruvboxTheme>::new("", Some('\u{e0d2}')));
    // prompt.add_module(VirtualEnv::<GruvboxTheme>::new());
    // prompt.add_module(Host::<GruvboxTheme>::new());
    prompt.add_module(modules::Cwd::<GruvboxTheme>::new(30, 4, 15, false));
    prompt.add_module(modules::Git::<GruvboxTheme>::new());
    prompt.add_module(modules::Cmd::<GruvboxTheme>::new());
    
    // prompt.add_module(VirtualEnv::<GruvboxTheme>::new())?;
    // prompt.add_module(ExitCode::<GruvboxTheme>::new())?;

    prompt.add_module(modules::NextLineBlock::<GruvboxTheme>::new("\u{2588}\u{e0b0}\u{e0b1}", Some(' ')));

    println!("{prompt}");
}
