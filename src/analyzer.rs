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

    pub fn analyze(&self, request: &Request<Body>, ip: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = request.uri();

        if self.analyze_query(url.clone())? {
            info!(
                "{} - Detected potential download command in URI: {:?}",
                ip,
                url.to_string()
            );
        }

        return Ok(());
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

        // pipe payload to shell
        if decode.contains("| sh") {
            score += 1.0;
        }

        // change the permission a downloaded file
        if decode.contains("chmod 777") {
            score += 1.0;
        }

        if decode.contains("/bin/sh") {
            score += 0.5;
        }

        for command in DOWNLOAD_COMMANDS {
            if decode.contains(command) {
                score += 1.0;
            }
        }

        return Ok(score > 1.0);
    }
}
