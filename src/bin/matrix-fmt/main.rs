use anyhow::Context;
use rustracer::prelude::Num;
use std::io::{stdin, Read};
use std::result::Result as StdResult;

// helps to format and print a table representing a matrix
fn main() -> anyhow::Result<()> {
    let mut buf = vec![];
    stdin()
        .read_to_end(&mut buf)
        .context("could not read stdin")?;
    let buf = String::from_utf8(buf).context("could not convert to utf8")?;
    let vals = buf
        .split('|')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Num>())
        .collect::<StdResult<Vec<_>, _>>()
        .context("failed to parse nums")?
        .into_iter()
        .map(f64str)
        .collect::<Vec<_>>();
    let dim = (vals.len() as f64).sqrt() as usize;
    assert_eq!(dim * dim, vals.len());
    let max = vals.iter().map(|s| s.len()).max().unwrap_or_default();
    let formatted = vals
        .chunks(dim)
        .map(|vals| {
            let vals = vals
                .iter()
                .map(|v| format!(" {v} {}", " ".repeat(max - v.len())))
                .collect::<Vec<_>>()
                .join("|");
            format!("|{}|", vals)
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("\n");
    println!("{formatted}");
    Ok(())
}

fn f64str(v: f64) -> String {
    format!("{v:.10}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
