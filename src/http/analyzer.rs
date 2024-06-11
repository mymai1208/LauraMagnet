use axum::{
    body::Body,
    extract::Request,
    http::Uri,
};
use once_cell::sync::OnceCell;
use tracing::info;

use crate::structs::Analyzer;

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
            info!(
                "Detected potential download command in URI: {:?}",
                url.to_string()
            );
        }

        if self.analyze_access_path(url.clone())? {
            info!(
                "Detected potential access to sensitive path in URI: {:?}",
                url.to_string()
            );
        }

        return Ok(());
    }

    fn analyze_access_path(&self, uri: Uri) -> Result<bool, Box<dyn std::error::Error>> {
        let path = uri.path();

        if path.ends_with(".env") {
            return Ok(true);
        }

        if path.ends_with("/config") {
            return Ok(true);
        }

        if path.ends_with("/eval-stdin.php") {
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
        let decode = percent_encoding::percent_decode_str(query)
            .decode_utf8()?
            .to_string()
            .replace('+', " ");

        let cd_back_count = decode.matches("/..").count() as f64;

        // pipe payload to shell
        if decode.contains("| sh") {
            score += 1.0;
        }

        // change the permission a downloaded file
        if decode.contains("chmod 777") {
            score += 1.0;
        }

        if decode.contains("/bin/sh") {
            score += (cd_back_count * 0.2) + 0.5;
        }

        for command in DOWNLOAD_COMMANDS {
            if decode.contains(command) {
                score += 1.0;
            }
        }
        return Ok(score > 1.0);
    }
}
