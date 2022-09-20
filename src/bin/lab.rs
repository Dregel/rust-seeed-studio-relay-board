extern crate seeed_studio_relay_board;

use seeed_studio_relay_board::RelayBoard;

fn main() {
    let mut rb = RelayBoard::new(0x20);

    rb.relay_on(1);
}
