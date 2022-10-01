extern crate seeed_studio_relay_board;
extern crate gameprng;

use std::{thread, time};
use seeed_studio_relay_board::RelayBoard;
use gameprng::prng::Prng;
use gameprng::xoroshiro128plus::XoRoShiRo128Plus;

fn main() {
    let mut rb = RelayBoard::new(0x20);
    let d = time::Duration::new(0, 100000000);
    let mut prng = Prng::<XoRoShiRo128Plus>::new(666);

    rb.relay_all_off();

    for _ in 0..500 {
        thread::sleep(d);
        rb.relay_toggle(prng.range(1, 4));
    }

    rb.relay_all_off();
}
