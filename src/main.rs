fn main() {
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use ctrlc;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Exiting...");
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        let secs = now.as_secs();
        let hours = (secs % 86400) / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        
        println!("Current time: {:02}:{:02}:{:02}", hours, minutes, seconds);
        
        thread::sleep(Duration::from_secs(1));
    }
}
