use crate::*;

pub fn get_thread_count() -> usize {
    num_cpus::get().max(1)
}
