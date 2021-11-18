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
                Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("PROJECT")
                .help("Provides a project to list stamps from")
                .takes_value(true),
            )
            .arg(
                Arg::with_name("last")
                .short("l")
                .long("last")
                .help("List last stamp only")
            ),
        )
        .subcommand(
            SubCommand::with_name("stamp")
            .about("Manage stamps")
            .arg(
                Arg::with_name("start")
                .short("s")
                .long("start")
                .help("Start a new stamp")
                .takes_value(false),
            )
            .arg(
                Arg::with_name("stop")
                .short("S")
                .long("stop")
                .help("Stop last stamp")
                .takes_value(false),
            )
            .arg(
                Arg::with_name("update")
                .short("u")
                .long("update")
                .help("Update last stamp")
                .takes_value(false),
            )
            .arg(
                Arg::with_name("task")
                .short("t")
                .long("task")
                .value_name("TASK")
                .help("Task related to the stamp")
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
            .arg(
                Arg::with_name("dstart")
                .long("dstart")
                .value_name("DATE_START")
                .help("Date stamp start")
                .takes_value(true),
            )
            .arg(
                Arg::with_name("dend")
                .long("dend")
                .value_name("DATE_END")
                .help("Date stamp end")
                .takes_value(true),
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
            )
            .arg(
                Arg::with_name("last")
                .short("l")
                .long("last")
                .help("List last task only")
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
