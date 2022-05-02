# diskfire
NonBlocking,  Blazing fast disk based logger


# Example 

```

fn main() {
    
    let path  = "loger";
    let buffer_size = 10_000;

    // Run
    let (jh, sender)  = FastLog::run(buffer_size, path);


    // NonBlcoknig Log
    let _ = sender.send(b"Record ....".to_vec());


    // copy
    let sender2 = sender.clone();


    // NonBlocking send from another thread
    std::thread::spawn(move || {
        let _ = sender2.send(b"Record ....".to_vec());
    });


    let _ = jh.join();

}
```
