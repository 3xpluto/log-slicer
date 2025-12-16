use anyhow::Result;
use serde_json::{json, Map, Value};
use std::io::Write;

use crate::cli::{Args, Output};
use crate::engine::record::Record;

#[derive(Debug, Clone)]
pub struct EmitPlan {
    pub mode: Output,
    pub field: Option<String>,
    pub select: Vec<String>,
}

impl EmitPlan {
    pub fn build(args: &Args) -> Result<Self> {
        let select = args
            .select
            .as_deref()
            .unwrap_or("")
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Self {
            mode: args.output,
            field: args.field.clone(),
            select,
        })
    }
}

pub fn emit_record(w: &mut dyn Write, rec: &Record, plan: &EmitPlan) -> Result<()> {
    match plan.mode {
        Output::Plain => writeln!(w, "{}", rec.raw)?,
        Output::Field => {
            let f = plan
                .field
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("--output field requires --field"))?;
            let s = rec.get_field_string(f).unwrap_or_default();
            writeln!(w, "{s}")?;
        }
        Output::Ndjson => {
            if !plan.select.is_empty() {
                let obj = select_fields(rec, &plan.select);
                writeln!(w, "{}", Value::Object(obj))?;
                return Ok(());
            }

            if let Some(v) = rec.json.as_ref() {
                writeln!(w, "{v}")?;
            } else {
                writeln!(w, "{}", json!({ "line": rec.raw }))?;
            }
        }
    }
    Ok(())
}

fn select_fields(rec: &Record, fields: &[String]) -> Map<String, Value> {
    let mut out = Map::new();
    for f in fields {
        let key = f.replace('.', "_");
        let v = rec.get_field(f).cloned().unwrap_or(Value::Null);
        out.insert(key, v);
    }
    out
}
