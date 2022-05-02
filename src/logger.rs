

use std::thread::JoinHandle;

use crossbeam::channel::{Sender, bounded, RecvError};
use simple_wal::LogFile;



pub struct FastLog;
impl FastLog {
    
    pub fn run(buffer: usize, path: &str) -> (JoinHandle<RecvError>, Sender<Vec<u8>>) {

        // Create Channel
        let (sx, rx) = bounded::<Vec<u8>>(buffer);

        // Open Log
        let mut lf = LogFile::open(path).unwrap();

        // Spawn Thread
        let jh = std::thread::spawn(move || {
            let reason = loop {
                let res = rx.recv();
                match res {
                    Ok(mut record) => {
                        let _ = lf.write(&mut record);
                    }
                    Err(e) => break e
                }
            };

            let _ = lf.flush();
            return reason

        });

        (jh, sx)
    }
}