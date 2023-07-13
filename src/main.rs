const MAX_WORKER: usize = 4;

use crossbeam::channel::{bounded, select, tick, unbounded, Receiver};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

use log::{debug, info, trace, warn};

fn main() {
    env_logger::init();

    use std::sync::mpsc::channel;
    let (results_tx, results_rx) = channel();

    use std::thread;
    use std::time::Duration;

    let (sender, receiver) = unbounded();
    let mut threads = Vec::new();

    info!("Spawning {} workers.", MAX_WORKER);
    for thread_num in 0..MAX_WORKER {
        let thread_results_tx = results_tx.clone();
        let r = receiver.clone();
        let handle = thread::Builder::new()
            .name(format!("worker_{}", thread_num))
            .spawn(move || {
                debug!(
                    "Thread {}: {:?}::{} started.",
                    thread_num,
                    thread::current().id(),
                    thread::current().name().unwrap(),
                );
                let mut work_done = 0;
                while let Ok(work) = r.recv() {
                    let result = fib(work);
                    work_done += 1;
                    match thread_results_tx.send((work, result)) {
                        Ok(_) => (),
                        Err(_) => {
                            break;
                        }
                    }
                }
                std::thread::yield_now();
                debug!(
                    "Thread {}: {:?}::{} did {} jobs.",
                    thread_num,
                    thread::current().id(),
                    thread::current().name().unwrap(),
                    work_done,
                );
            });
        threads.push(handle);
    }
    info!("Workers successfully started.");

    thread::Builder::new()
        .name("producer".to_string())
        .spawn(move || {
            let thread_results_tx = results_tx.clone();
            let ctrl_c_events = ctrl_channel().unwrap();
            let ticks = tick(Duration::from_millis(1));

            info!("Producer successfully started.");
            let mut total_jobs = 0;
            loop {
                select! {
                    recv(ticks) -> _ => {
                        sender.send(total_jobs % 90).unwrap();
                        total_jobs += 1;
                    }
                    recv(ctrl_c_events) -> _ => {
                        println!();
                        warn!("shutdown!");
                        break;
                    }
                }
            }
            drop(thread_results_tx);
            debug!(
                "Total of {} jobs inserted into the queue by {:#?}:{}",
                total_jobs,
                thread::current().id(),
                thread::current().name().unwrap()
            );
        })
        .unwrap();

    while let Ok((work, result)) = results_rx.recv() {
        trace!("fib({}) = {}", work, result);
    }

    for handle in threads {
        handle.unwrap().join().unwrap();
    }
}

fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut iteration = 0;
    let mut sum = 0;
    let mut last = fib(0);
    let mut current = fib(1);

    while iteration < n - 1 {
        sum = last + current;
        last = current;
        current = sum;
        iteration += 1;
    }

    return sum;
}
