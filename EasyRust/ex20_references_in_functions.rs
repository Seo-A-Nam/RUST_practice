// OWNERSHIP
fn print_country(country_name: String) {
	println!("My country is {}", country_name);
}

fn print_ref_country(country_name: &String) {
	println!("My country is {}", country_name);
}

fn main() {
	// 1. 다른 함수로 소유권 이전되는 경우 (move semantics)
	let country = "대한민국".to_string();
	print_country(country); // country 변수의 소유권은 저 함수로 넘어가서, main에서는 아예 사라지게 됨
	// print_country(country); -- 2번째 출력에서 에러남 ! (move semantics 때문에)
	
	// 2. 다른 함수에 ref(볼 수 있는 권한)만 주는 경우
	let country = "대한민국".to_string();
	print_ref_country(&country);
	print_ref_country(&country);
	print_ref_country(&country);
		// 다른 함수로 소유권 이전되지 않아서 안전함!
}
/*
출력결과:
My country is 대한민국
My country is 대한민국
My country is 대한민국
My country is 대한민국
*/