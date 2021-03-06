#![allow(incomplete_features)]
#![feature(min_type_alias_impl_trait)]
#![feature(map_into_keys_values)]
#![feature(total_cmp)]
#![allow(clippy::large_enum_variant)]
#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let _ = tracing_subscriber::fmt::try_init();

    let mut worker_thread_count = num_cpus::get();
    if let Ok(worker_thread_count_env) = std::env::var("MEMPARTRACK_WORKER_THREAD_COUNT") {
        let count: usize = worker_thread_count_env.parse().unwrap();
        if count > num_cpus::get() {
            panic!("worker thread count is greater than the number of CPU cores");
        }
        worker_thread_count = count;
    }

    let tokio_runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(worker_thread_count)
        .build()
        .unwrap();
    let _guard = tokio_runtime.enter();

    tokio_runtime.block_on(async move {});
}
