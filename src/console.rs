use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(dead_code)]
pub fn info(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true));
}

#[allow(dead_code)]
pub fn warn(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true).set_fg(Some(Color::Yellow)));
}

#[allow(dead_code)]
pub fn success(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true).set_fg(Some(Color::Green)));
}

#[allow(dead_code)]
pub fn error(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)));
}

fn colorize(message: &str, color: &ColorSpec) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(color).unwrap();
    write!(&mut stdout, "{}", message).unwrap();
    stdout.set_color(&ColorSpec::new()).unwrap();
    writeln!(&mut stdout).unwrap();
}
