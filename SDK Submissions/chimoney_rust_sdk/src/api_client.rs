use reqwest::Client;

#[derive()]
pub struct APIClient {
    production_base_url: String,
    sandbox_base_url: String,
    content_type: String,
    accept: String,
}

// impl APIClient
impl APIClient {
    pub fn new() -> APIClient {
        APIClient {
            production_base_url: "https://api.chimoney.io".to_string(),
            sandbox_base_url: "https://api-v2-sandbox.chimoney.io".to_string(),
            content_type: "application/json".to_string(),
            accept: "application/json".to_string(),
        }
    }

    pub async fn get(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.production_base_url, path);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn post(&self, path: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.production_base_url, path);
        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn put(&self, path: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.production_base_url, path);
        let client = Client::new();
        let res = client
            .put(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn delete(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.production_base_url, path);
        let client = Client::new();
        let res = client
            .delete(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn get_sandbox(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.sandbox_base_url, path);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn post_sandbox(
        &self,
        path: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.sandbox_base_url, path);
        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn put_sandbox(
        &self,
        path: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.sandbox_base_url, path);
        let client = Client::new();
        let res = client
            .put(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn delete_sandbox(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.sandbox_base_url, path);
        let client = Client::new();
        let res = client
            .delete(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    // Methods for handling authentication
    pub async fn get_auth(
        &self,
        path: &str,
        token: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.production_base_url, path);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn post_auth(
        &self,
        path: &str,
        body: &str,
        token: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.production_base_url, path);
        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .header("Authorization", format!("Bearer {}", token))
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    // Methods for sending requests with parameters/payloads
    pub async fn get_with_params(
        &self,
        path: &str,
        params: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.production_base_url, path, params);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn post_with_params(
        &self,
        path: &str,
        params: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.production_base_url, path, params);
        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn put_with_params(
        &self,
        path: &str,
        params: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.production_base_url, path, params);
        let client = Client::new();
        let res = client
            .put(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn delete_with_params(
        &self,
        path: &str,
        params: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.production_base_url, path, params);
        let client = Client::new();
        let res = client
            .delete(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn get_sandbox_with_params(
        &self,
        path: &str,
        params: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.sandbox_base_url, path, params);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn post_sandbox_with_params(
        &self,
        path: &str,
        params: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.sandbox_base_url, path, params);
        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn put_sandbox_with_params(
        &self,
        path: &str,
        params: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.sandbox_base_url, path, params);
        let client = Client::new();
        let res = client
            .put(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .body(body.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn delete_sandbox_with_params(
        &self,
        path: &str,
        params: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}.await?{}", self.sandbox_base_url, path, params);
        let client = Client::new();
        let res = client
            .delete(&url)
            .header("Content-Type", self.content_type.clone())
            .header("Accept", self.accept.clone())
            .send()
            .await?;
        Ok(res.text().await?)
    }

    // Methods for handling responses and error handling
    pub async fn handle_response(
        &self,
        response: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response_json: serde_json::Value = serde_json::from_str(&response)?;
        let response_code = response_json["code"].as_u64().unwrap();
        let response_message = response_json["message"].as_str().unwrap();
        if response_code == 200 {
            Ok(response_message.to_string())
        } else {
            Err(response_message.to_string().into())
        }
    }

    pub async fn handle_error(&self, error: String) -> Result<String, Box<dyn std::error::Error>> {
        let error_json: serde_json::Value = serde_json::from_str(&error)?;
        let error_code = error_json["code"].as_u64().unwrap();
        let error_message = error_json["message"].as_str().unwrap();
        if error_code == 400 {
            return Err(error_message.to_string().into());
        } else {
            return Err(error_message.to_string().into());
        }
    }

    pub async fn handle_error_with_code(
        &self,
        error: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let error_json: serde_json::Value = serde_json::from_str(&error)?;
        let error_code = error_json["code"].as_u64().unwrap();
        let error_message = error_json["message"].as_str().unwrap();
        if error_code == 400 {
            Err(error_message.to_string().into())
        } else {
            Err(error_message.to_string().into())
        }
    }

    pub async fn handle_error_with_code_and_message(
        &self,
        error: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let error_json: serde_json::Value = serde_json::from_str(&error)?;
        let error_code = error_json["code"].as_u64().unwrap();
        let error_message = error_json["message"].as_str().unwrap();
        if error_code == 400 {
            Err(error_message.to_string().into())
        } else {
            Err(error_message.to_string().into())
        }
    }
}
