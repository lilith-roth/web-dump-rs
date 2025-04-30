use lol_html::{HtmlRewriter, Settings, element};
use reqwest::blocking::{Client, RequestBuilder, Response};

pub(crate) fn retrieve_content_from_web_server(
    download_url: &str,
    client: &Client,
) -> Option<Response> {
    let request: RequestBuilder = client.get(download_url);
    let response: Response = match request.send() {
        Ok(res) => res,
        Err(_) => return None,
    };
    let response_status_code = response.status();
    // ToDo: Make okay status code configurable
    if response_status_code != 200 {
        return None;
    }
    Some(response)
}

pub(crate) fn parse_html_and_search_links(
    web_content_text: &str,
    base_domain: &str,
    crawl_external_domains: bool,
) -> Vec<String> {
    let mut links: Vec<String> = vec![];
    let mut parser: HtmlRewriter<fn(&[u8])> = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("a[href]", |el| {
                let href_raw: Option<String> = el.get_attribute("href");
                let href = href_raw.unwrap_or_else(|| {
                    log::warn!("<a> tag without `href` found!");
                    String::from("")
                });
                log::info!("Saving or no saving!");
                if (!href.contains(base_domain) || crawl_external_domains) && !href.is_empty() {
                    links.push(href);
                }
                Ok(())
            })],
            ..Settings::new()
        },
        |_: &[u8]| (),
    );
    parser
        .write(web_content_text.as_ref())
        .expect("Could not parse HTML!");
    parser.end().expect("Could not finalize HTML parser!");
    links
}

pub(crate) fn check_online(url: &str) -> reqwest::Result<Response> {
    let client: Client = Client::new();
    let request: RequestBuilder = client.get(url);
    request.send()
}
