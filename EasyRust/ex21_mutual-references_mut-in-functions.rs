// mutable references in functions

fn add_is_great(country_name: &mut String) {
	country_name.push_str(" is great!");
	println!("Now it says: {}", country_name);
}

fn add_is_great2(mut country_name: String) { // take by value, declare as mutable
	country_name.push_str(" is great!");
	println!("Now it says: {}", country_name);
}

fn main() {
	let mut my_country = "캐나다".to_string();
	
	add_is_great(&mut my_country); // by mutable reference
	add_is_great(&mut my_country);
	
	// 신기한 예제 (왜 돌아갈까요?)
	let my_country = "대한민국".to_string();
	add_is_great2(my_country)
		// 이유 : 변수 my_country가 소유권을 가지고 있다가, add_is_great2로 소유권이 완전히 넘어감.
		//       그래서 add_is_great2() 매개변수 형식에 mutable이 있어도 됨. (왜냐면 지가 완전한 소유권자라서)
}

/*
출력결과:
Now it says: 캐나다 is great!
Now it says: 캐나다 is great! is great!
Now it says: 대한민국 is great!
*/