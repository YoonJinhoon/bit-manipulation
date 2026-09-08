// RGB color packing and unpacking

pub fn run_graphics_packing() {
    println!("\n--- [3] Graphics RGB Color Packing ---");

    let red: u32    = 255;  // 0xFF
    let green: u32  = 128;  // 0x80
    let blue: u32   = 64;   // 0x40

    // Pack them into a single u32: R shifted 16, G shifted 8, B at the bottom
    let packed_color = (red << 16) | (green << 8) | blue;
//     red   << 16 : 00000000 11111111 00000000 00000000 
//     green << 8  : 00000000 00000000 10000000 00000000
//     blue        : 00000000 00000000 00000000 01000000
//   ---------------------------------------------------
//   = packed_color: 00000000 11111111 10000000 01000000

    println!("Packed Color (Hex): 0x{:06X}", packed_color);


    // Unpack them using right-shift and a mask (0xFF)
    let extracted_red   = (packed_color >> 16) & 0xFF;
    let extracted_green = (packed_color >> 8) & 0xFF;
    let extracted_blue = packed_color & 0xFF;

    println!("Unpacked -> R: {}, G: {}, B: {}", extracted_red, extracted_green, extracted_blue);
}
