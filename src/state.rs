use crate::input::Desktop;
use editable_word::EditableWord;

pub struct State {
    pub run: bool,
    pub update: bool,
    pub reboot: bool,
    pub shutdown: bool,
    pub auth_fails: u8,
    pub renders: u32,
    pub data: TextFeilds,
}

pub struct TextFeilds {
    pub selected: SelectedFeild,
    pub desktop: Desktop,
    pub name: EditableWord,
    pub pass: EditableWord,
}

pub enum SelectedFeild {
    Desktop,
    Username,
    Password,
    Enter,
}

impl Default for State {
    fn default() -> Self {
        State {
            run: true,
            update: true,
            reboot: false,
            shutdown: false,
            auth_fails: 0,
            data: TextFeilds {
                selected: SelectedFeild::Desktop,
                desktop: Desktop::Wayland, 
                name: EditableWord::new(String::new()), 
                pass: EditableWord::new(String::new()), 
            },
            renders: 0,
        }
    }

}

impl State { 
    pub fn append_active(&mut self, letter: char) {
        match self.data.selected {
            SelectedFeild::Desktop => {}, 
            SelectedFeild::Username => self.data.name.add(letter),
            SelectedFeild::Password => self.data.pass.add(letter),
            SelectedFeild::Enter => {}
        }
    }

    pub fn next_buffer(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => self.data.selected = SelectedFeild::Username,
            SelectedFeild::Username => self.data.selected = SelectedFeild::Password,
            SelectedFeild::Password => self.data.selected = SelectedFeild::Enter,
            _ => {}
        }
    }

    pub fn prev_buffer(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => {}
            SelectedFeild::Username => self.data.selected = SelectedFeild::Desktop,
            SelectedFeild::Password => self.data.selected = SelectedFeild::Username,
            SelectedFeild::Enter => self.data.selected = SelectedFeild::Password,
        }
    }

    pub fn handle_tab(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => {
                match self.data.desktop {
                    Desktop::Wayland => self.data.desktop = Desktop::Xorg,
                    Desktop::Xorg => self.data.desktop = Desktop::Shell,
                    Desktop::Shell => self.data.desktop = Desktop::Wayland,
                }
            }, // TODO cycle the selected desktop
            SelectedFeild::Username => self.data.selected = SelectedFeild::Password,
            SelectedFeild::Password => self.data.selected = SelectedFeild::Username,
            SelectedFeild::Enter => {}
        }
    }

    pub fn handle_enter(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => {}
            SelectedFeild::Username => {
                self.handle_tab();
            }
            SelectedFeild::Password => {
                self.handle_tab();
            }
            SelectedFeild::Enter => {
                // TODO have this be a seperate function
                // save the two input feilds
                // attempt to authenticate

                // if auth auth
                // > increment fails by 1
                // > move input back to password
                // > display pam message on info line
                // > clear the password
                // > reset the authenticate

                // else
                // > set into line to logout message?
                // > load(&desktop, &login);
                //
                // > system("tput cnorm");
            }
        }
    }

    pub fn del_active(&mut self) {
        match self.data.selected {
            SelectedFeild::Desktop => {}, 
            SelectedFeild::Username => self.data.name.del(),
            SelectedFeild::Password => self.data.pass.del(),
            SelectedFeild::Enter => {}
        }
    }
}
