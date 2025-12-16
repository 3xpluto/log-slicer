use anyhow::{Context, Result};
use serde_json::Value;
use time::{format_description, OffsetDateTime};

use crate::cli::Input;

#[derive(Debug, Clone)]
pub struct Record {
    pub raw: String,
    pub json: Option<Value>,
}

impl Record {
    pub fn from_line(
        line: std::result::Result<String, std::io::Error>,
        input: Input,
    ) -> Result<Self> {
        let raw = line.context("failed to read line")?;
        let json = match input {
            Input::Text => None,
            Input::Json => Some(parse_json(&raw)?),
            Input::Auto => {
                if looks_like_json(&raw) {
                    serde_json::from_str::<Value>(&raw).ok()
                } else {
                    None
                }
            }
        };
        Ok(Self { raw, json })
    }

    pub fn get_field(&self, dotted: &str) -> Option<&Value> {
        let v = self.json.as_ref()?;
        get_dotted(v, dotted)
    }

    pub fn get_timestamp(&self, dotted: &str) -> Option<OffsetDateTime> {
        let s = self.get_field_string(dotted)?;
        parse_timestamp(&s).ok()
    }

    pub fn get_field_string(&self, dotted: &str) -> Option<String> {
        let v = self.get_field(dotted)?;
        match v {
            Value::String(s) => Some(s.clone()),
            other => Some(other.to_string()),
        }
    }
}

fn looks_like_json(s: &str) -> bool {
    let t = s.trim_start();
    t.starts_with('{') || t.starts_with('[')
}

fn parse_json(s: &str) -> Result<Value> {
    serde_json::from_str::<Value>(s).context("failed to parse JSON line")
}

fn get_dotted<'a>(v: &'a Value, dotted: &str) -> Option<&'a Value> {
    let mut cur = v;
    for part in dotted.split('.') {
        match cur {
            Value::Object(map) => cur = map.get(part)?,
            Value::Array(arr) => {
                let idx: usize = part.parse().ok()?;
                cur = arr.get(idx)?;
            }
            _ => return None,
        }
    }
    Some(cur)
}

fn parse_timestamp(s: &str) -> Result<OffsetDateTime> {
    // Prefer RFC3339 (e.g. 2025-12-15T09:00:01Z)
    if let Ok(dt) = OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339) {
        return Ok(dt);
    }

    // Common "YYYY-MM-DD HH:MM:SS" assumed UTC
    let fmt = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
        .context("failed to build time format")?;
    let prim = time::PrimitiveDateTime::parse(s, &fmt).context("failed to parse timestamp")?;
    Ok(prim.assume_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotted_paths_work() {
        let v: Value = serde_json::json!({"a": {"b": {"c": 1}}, "arr": [ {"x": "y"} ]});
        assert_eq!(get_dotted(&v, "a.b.c").unwrap(), &serde_json::json!(1));
        assert_eq!(get_dotted(&v, "arr.0.x").unwrap(), &serde_json::json!("y"));
        assert!(get_dotted(&v, "missing").is_none());
    }
}
