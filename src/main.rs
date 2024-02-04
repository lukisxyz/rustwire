use log::info;
use rustwire::{
    args::{ArgType, Args},
    config,
};

fn main() {
    env_logger::init();
    let args = Args::parse();
    match args.arg_type {
        ArgType::Run => {
            info!("Configuration file: {}", &args.config_filename);
            let cfg = config::load(&args.config_filename);
            info!("DB Name: {}", &cfg.db.db_name);
            info!("DB Password {}", &cfg.db.password);
            info!("Production {}", args.is_production);
        }
        ArgType::Others => todo!(),
    }
}
