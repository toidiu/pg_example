use slog::{Logger, Drain};
use slog_async::Async;
use slog_term::{TermDecorator, FullFormat};


pub fn create_logger() -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let console_drain = Async::new(drain).build().fuse();

    Logger::root(console_drain, o!())
}
