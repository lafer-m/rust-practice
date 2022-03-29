
pub struct Config {
    pub query: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args")
        }
        Ok(Config {
            query: args[1].clone(),
            file: args[2].clone(),
        })
    }
}