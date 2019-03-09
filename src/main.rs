#![allow(warnings)]
extern crate lepton_sdk_rs_sys;
use lepton_sdk_rs_sys::*;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::channel;

#[inline]
fn is_valid(data: u8) -> bool {
    !((data & 0x0F) == 0x0F)
}


fn calculate_loop_delay_ns(speed: u32) -> u32 {
    let nanosec     = 1_000_000_000;

    let interval    = nanosec / 106;   // 1/106th sec in nanoseconds
    let bit_time    = nanosec / speed; // time to clock out one bit

    let packet_size = 192*8;
    let segment_size = 60*packet_size; // 60 packets per segment
    let transmission_time = segment_size * bit_time; // time to clock out a segment. Must be less than `interval`

    return interval - transmission_time;
}

fn main() {

    // Open I2C
    let mut lepton = CameraPortDescriptor::new(1);
    println!("{:#?}", lepton);

    let result = lepton.open();
    println!("{:#?}", result);

    let (tx, rx) = channel();

    // Receiver thread loop
    thread::spawn(move || {
        // Open SPI
        let spi_speed     = 22_000_000;
        let loop_delay_ns = calculate_loop_delay_ns(spi_speed);
        let mut lepton    = LeptonSpi::new(0, spi_speed).unwrap();

        // Read and send data
        loop {
            for _segment in 1..5 { // 1 thru 4
                for _packet in 0..60 { // 0 thru 59
                    let mut buffer = vec![0u8; 164];
                    lepton.read(&mut buffer).unwrap();

                    // tx.send(buffer).unwrap();
                    if is_valid(buffer[0]) {
                        tx.send(buffer).unwrap();
                    }
                }
            }
            thread::sleep(Duration::new(0, loop_delay_ns)); // secs, nanos
        }
    });

    // ||D
    // ||A
    // ||T
    // ||A
    // \/

    // Processing/sorting loop
    let mut image = vec![ vec![0u16; 160]; 120]; // 160 by 120
    loop {
        let packet;
        if let Ok(rx_pak) = rx.recv() {
            packet = rx_pak;
        }  else {
            continue;
        }

        println!("{:x?} {:x?}", packet[0], packet[1]);
        
    }
    // ||
    // ||
    // ||
    // \/

    // Output
}

