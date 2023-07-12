# threadid

This is a demo Rust program that demonstrates concurrent programming using named threads and channels. The program spawns multiple worker threads to calculate the Fibonacci sequence for a given range of numbers.

## Prerequisites
- Rust programming language should be installed on your system.
- The crossbeam crate should be added to the dependencies in your `Cargo.toml` file.

## Usage
1. Set the value of the `MAX_WORKER` constant to specify the number of worker threads to spawn.
2. Run the program using the `cargo run` command.

## Description
The code starts by importing the necessary crates and defining the `MAX_WORKER` constant.

### ctrl_channel() function
This function sets up a control channel using the `bounded` method from the `crossbeam::channel` module. It listens for the Ctrl+C signal and sends a message through the channel when it is triggered. The function returns the receiver end of the channel.

### main() function
In the main function, we create a channel called `results_tx` and `results_rx` to communicate the results of the Fibonacci calculations from the worker threads to the main thread.

We then create another unbounded channel called `sender` and `receiver` to pass the job to be executed by the worker threads.

Next, we create an empty vector `threads` to store the handles to the spawned worker threads.

We then iterate from 0 to `MAX_WORKER` and spawn `MAX_WORKER` worker threads. Inside the worker thread closure, the thread receives a job from the `receiver` channel and calculates the Fibonacci sequence for that particular job. The result is then sent back to the main thread using the `results_tx` channel.

After spawning the worker threads, we start another thread called "producer". This thread generates jobs at regular intervals and passes them to the worker threads using the `sender` channel. The producer thread also listens for the Ctrl+C signal and prints a message when it is triggered.

Finally, we use a loop to receive the results from the `results_rx` channel and print them. We also join all the worker threads by iterating over the `threads` vector.

### fib() function
This function calculates the Fibonacci sequence for a given number `n`. It uses an iterative approach to calculate the sequence and returns the result as a u64 integer.

## Conclusion
This program demonstrates how to build concurrent programs in Rust using threads and channels. It showcases the use of multiple worker threads to perform calculations in parallel and communicate the results back to the main thread using channels.
