#[cfg(target_os = "android")]
fn logging() {
    use android_logger::Config;
    use log::Level;
    android_logger::init_once(Config::default().with_min_level(Level::Debug));
}

#[cfg(target_os = "ios")]
fn logging() {
    use log::LevelFilter;
    use syslog::Facility;

    syslog::init_unix(Facility::LOG_USER, LevelFilter::max()).expect("could not connect to syslog");
}

#[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
fn logging() {}

#[no_mangle]
pub extern "C" fn camarim_setup_logger() {
    logging();
    std::panic::set_hook(Box::new(|e| {
        log::error!("Error: {:?}", e);
        println!("Custom panic hook");
    }));
}
