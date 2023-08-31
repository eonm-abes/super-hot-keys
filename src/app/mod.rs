mod main;
use main::MainApp;

mod pinger;
use pinger::Pinger;

pub enum SuperHotKeys {
    MainApp(MainApp),
    Pinger(Pinger),
}

impl SuperHotKeys {
    pub fn new() -> Self {
        match MainApp::new() {
            Ok(app) => Self::MainApp(app),
            Err(_) => Self::Pinger(Pinger),
        }
    }

    pub fn run(self) {
        match self {
            Self::MainApp(app) => app.run(),
            Self::Pinger(pinger) => pinger.run(),
        }
    }
}
