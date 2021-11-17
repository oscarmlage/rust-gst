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
            .about("List last stamps")
            .arg(
                Arg::with_name("last")
                .short("l")
                .long("last")
                .help("List last stamp only")
            ),
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
        .subcommand(
            SubCommand::with_name("addtask")
            .about("Add a new task to a project")
            .arg(
                Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("PROJECT")
                .help("Provides a project to list tasks from")
                .takes_value(true),
            )
            .arg(
                Arg::with_name("title")
                .short("t")
                .long("title")
                .value_name("TITLE")
                .help("Task title")
                .takes_value(true),
            )
            .arg(
                Arg::with_name("description")
                .short("d")
                .long("description")
                .value_name("DESCRIPTION")
                .help("Task description")
                .takes_value(true),
            )
        )
}
