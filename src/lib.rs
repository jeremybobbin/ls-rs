use std::env;

pub struct Config {
    pub all: bool,
    pub long: bool,
    pub path: String,
}

impl Config {
    pub fn new(args: env::Args) -> Config {
        let mut conf = ConfigBuilder::new();

        for a in args.into_iter().skip(1) {
            conf.parse_opt(a);
        }

        conf.finalize()
    }
}



struct ConfigBuilder {
     all: Option<bool>,
    long: Option<bool>,
    path: Option<String>,
}

impl ConfigBuilder {
    fn new() -> ConfigBuilder {
        ConfigBuilder { long: None, path: None, all: None }
    }

    fn parse_opt(&mut self, opt: String) {
        if opt == "-l" {
            self.long = Some(true);
        } else if opt == "-a" {
            self.all = Some(true);
        } else if self.path.is_some() {
            eprintln!("Unrecognized {}", opt);
        } else {
            self.path = Some(opt);
        }
    }

    fn finalize(&self) -> Config {
        let long = match self.long {
            Some(b) => b,
            None => false
        };
        
        let all = match self.all {
            Some(b) => b,
            None => false
        };

        let path = match &self.path {
            Some(p) => p.clone(),
            None => String::from(".")
        };

        Config { long, path, all }
    }
    
}
