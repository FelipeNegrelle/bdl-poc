pub mod rest {
    use reqwest::Client;
    use serde::de::DeserializeOwned;
    use std::time::Duration;

    pub struct RestClient {
        client: Client,
        base_url: String,
    }

    impl RestClient {
        pub fn new(base_url: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
            let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .gzip(true)
                .build()?;

            Ok(Self {
                client,
                base_url: base_url.into(),
            })
        }

        pub async fn get<T: DeserializeOwned>(
            &self,
            endpoint: &str,
        ) -> Result<T, Box<dyn std::error::Error>> {
            let url = format!("{}{}", self.base_url, endpoint);

            let response = self.client.get(&url).send().await?;

            if !response.status().is_success() {
                return Err(format!("HTTP error: {}", response.status()).into());
            }

            let data = response.json::<T>().await?;
            Ok(data)
        }

        pub async fn get_text(&self, endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!("{}{}", self.base_url, endpoint);
            let response = self.client.get(&url).send().await?;

            if !response.status().is_success() {
                return Err(format!("HTTP error: {}", response.status()).into());
            }

            let text = response.text().await?;
            Ok(text)
        }

        pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
            &self,
            endpoint: &str,
            body: &B,
        ) -> Result<T, Box<dyn std::error::Error>> {
            let url = format!("{}{}", self.base_url, endpoint);
            let response = self.client.post(&url).json(body).send().await?;

            if !response.status().is_success() {
                return Err(format!("HTTP error: {}", response.status()).into());
            }

            let data = response.json::<T>().await?;
            Ok(data)
        }

        pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
            &self,
            endpoint: &str,
            body: &B,
        ) -> Result<T, Box<dyn std::error::Error>> {
            let url = format!("{}{}", self.base_url, endpoint);
            let response = self.client.put(&url).json(body).send().await?;

            if !response.status().is_success() {
                return Err(format!("HTTP error: {}", response.status()).into());
            }

            let data = response.json::<T>().await?;
            Ok(data)
        }

        pub async fn delete<T: DeserializeOwned>(
            &self,
            endpoint: &str,
        ) -> Result<T, Box<dyn std::error::Error>> {
            let url = format!("{}{}", self.base_url, endpoint);
            let response = self.client.delete(&url).send().await?;

            if !response.status().is_success() {
                return Err(format!("HTTP error: {}", response.status()).into());
            }

            let data = response.json::<T>().await?;
            Ok(data)
        }
    }
}
