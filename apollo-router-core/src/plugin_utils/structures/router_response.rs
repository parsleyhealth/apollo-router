use super::{from_names_and_values, CompatRequest};
use crate::{Context, Error, Object, Path};
use http::{Response, StatusCode};
use serde_json_bytes::Value;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Default, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct RouterResponse {
    label: Option<String>,
    data: Option<Value>,
    path: Option<Path>,
    has_next: Option<bool>,
    #[builder(setter(!strip_option))]
    errors: Vec<Error>,
    #[builder(default, setter(!strip_option, transform = |extensions: Vec<(&str, Value)>| Some(from_names_and_values(extensions))))]
    extensions: Option<Object>,
    context: Option<Context<CompatRequest>>,
}

impl From<RouterResponse> for crate::RouterResponse {
    fn from(response: RouterResponse) -> Self {
        response.with_status(StatusCode::OK)
    }
}

impl RouterResponse {
    pub fn with_status(self, status: StatusCode) -> crate::RouterResponse {
        crate::RouterResponse {
            response: Response::builder()
                .status(status)
                .body(
                    crate::Response {
                        label: self.label,
                        data: self.data.unwrap_or_default(),
                        path: self.path,
                        has_next: self.has_next,
                        errors: self.errors,
                        extensions: self.extensions.unwrap_or_default(),
                    }
                    .into(),
                )
                .expect("crate::Response implements Serialize; qed")
                .into(),
            context: self
                .context
                .unwrap_or_else(|| Context::new().with_request(Arc::new(Default::default()))),
        }
    }
}
