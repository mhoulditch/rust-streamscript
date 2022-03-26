use std::fs;
use std::fmt;
use std::env;
use std::process::Command;
use serde::Deserialize;
extern crate clap;
use clap::{App, SubCommand};
use reqwest;

//the main config struct that we let toml deserialize into.
#[derive(Debug, Deserialize)]
struct Config {
    title: Option<String>,
    stream: Option<Vec<StreamConfig>>,
}
// the general trait stream and fn play we want implementors to implement
trait Stream {
    fn play(&self) {
        println!("playing stream")
    }
}
// a sub struct of config so we can let toml deserialize stream entries
#[derive(Debug, Deserialize,Clone)]
struct StreamConfig {
    name: Option<String>,
    url: Option<String>,
    mode: Option<String>,
}
// specific implemenation of stream trait for streamconfig
impl Stream for StreamConfig {
    fn play(&self) {
        if self.mode.as_ref().unwrap() == "audio" {
            println!("playing {} in audio mode\n", self.name.as_ref().unwrap());
            let _output = Command::new("mpv")
                .arg(self.url.as_ref().unwrap())
                .arg("--no-video")
                .output()
                .expect("failed to start mpv in audio mode");
        } else {
            println!("playing {} in audio/video mode", self.name.as_ref().unwrap());
            let _output = Command::new("mpv")
                .arg(self.url.as_ref().unwrap())
                .spawn()
                .expect("Failed to execute mpv on video and audio stream");
        }
    }
}
// specific implementation of Display so I can use the println macro
impl fmt::Display for StreamConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[96m{}\x1b[0m \nmode: {}\nstream url: '{}'", self.name.as_ref().unwrap(), self.mode.as_ref().unwrap(), self.url.as_ref().unwrap())
    }
}
//fn to print the stream data from the config and if they are online or offline
fn stream_status(y: &StreamConfig, url: &String) {
    println!("{}", y);
    if let Err(e) = http_get(url) {
        println!("Couldn't do an http get {}, url: {}", e, url)
    }
}
//fn to http get and use youtube-dl -j to check if there is an error on the page for twitch.
fn http_get(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    let output = Command::new("youtube-dl")
                    .arg("-j")
                    .arg(url)
                    .output()
                    .expect("Failed to output");
    if resp.status().as_u16() == 200 && output.status.success() == true {
      println!("\x1b[32mOnline\x1b[0m\n");
      } else {
      println!("\x1b[31mOffline\x1b[0m\n");
      };
    Ok(())
}

fn main() {
    // decode our hardcoded toml streams list
    // TODO: make a config file if there is not one
    let home_dir = env::var("HOME").expect("home not set can't find config file location");
    let toml_str = fs::read_to_string(home_dir + "/.config/streamscript/config.toml").expect("Unable to read config file");
    let decoded: Config = toml::from_str(&toml_str).unwrap();

    // process command line commands
    let matches = App::new("streamscript")
                        .version("1.0")
                        .author("Maxwell Houlditch <max@houlditch.com>")
                        .after_help("this program needs a file named config.toml. look at the included example config for reference.")
                        .subcommand(SubCommand::with_name("list")
                            .about("lists all streams"))
                        .subcommand(SubCommand::with_name("play")
                            .about("play a stream")
                            .args_from_usage("<STREAMNAME> 'the name of the stream in the config file.'"))
                        .get_matches();
    
    //if we match list
    // TODO: make the reqwest client in this if statement so only one is created and its faster
    //  explore async for stream_status
    if let Some(_matches) = matches.subcommand_matches("list") {
         println!("\n\x1b[94m{}\n---\x1b[0m\n", decoded.title.unwrap());
         decoded.stream.as_ref().unwrap().iter().for_each(|y| stream_status(y, y.url.as_ref().unwrap()));
    }

    // if we match play
    // unwrap the stream option and find if theres a match in the unwrapped
    // name
    if let Some(matches) = matches.subcommand_matches("play") {
        let streamname = matches.value_of("STREAMNAME").unwrap();
        match decoded.stream.unwrap().iter().find(|x| x.name.as_ref().unwrap() == streamname) {
            Some(v) => v.play(),
            None => println!("couldn't find stream: {}", streamname),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
