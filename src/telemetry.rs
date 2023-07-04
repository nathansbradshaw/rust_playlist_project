// TODO set upp telemetry with tower

use tokio::task::{self, JoinHandle};

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // TODO figure out how to add tracing to threads
    task::spawn_blocking(f)
}
