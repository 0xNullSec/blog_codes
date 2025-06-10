use reqwest::{header, blocking::Client};
use std::io::{self, Write};
use serde_json::{json, Value};

fn get_text() -> String {
    let mut text = String::new();

    print!("> ");
    io::stdout().flush().expect("stdout failed!");

    io::stdin()
        .read_line(&mut text)
        .expect("stdin failed");
        text.trim().to_string();
    text

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let url = "https://google-translator9.p.rapidapi.com/v2";

    let mut headers = header::HeaderMap::new();
    headers.insert("x-rapidapi-key", header::HeaderValue::from_static("<API KEY>"));
    headers.insert("x-rapidapi-host", header::HeaderValue::from_static("google-translator9.p.rapidapi.com"));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));



    let cliente = Client::builder()
            .default_headers(headers)
            .build()?;
    let r = cliente.post(url).send()?;
    if r.status() == 200 || r.status() == 302 {
        println!("[+] The API is accessible...");
        loop { 
            let text = get_text();
            
            let params = json!({	
                "q": text,
                "source": "en",
                "target": "es",
                "format": "text"
        });
        
            let re = cliente.post(url)
                    .json(&params)
                    .send()?;
            
            if re.status() == 200 {
                let json: Value = re.json()?;
                if let Some(translated) = json["data"]["translations"][0]["translatedText"].as_str(){
                        println!("Traslated text: {:?}", translated.trim());
                } else {
                println!("Error in the request")
                }
            }
        }
    } else {
        println!("[!] Could not establish connection to the API \n[!] status: {}", r.status());
    }

    Ok(())
}
