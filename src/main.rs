use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;
use colored::Colorize;
use std::time::SystemTime;
use std::{thread, time::Duration};

mod deepl_api;
mod google_api;
mod utils;
// mod youdao_api;

use deepl_api::{translate_free, translate_pro};
use google_api::{translate_longstring, translate_shortword};
use utils::{convert_language, get_text};

const TIMEOUT: u64 = 60;

/// Simple program to translate text
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source language
    #[clap(short, long, default_value = "English")]
    sl: String,
    /// Target translation language
    #[clap(short, long, default_value = "Chinese")]
    tl: String,
    /// Fast mode or slow mode
    #[clap(short, long, action)]
    fast_mode: bool,
    /// Proxy set (socks5://192.168.1.1:9000)
    #[clap(short, long, default_value = "null")]
    proxy: String,
    /// Translate new text and clear the screen
    #[clap(short, long, default_value_t = 0)]
    clear: i32,
    /// Show original text or not
    #[clap(long, action)]
    no_original: bool,
    /// Auto break the sentence or not
    #[clap(long, action)]
    disable_auto_break: bool,
    /// Linux get text from clipboard
    #[clap(long, action)]
    linux_use_clipboard: bool,
    /// Specify translation API provider
    #[clap(short, long, default_value = "google")]
    api: String,
    /// API auth key
    #[clap(long, default_value = "null")]
    auth_key: String,
}

async fn translate<'a>(
    sl: &'a str,
    tl: &'a str,
    translate_string: &'a str,
    index: usize,
    proxy_str: &'a str,
    api: &'a str,
    auth_key: &'a str,
) -> TranslateRets<'a> {
    let contains_symbol = |input_string: &str| -> bool { input_string.contains(" ") };
    let start_time = SystemTime::now();
    // let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000").expect("set proxy failed");

    let items = match api {
        "google" => match contains_symbol(translate_string) {
            true => match translate_longstring(sl, tl, translate_string, proxy_str).await {
                Ok(r) => r,
                Err(e) => {
                    println!("translate failed: {}", e);
                    vec![]
                }
            },
            false => match translate_shortword(sl, tl, translate_string, proxy_str).await {
                Ok(r) => r,
                Err(e) => {
                    println!("translate failed: {}", e);
                    vec![]
                }
            },
        },
        "deepl" => match translate_free(sl, tl, translate_string, proxy_str, auth_key).await {
            Ok(t) => t,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
        "deeplpro" => match translate_pro(sl, tl, translate_string, proxy_str, auth_key).await {
            Ok(t) => t,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
        _ => panic!("Unsupported API provider"),
    };
    // println!("{:?}", result_vec);
    let end_time = SystemTime::now();
    TranslateRets {
        items,
        proxy_str,
        start_time,
        end_time,
        index,
    }
}

#[derive(Debug)]
pub struct Item {
    orig: String,
    trans: String,
    alter: Vec<String>,
}

pub struct TranslateRets<'a> {
    items: Vec<Item>,
    proxy_str: &'a str,
    start_time: SystemTime,
    end_time: SystemTime,
    index: usize,
}

impl TranslateRets<'_> {
    fn show(&self, no_original: bool, disable_auto_break: bool) {
        let start_time = self.start_time;
        let end_time = self.end_time;
        let index = self.index;
        let result_vec = &self.items;
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
                "null" => println!(">>> {}", translate_title.on_bright_yellow()),
                _ => println!(
                    ">>> {} {}",
                    translate_title.on_bright_yellow(),
                    "=> proxy".on_bright_purple()
                ),
            }
            // windows part
            #[cfg(target_os = "windows")]
            match proxy_str {
                "null" => println!(">>> {}", translate_title),
                _ => println!(">>> {} {}", translate_title, "=> proxy"),
            }
            match disable_auto_break {
                true => {
                    let mut original_text = String::new();
                    let mut translate_text = String::new();
                    let mut alter_translate_text = String::new();
                    for v in result_vec {
                        original_text.push_str(&v.orig);
                        translate_text.push_str(&v.trans);
                        for i in 0..v.alter.len() {
                            alter_translate_text.push_str(&v.alter[i]);
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
                                println!("[{}] {}", "O".bright_blue().bold(), v.orig);
                                #[cfg(target_os = "windows")]
                                println!("[{}] {}", "O", v.orig);
                            }
                        }
                        println!("[{}] {}", "T".green().bold(), v.trans);
                        for i in 0..v.alter.len() {
                            #[cfg(target_os = "linux")]
                            println!("[{}] {}", "A".cyan().bold(), v.alter[i]);
                            #[cfg(target_os = "windows")]
                            println!("[{}] {}", "A", v.alter[i]);
                        }
                    }
                    #[cfg(target_os = "windows")]
                    println!(""); // use the empty line to split two translate result in windows
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut index: usize = 1;
    let args = Args::parse();
    let api = args.api;
    let auth_key = args.auth_key;
    let (sl, tl) = convert_language(&args.sl, &args.tl, &api);
    let sleep_time = match args.fast_mode {
        true => Duration::from_secs_f32(0.3),
        _ => Duration::from_secs(1),
    };
    let proxy_str = args.proxy;
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
        println!(
            "{}{}{}",
            "Working with ".green(),
            api.green().bold(),
            "...".green()
        );
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
                let translate_result =
                    translate(&sl, &tl, &get_text.text, index, &proxy_str, &api, &auth_key).await;
                translate_result.show(no_original, not_auto_break);
                index += 1;
            }
        }
    } else if cfg!(target_os = "windows") {
        println!("{}{}{}", "Working with ", api, "...");
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
                let translate_result = translate(
                    &sl,
                    &tl,
                    &clipboard_text.text,
                    index,
                    &proxy_str,
                    &api,
                    &auth_key,
                )
                .await;
                translate_result.show(no_original, not_auto_break);
                index += 1;
            }
        }
    } else {
        panic!("not support running at the other system!");
    }
}
