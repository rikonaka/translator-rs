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

async pub fn google_translate_longstring(
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
    let request_result = client`
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

async pub fn google_translate_shortword(
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