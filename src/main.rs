extern crate tun_tap;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    // nic - network interface card
    let nic = tun_tap::Iface::new("tun10", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        eprintln!("read {} bytes: {:?}", nbytes, &buf[..nbytes]);
    }

    Ok(())
}
