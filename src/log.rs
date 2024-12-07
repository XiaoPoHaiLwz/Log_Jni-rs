use chrono::Local;

pub struct Logger;
impl Logger {
    pub fn setup_logger(path: String) -> Result<(), fern::InitError> {
        let mut dispatch = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Info);

        #[cfg(debug_assertions)]
        {
            dispatch = dispatch.level(log::LevelFilter::Debug);
        }

        let log_file_handle = std::fs::File::create(path)?;
        dispatch = dispatch.chain(log_file_handle);
        dispatch.apply()?;
        Ok(())
    }

    pub fn info(msg: &str) {
        log::info!("{}", msg);
    }

    pub fn debug(msg: &str) {
        log::debug!("{}", msg);
    }

    pub fn warn(msg: &str) {
        log::warn!("{}", msg);
    }

    pub fn error(msg: &str) {
        log::error!("{}", msg);
    }
}
