pub mod error;
pub mod generation;
pub mod models;

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) host: String,
    pub(crate) reqwest_client: reqwest::Client,
}

impl Ollama {
    pub fn new(host: String) -> Self {
        Self {
            host,
            ..Default::default()
        }
    }

    /// Returns the http URI of the Ollama instance
    pub fn uri(&self) -> String {
        self.host.clone()
    }
}

impl Default for Ollama {
    /// Returns a default Ollama instance with the host set to `http://127.0.0.1:11434`.
    fn default() -> Self {
        Self {
            host: "http://127.0.0.1:11434".to_string(),
            reqwest_client: reqwest::Client::new(),
        }
    }
}
