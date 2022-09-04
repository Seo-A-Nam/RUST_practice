use std::mem::size_of;

fn main() {
	// println!("Size of a char: {}"); -- 컴파일 에러 (저 '{}' 위치에 넣을 다음 인자를 줘야함)
	println!("Size of a char : {}", size_of::<char>()); // 4 bytes
	
	// .len = length()
	println!("Size of string containing 'a' : {}", "a".len()); // .len() gives the size of the string in bytes.
	println!("Size of string containing 'ß' : {}", "ß".len());
	println!("Size of string containing '☮️' : {}", "☮️".len());
	println!("Size of string containing '💟' : {}", "💟".len());
	println!("Size of string '💟💟💟💟' : {}", "💟💟💟💟".len());
	
	let slice = "Hello!";
	println!("Slice is {} bytes and also {} charactors.", slice.len(), slice.chars().count());
	let slice2 = "안녕!";
	println!("Slice2 is {} bytes but only {} charactors.", slice2.len(), slice2.chars().count());
}

/*
출력결과:
Size of a char : 4
Size of string containing 'a' : 1
Size of string containing 'ß' : 2
Size of string containing '☮️' : 6
Size of string containing '💟' : 4
Size of string '💟💟💟💟' : 16
Slice is 6 bytes and also 6 charactors.
Slice2 is 7 bytes but only 3 charactors.
*/