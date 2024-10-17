use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;

pub fn setup_logger() -> Result<(), fern::InitError> {
    log::set_max_level(LevelFilter::Off); // Set the maximum log level for the entire application

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(LevelFilter::Info)
        .level_for("tracing::span", LevelFilter::Off)
        .level_for("serenity::http::request", LevelFilter::Off)
        .level_for("serenity::http::ratelimiting", LevelFilter::Off)
        .level_for("serenity::http::client", LevelFilter::Off)
        .level_for("serenity::gateway::ws", LevelFilter::Off)
        .level_for("serenity::gateway::shard", LevelFilter::Off)
        .level_for("serenity::gateway::bridge::shard_runner", LevelFilter::Off)
        .level_for("serenity::gateway::bridge::shard_queuer", LevelFilter::Off)
        .level_for("serenity::gateway::bridge::shard_manager", LevelFilter::Off)
        .level_for("serenity::client", LevelFilter::Off)
        .level_for("serenity::cache", LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
