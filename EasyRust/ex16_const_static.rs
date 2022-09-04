// attribute
// 컴파일러 warning 나온 메시지 가져와서 'warn'부분을 'allow'로 바꿔줬음
#[allow(dead_code)]

// const (주로 대문자로 선언, 뒤에 무조건 타입 붙여줘야함)
const HIGH_SCORE: i32 = 20; // global scope

// static
//static LOW_SCORE: i32 = 0; // 같은 메모리 공간을 쓴다 (메모리 위치 고정?)
static mut LOW_SCORE: i32 = 0; // unsafe 키워드 붙여서 써줘야만 함 (static을 mut하게 쓸 경우)
// 근데 unsafe 쓰는 거 비추함! 애초에 const를 쓰는 게 나음.

// 'static' lifetime : 프로그램의 처음 ~ 끝

fn print_score() {
	println!("The high score is {}", HIGH_SCORE);
	println!("The low score is {}", unsafe {LOW_SCORE});
}

fn main() {
	let _x = 8; // 'let' binding: i32
	unsafe {LOW_SCORE = 1;}
	print_score();
}
/*
출력결과:
The high score is 20
The low score is 1
*/