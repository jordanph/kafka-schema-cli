use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
struct CompatibilityRequestBody<'a> {
    schema: &'a str,
}

#[derive(Deserialize)]
struct CompatibilityResponseBody {
    is_compatible: bool,
}

pub struct SchemaRegistryClient<'a> {
    pub base_url: &'a str,
}

impl<'a> SchemaRegistryClient<'a> {
    pub async fn check_schema_compatibility(
        &self,
        schema_name: &str,
        schema_canonical_form: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let request_url = format!(
            "{}/compatibility/subjects/{}/versions/latest",
            self.base_url, schema_name
        );

        let compatibility_request_body = CompatibilityRequestBody {
            schema: schema_canonical_form,
        };

        let compatibility_response = reqwest::Client::new()
            .post(&request_url)
            .header(CONTENT_TYPE, "application/vnd.schemaregistry.v1+json")
            .header(USER_AGENT, "schema-registry-cli")
            .json(&compatibility_request_body)
            .send()
            .await?;

        match compatibility_response.status() {
            StatusCode::OK => {
                let response_body = compatibility_response
                    .json::<CompatibilityResponseBody>()
                    .await?;

                Ok(response_body.is_compatible)
            }
            StatusCode::NOT_FOUND => Ok(true),
            other_status_code => {
                println!(
                    "Unexpected response from the Schema Registry...{}",
                    other_status_code
                );
                Err(compatibility_response.text().await?.into())
            }
        }
    }

    pub async fn migrate_schema(
        &self,
        schema_name: &str,
        schema_canonical_form: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request_url = format!("{}/subjects/{}/versions", self.base_url, schema_name);

        let compatibility_request_body = CompatibilityRequestBody {
            schema: schema_canonical_form,
        };

        let compatibility_response = reqwest::Client::new()
            .post(&request_url)
            .header(CONTENT_TYPE, "application/vnd.schemaregistry.v1+json")
            .header(USER_AGENT, "schema-registry-cli")
            .json(&compatibility_request_body)
            .send()
            .await?;

        match compatibility_response.status() {
            StatusCode::OK => Ok(()),
            other_status_code => {
                println!(
                    "Unexpected response from the Schema Registry...{}",
                    other_status_code
                );
                Err(compatibility_response.text().await?.into())
            }
        }
    }
}
