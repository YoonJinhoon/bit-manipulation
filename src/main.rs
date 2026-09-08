mod bits;
mod registers;
mod graphics;
mod emulator;   // 에뮬레이터 모듈 연결

fn main() {
    println!("=== Bit Manipulation Playground Started ===");

    // bits.rs 에 만든 함수 호출
    bits::run_basics();
    bits::run_bitwise_operators();
    graphics::run_graphics_packing();
    registers::run_cpu_flags();
    emulator::cpu::Cpu::run_emulator_demo();
}
