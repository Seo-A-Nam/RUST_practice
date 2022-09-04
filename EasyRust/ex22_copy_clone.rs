// Ownership and copy types

// copy type이 뭐냐? : i32, char...
//  -> ownership과는 상관없이 쓸 수 있다!
// It's trivial to copy the bytes
// (= 이런 type들은 copy가 너무 쉽기 때문에, 굳이 소유권 같은 방식 쓸 필요 없다)
// (= copy type에서는 소유권 이전 문제를 신경쓸 필요가 없다!)

fn prints_number(number: i32) {
	println!("{}", number);
}

fn prints_string(input: String) {
	println!("{}", input);
}

// copy - copy types
// clone - non-copy types 
fn main() {
	let my_number = 8;
	prints_number(my_number); // 함수로 소유권이 넘어가는 게 아니라, copy본이 넘어감
	prints_number(my_number);
	
	let my_country = "Austria".to_string();
	prints_string(my_country.clone()); 
		// 소유권 이전하지 않고 복사본만 전달 (외부 라이브러리를 가져왔다거나 해서 그 함수 코드 바꾸기 힘든 경우에, 이렇게 복사본만 넘기는 식으로도 ㄱㄴ)
		// 어차피 rust가 워낙에 컴파일이 빨라서, 코드 최적화 하기 전에는 이렇게 clone()을 좀 써서 기능만 먼저 돌아가게 만드는 수도 있음.
	prints_string(my_country);
	//prints_string(my_country); -- error! (소유권 이전되어 소멸했음)
}
/*
출력결과:
8
8
Austria
Austria
*/