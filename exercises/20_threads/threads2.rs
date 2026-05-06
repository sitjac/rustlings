// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, sync::Mutex, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // `Arc` lets multiple threads own the same value.
    // `Metux` makes inner value safely mutable across multiple threads.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0}));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the shared value before mutating it.
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // `unwarp` the lock to access the inner value.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
