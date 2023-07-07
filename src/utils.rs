use anyhow::Result;
use cli_clipboard;
use reqwest::Proxy;
use std::process::Command;

pub fn get_clipboard_text() -> Result<String> {
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

pub fn get_select_text_linux() -> Result<String> {
    // return "" at least
    let output = match Command::new("xsel").output() {
        Ok(o) => o,
        Err(_) => {
            panic!("Please install xsel first...");
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

pub struct GetText {
    pub text: String,
    pub text_type: String,
}

impl GetText {
    pub fn new(text: &str, text_type: &str) -> GetText {
        let text = text.to_string();
        let text_type = text_type.to_string();
        GetText { text, text_type }
    }
}

pub fn get_text(linux_use_clipboard: bool) -> GetText {
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

pub fn convert_language<'a>(
    source_language: &'a str,
    target_language: &'a str,
    api: &'a str,
) -> (&'a str, &'a str) {
    let convert_language = match api {
        "google" => {
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
            convert_language
        }
        "deepl" => {
            let convert_language = |x: &str| -> &str {
                let result = match x {
                    "English" => "EN",
                    "Chinese" => "ZH",
                    "Japanese" => "JA",
                    "French" => "FR",
                    "German" => "DE",
                    "English (American)" => "EN-US",
                    "English (British)" => "EN-GB",
                    "Chinese (simplified)" => "ZH",
                    _ => "EN-US",
                };
                result
            };
            convert_language
        }
        _ => panic!("Unsupport api provider!"),
    };  
    let sl_result = convert_language(source_language);
    let tl_result = convert_language(target_language);
    (sl_result, tl_result)
}

pub fn fliter_long(input: &str) -> String {
    input.replace("al.", "al")
}

pub fn fliter_short(input: &str) -> String {
    input
        .replace(".", "")
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
}

pub fn build_proxy(proxy_str: &str) -> Option<Proxy> {
    let proxy = match proxy_str {
        "null" => None,
        _ => Some(reqwest::Proxy::https(proxy_str).expect("set proxy failed")),
    };
    proxy
}
