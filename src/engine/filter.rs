use anyhow::{Context, Result};
use regex::Regex;
use time::OffsetDateTime;

use crate::cli::Args;
use crate::engine::record::Record;

#[derive(Debug)]
pub struct Filter {
    contains: Option<String>,
    regex: Option<Regex>,
    field: Option<String>,
    equals: Option<String>,
    since: Option<OffsetDateTime>,
    until: Option<OffsetDateTime>,
    time_field: String,
}

impl Filter {
    pub fn build(args: &Args) -> Result<Self> {
        let since = if let Some(s) = &args.since {
            Some(
                OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
                    .with_context(|| format!("invalid --since RFC3339: {s}"))?,
            )
        } else {
            None
        };

        let until = if let Some(s) = &args.until {
            Some(
                OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
                    .with_context(|| format!("invalid --until RFC3339: {s}"))?,
            )
        } else {
            None
        };

        let regex = if let Some(pat) = &args.regex {
            Some(Regex::new(pat).with_context(|| format!("invalid regex: {pat}"))?)
        } else {
            None
        };

        Ok(Self {
            contains: args.contains.clone(),
            regex,
            field: args.field.clone(),
            equals: args.equals.clone(),
            since,
            until,
            time_field: args.time_field.clone(),
        })
    }

    pub fn matches(&self, rec: &Record) -> bool {
        if self.since.is_some() || self.until.is_some() {
            let ts = match rec.get_timestamp(&self.time_field) {
                Some(t) => t,
                None => return false, // if asked to time-slice, require parseable timestamp
            };

            if let Some(since) = self.since {
                if ts < since {
                    return false;
                }
            }
            if let Some(until) = self.until {
                if ts > until {
                    return false;
                }
            }
        }

        let target = if let Some(field) = &self.field {
            rec.get_field_string(field).unwrap_or_default()
        } else {
            rec.raw.clone()
        };

        if let Some(eq) = &self.equals {
            if target != *eq {
                return false;
            }
        }

        if let Some(substr) = &self.contains {
            if !target.contains(substr) {
                return false;
            }
        }

        if let Some(re) = &self.regex {
            if !re.is_match(&target) {
                return false;
            }
        }

        true
    }
}
