#[allow(unused_imports)]
use log::LevelFilter;

#[ctor::ctor]
fn initialise() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();
}
