// OWNERSHIP - 소유권
// 어떤 변수가 자기 데이터를 언제까지 가질 수 있는 지, 누가 그 데이터를 가지는 지.

// & = reference

//fn return_it() -> &String {
//	let country = String::from("대한민국");
//	&country // return &String
//}

fn main() {
	let country = String::from("대한민국");
	let ref_one = &country;
	let _ref_two = &country;
	
	println!("Country is {}", ref_one);
	
	// let my_country = return_it(); -- error! : 종료된 함수 내 로컬 변수니까 소멸되어 포인터 리턴 불가!
}
/*
출력결과:
Country is 대한민국
*/