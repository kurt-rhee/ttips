use std::fs::File;
use std::io::Write;
use scraper::{Html, Selector, ElementRef};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tip {
    pub command: String,
    pub description: String,
}

pub fn scrape_and_save() -> Result<(), Box<dyn std::error::Error>> {
    // Scrape Vim tips
    let vim_tips = scrape_vim_tips()?;
    let mut f = File::create("src/vim_tips.json")?;
    f.write_all(serde_json::to_string_pretty(&vim_tips)?.as_bytes())?;

    // Get Korean tips
    let korean_tips = get_korean_tips();
    let mut f = File::create("src/korean_tips.json")?;
    f.write_all(serde_json::to_string_pretty(&korean_tips)?.as_bytes())?;

    Ok(())
}

fn scrape_vim_tips() -> Result<Vec<Tip>, Box<dyn std::error::Error>> {
    let url = "https://vim.rtorr.com/";
    let body = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);

    let mut tips = Vec::new();
    let section_selector = Selector::parse("h2").unwrap();
    let item_selector = Selector::parse("li").unwrap();

    for section in document.select(&section_selector) {
        if let Some(list) = section.next_siblings().find(|node| node.value().is_element()) {
            if let Some(element) = ElementRef::wrap(list) {
                 if element.value().name() == "ul" {
                    for item in element.select(&item_selector) {
                        let text = item.text().collect::<String>();
                        let parts: Vec<&str> = text.splitn(2, " - ").collect();
                        if parts.len() == 2 {
                            let command = parts[0].trim().to_string();
                            let description = parts[1].trim().to_string();
                            if !command.is_empty() && !description.is_empty() {
                                tips.push(Tip {
                                    command,
                                    description,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(tips)
}

fn get_korean_tips() -> Vec<Tip> {
    vec![
        Tip { command: "안녕하세요".to_string(), description: "Hello".to_string() },
        Tip { command: "감사합니다".to_string(), description: "Thank you".to_string() },
        Tip { command: "죄송합니다".to_string(), description: "I'm sorry".to_string() },
        Tip { command: "네".to_string(), description: "Yes".to_string() },
        Tip { command: "아니요".to_string(), description: "No".to_string() },
        Tip { command: "사랑해요".to_string(), description: "I love you".to_string() },
        Tip { command: "주세요".to_string(), description: "Please give me".to_string() },
        Tip { command: "얼마예요?".to_string(), description: "How much is it?".to_string() },
        Tip { command: "맛있어요".to_string(), description: "It's delicious".to_string() },
        Tip { command: "도와주세요".to_string(), description: "Please help me".to_string() },
    ]
}
