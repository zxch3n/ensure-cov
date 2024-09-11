use ensure_cov::{assert_cov, assert_cov_at_least, notify_cov};

#[test]
fn test_coverage() {
    notify_cov("section_a");
    notify_cov("section_b");
    notify_cov("section_b");

    assert_cov("section_a");
    assert_cov_at_least("section_b", 2);
}

#[test]
#[should_panic]
fn test_coverage_panic() {
    assert_cov("section_c");
}

#[test]
#[should_panic]
fn test_coverage_panic_at_least() {
    assert_cov("section_c");
    assert_cov_at_least("section_c", 2);
}
