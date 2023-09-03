use interprocess::local_socket::LocalSocketStream;
use std::{env, process::exit};

use crate::config::SOCKET_NAME;

pub struct Pinger;

impl Pinger {
    pub fn run(self) {
        let _ = LocalSocketStream::connect(env::temp_dir().join(SOCKET_NAME));
        exit(0)
    }
}
