use chrono::prelude::*;
use clap::Parser;
use cli_clipboard;
use colored::Colorize;
use reqwest;
use serde_json;
use std::error::Error;
use std::process::Command;
use std::time::SystemTime;
use std::{thread, time::Duration};

mod google_api;

const TIMEOUT: u64 = 9;

/// Simple program to translate text
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// source language
    #[clap(short, long, value_parser, default_value = "English")]
    sourcelanguage: String,
    /// target translation language
    #[clap(short, long, value_parser, default_value = "Chinese")]
    targetlanguage: String,
    /// fast mode or slow mode
    #[clap(short, long, action)]
    fast_mode: bool,
    /// proxy set (socks5://192.168.1.1:9000)
    #[clap(short, long, value_parser, default_value = "null")]
    proxy: String,
    /// translate new text and clear the screen
    #[clap(short, long, value_parser, default_value_t = 0)]
    clear: i32,
    /// show original text or not
    #[clap(long, action)]
    no_original: bool,
    /// auto break the sentence or not
    #[clap(long, action)]
    disable_auto_break: bool,
    /// linux stop get text from clipboard
    #[clap(long, action)]
    linux_use_clipboard: bool,
}

async fn translate<'a>(
    sl: &'a str,
    tl: &'a str,
    translate_string: &'a str,
    index: usize,
    proxy_str: &'a Option<String>,
) -> TranslateResult<'a> {
    let contains_symbol = |input_string: &str| -> bool { input_string.contains(" ") };
    let start_time = SystemTime::now();
    // let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000").expect("set proxy failed");
    let proxy = match proxy_str {
        Some(s) => Some(reqwest::Proxy::https(s).expect("set proxy failed")),
        _ => None,
    };
    let result_vec = match contains_symbol(translate_string) {
        true => match google_translate_longstring(sl, tl, translate_string, proxy).await {
            Ok(r) => r,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
        false => match google_translate_shortword(sl, tl, translate_string, proxy).await {
            Ok(r) => r,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
    };
    // println!("{:?}", result_vec);
    let end_time = SystemTime::now();
    TranslateResult {
        result_vec,
        proxy_str,
        start_time,
        end_time,
        index,
    }
}

pub struct TranslateResult<'a> {
    result_vec: Vec<Vec<String>>,
    proxy_str: &'a Option<String>,
    start_time: SystemTime,
    end_time: SystemTime,
    index: usize,
}

impl TranslateResult<'_> {
    fn show(&self, no_original: bool, disable_auto_break: bool) {
        let start_time = self.start_time;
        let end_time = self.end_time;
        let index = self.index;
        let result_vec = &self.result_vec;
        let proxy_str = self.proxy_str;

        let duration = end_time.duration_since(start_time).unwrap();
        let dt = Local::now();
        let dt_str = dt.format("%H:%M:%S").to_string();
        let translate_title = format!(
            "Translate[{}]({}) => {:.3}s",
            index,
            dt_str,
            duration.as_secs_f32()
        );

        if result_vec.len() > 0 {
            // linux part
            #[cfg(target_os = "linux")]
            match proxy_str {
                Some(_) => println!(
                    ">>> {} {}",
                    translate_title.bold().red(),
                    "=> proxy".bright_purple()
                ),
                _ => println!(">>> {}", translate_title.bold().red()),
            }
            // windows part
            #[cfg(target_os = "windows")]
            match proxy_str {
                Some(_) => println!(">>> {} {}", translate_title, "=> proxy"),
                _ => println!(">>> {}", translate_title),
            }
            match disable_auto_break {
                true => {
                    let mut original_text = String::new();
                    let mut translate_text = String::new();
                    let mut alter_translate_text = String::new();
                    for v in result_vec {
                        original_text.push_str(&v[1]);
                        translate_text.push_str(&v[0]);
                        if v.len() > 2 {
                            for i in 2..v.len() {
                                alter_translate_text.push_str(&v[i]);
                            }
                        }
                    }
                    match no_original {
                        true => (),
                        _ => {
                            #[cfg(target_os = "linux")]
                            println!("[{}] {}", "O".bright_blue().bold(), &original_text);
                            #[cfg(target_os = "windows")]
                            println!("[{}] {}", "O", &original_text);
                        }
                    }
                    #[cfg(target_os = "linux")]
                    println!("[{}] {}", "T".green().bold(), &translate_text);
                    #[cfg(target_os = "windows")]
                    println!("[{}] {}", "T", &translate_text);
                    if alter_translate_text.len() > 0 {
                        #[cfg(target_os = "linux")]
                        println!("[{}] {}", "A".cyan().bold(), &alter_translate_text);
                        #[cfg(target_os = "windows")]
                        println!("[{}] {}", "A", &alter_translate_text);
                    }
                    #[cfg(target_os = "windows")]
                    println!(""); // use the empty line to split two translate result in windows
                }
                _ => {
                    for v in result_vec {
                        match no_original {
                            true => (),
                            _ => {
                                #[cfg(target_os = "linux")]
                                println!("[{}] {}", "O".bright_blue().bold(), v[1]);
                                #[cfg(target_os = "windows")]
                                println!("[{}] {}", "O", v[1]);
                            }
                        }
                        println!("[{}] {}", "T".green().bold(), v[0]);
                        if v.len() > 2 {
                            for i in 2..v.len() {
                                #[cfg(target_os = "linux")]
                                println!("[{}] {}", "A".cyan().bold(), v[i]);
                                #[cfg(target_os = "windows")]
                                println!("[{}] {}", "A", v[i]);
                            }
                        }
                    }
                    #[cfg(target_os = "windows")]
                    println!(""); // use the empty line to split two translate result in windows
                }
            }
        }
    }
}

fn get_clipboard_text() -> Result<String, Box<dyn Error>> {
    let output = match cli_clipboard::get_contents() {
        Ok(o) => o.trim().to_string(),
        Err(_) => {
            // println!("get clipboard contents failed: {}", e);
            return Ok("".to_string());
        }
    };
    if output.len() > 0 {
        // set the clipboard to null
        match cli_clipboard::set_contents("".to_owned()) {
            _ => (),
        }
    }
    return Ok(output);
}

fn get_select_text_linux() -> Result<String, Box<dyn Error>> {
    // return "" at least
    let output = match Command::new("xsel").output() {
        Ok(o) => o,
        Err(e) => {
            println!("Please install xsel first...");
            return Err(Box::new(e));
        }
    };
    let output = String::from_utf8_lossy(&output.stdout).to_string();
    // println!("select text linux: {}", &output);
    if output.trim().len() > 0 {
        return Ok(output.trim().to_string());
    } else {
        return Ok("".to_string());
    }
}

struct GetText {
    text: String,
    text_type: String,
}

impl GetText {
    fn new(text: &str, text_type: &str) -> GetText {
        let text = text.to_string();
        let text_type = text_type.to_string();
        GetText { text, text_type }
    }
}

fn get_text(linux_use_clipboard: bool) -> GetText {
    let filter = |x: &str| -> String {
        let x = x.trim();
        let x = match x.strip_prefix(".") {
            Some(x) => x,
            _ => x,
        };
        let x = match x.strip_prefix(",") {
            Some(x) => x,
            _ => x,
        };
        x.replace("-\n", "")
            .replace("%", "%25")
            .replace("&", "%26")
            .replace("#", "%23")
            .replace("\n", " ")
            .trim()
            .to_string()
    };
    if cfg!(target_os = "linux") {
        match linux_use_clipboard {
            true => {
                let t = match get_select_text_linux() {
                    Ok(t) => {
                        // get text from select, if select is none, get from clipboard
                        let t = t.trim().to_string();
                        if t.len() > 0 {
                            t
                        } else {
                            match get_clipboard_text() {
                                Ok(t) => t,
                                Err(e) => {
                                    println!("get clipboard text (linux) failed: {}", e);
                                    "".to_string()
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("get clipboard text (linux) failed: {}", e);
                        "".to_string()
                    }
                };
                let ft = filter(&t);
                return GetText::new(&ft, "clipboard");
            }
            _ => {
                let t = match get_select_text_linux() {
                    Ok(t) => {
                        // only get text from select
                        t
                    }
                    Err(e) => {
                        println!("get select text (linux) failed: {}", e);
                        "".to_string()
                    }
                };
                let ft = filter(&t);
                return GetText::new(&ft, "select");
            }
        }
    } else if cfg!(target_os = "windows") {
        let t = match get_clipboard_text() {
            Ok(t) => t,
            Err(e) => {
                println!("get select text (windows) failed: {}", e);
                "".to_string()
            }
        };
        let ft = filter(&t);
        return GetText::new(&ft, "clipboard");
    } else {
        GetText::new("", "clipboard")
    }
}

fn convert_args<'a>(source_language: &'a str, target_language: &'a str) -> (&'a str, &'a str) {
    let convert_language = |x: &str| -> &str {
        let result = match x {
            "English" => "en",
            "Chinese" => "zh-CN",
            "Japanese" => "ja",
            "French" => "fr",
            "German" => "de",
            _ => "en",
        };
        result
    };
    let sl_result = convert_language(source_language);
    let tl_result = convert_language(target_language);
    (sl_result, tl_result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut index: usize = 1;
    let args = Args::parse();
    let (sl, tl) = convert_args(&args.sourcelanguage, &args.targetlanguage);
    let sleep_time = match args.fast_mode {
        true => Duration::from_secs_f32(0.3),
        _ => Duration::from_secs(1),
    };
    let proxy_str = match args.proxy.as_str() {
        "none" => None,
        _ => Some(args.proxy),
    };
    let clear_times = args.clear;
    // println!("c: {}", clear_times);
    let clear_mode = match clear_times {
        0 => false,
        _ => true,
    };
    // println!("clear: {}", clear_times);
    let no_original = args.no_original;
    let not_auto_break = args.disable_auto_break;
    let linux_use_clipboard = args.linux_use_clipboard;
    // println!("l: {}", linux_use_clipboard);
    if cfg!(target_os = "linux") {
        println!("{}", "Working...".bold().yellow());
        let mut sub_clear_times = clear_times;
        // let mut last_selected_text = String::from("");
        let mut last_select_texts: String = String::from("");
        let mut last_clipboard_texts: String = String::from("");
        loop {
            // worker area
            thread::sleep(sleep_time);
            let get_text = get_text(linux_use_clipboard);
            let condition = if get_text.text.len() > 0 {
                let ret = if get_text.text != last_clipboard_texts
                    && get_text.text != last_clipboard_texts
                {
                    true
                } else {
                    false
                };
                match get_text.text_type.as_str() {
                    "select" => {
                        if last_select_texts != get_text.text {
                            last_select_texts = get_text.text.clone();
                        }
                    }
                    _ => {
                        if last_clipboard_texts != get_text.text {
                            last_clipboard_texts = get_text.text.clone();
                        }
                    }
                }
                ret
            } else {
                false
            };

            if condition {
                // println!("last: {}", &last_selected_text);
                // println!("now: {}", &selected_text);
                if clear_mode {
                    if sub_clear_times == 0 {
                        // send a control character to clear the terminal screen
                        // print!("{}[2J", 27 as char);
                        // set position the cursor at row 1, column 1
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        sub_clear_times = clear_times;
                    }
                    sub_clear_times -= 1;
                }
                let translate_result = translate(&sl, &tl, &get_text.text, index, &proxy_str).await;
                translate_result.show(no_original, not_auto_break);
                index += 1;
            }
        }
    } else if cfg!(target_os = "windows") {
        println!("Working...");
        let mut sub_clear = clear_times;
        loop {
            thread::sleep(sleep_time);
            let clipboard_text = get_text(false);
            if clipboard_text.text.len() > 0 {
                if clear_mode {
                    if sub_clear == 0 {
                        // send a control character to clear the terminal screen
                        // print!("{}[2J", 27 as char);
                        // set position the cursor at row 1, column 1
                        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        sub_clear = clear_times;
                    }
                    sub_clear -= 1;
                }
                let translate_result =
                    translate(&sl, &tl, &clipboard_text.text, index, &proxy_str).await;
                translate_result.show(no_original, not_auto_break);
                index += 1;
            }
        }
    } else {
        panic!("not support running at the other system!");
    }
}
