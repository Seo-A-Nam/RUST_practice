/// document 주석 (이 주석 다음 문장에는 어떤 코드라도 와야함) - documentation
struct _Book;

fn main () { 
	// 변수 선언했으면 꼭 써주거나, 아니면 '_변수' 이런 형식으로 선언해야함. (그렇지 않으면 unused variable 컴파일 에러)
	let _x/*: i16*/ = 10;
	let _y: i16 = 12;
	// -- comment
	println!("Hello, world!");
}

/* 코드 한 줄 중간에 넣는 주석 */
// 일반 주석
// _변수명 -> warning 무시하라는 뜻