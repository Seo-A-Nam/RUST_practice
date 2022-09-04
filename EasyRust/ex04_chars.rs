fn main() {
	println!("Hello, world!");
	let first_letter = 'A'; // char은 1글자만 포함
	let space = ' '; // A space inside '' is also a char
	let other_language_char = '⍦';
		// Thanks to Unicode, other languages like Cherokee display just fine 
	let cat_face = '😸'; // Emojis are chars too
	// 유니코드는 크기에 한계가 없음 (바이트 연속으로 써주면 되는 거라서)
	
	println!("{} {} {} {} ", first_letter, space, other_language_char, cat_face);
	
	// 	casting = simple type change
	let my_number: u16 = 8;
	let second_number: u8 = 10;
	let third_number = my_number + second_number as u16; // 이런 식으로 캐스팅을 한다!
	println!("{}", third_number);
	
	let nb = 'a' as u8;
	println!("Hello, world! My number is {}", nb);
}

/*
출력결과 : 
Hello, world!
A   ⍦ 😸 
18
Hello, world! My number is 97
*/