use crate::request;
use client::Client;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
pub fn parser(client: &Client) -> request::Request {
    //client.clone().read_to_string(&mut buffer_string).unwrap();

    let args = r"\r\n\r\n";
    let args_len = args.len();
    let mut buffer = BufReader::with_capacity(args_len, client.read());

    let mut raw_request = String::new();
    let mut body = Vec::new();
    let mut is_header = true;
    let mut last = Vec::new();

    loop {
        //    let buf = buffer.fill_buf().unwrap();

        match buffer.fill_buf() {
            Ok(buf) => {
                let mut byts = buf.to_vec();
                let bytslen = byts.len();
                //    println!()len byts {} contador_loop {},bytslen,contador_loop
                let mut e = 0;

                if args_len == bytslen && last.len() == bytslen {
                    if last[0] == byts[0] {
                        for i in 0..bytslen {
                            if last[i] == byts[i] {
                                e += 1;
                            }
                        }
                    }
                }
                last = byts.clone();
                if e == args_len {
                    break;
                }
                if is_header {
                    match String::from_utf8(byts.clone()) {
                        Ok(t) => {
                            if t == args {
                                is_header = false
                            }
                        }
                        Err(_) => {
                            is_header = false;
                        }
                    }
                }
                if is_header {
                    if let Ok(txt_raw) = String::from_utf8(byts.clone()) {
                        raw_request.push_str(&txt_raw.to_owned());
                    } //txt_raw.split(&args).collect();
                } else {
                    body.append(&mut byts);
                }

                buffer.consume(args_len);

                if args_len > bytslen {
                    break;
                }
            }
            Err(e) => {
                println!("{e} soy error");
                break;
            }
        }
    }
    // println!("fin loop");
    let (method, endpoint, version, headers, parameters) = parser_http(raw_request);

    if let Some(algoxd) = headers.get("Content-Length") {
        let alg_numero = algoxd.parse().unwrap_or(0);
        if body.len() > alg_numero {
            let resta = body.len() - alg_numero;
            println!("resta {resta}")
        }
    }

    request::Request::new(method, endpoint, version, headers, parameters)
}

fn parser_http(
    raw_request: String,
) -> (
    String,
    String,
    String,
    HashMap<String, String>,
    HashMap<String, String>,
) {
    let mut headers = HashMap::new();
    //String::from_utf8(buffer).unwrap();
    let mut lines: Vec<_> = raw_request.lines().collect();
    let yew = lines.remove(0).split(" ").collect::<Vec<_>>();
    let method = yew.get(0).unwrap().to_string();
    let url = yew.get(1).unwrap().to_string();
    let version = yew
        .get(2)
        .unwrap()
        .to_string()
        .split("/")
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
        .to_string();

    for header in lines {
        if header.is_empty() {
            break;
        }
        let header_split: Vec<_> = header.split(": ").collect();

        if header_split.len() == 2 {
            headers.insert(header_split[0].to_string(), header_split[1].to_string());
        }
    }

    let parameters = HashMap::new();
    let url_raw: Vec<&str> = url.split("?").collect();
    let endpoint = url_raw[0].to_string();

    match url_raw.get(1) {
        Some(params) => {
            let params: Vec<_> = params.split("&").collect();
            // println!("params {:#?}", params);
        }
        None => (),
    }

    (method, endpoint, version, headers, parameters)
}
