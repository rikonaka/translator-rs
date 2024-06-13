use anyhow::Result;
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod deepl_api;
mod errors;
mod google_api;
mod utils;

use deepl_api::translate_free;
use deepl_api::translate_pro;
use errors::UnsupportApiError;
use errors::UnsupportOsError;
use google_api::translate_longstring;
use google_api::translate_shortword;
use utils::standardized_lang;
use utils::Text;

const TIMEOUT: u64 = 60;

/// Simple program to translate text
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source language
    #[clap(short, long, default_value = "English")]
    sl: String,
    /// Target translation language
    #[clap(short, long, default_value = "Chinese (Simplified)")]
    tl: String,
    /// Fast mode or slow mode
    #[clap(short, long, default_value_t = 1.0)]
    fast_mode: f32,
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
    use_clipboard: bool,
    /// Specify translation API provider
    #[clap(short, long, default_value = "google")]
    api: String,
    /// API auth key
    #[clap(long, default_value = "null")]
    auth_key: String,
    /// Theme (light or dark)
    #[clap(long, default_value = "light")]
    theme: String,
}

async fn translate<'a>(
    sl: &'a str,
    tl: &'a str,
    content: &'a str,
    index: usize,
    proxy: &'a str,
    theme: &'a str,
    api: &'a str,
    auth_key: &'a str,
) -> Result<TranslateResults<'a>> {
    let contains_symbol = |input_string: &str| -> bool { input_string.contains(" ") };
    let start_time = SystemTime::now();
    // let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000").expect("set proxy failed");

    let results = match api {
        "google" => match contains_symbol(content) {
            true => match translate_longstring(sl, tl, content, proxy).await {
                Ok(r) => r,
                Err(e) => {
                    println!("translate failed: {}", e);
                    vec![]
                }
            },
            false => match translate_shortword(sl, tl, content, proxy).await {
                Ok(r) => r,
                Err(e) => {
                    println!("translate failed: {}", e);
                    vec![]
                }
            },
        },
        "deepl" => match translate_free(sl, tl, content, proxy, auth_key).await {
            Ok(t) => t,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
        "deeplpro" => match translate_pro(sl, tl, content, proxy, auth_key).await {
            Ok(t) => t,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
        _ => return Err(UnsupportApiError.into()),
    };
    // println!("{:?}", result_vec);
    let end_time = SystemTime::now();
    let trets = TranslateResults {
        results,
        proxy,
        theme,
        start_time,
        end_time,
        index,
    };
    Ok(trets)
}

#[derive(Debug)]
pub struct TranslateResult {
    orig: String,
    trans: String,
    alter: Vec<String>,
}

pub struct TranslateResults<'a> {
    results: Vec<TranslateResult>,
    proxy: &'a str,
    theme: &'a str,
    start_time: SystemTime,
    end_time: SystemTime,
    index: usize,
}

impl TranslateResults<'_> {
    fn show(&self, no_original: bool, disable_auto_break: bool) {
        let start_time = self.start_time;
        let end_time = self.end_time;
        let index = self.index;
        let result_vec = &self.results;
        let proxy_str = self.proxy;

        let duration = end_time.duration_since(start_time).unwrap();
        let dt = Local::now();
        let dt_str = dt.format("%H:%M:%S").to_string();
        let t_str = "Translate";
        let theme = self.theme;

        if result_vec.len() > 0 {
            let index_str = format!("[{}]", index);
            let dt_str = format!("({})", dt_str);
            let dura_str = format!("=>{:.3}s", duration.as_secs_f32());
            let mut title = match theme {
                "light" => {
                    format!(
                        "{}{}{}{}{}",
                        ">>>".on_truecolor(243, 56, 42),       // red
                        t_str.on_truecolor(255, 165, 0),       // orange
                        index_str.on_truecolor(244, 223, 183), // yellow
                        dt_str.on_truecolor(148, 215, 199),    // green
                        dura_str.on_truecolor(124, 176, 250),  // blue
                    )
                }
                "dark" | _ => {
                    format!(
                        "{}{}{}{}{}",
                        ">>>".truecolor(243, 56, 42),       // red
                        t_str.truecolor(255, 165, 0),       // orange
                        index_str.truecolor(244, 223, 183), // yellow
                        dt_str.truecolor(148, 215, 199),    // green
                        dura_str.truecolor(124, 176, 250),  // blue
                    )
                }
            };

            match proxy_str {
                "null" => (),
                _ => match theme {
                    "light" => {
                        title = format!("{}{}", title, "=>proxy".on_truecolor(245, 125, 197))
                    }
                    "dark" | _ => {
                        title = format!("{}{}", title, "=>proxy".truecolor(245, 125, 197))
                    }
                },
            }
            println!("{}", title);
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
                            println!("[{}] {}", "O".bright_blue().bold(), &original_text);
                        }
                    }
                    println!("[{}] {}", "T".green().bold(), &translate_text);
                    if alter_translate_text.len() > 0 {
                        println!("[{}] {}", "A".cyan().bold(), &alter_translate_text);
                    }
                }
                _ => {
                    for v in result_vec {
                        match no_original {
                            true => (),
                            _ => {
                                println!("[{}] {}", "O".bright_blue().bold(), v.orig);
                            }
                        }
                        println!("[{}] {}", "T".green().bold(), v.trans);
                        for i in 0..v.alter.len() {
                            println!("[{}] {}", "A".cyan().bold(), v.alter[i]);
                        }
                    }
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
    if cfg!(not(target_os = "linux")) && cfg!(not(target_os = "windows")) {
        return Err(UnsupportOsError.into());
    }

    let args = Args::parse();
    let (sl, tl) = standardized_lang(&args.sl, &args.tl, &args.api)?;
    let fast_mode_sleep_time = Duration::from_secs_f32(args.fast_mode);
    let clear_mode = match args.clear {
        0 => false,
        _ => true,
    };

    // show title
    println!(
        "{}{}{}",
        "Working with ".green(),
        args.api.green().bold(),
        "...".green()
    );

    let mut clear_count = args.clear;
    let mut last_text: String = String::from("");

    let mut index: usize = 1;
    loop {
        let text = Text::get_text(args.use_clipboard);
        let avoid_one_content_translate_twice = if text.content.len() > 0 {
            let ret = if text.content != last_text {
                true
            } else {
                false
            };

            if last_text != text.content {
                last_text = text.content.clone();
            }

            ret
        } else {
            false
        };

        if avoid_one_content_translate_twice {
            if clear_mode {
                if clear_count == 0 {
                    // send a control character to clear the terminal screen
                    // print!("{}[2J", 27 as char);
                    // set position the cursor at row 1, column 1
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    clear_count = args.clear;
                }
                clear_count -= 1;
            }
            let translate_result = translate(
                &sl,
                &tl,
                &text.content,
                index,
                &args.proxy,
                &args.theme,
                &args.api,
                &args.auth_key,
            )
            .await?;
            translate_result.show(args.no_original, args.disable_auto_break);
            index += 1;
        }
        thread::sleep(fast_mode_sleep_time);
    }
}
