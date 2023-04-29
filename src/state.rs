use crate::input::Desktop;

pub struct State {
    pub error: Option<i8>,
    pub run: bool,
    pub update: bool,
    pub reboot: bool,
    pub shutdown: bool,
    pub auth_fails: u8,
    pub data: TextFeilds,
    pub renders: u32,
}

pub struct TextFeilds {
    pub selected: SelectedFeild,
    pub desktop: Desktop,
    pub name: String,
    pub pass: String,
}

pub enum SelectedFeild {
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
                selected: SelectedFeild::Desktop,
                desktop: Desktop {
                    display: "test".to_string(), //String::new(),
                    // command: String::new(),
                },
                name: String::new(),
                pass: String::new(),
            },
            renders: 0,
        }
    }

    pub fn append_active(&mut self, letter: char) {
        match self.data.selected {
            SelectedFeild::Desktop => {}, 
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

    pub fn handle_tab(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => {}, // TODO cycle the selected desktop
            SelectedFeild::Username => self.data.selected = SelectedFeild::Password,
            SelectedFeild::Password => self.data.selected = SelectedFeild::Username,
        }
    }
}
