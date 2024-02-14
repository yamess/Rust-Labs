use chrono::Local;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use log::{error, info};

pub fn setup_logger() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Magenta)
        .error(Color::Red)
        .trace(Color::BrightBlack)
        .warn(Color::Yellow);

    let logger = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{datetime} [{level}] [{module}] {message}",
                datetime = Local::now().format("%Y-%m-%d %H:%M:%S"),
                level = colors.color(record.level()),
                module = record.target(),
                message = message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply();

    match logger {
        Ok(_) => info!("Logger initialized"),
        Err(e) => error!("Error initializing logger: {}", e),
    }
}
