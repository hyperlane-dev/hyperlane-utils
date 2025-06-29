use crate::*;

pub fn get_thread_count() -> usize {
    let cpu_count: usize = num_cpus::get_physical();
    let total_threads: usize = num_cpus::get();
    let threads_per_cpu: usize = if cpu_count > 0 {
        total_threads / cpu_count
    } else {
        1
    };
    cpu_count * threads_per_cpu
}
