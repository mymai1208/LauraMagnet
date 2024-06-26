use axum::{body::Body, extract::Request, http::Uri};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use once_cell::sync::OnceCell;
use regex::Regex;
use tracing::{info, warn};

use crate::{structs::Analyzer, utils::url_decode};

static INSTANCE: OnceCell<Analyzer> = OnceCell::new();

//const DOWNLOAD_COMMANDS: [&str; 2] = ["wget", "curl"];

impl Analyzer {
    pub fn global() -> &'static Analyzer {
        INSTANCE.get_or_init(Analyzer::new)
    }

    fn new() -> Self {
        Analyzer {}
    }

    pub fn analyze(&self, request: &Request<Body>) -> Result<(), Box<dyn std::error::Error>> {
        let url = request.uri();

        if self.analyze_query(url.clone())? {
            info!("detected command injection");
        }

        if self.analyze_access_path(url.clone())? {
            info!("detected threat request: {:?}", url);
        }

        return Ok(());
    }

    fn analyze_access_path(&self, uri: Uri) -> Result<bool, Box<dyn std::error::Error>> {
        let path = uri.path();

        let decode = url_decode(path.to_string())?;

        if decode.ends_with(".env") {
            return Ok(true);
        }

        if decode.ends_with("/config") {
            return Ok(true);
        }

        if decode.ends_with("/eval-stdin.php") {
            return Ok(true);
        }

        if decode.ends_with("/credentials") {
            return Ok(true);
        }

        let cd_back_count = decode.matches("../").count() as f64;

        if decode.contains("/bin/sh") && cd_back_count * 0.2 + 0.5 > 1.0 {
            info!("detected shell hijack");

            return Ok(true);
        }

        return Ok(false);
    }

    fn analyze_query(&self, uri: Uri) -> Result<bool, Box<dyn std::error::Error>> {
        let mut score = 0.0;

        if uri.query().is_none() {
            return Ok(false);
        }

        let query = uri.query().unwrap();
        let queries = query.split('&').collect::<Vec<&str>>();

        for query in queries {
            let args = query.split('=').collect::<Vec<&str>>();
            let decode = url_decode(query.to_string());

            if decode.is_err() {
                warn!("failed to decode query: {:?}", decode.err().unwrap());
                continue;
            }

            score += self.is_suspicion_score(&decode.unwrap());

            if args.len() < 2 {
                continue;
            }

            if args[1].len() > 50 {
                let base64 = BASE64_STANDARD_NO_PAD.decode(args[1].as_bytes());

                if base64.is_err() {
                    score += 0.2;
                    continue;
                }

                let payload = String::from_utf8(base64.unwrap())?;

                info!("detected base64 payload: {}", payload);

                score += self.is_suspicion_score(&payload)
            }
        }

        return Ok(score >= 1.0);
    }

    fn is_suspicion_score(&self, message: &str) -> f64 {
        let mut score = 0.0;
        // using pipe a sending the payload to shell
        if Regex::new(r"\|(\s{0,1})sh").unwrap().is_match(message) {
            score += 1.0;
        }

        // change the permission a downloaded file
        if message.contains("chmod 777") {
            score += 1.0;
        }

        if message.contains("rm -") {
            score += 1.0;
        }

        if message.contains("wget ") {
            score += 1.0;
        }

        return score;
    }
}
