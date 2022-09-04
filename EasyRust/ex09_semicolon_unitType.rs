// () - empty tuple, unit type (void)
// expression-based language

//fn number() -> i32 {
//	8; // return 8 이렇게 안해도 됨
//}

fn empty_tuple() {

}
// 여기서 main()은 i32 이런거 리턴 못함. empty_tuple(= void)로 하거나 아니라면 return 하는 게 있어야함.
fn main() {
	// let my_number = number(); -- error!
	let tuple = empty_tuple();
	
	// Display {}
	// println!("{}", tuple) -- error! (display print가 안되는 경우)
	
	// Debug print
	println!("{:?}", tuple)
		// 보통의 display print가 안 되는 경우에 이런 식의 debug print를 대신 해줄 수 있음	
}

/*
출력결과:
()
*/