use tokio::time::sleep;
use tokio::time::Duration;

pub async fn retry_loop<F, Fut, T, E>(
    mut operation: F,
    retries: usize,
    delay: Duration,
    label: &str,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    for attempt in 0..=retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                log::warn!(
                    "Attempt {}/{} failed during {}: {:?}",
                    attempt + 1,
                    retries + 1,
                    label,
                    e
                );
                sleep(delay).await;
            }
        }
    }

    operation().await
}
