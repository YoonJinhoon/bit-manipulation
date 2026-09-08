pub fn run_basics() {
    println!("\n--- [1] Padded Binary & Shift Basics ---");

    let a: u8 = 5;
    println!("Standard: {:b}", a);      // 101
    println!("Padded:   {:08b}", a);    // 000000101

    let z: u8 = 1;
    println!("z << 1:   {:08b} ({})", z << 1, z << 1);  // 00000010 (2)
    println!("z << 2:   {:08b} ({})", z << 2, z << 2);  // 00000100 (4)
    println!("z << 7:   {:08b} ({})", z << 7, z << 7);  // 10000000 (128)
}

pub fn run_bitwise_operators() {
    println!("\n--- [2] Bitwise Operators & Masking ---");

    let original: u8 = 0b10101010;
    let mask: u8     = 0b11110000;      // 어떤 수든, 0과 XOR 하면, 그 자신이 된다.

    // XOR 토글 예시 (XOR 연산을 2번 적용하면, 원래 숫자로 되돌려진다)
    let encrypted = original ^ mask;
    let descrypted = encrypted ^ mask;

    println!("Original: {:08b}", original);
    println!("Encrypted: {:08b}", encrypted);
    println!("Decrypted: {:08b}", descrypted);
}
