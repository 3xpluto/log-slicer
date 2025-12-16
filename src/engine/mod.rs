pub mod record;

mod filter;
mod read;
mod stats;

use anyhow::Result;
use std::collections::VecDeque;
use std::io::{self, Write};

use crate::cli::Args;
use crate::engine::filter::Filter;
use crate::engine::stats::Stats;
use crate::output::{emit_record, EmitPlan};

pub fn run(args: Args) -> Result<()> {
    let filter = Filter::build(&args)?;
    let plan = EmitPlan::build(&args)?;

    let mut stats = Stats::default();
    let mut matched: usize = 0;

    let mut tail_buf: Option<VecDeque<crate::engine::record::Record>> =
        args.tail.map(|n| VecDeque::with_capacity(n.saturating_add(1)));

    let mut out = io::BufWriter::new(io::stdout());

    for rec in read::iter_records(&args)? {
        stats.seen += 1;

        if !filter.matches(&rec) {
            continue;
        }

        matched += 1;
        stats.matched += 1;
        stats.observe(&rec, args.field.as_deref());

        if let Some(max) = args.head {
            if matched > max {
                break;
            }
        }

        if let Some(buf) = tail_buf.as_mut() {
            buf.push_back(rec);
            let cap = args.tail.unwrap_or(0);
            while buf.len() > cap {
                buf.pop_front();
            }
            continue;
        }

        emit_record(&mut out, &rec, &plan)?;
    }

    if let Some(buf) = tail_buf {
        for rec in buf {
            emit_record(&mut out, &rec, &plan)?;
        }
    }

    out.flush()?;

    if args.stats {
        let mut err = io::BufWriter::new(io::stderr());
        stats.print(&mut err, args.field.as_deref())?;
        err.flush()?;
    }

    Ok(())
}
