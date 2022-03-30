use clap::{Arg, ArgMatches, Command};

pub fn cli_matches() -> ArgMatches {
    let args = Command::new("consumer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .arg(
            Arg::new("brokers")
                .short('b')
                .long("brokers")
                .takes_value(true)
                .env("BROKERS")
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::new("topics")
                .short('t')
                .long("topics")
                .takes_value(true)
                .multiple_occurrences(true)
                .env("TOPICS")
                .default_values(&["test"]),
        )
        .arg(
            Arg::new("group-id")
                .short('g')
                .long("group-id")
                .takes_value(true)
                .env("GROUP_ID")
                .default_value("gid"),
        )
        .arg(
            Arg::new("destination")
                .short('d')
                .long("destination")
                .help("Destination topic")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    args
}
