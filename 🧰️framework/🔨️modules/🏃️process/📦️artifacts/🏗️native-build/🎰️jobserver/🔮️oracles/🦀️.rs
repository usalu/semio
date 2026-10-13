//! 🔮️ Third-party jobserver oracle: the `jobserver` crate (the client Cargo and rustc embed) joins the pool the
//! environment names and reports the largest number of job slots it ever held at once, its implicit slot included.
//! Usage: `jobserver-oracle <workers> <hold-ms> <rounds>`; prints `{"maxConcurrent":N,"acquired":M}`.
//! @see https://docs.rs/jobserver/0.1.35/jobserver/struct.Client.html#method.from_env_ext

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// 🚀️ Runs the oracle workers against the inherited pool.
fn main() {
    let args: Vec<usize> = std::env::args().skip(1).map(|value| value.parse().expect("numeric argument")).collect();
    let [workers, hold_ms, rounds] = args[..] else { panic!("expected <workers> <hold-ms> <rounds>") };
    let client = unsafe { jobserver::Client::from_env_ext(true) }.client.expect("inherited jobserver");
    let (running, peak, acquired) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));
    let handles: Vec<_> = (0..workers)
        .map(|worker| {
            let (client, running, peak, acquired) = (client.clone(), running.clone(), peak.clone(), acquired.clone());
            std::thread::spawn(move || {
                for _ in 0..rounds {
                    let token = (worker > 0).then(|| client.acquire().expect("token"));
                    if token.is_some() { acquired.fetch_add(1, Ordering::SeqCst); }
                    let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(now, Ordering::SeqCst);
                    std::thread::sleep(Duration::from_millis(hold_ms as u64));
                    running.fetch_sub(1, Ordering::SeqCst);
                    drop(token);
                }
            })
        })
        .collect();
    for handle in handles { handle.join().expect("worker"); }
    println!("{{\"maxConcurrent\":{},\"acquired\":{}}}", peak.load(Ordering::SeqCst), acquired.load(Ordering::SeqCst));
}
