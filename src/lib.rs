mod pdk;

use extism_pdk::*;
use pdk::types::{CallToolResult, Content, ContentType, ToolDescription, ListToolsResult};
use pdk::*;
use serde_json::json;
use std::error::Error as StdError;

use chrono::Utc;

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for CustomError {}

// Called when the tool is invoked.
pub(crate) fn call(input: types::CallToolRequest) -> Result<types::CallToolResult, Error> {
    let args = input.params.arguments.unwrap_or_default();
    let name = args.get("name").unwrap().as_str().unwrap();
    match name {
        "get_time_utc" => {
            let now = Utc::now();
            let timestamp = now.timestamp().to_string();
            let rfc2822 = now.to_rfc2822().to_string();
            Ok(CallToolResult {
                content: vec![Content {
                    text: Some(json!({
                        "utc_time": timestamp,
                        "utc_time_rfc2822": rfc2822,
                    }).to_string()),
                    r#type: ContentType::Text,
                    ..Default::default()
                }],
                is_error: Some(false),
            })
        }
        "parse_time" => {
            let time = args.get("time_rfc2822").unwrap().as_str().unwrap();
            let t = chrono::DateTime::parse_from_rfc2822(time).unwrap();
            let timestamp = t.timestamp().to_string();
            let rfc2822 = t.to_rfc2822().to_string();
            Ok(CallToolResult {
                content: vec![Content {
                    text: Some(json!({
                        "utc_time": timestamp,
                        "utc_time_rfc2822": rfc2822,
                    }).to_string()),
                    r#type: ContentType::Text,
                    ..Default::default()
                }],
                is_error: Some(false),
            })
        }
        "time_offset" => {
            let t1 = args.get("timestamp").unwrap().as_i64().unwrap();
            let offset = args.get("offset").unwrap().as_i64().unwrap();
            let t1 = chrono::DateTime::from_timestamp(t1, 0).unwrap();
            let t2 = t1 + chrono::Duration::seconds(offset);
            let timestamp = t2.timestamp().to_string();
            let rfc2822 = t2.to_rfc2822().to_string();
            Ok(CallToolResult {
                content: vec![Content {
                    text: Some(json!({
                        "utc_time": timestamp,
                        "utc_time_rfc2822": rfc2822,
                    }).to_string()),
                    r#type: ContentType::Text,
                    ..Default::default()
                }],
                is_error: Some(false),
            })
        }
        _ => Err(Error::new(CustomError("unknown command".to_string()))),
    }
}


// Describe the available functions (tools) and their argument schemas.
pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![
            ToolDescription {
                name: "get_time_utc".into(),
                description: "Get the current UTC time as a UNIX timestamp and RFC 2822 string".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": [],
                    "additionalProperties": false
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "parse_time".into(),
                description: "Parse an RFC 2822 datetime string and return UTC timestamp and RFC 2822".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "time_rfc2822": {
                            "type": "string",
                            "description": "Datetime in RFC 2822 format, e.g. 'Mon, 02 Jan 2006 15:04:05 +0000'"
                        }
                    },
                    "required": ["time_rfc2822"],
                    "additionalProperties": false
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "time_offset".into(),
                description: "Apply an offset (in seconds) to a UTC timestamp and return the new time".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "timestamp": {
                            "type": "integer",
                            "description": "Base UNIX timestamp (seconds)"
                        },
                        "offset": {
                            "type": "integer",
                            "description": "Offset in seconds to add (negative to subtract)"
                        }
                    },
                    "required": ["timestamp", "offset"],
                    "additionalProperties": false
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}




