pub mod accounts;
pub mod currencies;
pub mod data;
pub mod transactions;

use std::time::Duration;

use reqwest::{Method, Url};

pub struct Client {
    client: reqwest::Client,
    token: String,
    site: Url,
}

impl Client {
    pub fn new<S>(site: S, token: S) -> Self
    where
        S: AsRef<str>,
    {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json, */*;q=0.5"),
        );
        headers.insert(
            reqwest::header::CONNECTION,
            reqwest::header::HeaderValue::from_static("keep-alive"),
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .user_agent(env!("CARGO_PKG_NAME"))
            .http1_title_case_headers()
            .redirect(reqwest::redirect::Policy::none())
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            client,
            token: token.as_ref().to_string(),
            site: Url::parse(site.as_ref()).unwrap(),
        }
    }

    fn request_builder<S: AsRef<str>>(&self, method: Method, path: S) -> reqwest::RequestBuilder {
        self.client
            .request(method, self.site.join(path.as_ref()).unwrap())
            .bearer_auth(&self.token)
    }

    async fn send(&self, req: reqwest::Request) -> serde_json::Value {
        let res = self.client.execute(req).await.unwrap();
        let code = res.status();
        let text = res.text().await.unwrap();

        match code.is_success() {
            true => {
                if text.is_empty() {
                    serde_json::Value::Null
                } else {
                    match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("code: {}", code);
                            panic!("{} -: {}", e, text);
                        }
                    }
                }
            }
            false => {
                panic!("{}", text);
            }
        }
    }
}
