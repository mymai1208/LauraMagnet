use axum::http::Uri;
use once_cell::sync::OnceCell;

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

    pub fn analyze(&self, uri: Uri) -> Result<(), Box<dyn std::error::Error>> {
        if !self.analyze_query(uri.clone())? {
            return Ok(());
        }

        println!(
            "Detected potential download command in URI: {:?}",
            uri.to_string()
        );

        return Ok(());
    }

    fn analyze_query(&self, uri: Uri) -> Result<bool, Box<dyn std::error::Error>> {
        let mut score = 0;

        if uri.query().is_none() {
            return Ok(false);
        }

        let query = uri.query().unwrap();
        let decode = percent_encoding::percent_decode_str(query)
            .decode_utf8()?
            .to_string()
            .replace('+', " ");

        if decode.contains("| sh") {
            score += 1;
        }

        for command in DOWNLOAD_COMMANDS {
            if decode.contains(command) {
                score += 1;
            }
        }

        return Ok(score > 1);
    }
}
