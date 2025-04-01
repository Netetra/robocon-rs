use bitfield_struct::bitfield;

#[bitfield(u16)]
struct Buttons {
    #[bits(1)]
    square: bool,

    #[bits(1)]
    cross: bool,

    #[bits(1)]
    circle: bool,

    #[bits(1)]
    triangle: bool,

    #[bits(1)]
    l1: bool,

    #[bits(1)]
    r1: bool,

    #[bits(1)]
    l2: bool,

    #[bits(1)]
    r2: bool,

    #[bits(1)]
    select: bool,

    #[bits(1)]
    start: bool,

    #[bits(1)]
    l3: bool,

    #[bits(1)]
    r3: bool,

    #[bits(1)]
    home: bool,

    #[bits(1)]
    share: bool,

    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
struct Dpad {
    #[bits(1)]
    up: bool,
    #[bits(1)]
    down: bool,
    #[bits(1)]
    left: bool,
    #[bits(1)]
    right: bool,

    #[bits(4)]
    __: u8,
}

#[derive(Debug, Default)]
pub struct Gamepad {
    left_joystick: (i8, i8),
    right_joystick: (i8, i8),
    buttons: Buttons,
    left_trigger: u8,
    right_trigger: u8,
    dpad: Dpad,
}

impl From<&[u8; 9]> for Gamepad {
    fn from(value: &[u8; 9]) -> Self {
        Self {
            left_joystick: (value[0] as i8, value[1] as i8),
            right_joystick: (value[2] as i8, value[3] as i8),
            buttons: Buttons(u16::from_be_bytes([value[4], value[5]])),
            left_trigger: value[6],
            right_trigger: value[7],
            dpad: value[8].into(),
        }
    }
}

impl From<Gamepad> for [u8; 9] {
    fn from(value: Gamepad) -> Self {
        value.into_array()
    }
}

impl From<&Gamepad> for [u8; 9] {
    fn from(value: &Gamepad) -> Self {
        value.into_array()
    }
}

impl Gamepad {
    pub fn into_array(&self) -> [u8; 9] {
        let raw_buttons: u16 = self.buttons.into();
        let buttons_bytes = raw_buttons.to_be_bytes();
        [
            self.left_joystick.0 as u8,
            self.left_joystick.1 as u8,
            self.right_joystick.0 as u8,
            self.right_joystick.1 as u8,
            buttons_bytes[0],
            buttons_bytes[1],
            self.left_trigger,
            self.right_trigger,
            self.dpad.into(),
        ]
    }
    pub fn reset(&mut self) {
        self.buttons = Buttons::default();
        self.dpad = Dpad::default();
    }
    pub fn update(&mut self, new_state: &Gamepad) {
        if !self.buttons.circle() && new_state.circle() {
            self.buttons.set_circle(true);
        }

        if !self.buttons.cross() && new_state.cross() {
            self.buttons.set_cross(true);
        }

        if !self.buttons.square() && new_state.square() {
            self.buttons.set_square(true);
        }

        if !self.buttons.triangle() && new_state.triangle() {
            self.buttons.set_triangle(true);
        }

        if !self.dpad.up() && new_state.up() {
            self.dpad.set_up(true);
        }

        if !self.dpad.down() && new_state.down() {
            self.dpad.set_down(true);
        }

        if !self.dpad.left() && new_state.left() {
            self.dpad.set_left(true);
        }

        if !self.dpad.right() && new_state.right() {
            self.dpad.set_right(true);
        }

        if !self.buttons.l1() && new_state.l1() {
            self.buttons.set_l1(true);
        }

        if !self.buttons.r1() && new_state.r1() {
            self.buttons.set_r1(true);
        }

        if !self.buttons.l2() && new_state.l2() {
            self.buttons.set_l2(true);
        }

        if !self.buttons.r2() && new_state.r2() {
            self.buttons.set_r2(true);
        }

        if !self.buttons.l3() && new_state.l3() {
            self.buttons.set_l3(true);
        }

        if !self.buttons.r3() && new_state.r3() {
            self.buttons.set_r3(true);
        }

        if !self.buttons.select() && new_state.select() {
            self.buttons.set_select(true);
        }

        if !self.buttons.start() && new_state.start() {
            self.buttons.set_start(true);
        }

        if !self.buttons.share() && new_state.share() {
            self.buttons.set_share(true);
        }

        if !self.buttons.home() && new_state.home() {
            self.buttons.set_home(true);
        }

        self.left_joystick = new_state.left_joystick;
        self.right_joystick = new_state.right_joystick;
    }

    pub fn left_joystick(&self) -> (i8, i8) {
        self.left_joystick
    }

    pub fn right_joystick(&self) -> (i8, i8) {
        self.right_joystick
    }

    pub fn circle(&self) -> bool {
        self.buttons.circle()
    }

    pub fn cross(&self) -> bool {
        self.buttons.cross()
    }

    pub fn triangle(&self) -> bool {
        self.buttons.triangle()
    }

    pub fn square(&self) -> bool {
        self.buttons.square()
    }

    pub fn up(&self) -> bool {
        self.dpad.up()
    }

    pub fn down(&self) -> bool {
        self.dpad.down()
    }

    pub fn left(&self) -> bool {
        self.dpad.left()
    }

    pub fn right(&self) -> bool {
        self.dpad.right()
    }

    pub fn l1(&self) -> bool {
        self.buttons.l1()
    }

    pub fn r1(&self) -> bool {
        self.buttons.r1()
    }

    pub fn l2(&self) -> bool {
        self.buttons.l2()
    }

    pub fn r2(&self) -> bool {
        self.buttons.r2()
    }

    pub fn l3(&self) -> bool {
        self.buttons.l3()
    }

    pub fn r3(&self) -> bool {
        self.buttons.r3()
    }

    pub fn select(&self) -> bool {
        self.buttons.select()
    }

    pub fn start(&self) -> bool {
        self.buttons.start()
    }

    pub fn share(&self) -> bool {
        self.buttons.share()
    }

    pub fn home(&self) -> bool {
        self.buttons.home()
    }

    pub fn l2_value(&self) -> u8 {
        self.left_trigger
    }

    pub fn r2_value(&self) -> u8 {
        self.right_trigger
    }

    pub fn is_circle_pushed(&mut self) -> bool {
        if self.buttons.circle() {
            self.buttons.set_circle(false);
            return true;
        }
        false
    }

    pub fn is_cross_pushed(&mut self) -> bool {
        if self.buttons.cross() {
            self.buttons.set_cross(false);
            return true;
        }
        false
    }

    pub fn is_triangle_pushed(&mut self) -> bool {
        if self.buttons.triangle() {
            self.buttons.set_triangle(false);
            return true;
        }
        false
    }

    pub fn is_square_pushed(&mut self) -> bool {
        if self.buttons.square() {
            self.buttons.set_square(false);
            return true;
        }
        false
    }

    pub fn is_up_pushed(&mut self) -> bool {
        if self.dpad.up() {
            self.dpad.set_up(false);
            return true;
        }
        false
    }

    pub fn is_down_pushed(&mut self) -> bool {
        if self.dpad.down() {
            self.dpad.set_down(false);
            return true;
        }
        false
    }

    pub fn is_left_pushed(&mut self) -> bool {
        if self.dpad.left() {
            self.dpad.set_left(false);
            return true;
        }
        false
    }

    pub fn is_right_pushed(&mut self) -> bool {
        if self.dpad.right() {
            self.dpad.set_right(false);
            return true;
        }
        false
    }

    pub fn is_l1_pushed(&mut self) -> bool {
        if self.buttons.l1() {
            self.buttons.set_l1(false);
            return true;
        }
        false
    }

    pub fn is_r1_pushed(&mut self) -> bool {
        if self.buttons.r1() {
            self.buttons.set_r1(false);
            return true;
        }
        false
    }

    pub fn is_l2_pushed(&mut self) -> bool {
        if self.buttons.l2() {
            self.buttons.set_l2(false);
            return true;
        }
        false
    }

    pub fn is_r2_pushed(&mut self) -> bool {
        if self.buttons.r2() {
            self.buttons.set_r2(false);
            return true;
        }
        false
    }

    pub fn is_l3_pushed(&mut self) -> bool {
        if self.buttons.l3() {
            self.buttons.set_l3(false);
            return true;
        }
        false
    }

    pub fn is_r3_pushed(&mut self) -> bool {
        if self.buttons.r3() {
            self.buttons.set_r3(false);
            return true;
        }
        false
    }

    pub fn is_select_pushed(&mut self) -> bool {
        if self.buttons.select() {
            self.buttons.set_select(false);
            return true;
        }
        false
    }

    pub fn is_start_pushed(&mut self) -> bool {
        if self.buttons.start() {
            self.buttons.set_start(false);
            return true;
        }
        false
    }

    pub fn is_share_pushed(&mut self) -> bool {
        if self.buttons.share() {
            self.buttons.set_share(false);
            return true;
        }
        false
    }

    pub fn is_home_pushed(&mut self) -> bool {
        if self.buttons.home() {
            self.buttons.set_home(false);
            return true;
        }
        false
    }
}
