fn main() {
	// 값 중간에 '_' 얼마나 많이 넣던 간에, 컴파일러가 알아서 제거해서 읽는다.
	let _my_number = 9___u8;
	let _other_number = 1_000_000_000u64;
	let my_nb = 9.; // float(f64) -- 거의 대부분 64씀
	let other_number = 0; // i32
	println!("{}", my_nb as i32 + other_number); // i32로 캐스팅해서 계산
}
/*
출력결과:
9
*/