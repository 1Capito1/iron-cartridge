use crate::cpu::cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

enum Mapping {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select
}

impl Cpu {
    pub fn handle_input(&mut self, event: Event) -> bool {
        match event {
            Event::Quit {..} => { return false }
                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::W => self.update_button_state(Mapping::Up, true),
                    Keycode::A => self.update_button_state(Mapping::Down, true),
                    Keycode::S => self.update_button_state(Mapping::Left, true),
                    Keycode::D => self.update_button_state(Mapping::Right, true),
                    Keycode::Q => self.update_button_state(Mapping::A, true),
                    Keycode::E => self.update_button_state(Mapping::B, true),
                    Keycode::P => self.update_button_state(Mapping::Select, true),
                    Keycode::O => self.update_button_state(Mapping::Start, true),
                    Keycode::Escape => return false,
                    _ => {},
                },
                Event::KeyUp { keycode: Some(key), .. } => match key {
                    Keycode::W => self.update_button_state(Mapping::Up, false),
                    Keycode::A => self.update_button_state(Mapping::Down, false),
                    Keycode::S => self.update_button_state(Mapping::Left, false),
                    Keycode::D => self.update_button_state(Mapping::Right, false),
                    Keycode::Q => self.update_button_state(Mapping::A, false),
                    Keycode::E => self.update_button_state(Mapping::B, false),
                    Keycode::P => self.update_button_state(Mapping::Select, false),
                    Keycode::O => self.update_button_state(Mapping::Start, false),
                    Keycode::Escape => return false,
                    _ => {},
                },
            _ => {}
        }
        true
    }

    fn update_button_state(&mut self, mapping: Mapping, pressed: bool) {
        let bit = match mapping {
            Mapping::Up => 0b10000,
            Mapping::Down => 0b100000,
            Mapping::Left => 0b1000000,
            Mapping::Right => 0b10000000,
            Mapping::A => 0b1,
            Mapping::B => 0b10,
            Mapping::Select => 0b100,
            Mapping::Start => 0b1000,
        };

        if pressed {
            self.controller_state |= bit;
        }
        else {
            self.controller_state &= !bit;
        }

    }

    pub fn write_strobe(&mut self, value: u8) {
        if value & 1 != 0 {
            self.shift_register = self.controller_state;
        }
    }

    pub fn read_strobe(&mut self) -> u8 {
        let bit = self.shift_register & 1;
        self.shift_register >>= 1;
        bit
    }
}
