// CPU 가 연산을 하면, 단순히 숫자만 나오지 않는다.
// "연산 결과가 어떤 상태를 가지는지" 함께 나와야 한다.
// 그 묶음 포장이 AluResult 다.

pub struct AluResult {
    pub value: u8,      // 덧셈이 끝난 최종 결과값
    pub zero: bool,     // 결과가 0이면 true
    pub negative: bool, // 결과가 음수면 true (7번 비트가 1)
}

// 덧셈 연산 및 플래그 판정
pub fn add(a: u8, b: u8) -> AluResult {
//  (연산 결과값, overflow 발생 여부 boolean)
    let (result, _) = a.overflowing_add(b);

    AluResult {
        value: result,
        zero: result == 0,
        negative: (result & 0x80) != 0, // 7번 비트가 1이면 음수
    }
}
