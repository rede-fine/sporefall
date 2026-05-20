mod export;
mod inat;
mod records;

use export::{build_report, export_game_cards, normalize_records};
use inat::fetch_observations;
use records::ImportOptions;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let options = parse_args(std::env::args().skip(1).collect())?;
    let observations = fetch_observations(&options)?;
    let records = normalize_records(&observations, options.download_dir.as_deref())?;
    let cards = export_game_cards(&records);
    let report = build_report(observations.len(), &records, &cards);

    let payload = serde_json::json!({
        "query": {
            "user_login": options.user_login,
            "start_date": options.start_date,
            "end_date": options.end_date,
            "download_dir": options.download_dir,
        },
        "records": records,
        "cards": cards,
        "report": report,
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&payload)
            .map_err(|error| format!("Failed to serialize export payload: {error}"))?
    );
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<ImportOptions, String> {
    let mut user_login = None;
    let mut start_date = None;
    let mut end_date = None;
    let mut download_dir = None;

    let mut index = 0usize;
    while index < args.len() {
        let flag = &args[index];
        if matches!(flag.as_str(), "--help" | "-h") {
            return Err(usage());
        }
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("Missing value for {flag}."))?;

        match flag.as_str() {
            "--user-login" => user_login = Some(value.clone()),
            "--start-date" => start_date = Some(value.clone()),
            "--end-date" => end_date = Some(value.clone()),
            "--download-dir" => download_dir = Some(value.clone()),
            other => return Err(format!("Unknown argument: {other}\n\n{}", usage())),
        }

        index += 2;
    }

    Ok(ImportOptions {
        user_login: user_login.ok_or_else(|| usage())?,
        start_date: start_date.ok_or_else(|| usage())?,
        end_date: end_date.ok_or_else(|| usage())?,
        download_dir,
    })
}

fn usage() -> String {
    "Usage: cargo run -p data_pipeline -- --user-login <login> --start-date YYYY-MM-DD --end-date YYYY-MM-DD [--download-dir path]".to_owned()
}
