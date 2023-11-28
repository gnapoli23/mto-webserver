use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HttpBinPayload {
    pub value: u8,
}

impl HttpBinPayload {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HttpBinResponse {
    data: String,
}

#[cfg(test)]
mod httpbin_tests {
    use super::{HttpBinPayload, HttpBinResponse};
    use reqwest::Client;

    #[tokio::test]
    async fn test_serde() {
        // Send a simple request and deserialize the `data` field from the response
        let client = Client::new();
        let req = HttpBinPayload::new(1);
        let res = client
            .post("https://httpbin.org/post")
            .json(&req)
            .send()
            .await
            .unwrap()
            .json::<HttpBinResponse>()
            .await
            .map(|resp| serde_json::from_str::<HttpBinPayload>(&resp.data))
            .unwrap()
            .unwrap();
        assert_eq!(res.value, 1)
    }
}
