# async-await
As we all are familiar with native OS threads in Rust programming which helps for executing a Rust program that contains 
their own stack and local state but using these threads can cause following problems:

* Race conditions
* Deadlocks
* Non Reproducible bugs

## async
**async** is a Rust’s built-in tool for writing asynchronous code that allows us to run multiple tasks concurrently on the same OS thread. `async` uses await to wait for the completion of another type that implements a trait called `Future`.
## await
**await** doesn’t require to block the current thread and allows other tasks to run by waiting for future completion asynchronously.

### Blocking
Blocking is a process where multiple tasks execute in the same OS thread in a row, which means other tasks have to wait 
for the previous task to complete first. 

### Non-Blocking
Non-Blocking is a process where other tasks will not need to wait for previous task to be completed if that is in a 
waiting state, whenever the previous task goes into a waiting state then the next task will start its execution.
