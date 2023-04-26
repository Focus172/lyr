

pub struct State {
    pub error: Option<i8>,
    pub run: bool,
    pub update: bool,
    pub reboot: bool,
    pub shutdown: bool,
    pub auth_fails: u8,
}

impl State {
    pub fn new() -> State {
        State {
            error: None,
            run: true,
            update: true,
            reboot: false,
            shutdown: false,
            auth_fails: 0,
        }
    }
}
