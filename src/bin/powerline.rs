extern crate powerline;

#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::modules::*;
use powerline::theme::SimpleTheme;

fn main() {
    let mut prompt = powerline::Powerline::new();

    #[cfg(feature = "time")]
    prompt.add_module(Time::<SimpleTheme>::with_time_format("%H:%M:%S"))?;

    prompt.add_module(PaddingBlock::<SimpleTheme>::new("╭", Some('\u{e0d2}')));
    // prompt.add_module(VirtualEnv::<SimpleTheme>::new());
    // prompt.add_module(Host::<SimpleTheme>::new());
    prompt.add_module(Cwd::<SimpleTheme>::new(30, 4, 10, false));
    prompt.add_module(Git::<SimpleTheme>::new());
    prompt.add_module(Cmd::<SimpleTheme>::new());
    
    // prompt.add_module(VirtualEnv::<SimpleTheme>::new())?;
    // prompt.add_module(ExitCode::<SimpleTheme>::new())?;

    prompt.add_module(NextLineBlock::<SimpleTheme>::new("╰\u{ead3}", Some('\u{e0b0}')));

    println!("{prompt}");
}
