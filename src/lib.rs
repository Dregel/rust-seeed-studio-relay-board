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
        if relay_num <= self.num_relays && relay_num > 0{
            println!("Turning relay {} ON!", relay_num);
            self.dev_reg_data &= !(0x1 << (relay_num - 1));
            self.bus.smbus_write_byte_data(self.dev_reg_mode1, self.dev_reg_data).unwrap();
        } else {
            println!("Invalid relay #:{}!", relay_num);
        }
    }
}
