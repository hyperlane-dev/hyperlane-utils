use hyperlane_utils::*;

#[test]
fn test_get_thread_count() {
    let thread_count: usize = get_thread_count();
    assert!(thread_count > 0);
    assert!(thread_count <= num_cpus::get() * 2);
}
