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

const TIMEOUT: u64 = 9;

/// Simple program to translate text
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// source language
    #[clap(
        short,
        long,
        value_parser,
        default_value = "English",
        default_missing_value = "English"
    )]
    sourcelanguage: String,
    /// target translation language
    #[clap(
        short,
        long,
        value_parser,
        default_value = "Chinese",
        default_missing_value = "Chinese"
    )]
    targetlanguage: String,
    /// fast mode or slow mode
    #[clap(
        short,
        long,
        value_parser,
        default_value = "slow",
        default_missing_value = "slow"
    )]
    mode: String,
    /// proxy set (socks5://192.168.1.1:9000)
    #[clap(
        short,
        long,
        value_parser,
        default_value = "none",
        default_missing_value = "none"
    )]
    proxy: String,
    /// translate new text and clear the screen
    #[clap(
        short,
        long,
        value_parser,
        default_value = "0",
        default_missing_value = "0"
    )]
    clear: String,
    /// show original text or not
    #[clap(
        short,
        long,
        value_parser,
        default_value = "false",
        default_missing_value = "true"
    )]
    no_original: String,
    /// auto break the sentence or not
    #[clap(
        short,
        long,
        value_parser,
        default_value = "false",
        default_missing_value = "true"
    )]
    disable_auto_break: String,
    /// linux stop get text from clipboard
    #[clap(
        long,
        value_parser,
        default_value = "false",
        default_missing_value = "true"
    )]
    stop_linux_clipboard: String,
}

#[tokio::main]
async fn google_translate_longstring(
    sl: &str, // source language
    tl: &str, // target language
    translate_string: &str,
    proxy: Option<reqwest::Proxy>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let fliter_char = |x: &str| -> String { x.replace("al.", "al") };
    let max_loop = 100;
    let translate_url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        sl,
        tl,
        fliter_char(translate_string)
    );
    let client = match proxy {
        Some(p) => reqwest::Client::builder()
            .proxy(p)
            .build()
            .expect("proxy client build failed"),
        _ => reqwest::Client::new(),
    };
    let request_result = client
        .get(translate_url)
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // println!("{:#?}", request_result);
    // [[["翻译","translate",null,null,10]],null,"en",null,null,null,null,[]]
    let mut i = 0;
    let mut result_vec: Vec<Vec<String>> = Vec::new();
    loop {
        let result_string_0 = format!("{}", request_result[0][i][0]);
        let result_string_1 = format!("{}", request_result[0][i][1]);
        match result_string_0.as_str() {
            "null" => break,
            _ => {
                let string_0 = result_string_0.replace("\"", "");
                let string_1 = result_string_1.replace("\"", "");
                if string_0.len() == 1 && string_0 == "." {
                    // there is no possible for length of result is 1
                } else {
                    let mut tmp_vec: Vec<String> = Vec::new();
                    tmp_vec.push(string_0);
                    tmp_vec.push(string_1);
                    result_vec.push(tmp_vec);
                }
            }
        }
        i += 1;
        if i > max_loop {
            break;
        }
    }
    Ok(result_vec)
}

#[tokio::main]
async fn google_translate_shortword(
    sl: &str,
    tl: &str,
    translate_string: &str,
    proxy: Option<reqwest::Proxy>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let fliter_char = |x: &str| -> String {
        x.replace(".", "")
            .replace(",", "")
            .replace("?", "")
            .replace("!", "")
            .replace(":", "")
            .replace("\"", "")
            .replace("(", "")
            .replace(")", "")
            .replace("<", "")
            .replace(">", "")
            // UTF-8 char
            .replace("“", "")
            .replace("”", "")
            .replace("。", "")
            .replace("，", "")
            .replace("：", "")
            .replace("（", "")
            .replace("）", "")
            .replace("《", "")
            .replace("》", "")
    };
    let translate_url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dj=1&dt=t&dt=bd&dt=qc&dt=rm&dt=ex&dt=at&dt=ss&dt=rw&dt=ld&q={}&button&tk=233819.233819",
        sl, tl, fliter_char(translate_string)
    );
    let client = match proxy {
        Some(p) => reqwest::Client::builder()
            .proxy(p)
            .build()
            .expect("proxy client build failed"),
        _ => reqwest::Client::new(),
    };
    let request_result = client
        .get(translate_url)
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // println!("{:#?}", request_result);
    // {"sentences":[{"trans":"这","orig":"The","backend":10},{"translit":"Zhè"}],"src":"en","alternative_translations":[{"src_phrase":"The","alternative":[{"word_postproc":"这","score":1000,"has_preceding_space":true,"attach_to_next_token":false,"backends":[10]},{"word_postproc":"该","score":0,"has_preceding_space":true,"attach_to_next_token":false,"backends":[3],"backend_infos":[{"backend":3}]},{"word_postproc":"那个","score":0,"has_preceding_space":true,"attach_to_next_token":false,"backends":[8]}],"srcunicodeoffsets":[{"begin":0,"end":3}],"raw_src_segment":"The","start_pos":0,"end_pos":0}],"confidence":1.0,"spell":{},"ld_result":{"srclangs":["en"],"srclangs_confidences":[1.0],"extended_srclangs":["en"]}}
    let mut result_vec: Vec<Vec<String>> = Vec::new();
    let mut tmp_vec: Vec<String> = Vec::new();
    let trans_string = format!(
        "{}",
        request_result.get("sentences").unwrap()[0]
            .get("trans")
            .unwrap()
    );
    let orig_string = format!(
        "{}",
        request_result.get("sentences").unwrap()[0]
            .get("orig")
            .unwrap()
    );
    tmp_vec.push(trans_string.replace("\"", ""));
    tmp_vec.push(orig_string.replace("\"", ""));
    let alter_vec = request_result.get("alternative_translations").unwrap()[0]
        .get("alternative")
        .unwrap();
    let mut i = 0;
    loop {
        let av = match alter_vec[i].get("word_postproc") {
            Some(a) => a,
            _ => break,
        };
        let alter_string = format!("{}", av);
        // jump the first word
        if i != 0 {
            tmp_vec.push(alter_string.replace("\"", ""));
        }
        i += 1;
    }
    result_vec.push(tmp_vec);
    Ok(result_vec)
}

fn contains_symbol(input_string: &str) -> bool {
    input_string.contains(" ")
}

pub fn translate<'a>(
    sl: &'a str,
    tl: &'a str,
    translate_string: &'a str,
    index: usize,
    proxy_str: &'a Option<String>,
) -> TranslateResult<'a> {
    let start_time = SystemTime::now();
    // let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000").expect("set proxy failed");
    let proxy = match proxy_str {
        Some(s) => Some(reqwest::Proxy::https(s).expect("set proxy failed")),
        _ => None,
    };
    let result_vec = match contains_symbol(translate_string) {
        true => match google_translate_longstring(sl, tl, translate_string, proxy) {
            Ok(r) => r,
            Err(e) => {
                println!("translate failed: {}", e);
                vec![]
            }
        },
        false => match google_translate_shortword(sl, tl, translate_string, proxy) {
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

pub fn get_text(stop_linux_clipboard: bool) -> String {
    /*
    type, result
    0 => select text
    1 => copy text
    */
    let filter = |x: &str| -> String {
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
        match get_select_text_linux() {
            Ok(t) => match stop_linux_clipboard {
                false => {
                    // we still need get text from linux's clipboard
                    if t.trim().len() != 0 {
                        return filter(&t);
                    } else {
                        let t = match get_clipboard_text() {
                            Ok(t) => t,
                            Err(e) => {
                                println!("get select text (linux) failed: {}", e);
                                return "".to_string();
                            }
                        };
                        return filter(&t);
                    }
                }
                _ => return filter(&t),
            },
            Err(e) => {
                println!("get select text (linux) failed: {}", e);
                return "".to_string();
            }
        }
    } else if cfg!(target_os = "windows") {
        match get_clipboard_text() {
            Ok(t) => return filter(&t),
            Err(e) => {
                println!("get select text (windows) failed: {}", e);
                return "".to_string();
            }
        }
    }
    "".to_string()
}

pub fn convert_args<'a>(source_language: &'a str, target_language: &'a str) -> (&'a str, &'a str) {
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

fn main() {
    let mut index: usize = 1;
    let args = Args::parse();
    let (sl, tl) = convert_args(&args.sourcelanguage, &args.targetlanguage);
    let sleep_time = match args.mode.as_str() {
        "fast" => Duration::from_secs_f32(0.3),
        _ => Duration::from_secs(1),
    };
    let proxy_str = match args.proxy.as_str() {
        "none" => None,
        _ => Some(args.proxy),
    };
    let clear_times: i32 = args.clear.parse().unwrap();
    // println!("c: {}", clear_times);
    let clear_mode = match clear_times {
        0 => false,
        _ => true,
    };
    // println!("clear: {}", clear_times);
    let no_original = match args.no_original.as_str() {
        "true" => true,
        _ => false,
    };
    let not_auto_break = match args.disable_auto_break.as_str() {
        "true" => true,
        _ => false,
    };
    let stop_linux_clipboard = match args.stop_linux_clipboard.as_str() {
        "true" => true,
        _ => false,
    };
    println!("s: {}", stop_linux_clipboard);

    if cfg!(target_os = "linux") {
        println!("{}", "Working...".bold().yellow());
        let mut sub_clear_times = clear_times;
        loop {
            thread::sleep(sleep_time);
            let selected_text = get_text(stop_linux_clipboard);
            if selected_text.len() > 0 {
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
                let translate_result = translate(&sl, &tl, &selected_text, index, &proxy_str);
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
            if clipboard_text.len() > 0 {
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
                let translate_result = translate(&sl, &tl, &clipboard_text, index, &proxy_str);
                translate_result.show(no_original, not_auto_break);
                index += 1;
            }
        }
    } else {
        panic!("not support running at the other system!");
    }
}
