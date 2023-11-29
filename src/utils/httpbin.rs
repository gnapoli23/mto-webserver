use std::collections::HashMap;

use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpBinPayload {
    pub value: u8,
}

impl HttpBinPayload {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpBinResponse {
    data: String,
}

pub async fn send_request(client: &Client, req: &HttpBinPayload) -> Option<u8> {
    let fut = client.post("https://httpbin.org/post").json(req).send();
    match fut.await {
        Ok(res) => match res.json::<HttpBinResponse>().await {
            Ok(json_res) => {
                if let Ok(data) = serde_json::from_str::<HttpBinPayload>(&json_res.data) {
                    Some(data.value)
                } else {
                    error!("Unable to deserialize response");
                    None
                }
            }
            Err(e) => {
                error!("Unable to deserialize response into JSON: {e}");
                None
            }
        },
        Err(e) => {
            error!("Unable to send request: {e}");
            None
        }
    }
}

pub fn find_mto_numbers(values: Vec<u8>) -> Vec<u8> {
    let mut count_map = HashMap::new();

    // Count occurrences of each number
    values
        .iter()
        .for_each(|v| *count_map.entry(v).or_insert(0) += 1);

    // Find numbers that appear more than once
    let mut mto_values = count_map
        .into_iter()
        .filter_map(|(&num, count)| if count > 1 { Some(num) } else { None })
        .collect::<Vec<u8>>();

    // Sort the result in ascending order
    mto_values.sort();

    mto_values
}

#[cfg(test)]
mod httpbin_tests {
    use crate::utils::httpbin::find_mto_numbers;

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

    #[test]
    fn test_mto() {
        let values = vec![3, 2, 5, 1, 5, 7, 2, 1];
        let expected = vec![1, 2, 5];
        assert_eq!(find_mto_numbers(values), expected);

        let values = vec![5, 7, 7];
        let expected = vec![7];
        assert_eq!(find_mto_numbers(values), expected);
    }
}
