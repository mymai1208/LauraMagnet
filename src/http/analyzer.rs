use axum::{body::Body, extract::Request, http::Uri};
use futures::select;
use once_cell::sync::OnceCell;
use tracing::{debug, field::debug, info};

use crate::{structs::Analyzer, utils::url_decode};

static INSTANCE: OnceCell<Analyzer> = OnceCell::new();

const DOWNLOAD_COMMANDS: [&str; 2] = ["wget", "curl"];

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

        let decode = url_decode(path.to_string());

        if decode.ends_with(".env") {
            return Ok(true);
        }

        if decode.ends_with("/config") {
            return Ok(true);
        }

        if decode.ends_with("/eval-stdin.php") {
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
        let decode = url_decode(query.to_string());

        // pipe payload to shell
        if decode.contains("| sh") {
            score += 1.0;
        }

        // change the permission a downloaded file
        if decode.contains("chmod 777") {
            score += 1.0;
        }

        for command in DOWNLOAD_COMMANDS {
            if decode.contains(command) {
                score += 1.0;
            }
        }
        return Ok(score > 1.0);
    }
}
