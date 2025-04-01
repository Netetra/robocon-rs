use bitfield_struct::bitfield;

#[derive(PartialEq, Debug)]
pub enum Dpad {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
    Neutral = 8,
}

impl From<u8> for Dpad {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::UpRight,
            2 => Self::Right,
            3 => Self::DownRight,
            4 => Self::Down,
            5 => Self::DownLeft,
            6 => Self::Left,
            7 => Self::UpLeft,
            _ => Self::Neutral,
        }
    }
}

impl From<Dpad> for u8 {
    fn from(value: Dpad) -> Self {
        match value {
            Dpad::Up => 0,
            Dpad::UpRight => 1,
            Dpad::Right => 2,
            Dpad::DownRight => 3,
            Dpad::Down => 4,
            Dpad::DownLeft => 5,
            Dpad::Left => 6,
            Dpad::UpLeft => 7,
            Dpad::Neutral => 8,
        }
    }
}

#[derive(Debug)]
pub struct Data {
    left_joystick: (i8, i8),
    right_joystick: (i8, i8),
    buttons: u16,
    left_trigger: u8,
    right_trigger: u8,
    dpad: Dpad,
}

impl Data {
    pub fn left_joystick(&self) -> (i8, i8) {
        self.left_joystick
    }

    pub fn right_joystick(&self) -> (i8, i8) {
        self.right_joystick
    }
    pub fn square(&self) -> bool {
        self.buttons & (1 << 0) != 0
    }

    pub fn cross(&self) -> bool {
        self.buttons & (1 << 1) != 0
    }

    pub fn circle(&self) -> bool {
        self.buttons & (1 << 2) != 0
    }

    pub fn triangle(&self) -> bool {
        self.buttons & (1 << 3) != 0
    }

    pub fn l1(&self) -> bool {
        self.buttons & (1 << 4) != 0
    }

    pub fn r1(&self) -> bool {
        self.buttons & (1 << 5) != 0
    }

    pub fn l2(&self) -> bool {
        self.buttons & (1 << 6) != 0
    }

    pub fn r2(&self) -> bool {
        self.buttons & (1 << 7) != 0
    }

    pub fn select(&self) -> bool {
        self.buttons & (1 << 8) != 0
    }

    pub fn start(&self) -> bool {
        self.buttons & (1 << 9) != 0
    }

    pub fn l3(&self) -> bool {
        self.buttons & (1 << 10) != 0
    }

    pub fn r3(&self) -> bool {
        self.buttons & (1 << 11) != 0
    }

    pub fn home(&self) -> bool {
        self.buttons & (1 << 12) != 0
    }

    pub fn share(&self) -> bool {
        self.buttons & (1 << 13) != 0
    }

    pub fn l2_value(&self) -> u8 {
        self.left_trigger
    }

    pub fn r2_value(&self) -> u8 {
        self.right_trigger
    }

    pub fn dpad(&self) -> &Dpad {
        &self.dpad
    }

    pub fn up(&self) -> bool {
        *self.dpad() == Dpad::Up || *self.dpad() == Dpad::UpRight || *self.dpad() == Dpad::UpLeft
    }

    pub fn right(&self) -> bool {
        *self.dpad() == Dpad::Right
            || *self.dpad() == Dpad::UpRight
            || *self.dpad() == Dpad::DownRight
    }

    pub fn down(&self) -> bool {
        *self.dpad() == Dpad::Down
            || *self.dpad() == Dpad::DownRight
            || *self.dpad() == Dpad::DownLeft
    }

    pub fn left(&self) -> bool {
        *self.dpad() == Dpad::Left || *self.dpad() == Dpad::UpLeft || *self.dpad() == Dpad::DownLeft
    }
}

impl Default for Data {
    fn default() -> Self {
        Data {
            left_joystick: (0, 0),
            right_joystick: (0, 0),
            buttons: 0,
            left_trigger: 0,
            right_trigger: 0,
            dpad: Dpad::Neutral,
        }
    }
}

impl From<&[u8; 9]> for Data {
    fn from(value: &[u8; 9]) -> Self {
        Self {
            left_joystick: (value[0] as i8, value[1] as i8),
            right_joystick: (value[2] as i8, value[3] as i8),
            buttons: u16::from_be_bytes([value[4], value[5]]),
            left_trigger: value[6],
            right_trigger: value[7],
            dpad: value[8].into(),
        }
    }
}

impl From<Data> for [u8; 9] {
    fn from(value: Data) -> Self {
        let buttons_bytes = value.buttons.to_be_bytes();
        [
            value.left_joystick.0 as u8,
            value.left_joystick.1 as u8,
            value.right_joystick.0 as u8,
            value.right_joystick.1 as u8,
            buttons_bytes[0],
            buttons_bytes[1],
            value.left_trigger,
            value.right_trigger,
            value.dpad.into(),
        ]
    }
}

#[bitfield(u32)]
struct Flags {
    #[bits(1)]
    is_circle_pushed: bool,
    #[bits(1)]
    is_cross_pushed: bool,
    #[bits(1)]
    is_triangle_pushed: bool,
    #[bits(1)]
    is_square_pushed: bool,
    #[bits(1)]
    is_l1_pushed: bool,
    #[bits(1)]
    is_r1_pushed: bool,
    #[bits(1)]
    is_l2_pushed: bool,
    #[bits(1)]
    is_r2_pushed: bool,

    #[bits(1)]
    is_l3_pushed: bool,
    #[bits(1)]
    is_r3_pushed: bool,
    #[bits(1)]
    is_up_pushed: bool,
    #[bits(1)]
    is_down_pushed: bool,
    #[bits(1)]
    is_left_pushed: bool,
    #[bits(1)]
    is_right_pushed: bool,
    #[bits(1)]
    is_select_pushed: bool,
    #[bits(1)]
    is_start_pushed: bool,

    #[bits(1)]
    is_share_pushed: bool,
    #[bits(1)]
    is_home_pushed: bool,

    #[bits(14)]
    __: u16,
}

#[derive(Debug, Default)]
pub struct Gamepad {
    state: Data,
    flags: Flags,
}

impl Gamepad {
    pub fn reset(&mut self) {
        self.state = Data::default();
        self.flags = Flags::default();
    }
    pub fn update(&mut self, new_state: Data) {
        if !self.state.circle() && new_state.circle() {
            self.flags.set_is_circle_pushed(true);
        }

        if !self.state.cross() && new_state.cross() {
            self.flags.set_is_cross_pushed(true);
        }

        if !self.state.square() && new_state.square() {
            self.flags.set_is_square_pushed(true);
        }

        if !self.state.triangle() && new_state.triangle() {
            self.flags.set_is_triangle_pushed(true);
        }

        if !self.state.up() && new_state.up() {
            self.flags.set_is_up_pushed(true);
        }

        if !self.state.down() && new_state.down() {
            self.flags.set_is_down_pushed(true);
        }

        if !self.state.left() && new_state.left() {
            self.flags.set_is_left_pushed(true);
        }

        if !self.state.right() && new_state.right() {
            self.flags.set_is_right_pushed(true);
        }

        if !self.state.l1() && new_state.l1() {
            self.flags.set_is_l1_pushed(true);
        }

        if !self.state.r1() && new_state.r1() {
            self.flags.set_is_r1_pushed(true);
        }

        if !self.state.l2() && new_state.l2() {
            self.flags.set_is_l2_pushed(true);
        }

        if !self.state.r2() && new_state.r2() {
            self.flags.set_is_r2_pushed(true);
        }

        if !self.state.l3() && new_state.l3() {
            self.flags.set_is_l3_pushed(true);
        }

        if !self.state.r3() && new_state.r3() {
            self.flags.set_is_r3_pushed(true);
        }

        if !self.state.select() && new_state.select() {
            self.flags.set_is_select_pushed(true);
        }

        if !self.state.start() && new_state.start() {
            self.flags.set_is_start_pushed(true);
        }

        if !self.state.share() && new_state.share() {
            self.flags.set_is_share_pushed(true);
        }

        if !self.state.home() && new_state.home() {
            self.flags.set_is_home_pushed(true);
        }

        self.state = new_state;
    }

    pub fn left_joystick(&self) -> (i8, i8) {
        self.state.left_joystick()
    }

    pub fn right_joystick(&self) -> (i8, i8) {
        self.state.right_joystick()
    }

    pub fn circle(&self) -> bool {
        self.state.circle()
    }

    pub fn cross(&self) -> bool {
        self.state.cross()
    }

    pub fn triangle(&self) -> bool {
        self.state.triangle()
    }

    pub fn square(&self) -> bool {
        self.state.square()
    }

    pub fn up(&self) -> bool {
        self.state.up()
    }

    pub fn down(&self) -> bool {
        self.state.down()
    }

    pub fn left(&self) -> bool {
        self.state.left()
    }

    pub fn right(&self) -> bool {
        self.state.right()
    }

    pub fn l1(&self) -> bool {
        self.state.l1()
    }

    pub fn r1(&self) -> bool {
        self.state.r1()
    }

    pub fn l2(&self) -> bool {
        self.state.l2()
    }

    pub fn r2(&self) -> bool {
        self.state.r2()
    }

    pub fn l3(&self) -> bool {
        self.state.l3()
    }

    pub fn r3(&self) -> bool {
        self.state.r3()
    }

    pub fn select(&self) -> bool {
        self.state.select()
    }

    pub fn start(&self) -> bool {
        self.state.start()
    }

    pub fn share(&self) -> bool {
        self.state.share()
    }

    pub fn home(&self) -> bool {
        self.state.home()
    }

    pub fn l2_value(&self) -> u8 {
        self.state.l2_value()
    }

    pub fn r2_value(&self) -> u8 {
        self.state.r2_value()
    }

    pub fn is_circle_pushed(&mut self) -> bool {
        if self.flags.is_circle_pushed() {
            self.flags.set_is_circle_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_cross_pushed(&mut self) -> bool {
        if self.flags.is_cross_pushed() {
            self.flags.set_is_cross_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_triangle_pushed(&mut self) -> bool {
        if self.flags.is_triangle_pushed() {
            self.flags.set_is_triangle_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_square_pushed(&mut self) -> bool {
        if self.flags.is_square_pushed() {
            self.flags.set_is_square_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_up_pushed(&mut self) -> bool {
        if self.flags.is_up_pushed() {
            self.flags.set_is_up_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_down_pushed(&mut self) -> bool {
        if self.flags.is_down_pushed() {
            self.flags.set_is_down_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_left_pushed(&mut self) -> bool {
        if self.flags.is_left_pushed() {
            self.flags.set_is_left_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_right_pushed(&mut self) -> bool {
        if self.flags.is_right_pushed() {
            self.flags.set_is_right_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_l1_pushed(&mut self) -> bool {
        if self.flags.is_l1_pushed() {
            self.flags.set_is_l1_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_r1_pushed(&mut self) -> bool {
        if self.flags.is_r1_pushed() {
            self.flags.set_is_r1_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_l2_pushed(&mut self) -> bool {
        if self.flags.is_l2_pushed() {
            self.flags.set_is_l2_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_r2_pushed(&mut self) -> bool {
        if self.flags.is_r2_pushed() {
            self.flags.set_is_r2_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_l3_pushed(&mut self) -> bool {
        if self.flags.is_l3_pushed() {
            self.flags.set_is_l3_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_r3_pushed(&mut self) -> bool {
        if self.flags.is_r3_pushed() {
            self.flags.set_is_r3_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_select_pushed(&mut self) -> bool {
        if self.flags.is_select_pushed() {
            self.flags.set_is_select_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_start_pushed(&mut self) -> bool {
        if self.flags.is_start_pushed() {
            self.flags.set_is_start_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_share_pushed(&mut self) -> bool {
        if self.flags.is_share_pushed() {
            self.flags.set_is_share_pushed(false);
            return true;
        }
        return false;
    }

    pub fn is_home_pushed(&mut self) -> bool {
        if self.flags.is_home_pushed() {
            self.flags.set_is_home_pushed(false);
            return true;
        }
        return false;
    }
}
