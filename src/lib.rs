#[cfg(not(all(not(feature = "disable"), any(test, debug_assertions))))]
pub use self::disabled::*;
#[cfg(all(not(feature = "disable"), any(test, debug_assertions)))]
pub use self::enabled::*;

#[cfg(all(not(feature = "disable"), any(test, debug_assertions)))]
mod enabled {
    use std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    };

    static COVERED: LazyLock<Mutex<HashMap<String, usize>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    #[inline(always)]
    pub fn notify_cov(name: &str) {
        let mut covered = COVERED.lock().unwrap();
        *covered.entry(name.to_string()).or_insert(0) += 1;
    }

    #[inline(always)]
    fn check_cov(name: &str) -> bool {
        let covered = COVERED.lock().unwrap();
        covered.contains_key(name)
    }

    #[inline(always)]
    pub fn get_cov_for(name: &str) -> usize {
        let covered = COVERED.lock().unwrap();
        *covered.get(name).unwrap_or(&0)
    }

    #[inline(always)]
    pub fn assert_cov(name: &str) {
        assert!(check_cov(name), "Expected coverage for {}", name);
    }

    #[inline(always)]
    pub fn assert_cov_at_least(name: &str, expected: usize) {
        let covered = get_cov_for(name);
        assert!(
            covered >= expected,
            "Expected at least {} for {}, got {}",
            expected,
            name,
            covered
        );
    }

    #[inline(always)]
    pub fn clear_cov() {
        let mut covered = COVERED.lock().unwrap();
        covered.clear();
    }
}

#[cfg(not(all(not(feature = "disable"), any(test, debug_assertions))))]
mod disabled {
    #[inline(always)]
    pub fn notify_cov(_name: &str) {}
    #[inline(always)]
    pub fn check_cov(_name: &str) -> bool {
        true
    }

    #[inline(always)]
    pub fn get_cov_for(_name: &str) -> usize {
        0
    }

    #[inline(always)]
    pub fn assert_cov(_name: &str) {}
    #[inline(always)]
    pub fn assert_cov_at_least(_name: &str, _expected: usize) {}
    #[inline(always)]
    pub fn clear_cov() {}
}
