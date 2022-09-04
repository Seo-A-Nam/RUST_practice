fn give_number(one: i16, two: i16) -> i16 {
	one * two // !! 주의 !! : 리턴 시 ';' 없음
}

fn print_number(one: i32, two: i32) {
	let multiplied = one * two;
	println!("{}", multiplied);
}

fn give_number2(one: i32, two: i32) -> i32 {
	let multiplied_by_ten = {
		let first_number = 10;
		first_number * one * two
	};
	multiplied_by_ten
}

fn main() {
	let my_nb = give_number(9, 8);
	let my_nb2 = give_number2(9, 8);
	
	println!("{}", my_nb);
	println!("{}", my_nb2);
	print_number(7, 8);
}

/*
출력결과:
72
720
56
*/