// TODO set upp telemetry with tower

use tokio::task::JoinHandle;

// pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
// where
//     F: FnOnce() -> R + Send + 'static,
//     R: Send + 'static,
// {
//     // TODO figure out how to add tracing to threads
//     task::spawn_blocking(f)
// }
pub fn spawn_blocking_with_tracing<R, F>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
