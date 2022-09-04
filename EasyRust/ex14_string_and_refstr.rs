// String
// - growable string (커질수도 있고, 작아질수도 있고)
// - owned type (ref를 통해서 하는게 아니라, string 자체가 자기 데이터를 갖고 있음.
//                  소유권 없어지면 해당 변수의 데이터도 없어짐)
	
// &str ref str "string slice" -- 우리가 아는 "문자열" 형태는 &str 타입이고, String은 이거랑 다른 자료형임

fn main() {
	// String = Sized type
	// str = dynamic type (e.g. "Wendy!" -- 따옴표 안에 문자열 얼마든 지 넣을 수 o)
	
	let my_name = "Wendy"; // &str
	println!("{my_name}");
	
	let my_name = "Wendy".to_string(); // String
	let other_name = String::from("Wendy2"); // String
	println!("{my_name} {other_name}\n");
	
	// growable + shrinkable 
	let mut my_other_name = "Wendy3".to_string();
	my_other_name.push('!');
	println!("{my_other_name}");
}

/*
출력결과:
Wendy
Wendy Wendy2

Wendy3!
*/