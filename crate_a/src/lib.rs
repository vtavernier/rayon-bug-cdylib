pub use rayon;

#[no_mangle]
pub extern "C" fn new_rayon_pool() -> *mut rayon::ThreadPool {
    Box::into_raw(Box::new(rayon::ThreadPoolBuilder::new().build().expect("failed to build thread pool")))
}
