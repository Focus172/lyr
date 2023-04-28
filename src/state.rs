use crate::input::Desktop;

pub struct State {
    pub error: Option<i8>,
    pub run: bool,
    pub update: bool,
    pub reboot: bool,
    pub shutdown: bool,
    pub auth_fails: u8,
    data: TextFeilds,
}

struct TextFeilds {
    selected: SelectedFeild,
    desktop: Desktop,
    name: String,
    pass: String,
}

enum SelectedFeild {
    Desktop,
    Username,
    Password,
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
            data: TextFeilds {
                selected: SelectedFeild::Username,
                desktop: Desktop {
                    // display: String::new(),
                    // command: String::new(),
                },
                name: String::new(),
                pass: String::new(),
            },
        }
    }

    pub fn append_active(&mut self, letter: char) {
        match self.data.selected {
            SelectedFeild::Desktop => {}
            SelectedFeild::Username => self.data.name.push(letter),
            SelectedFeild::Password => self.data.pass.push(letter),
        }
    }

    pub fn next_buffer(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => self.data.selected = SelectedFeild::Username,
            SelectedFeild::Username => self.data.selected = SelectedFeild::Password,
            SelectedFeild::Password => {}
        }
    }

    pub fn prev_buffer(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => {}
            SelectedFeild::Username => self.data.selected = SelectedFeild::Desktop,
            SelectedFeild::Password => self.data.selected = SelectedFeild::Username,
        }
    }
}
