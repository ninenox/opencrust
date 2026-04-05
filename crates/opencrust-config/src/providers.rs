/// Known LLM provider registry. Single source of truth for provider metadata
/// used by the wizard, router, and anywhere else that needs to enumerate providers.
pub struct KnownProvider {
    pub id: &'static str,
    pub display_name: &'static str,
    pub env_var: &'static str,
    pub default_base_url: Option<&'static str>,
    pub default_model: Option<&'static str>,
    pub requires_api_key: bool,
    pub is_local: bool,
}

pub static KNOWN_PROVIDERS: &[KnownProvider] = &[
    KnownProvider {
        id: "anthropic",
        display_name: "Anthropic Claude",
        env_var: "ANTHROPIC_API_KEY",
        default_base_url: None,
        default_model: None,
        requires_api_key: true,
        is_local: false,
    },
    KnownProvider {
        id: "openai",
        display_name: "OpenAI",
        env_var: "OPENAI_API_KEY",
        default_base_url: None,
        default_model: None,
        requires_api_key: true,
        is_local: false,
    },
    KnownProvider {
        id: "ollama",
        display_name: "Ollama (local)",
        env_var: "OLLAMA_API_KEY",
        default_base_url: Some("http://localhost:11434"),
        default_model: Some("llama3.1"),
        requires_api_key: false,
        is_local: true,
    },
    KnownProvider {
        id: "sansa",
        display_name: "Sansa",
        env_var: "SANSA_API_KEY",
        default_base_url: None,
        default_model: None,
        requires_api_key: true,
        is_local: false,
    },
    KnownProvider {
        id: "deepseek",
        display_name: "DeepSeek",
        env_var: "DEEPSEEK_API_KEY",
        default_base_url: None,
        default_model: None,
        requires_api_key: true,
        is_local: false,
    },
    KnownProvider {
        id: "mistral",
        display_name: "Mistral",
        env_var: "MISTRAL_API_KEY",
        default_base_url: None,
        default_model: None,
        requires_api_key: true,
        is_local: false,
    },
    KnownProvider {
        id: "gemini",
        display_name: "Gemini",
        env_var: "GEMINI_API_KEY",
        default_base_url: None,
        default_model: None,
        requires_api_key: true,
        is_local: false,
    },
    KnownProvider {
        id: "vllm",
        display_name: "vLLM (self-hosted)",
        env_var: "VLLM_API_KEY",
        default_base_url: Some("http://localhost:8000"),
        default_model: None,
        requires_api_key: false,
        is_local: true,
    },
];

pub fn find_provider(id: &str) -> Option<&'static KnownProvider> {
    KNOWN_PROVIDERS.iter().find(|p| p.id == id)
}

pub fn env_var_for_provider(id: &str) -> &'static str {
    find_provider(id).map(|p| p.env_var).unwrap_or("API_KEY")
}
