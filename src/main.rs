
use std::process;

use anyhow::{Ok, Result};
use html_parser::{Dom, Element, Node};

fn is_node(node: &Node) -> bool {
    match node {
        Node::Element(..) => true,
        _ => false
    }
}



fn crawl_element(el: Element) -> Result<Vec<String>> {
    let mut links = Vec::new();
   if el.name == "a" {
        let href_attrib = el
            .attributes
            .iter()
            .filter(|(name, _)| name.as_str() == "href")
            .last()
            .ok_or_else(|| anyhow::anyhow!("no href found in a"));

        match href_attrib {
            Result::Ok((_key, Some(val))) => {
                log::info!("found link: {}", val);
                links.push(val.to_string());
            },
            _ => {
                log::error!("no link found for element {}", el.name);
            }
        }
    }
    for node in el
        .children
        .iter()
        .filter(|c| is_node(c)){
            match node {
                Node::Element(elem) => {
                    let mut children_links = crawl_element(elem.clone())?;
                    links.append(&mut children_links);
                },
                _ => {}
            }
        }

    Ok(links)
}

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    let html = reqwest::get(url).await?.text().await?;
    let dom = Dom::parse(&html)?;

    for child in dom.children {
        match child {
            Node::Element(elem) => {
                log::info!("links found for element {}: {:?}", elem.name.clone(), crawl_element(elem))
            },
            _ => {}
        }
    }
    let res = Vec::new();
    Ok(res)
}



async fn try_main() -> Result<()> {
    crawl_url("https://google.com").await?;
log::info!("hello world");
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    match try_main().await {
        Result::Ok(_) => {
            log::info!("Finished")
        },
        Err(e) => {
            log::error!("Error: {:?}", e);
            process::exit(-1)
        }
    }

}