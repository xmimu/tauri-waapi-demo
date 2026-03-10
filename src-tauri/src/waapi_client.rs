use std::collections::HashSet;

use serde_json::{Map, Value};

use crate::state::AppState;

const DEFAULT_RETURN_FIELDS: [&str; 4] = ["id", "name", "type", "notes"];

pub async fn ensure_client(state: &mut AppState) -> Result<(), String> {
    if state.client.is_none() {
        state.client = Some(
            waapi_rs::WaapiClient::connect()
                .await
                .map_err(|err| err.to_string())?,
        );
    }
    Ok(())
}

pub async fn call_json(
    state: &mut AppState,
    uri: &str,
    args: Option<Value>,
    options: Option<Value>,
) -> Result<Value, String> {
    ensure_client(state).await?;

    let call_result = state
        .client
        .as_ref()
        .expect("client should exist after ensure_client")
        .call(uri, args, options)
        .await;

    match call_result {
        Ok(value) => Ok(value.unwrap_or(Value::Object(Map::new()))),
        Err(err) => {
            state.client = None;
            Err(err.to_string())
        }
    }
}

pub fn parse_return_fields(return_str: Option<&str>) -> Vec<String> {
    let parsed: Vec<String> = return_str
        .map(str::trim)
        .filter(|input| !input.is_empty())
        .map(|input| {
            input
                .split_whitespace()
                .map(str::trim)
                .filter(|field| !field.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let source_fields = if parsed.is_empty() {
        DEFAULT_RETURN_FIELDS
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    } else {
        parsed
    };

    let mut seen = HashSet::new();
    source_fields
        .into_iter()
        .filter(|field| seen.insert(field.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_return_fields;

    #[test]
    fn parse_return_fields_uses_default_when_missing() {
        assert_eq!(
            parse_return_fields(None),
            vec!["id", "name", "type", "notes"]
        );
    }

    #[test]
    fn parse_return_fields_uses_default_when_blank() {
        assert_eq!(
            parse_return_fields(Some("   ")),
            vec!["id", "name", "type", "notes"]
        );
    }

    #[test]
    fn parse_return_fields_deduplicates_fields() {
        assert_eq!(
            parse_return_fields(Some("id name id type name")),
            vec!["id", "name", "type"]
        );
    }
}
