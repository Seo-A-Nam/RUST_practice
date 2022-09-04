// MUTABILITY (기본적으로는 변경 불가 / 'mut' 키워드 붙여 선언하면 변경 가능)
// immutable by default - rust의 특징, 한번 선언한 변수는 바꿀 수가 없다!
// mut <- 변수를 바꿀 수 있게하려면 이런 'mut'라는 키워드 붙여 선언해줘야함.

// SHADOWING (같은 이름을 다시 쓰는 것)
// 안전하다!
// 접속할 수 없는 것을 접속할 수 없게 함
//  --> 이전 값을 덮어씌워서 완전히 바꿔주기 때문에, 이전 값을 볼 수 없음

fn double(input: i32) -> i32 {
	input * 2
}

fn triple(input: i32) -> i32 {
	input * 3
}

fn main() {
	//let my_number = 10;
	//my_number = 9; -- error!
	let mut my_number = 10;
	println!("{my_number}");
	my_number = 9;
	println!("{my_number}");
	println!("");
	
	let my_variable = 10;
	println!("{my_variable}");
	let my_variable = "My variable";
	println!("{my_variable}");
	println!("");
	
	let x = 9;
	println!("{}", x);
	let x = double(x);
	println!("{}", x);
	let x = triple(x);
	println!("{}", x);
	println!("");
	
	let my_variable = 9;
	{
		println!("{}", my_variable);
		let my_variable = "Some string";
		println!("{}", my_variable);
		// code block 안에서 변경된 내용은 코드블록 밖에서는 적용되지 x
	}
	println!("{}", my_variable); // code block 안에서 변경했던 내용이 적용되지 x
}
/*
출력결과:
10
9

10
My variable

9
18
54

9
Some string
9
*/
