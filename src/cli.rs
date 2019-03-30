use clap::{App, ArgMatches, SubCommand};

pub fn args<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("inner")
                .about("Internal command to run inside a new tmux window"),
        )
        .subcommand(SubCommand::with_name("config").about("Print current configuration"))
        .get_matches()
}
