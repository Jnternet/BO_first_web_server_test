pub mod 请求处理 {
    use super::线程池::ThreadPool;
    use std::io::BufReader;
    use std::path::Path;
    use std::sync::Arc;
    use std::{
        fs,
        io::prelude::*,
        net::{TcpListener, TcpStream},
    };

    pub fn 监听端口等待并处理任务(path: Arc<Path>) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let p = path.clone();
            pool.execute(move || {
                handle_connection(stream, p);
            });
        }
        println!("关闭服务中...");
    }

    fn handle_connection(mut stream: TcpStream, path: Arc<Path>) {
        let _ = BufReader::new(&mut stream).lines().next().unwrap().unwrap();

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(path).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

mod 线程池 {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let mut workers = Vec::with_capacity(size);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..size {
                workers.push(Worker::new(id, receiver.clone()))
            }

            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());

            for worker in &mut self.workers {
                println!("正在关闭{}号线程工人", worker.id);

                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("线程工人 {id} 得到任务, 执行中...");
                        job();
                    }
                    Err(e) => {
                        println!("线程工人 {id} 失联; 原因: {e}");
                        break;
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;
}
