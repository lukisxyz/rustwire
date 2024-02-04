use clap::{App, Arg};

pub enum ArgType {
    Run,
    Others,
}

pub struct Args {
    pub arg_type: ArgType,
    pub config_filename: String,
    pub is_production: bool,
}

impl Args {
    pub fn parse() -> Self {
        let mut config_filename = "";
        let mut arg_type: ArgType = ArgType::Others;
        let matches = App::new("htmxxx")
            .version("0.1")
            .author("Fahmi Lukistriya")
            .about("HTMXxx, trying hyper rust")
            .subcommand(
                App::new("run")
                    .about("Running server")
                    .help("Running server")
                    .arg(
                        Arg::with_name("config_filename")
                            .required(false)
                            .short("c")
                            .long("config")
                            .takes_value(true)
                            .default_value("config.yml")
                            .help("Name of configuration file in *.yml format"),
                    )
                    .arg(
                        Arg::with_name("is in production mode")
                            .short("prod")
                            .long("production")
                            .takes_value(false)
                            .help("Program is in production mode"),
                    ),
            )
            .get_matches();
        match matches.subcommand() {
            ("run", Some(init_matches)) => {
                config_filename = init_matches.value_of("config_filename").unwrap();
                arg_type = ArgType::Run;
            }
            _ => {
                println!("Invalid command. Use 'run'")
            }
        }
        let mut is_production = false;
        if matches.is_present("production") {
            is_production = true;
        }
        Self {
            config_filename: config_filename.to_string(),
            arg_type,
            is_production,
        }
    }
}
