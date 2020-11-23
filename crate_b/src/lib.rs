#[no_mangle]
pub unsafe extern "C" fn do_something_with_pool(pool: *mut crate_a::rayon::ThreadPool) {
    let pool = Box::from_raw(pool);
    println!("{}", pool.install(|| 30));
    Box::leak(pool);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_c_api_from_rust() {
        let pool = crate_a::new_rayon_pool();
        unsafe { super::do_something_with_pool(pool); }
    }
}
