use std::error::Error;
use std::fs;
use std::path::Path;
use chrono::prelude::*;

pub enum Command {
    Start,
    Stop,
    Show,
}

pub struct Config {
    pub command: Command,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Usage: tmpltimer [start|show|stop]")
        }

        let command = match args[1].as_str() {
            "start" => Command::Start,
            "stop"  => Command::Stop,
            "show"  => Command::Show,
            _       => return Err("Unknown command"),
        };
        let path = String::from("./.tptmr");

        Ok(Config {
            command,
            path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::Start => start(config),
        Command::Stop  => stop(config),
        Command::Show => show(config),
    }

    Ok(())
}

fn start(config: Config) {
    let dt = Utc::now();
    fs::write(&config.path, &dt.to_string()).expect("Unable to write file");
}

fn stop(config: Config) {
    if Path::new(&config.path).exists() {
        fs::remove_file(&config.path).expect("Unable to remove file");
    }
}

fn show(config: Config) {
    if Path::new(&config.path).exists() {
        let datetime_string = fs::read_to_string(&config.path).unwrap();
        let timer_started = datetime_string.parse::<DateTime<Utc>>().unwrap();
        let time_now = Utc::now();
        let diff = time_now - timer_started;

        let seconds = diff.num_seconds() % 60;
        let minutes = diff.num_minutes() % 60;
        let hours   = diff.num_hours();
        println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    }
}
