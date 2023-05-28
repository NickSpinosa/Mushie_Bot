pub mod secrets {
    use once_cell::sync::Lazy;
    use securestore::{KeySource, SecretsManager};
    use std::path::Path;

    static SECRETS: Lazy<SecretsManager> = Lazy::new(|| {
        let keyfile = Path::new("secure/secrets.key");
        SecretsManager::load("secure/secrets.json", KeySource::File(keyfile))
            .expect("Failed to load SecureStore vault!")
    });

    pub fn get_discord_token() -> Result<String, securestore::Error> {
        SECRETS.get("discord_token")
    }
}
