use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct CopyTemplatesRequest {
    #[serde(rename = "SourceServerID")]
    pub source_server_id: i32,
    #[serde(rename = "DestinationServerID")]
    pub destination_server_id: i32,
    pub perform_changes: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CopyTemplatesResponse {
    pub total_count: i32,
    pub templates: Vec<Template>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    pub action: String,
    pub template_id: i32,
    pub alias: String,
    pub name: String,
    pub template_type: String,
}

impl Endpoint for CopyTemplatesRequest {
    type Request = CopyTemplatesRequest;
    type Response = CopyTemplatesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/templates/push".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    #[tokio::test]
    pub async fn push_templates() {
        let server = Server::run();

        const SOURCE_SERVER: i32 = 12345;
        const DESTINATION_SERVER: i32 = 23456;

        server.expect(
            Expectation::matching(request::method_path("PUT", "/templates/push")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Templates": [
                        {
                            "Action": "Create",
                            "TemplateId": 7270,
                            "Alias": "comment-notification",
                            "Name": "Comment notification",
                            "TemplateType": "Standard"
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CopyTemplatesRequest::builder()
            .source_server_id(SOURCE_SERVER)
            .destination_server_id(DESTINATION_SERVER)
            .perform_changes(true)
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "SourceServerID": SOURCE_SERVER,
                "DestinationServerID": DESTINATION_SERVER,
                "PerformChanges": true,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
