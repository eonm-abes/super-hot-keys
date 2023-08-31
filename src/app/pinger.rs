use interprocess::local_socket::LocalSocketStream;
use std::process::exit;

pub struct Pinger;

impl Pinger {
    pub fn run(self) {
        let _ = LocalSocketStream::connect(".shk");
        exit(0)
    }
}
