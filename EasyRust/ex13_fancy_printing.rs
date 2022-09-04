fn main() {
	// print newlines
	print!("This\nis\na\nbunch\nof\nlines\n\n");
	
	// print '\n' in literal
	print!("This\\nis\\na\\nbunch\\nof\\nlines\n\n");
	
	// raw text
	print!(r#"c:\thisdrive\new_drive"#); // 중간에 \t\n 이런거 그대로 글자로 출력
	
	// rust 컴파일러가 자동으로 개행을 인식
	println!("Let me tell you
어떤 이야기를
봅시다\n");

	// print format option
	let my_variable = &9;
	println!("{:p}", my_variable); // pointer address
	println!("{:x}", my_variable); // hex-decimal
	println!("{:X}", my_variable); // hex-decimal (capital)
	println!("{:b}", my_variable); // byte
	println!("");
	
	// complicated print
	let title = "TODAY'S NEWS";
	println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
	let bar = "|";
	println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 charaters each, one to ...
	let city1 = "SEOUL";
	let city2 = "TOKYO";
	println!("{b:-<15}{a:->15}", a=city1, b=city2); // variable names city1 and city2, pad with ...
}

/*
출력결과:
This
is
a
bunch
of
lines

This\nis\na\nbunch\nof\nlines

c:\thisdrive\new_driveLet me tell you
어떤 이야기를
봅시다

0x104099968
9
9
1001

---------TODAY'S NEWS---------
|                            |
TOKYO--------------------SEOUL
*/
