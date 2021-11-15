extern crate clap;
use clap::{Arg, App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("gst")
        .version("v0.1")
        .author("Óscar M. Lage <info@oscarmlage.com>")
        .about("Cli tool to deal with gestion actions")
        .arg(
            Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
            .default_value("/Users/oscar/.config/gst/gstrc"),
        )
        .subcommand(
            SubCommand::with_name("projects")
            .about("List of active projects"),
        )
        .subcommand(
            SubCommand::with_name("stamps")
            .about("List last stamps"),
        )
        .subcommand(
            SubCommand::with_name("tasks")
            .about("List of active tasks grouped by project")
            .arg(
                Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("PROJECT")
                .help("Provides a project to list tasks from")
                .takes_value(true),
            ),
        )
}
