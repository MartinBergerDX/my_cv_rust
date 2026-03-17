#[cfg(feature = "ssr")]
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

/// Very small in-memory IP rate limiter.
///
/// Notes:
/// - Per-process only (does not share limits across multiple server instances).
/// - Uses a simple sliding window by storing recent timestamps per IP.
#[cfg(feature = "ssr")]
pub struct IpRateLimiter {
    window: Duration,
    max_requests: usize,
    state: Mutex<HashMap<IpAddr, Vec<Instant>>>,
}

#[cfg(feature = "ssr")]
impl IpRateLimiter {
    pub fn new(window: Duration, max_requests: usize) -> Self {
        Self {
            window,
            max_requests,
            state: Mutex::new(HashMap::new()),
        }
    }

    /// Returns `true` if allowed, `false` if rate-limited.
    pub fn allow(&self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let cutoff = now.checked_sub(self.window).unwrap_or(now);

        let mut state = self
            .state
            .lock()
            .expect("IpRateLimiter mutex poisoned");

        let entries = state.entry(ip).or_default();
        entries.retain(|t| *t >= cutoff);

        if entries.len() >= self.max_requests {
            return false;
        }

        entries.push(now);
        true
    }
}

#[cfg(feature = "ssr")]
pub fn default_contact_rate_limiter() -> IpRateLimiter {
    IpRateLimiter::new(Duration::from_secs(10 * 60), 3)
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::IpRateLimiter;
    use std::{net::IpAddr, time::Duration};

    #[test]
    fn allows_up_to_max_requests_per_ip_within_window() {
        let limiter = IpRateLimiter::new(Duration::from_secs(60), 3);
        let ip: IpAddr = "203.0.113.10".parse().unwrap();

        assert!(limiter.allow(ip));
        assert!(limiter.allow(ip));
        assert!(limiter.allow(ip));
        assert!(!limiter.allow(ip));
        assert!(!limiter.allow(ip));
    }

    #[test]
    fn different_ips_have_independent_buckets() {
        let limiter = IpRateLimiter::new(Duration::from_secs(60), 1);
        let ip1: IpAddr = "203.0.113.10".parse().unwrap();
        let ip2: IpAddr = "203.0.113.11".parse().unwrap();

        assert!(limiter.allow(ip1));
        assert!(!limiter.allow(ip1));

        assert!(limiter.allow(ip2));
        assert!(!limiter.allow(ip2));
    }

    #[test]
    fn allows_again_after_window_expires() {
        let limiter = IpRateLimiter::new(Duration::from_millis(20), 1);
        let ip: IpAddr = "203.0.113.10".parse().unwrap();

        assert!(limiter.allow(ip));
        assert!(!limiter.allow(ip));

        std::thread::sleep(Duration::from_millis(30));

        assert!(limiter.allow(ip));
    }
}

