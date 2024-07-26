use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>
}


//impl keyword is used to define methods and associated functions for structs (and enums).
type Job = Box<dyn FnOnce() + Send + 'static>;
//represents a unit of work that can be sent to a worker
//Box is a smart pointer that allows us to store data on the heap rather than the stack and provides ownership of the data
// It uses a Box to store a closure on the heap, which can be called exactly once (FnOnce), sent to another thread (Send), and has a 'static lifetime.



impl ThreadPool{
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let (sender,reciever)=mpsc::channel();
        let reciever=Arc::new(Mutex::new(reciever));
        //initialize sender and reciever for channel
        //Multiple Producers, Single Consumer //Therefore multiple threads can send messages to a single thread
        //since same reciever is used for all senders we need to wrap it in Arc and Mutex

        let mut workers=Vec::with_capacity(size);
//wtih_capacity is similar to new except it preallocates space in vector and this is more ffecient
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&reciever)));
            //if 4 threads are created then 4 workers are created

//what we want is to creates thread and store them in vector
//we don't want to use them immediately so we can't use thread::spawn
//size is given by user which tells number of threads in pool

        }
        ThreadPool{workers,sender}
    }
    //creates a new instance of the struct threadpool
//Represents an unsigned integer that is the same size as the pointer type on the current platform. 
//This means that on a 32-bit platform, usize is 32 bits, and on a 64-bit platform, usize is 64 bit
    pub fn execute<F>(&self,f:F )
    where F:FnOnce()+Send+'static,
    {
        let job=Box::new(f);
        self.sender.send(job).unwrap();

        //f is a closure 
        //a closure is an anonymous function (They do not have a name )that can capture variables from its surrounding scope.
        //Closures are similar to functions, but they have the capability to capture and use variables from the context in which they are defined.
        //They implement one of the three Fn traits (Fn, FnMut, or FnOnce), 
        //FnOnce: The closure is allowed to be caleld once
        //Send: CLosure can be safely transferered to another thread
        //Static lifeline ensures that closure can outlive the life of the current scope 
    }
    //The execute function is a way to submit tasks (closures) to the ThreadPool. These tasks are then executed by one of the threads in the pool. 
    //The traits FnOnce, Send, and 'static ensure that the closure is safe to be sent and executed by a different thread without any issues.
}
struct Worker{
    id:usize,
    thread: thread::JoinHandle<()>,
}
//Each worker is associated with a thread
//Each worker spawns a thread and locks the reciever to get a job
impl Worker{
    fn new(id:usize,reciever:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread=thread::spawn(move||loop{

            let job=reciever.lock().unwrap().recv().unwrap();
            //lock reciever to acquire mutex 
            //.recv to recieve job from reciever
            //.recv blocks the current thread until a value is available
            //if no job is available it will wait 
            println!("Worker {} got a job; executing",id);
            job();
            //reciever is moved to each worker instance so once it is used it won't be available for next itearation in for loop on line 20
            //(Since Reciever type  can not be copied)
            //So we make reciever of type Arc<Mutex<Reciever<Job>>> so that it can be shared among multiple threads
            //Mutex ensures only one worker gets a job from reciever at  a time
            //Arc ensures that multiple workers can access the reciever(own it)
        });
        Worker{id,thread}
        
    }
}
//Note external code doesn't need to know implementation details 
//so it is not public 

//Execute : Sends job from ThreadPool to worker instance
//Worker instance sends job to available thread
//Threadpool holds on to sender and creates a channel
//Worker holds on to reciever
//Job struct will be used to hold the closures 
