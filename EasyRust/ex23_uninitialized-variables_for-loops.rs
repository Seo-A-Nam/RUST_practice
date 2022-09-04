// uninitialized variable
// control flow

fn loop_then_return(mut counter: i32) -> i32 {
	loop {
		counter +=1;
		if counter % 50  == 0 {
			break;
		}
	}
	counter
}

// 중요 : rust에서는 절대로 unitialized variable을 쓸 수 없음 (이후에 꼭 쓰기 전 초기화 들어가야함)
fn main() {
	// let my_number: u8; -- error! : possibly unitialized = maybe doesn't have a value yet.
	
	// 1. 블록 안에서 값 초기화 해주는 경우 (1)
	let my_number;
	{
		my_number = 9;
	}
	println!("{}", my_number);
	
	// 2. 블록 내부에서 값 초기화해주는 경우 (2)
	let my_number = {
		// 복잡한 것들
		let x = 9;
		x + 9
	};
	println!("{}", my_number);
	
	// 3. 블록 내부에서 값 초기화 (블록 내부에 함수도 존재)
	let my_number;
	{
		// 복잡한 것들
		let x = loop_then_return(43); // 50
		my_number = x
	}
	println!("{}", my_number);
}
/*
출력결과:
9
18
50
*/