// String (method = .function)
//  .capacity
//  .push
//  .push_str
//  .pop
//  with_capacity
	
fn main() {
	let mut my_name = "Wendy".to_string();
	my_name.push('!');
	my_name.push_str(" and I live in Seoul");
	println!("My name is {my_name}\n");
	
	// reallocation
	let mut my_name = "".to_string();
	println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
	my_name.push_str("Wendy!");
	println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
	my_name.push_str(" and I live in Seoul");
	println!("Length is {} and Capacity is: {}\n", my_name.len(), my_name.capacity());
		// capacity는 실제 length 보다 좀 크다. 만일 length가 capacity 초과하면 다시 할당하여 capacity를 늘려준다.

	// reallocation 없도록 하기
	let mut my_name = String::with_capacity(26); // 이렇게 직접 capcity 정해줄 수도 있음.
	println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
	my_name.push_str("Wendy!");
	println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
	my_name.push_str(" and I live in Seoul");
	println!("Length is {} and Capacity is: {}", my_name.len(), my_name.capacity());
		// capacity를 미리 크게 정해줬기 때문에, 중간에 초과되지 않아 reallocation이 없었다
	
	// "David".to_string(); // 이미 있는 &str -> String
	// String::from("David"); // 새로 String 생성 -- 처음에 "문자열" 주지 않는 경우는 String:: 이런 형식을 쓰게 됨
}

/*
출력결과:
My name is Wendy! and I live in Seoul

Length is 0 and Capacity is: 0
Length is 6 and Capacity is: 8
Length is 26 and Capacity is: 26

Length is 0 and Capacity is: 26
Length is 6 and Capacity is: 26
Length is 26 and Capacity is: 26
*/