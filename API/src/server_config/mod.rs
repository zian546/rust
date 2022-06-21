use color_eyre::Result;
use serde::Deserialize;
use dotenv::dotenv;
use eyre::WrapErr;

#[derive(Debug, Deserialize)]
pub struct Config{
    pub host : String,
    pub port : i32
}

impl Config {
    
    pub fn from_env() -> Result<Config>{
        let _env  =dotenv().ok();

        //make an empty config variable using Config struct
        let mut c = config::Config::new();
        //merge the empty config variable with the .env variable
        c.merge(config::Environment::default())?;
        //try to convert the config variable into the Config struct
        println!("{:?}",_env);
        c.try_into()
            .context("loading configurations from environment variable")
    }
}