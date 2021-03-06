extern crate i2c_linux;

use i2c_linux::I2c;

pub struct RelayBoard {
    pub num_relays: usize,
    pub dev_addr: u8,
    pub dev_reg_mode1: u8,
    pub dev_reg_data: u8,
    pub bus: I2c<std::fs::File>,
}

impl RelayBoard {
    pub fn new(addr: u8) -> Self {
        let mut b = I2c::from_path("/dev/i2c-1").unwrap();
        b.smbus_set_slave_address(addr.into(), false).unwrap();

        RelayBoard {
            num_relays: 4,
            dev_addr: addr,
            dev_reg_mode1: 0x06,
            dev_reg_data: 0xff,
            bus: b,
        }
    }

    pub fn relay_on(&mut self, relay_num: usize) {
        if relay_num <= self.num_relays && relay_num > 0 {
            self.dev_reg_data &= !(0x1 << (relay_num - 1));
            self.bus
                .smbus_write_byte_data(self.dev_reg_mode1, self.dev_reg_data)
                .unwrap();
        }
    }

    pub fn relay_off(&mut self, relay_num: usize) {
        if relay_num <= self.num_relays && relay_num > 0 {
            self.dev_reg_data |= 0x1 << (relay_num - 1);
            self.bus
                .smbus_write_byte_data(self.dev_reg_mode1, self.dev_reg_data)
                .unwrap();
        }
    }

    pub fn relay_toggle(&mut self, relay_num: usize) {
        if self.relay_status(relay_num) {
            self.relay_off(relay_num);
        } else {
            self.relay_on(relay_num);
        }
    }

    pub fn relay_all_on(&mut self) {
        self.dev_reg_data &= !(0xf << 0);
        self.bus
            .smbus_write_byte_data(self.dev_reg_mode1, self.dev_reg_data)
            .unwrap();
    }

    pub fn relay_all_off(&mut self) {
        self.dev_reg_data |= 0xf << 0;
        self.bus
            .smbus_write_byte_data(self.dev_reg_mode1, self.dev_reg_data)
            .unwrap();
    }

    fn relay_status(&mut self, relay_num: usize) -> bool {
        let d = self.read_reg_data();

        if d > 0 {
            let mask = 1 << (relay_num - 1);

            (self.dev_reg_data & mask) == 0
        } else {
            false
        }
    }

    fn read_reg_data(&mut self) -> u8 {
        self.dev_reg_data = self.bus.smbus_read_byte_data(self.dev_reg_mode1).unwrap();
        self.dev_reg_data
    }
}
