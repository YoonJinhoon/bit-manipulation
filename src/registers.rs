// CPU 의 상태 플래그(스위치) 를 하나의 8비트 (u8) 상자에 집어넣고 꺼내기

pub fn run_cpu_flags() {
    println!("\n--- [4] CPU Status Register Packing (6502 style) ---");

    // 개별 플래그 상태 (0 또는 1)
    let c: u8 = 1;  // Carry flag       (0번 방)
    let z: u8 = 1;  // Zero flag        (1번 방)
    let i: u8 = 0;  // Interrupt flag   (2번 방)
    let n: u8 = 1;  // Negative flag    (7번 방)

    // 1. Packing: 시프트(<<)로 자리를 잡고, OR(|)로 쾅 합치기
    let status = (c << 0) | (z << 1) | (i << 2) | (n << 7);
    println!("Status register: {:08b}", status);

    // 2. Unpacking & Masking: 마스크(&)와 시프트(<<)로 특정 방의 불이 켜져 있는지 확인하기
    let is_carry    = (status & (1 << 0)) != 0;
    let is_zero     = (status & (1 << 1)) != 0;
    let is_negative = (status & (1 << 7)) != 0;

    println!("Is carry set?     {}", is_carry);
    println!("Is zero set?      {}", is_zero);
    println!("Is negative set?  {}", is_negative);
}
