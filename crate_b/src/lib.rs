#[no_mangle]
unsafe extern "C" fn do_something_with_pool(pool: *mut crate_a::rayon::ThreadPool) {
    let pool = Box::from_raw(pool);
    println!("{}", pool.install(|| 30));
    Box::leak(pool);
}
