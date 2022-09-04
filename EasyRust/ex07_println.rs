// macro = function that writes code

fn give_age() -> i32 {
	22
}

fn main() {
	let my_name = "Wendy";
	
	println!("Hello, world!");
	
	// println!("My name is {}"); -- error! ('{}' 한 개 줬으니 인자도 한 개 줘야함)
	println!("My name is {} and my age is {}", my_name, give_age());
	println!("My name is {my_name} and my age is {}", give_age());
}
/*
출력결과:
Hello, world!
My name is Wendy and my age is 22
My name is Wendy and my age is 22
*/