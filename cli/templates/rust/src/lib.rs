mod wasm4;
use wasm4::*;

// use std::cell::RefCell;

const BUNNY_WIDTH: u32 = 48;
const BUNNY_HEIGHT: u32 = 16;
const BUNNY_FLAGS: u32 = 1; // BLIT_2BPP
const BUNNY: [u8; 192] = [ 0x00,0x2c,0x0d,0x00,0x02,0x90,0x03,0xd0,0x02,0x90,0x03,0xd0,0x00,0xab,0x3f,0x40,0x0a,0xa4,0x0f,0xf4,0x0a,0xa4,0x0f,0xf4,0x00,0xab,0x3f,0x40,0x0a,0xa9,0x3f,0xf4,0x0a,0xa9,0x3f,0xf4,0x00,0x6b,0x3f,0x40,0x02,0xa9,0x57,0x50,0x02,0xa9,0x57,0x50,0x00,0x6b,0x57,0x40,0x00,0x69,0xa9,0x40,0x00,0x69,0xa9,0x40,0x00,0xab,0xa9,0x40,0x01,0xaa,0xaa,0x80,0x01,0xaa,0xaa,0x80,0x01,0xaa,0xaa,0x80,0x02,0xaa,0x5a,0x50,0x02,0xaa,0x5a,0x50,0x02,0xaa,0x5a,0x50,0x0f,0xea,0xaa,0x90,0x0f,0xea,0xaa,0x90,0x0f,0xea,0xaa,0x90,0x01,0xba,0xaa,0x90,0x01,0xba,0xaa,0x90,0x01,0xba,0xaa,0x90,0x00,0xea,0xab,0x40,0x00,0xea,0xab,0x40,0x00,0xea,0xab,0x40,0x01,0xaa,0xbd,0x00,0x00,0x6a,0x5d,0x00,0x00,0x6e,0xbd,0x00,0x0a,0xa6,0xaa,0x40,0x01,0xae,0xa6,0x40,0x01,0xba,0x6a,0x40,0x06,0x9a,0xaa,0x40,0x02,0xab,0xa6,0x40,0x01,0x9a,0x6a,0x40,0x01,0x6b,0xa9,0x40,0x01,0xda,0x5a,0x40,0x00,0x65,0xa9,0x00,0x06,0xad,0x57,0xd0,0x0f,0xf5,0x5a,0x90,0x00,0x1a,0xa4,0x00,0x01,0xa4,0x17,0x40,0x07,0xd0,0x06,0x90 ];

// struct State {
//     x: i32,
//     y: i32,
// }
// // // static GLOBAL :u32 = 0xab;
// // const image_WIDTH: u32 = 100;
// // const image_FLAGS: u32 = 2; // DRAW_2BPP
// // const image: [u8; 2] = [ 0xff,0x10 ];
// static mut STATE :State = State {x: 0, y: 0};
//
// thread_local! {
//     static X :RefCell<i32> = RefCell::new(0);
// }

#[no_mangle]
fn update () {
    // unsafe {
    //     STATE.x += 1;
    // }
    // x = x+1;
    // // let GAMEPAD1 :*const u8 = 0xff as *const u8;
    // let GAMEPAD1 :*mut u8 = 0xcafe as *mut u8;
    //
    // unsafe { *GAMEPAD1 = 69; }

    // let gamepad1 :Register<u8> = Register::new(0xcafe);
    // gamepad1.set(gamepad1.get() + 10);

    // DRAW_COLORS.set(0x012f);
    // gamepad1.get();

    // DRAW_COLORS.set(1);
    // rect(20, 20, 50, 50);
    //
    // DRAW_COLORS.set(0x3210);
    // let mut y = 0i32;
    // for _i in 0..5 {
    //     blit(&BUNNY, 10, y, BUNNY_WIDTH, BUNNY_HEIGHT, BUNNY_FLAGS);
    //     y += 10;
    // }

    // unsafe { *DRAW_COLORS = 69 };

    // TEST.set(123);

    text("Hello from Rust", 0, 50);
    // print("Debug from Rust");

    // let gamepad = unsafe { *GAMEPAD1 };
    // let framebuffer = unsafe { &*FRAMEBUFFER };
    // framebuffer[1] = 222;

    // unsafe {
    //     let x = &*(69 as *mut [u32; 384]);
    //     x[5] = x[4];
    // }
    // let mut framebuffer = unsafe { *FRAMEBUFFER };
    // framebuffer[0] = 44;
    // framebuffer[1] = 222;

    // framebuffer[0] += 1;

    // let a:u32 = 0x12345678;
    // let b:[u8; 4];

    // b = unsafe{mem::transmute(0x0a)};

    // rect(b[0].into(), 0, 0, 0);
    // println!("{:x},{:x},{:x},{:x}", b[0],b[1],b[2],b[3]);
    // unsafe {
    //     let fb = *FRAMEBUFFER;
    //     fb[0] = 65;
    //     fb[1] = 65;
    // }
    // blit(&BUNNY, 10, 20, BUNNY_WIDTH, BUNNY_HEIGHT, BUNNY_FLAGS);
}
