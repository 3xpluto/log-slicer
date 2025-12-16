use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;

use crate::engine::record::Record;

#[derive(Default, Debug)]
pub struct Stats {
    pub seen: usize,
    pub matched: usize,
    top: HashMap<String, usize>,
}

impl Stats {
    pub fn observe(&mut self, rec: &Record, field: Option<&str>) {
        if let Some(f) = field {
            if let Some(v) = rec.get_field(f) {
                self.bump(v);
            }
        }
    }

    fn bump(&mut self, v: &Value) {
        let key = match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        *self.top.entry(key).or_insert(0) += 1;
    }

    pub fn print(&self, w: &mut dyn Write, field: Option<&str>) -> Result<()> {
        writeln!(w, "seen: {}", self.seen)?;
        writeln!(w, "matched: {}", self.matched)?;
        if let Some(f) = field {
            writeln!(w, "top values for field '{f}':")?;
            let mut pairs: Vec<_> = self.top.iter().collect();
            pairs.sort_by_key(|(_, c)| std::cmp::Reverse(**c));
            for (k, c) in pairs.into_iter().take(20) {
                writeln!(w, "  {c:>6}  {k}")?;
            }
        }
        Ok(())
    }
}
