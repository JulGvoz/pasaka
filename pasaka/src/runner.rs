pub mod cli;
pub use cli::CliRunner;

#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "web")]
pub use web::WebRunner;

use crate::Passage;

pub struct DefaultRunner;

impl DefaultRunner {
    pub fn run(passage: Passage) {
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        {
            WebRunner::run(passage);
        }
        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        {
            CliRunner::run(passage);
        }
    }
}
