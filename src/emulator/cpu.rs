pub struct Cpu {
    pub accumulator: u8,    // A 레지스터 
    pub status: u8,         // P 상태 레지스터 (8비트 플래그 스위치: c, z, i, n)
}

impl Cpu {
    pub fn new() -> Self {  // CPU 전원을 처음 켰을 때, 모든 레지스터를 0으로 초기화
        Self {
            accumulator: 0, // &mut self
            status: 0,      // &mut self
        }
    }
    
    pub fn step_add(&mut self, value: u8) {
        // 현재 누산기에 있는 값(self.accumulator) + 외부에서 들어온 값(value)
        let res = crate::emulator::alu::add(self.accumulator, value); 
        self.accumulator = res.value;   // 그 결과값(res.value)을 다시 누산기에 덮어쓴다.


        // 전체 레지스터의 다른 방들은 절대 건드리지 않고,
        // 오직 내가 원하는 방의 불만 골라서 조작

        // Zero 플래그 (1번 방) 업데이트
        if res.zero {
            self.status |= 1 << 1;  // 불 켜기 (OR 연산)
        } else {
            self.status &= !(1 << 1);   // 불 끄기 (AND + NOT 마스킹)
        }

        // Negative 플래그 (7번 방) 업데이트
        if res.negative {
            self.status |= 1 << 7;  // 불 켜기
        } else {
            self.status &= !(1 << 7);   // 불 끄기
        }
    }

    pub fn run_emulator_demo() {
        println!("\n--- [5] CPU & ALU Emulator Test ---");
        let mut cpu = Cpu:: new();

        println!("Initial       -> A: {}, Status: {:08b}", cpu.accumulator, cpu.status);

        // 1. 10 더하기 (일반적인 양수 연산: 0 + 10)
        cpu.step_add(10);
        println!("After add(10) -> A: {}, Status: {:08b}", cpu.accumulator, cpu.status);

        // 2. 246 더하기 (10 + 246 = 256, u8 범위를 넘어가 0으로 바뀜 -> Zero 플래그 켜져야 함!)
        cpu.step_add(246);
        println!("After add(246) -> A: {}, Status: {:08b} (Zero flag ON?)", cpu.accumulator, cpu.status);

        // 3. 130 더하기 (0 + 130 = 130, 7번 비트가 켜져서 음수가 됌 -> Negative 플래그 켜져야 함!)
        cpu.step_add(130);
        println!("After add(130) -> A: {}, Status: {:08b} (Negative flag ON?)", cpu.accumulator, cpu.status);
                    
// [참고] 8비트 값 130 을 2의 보수(음수)로 해석할 때, -126 가 되는 과정:
// - 이진수 표현: 130 = 1000_0010 (2)
// - 비트 반전:         0111_1101 (2)
// - 1더하기:           0111_1110 (2) = 126 (10)
// - 음수 부호 부여:    -126 (10)

    }
} 
 
