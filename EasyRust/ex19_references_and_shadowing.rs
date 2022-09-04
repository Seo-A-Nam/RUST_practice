fn main() {
	// 1. 에러 나는 상황
	
	//let mut number = 10;
	//let number_ref = &number; // immutable ref
	//let number_change = &mut number; // mutable ref
	//*number_change += 10;
	//println!("{}", number_ref);
	
	// error[E0502]: cannot borrow `number` as mutable because it is also borrowed as immutable
	
	
	// 2. 에러가 날 만한데도 안나는 경우 (mutable ref 이후 immutable ref 사용)
	let mut number = 10;
	let number_change = &mut number; // mutable ref

	*number_change += 10;
	let number_ref = &number;
	println!("{}", number_ref);
	// 현재 버전 rust에서는 에러 안뜸 (예전 버전에서는 에러 떳다고 함)
	// 에러 안 뜨는 이유 : mutable에 immutable ref포인터 연결하긴 했지만, 적어도 immutable 선언 이후에 바뀐 내용은 없으니 문제 없음
	// (뭔가 rust 컴파일러가 무지 똑똑한 것 같다...)
	
	// 3. shadowing (새로 정의해 덮어씌웠던 변수에서, 그 이전 값이 이전의 포인터에 남아있다?!)
	let country = "대한민국";
	let country_ref = &country;
	let country = 8;
	
	println!("{country_ref}, {country}")
	// country의 바꾸기 전 값은 shadowing으로 인해 볼 수 없으나, ref를 통해서는 볼 수 있다! (그 데이터가 없어지지 않았다)
} 
/*
출력결과:
20
대한민국, 8
*/