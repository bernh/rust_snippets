use std::thread;

pub fn count_in_threads() {
    let mut counter = 0;
    let mut thread_handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                counter += 1;
            }
        });
        thread_handles.push(handle);
    }

    // wait for all threads to finish
    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter);
}
