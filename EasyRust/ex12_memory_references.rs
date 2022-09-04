// rust는 시스템 프로그래밍 언어로 만들어서, stack/heap/pointers에 대해 알아야 함
// stack : 함수 종료되면 로컬변수 소멸
// heap : stack 공간이 부족할 때 이 영역도 씀. pointer로 접근. 접근하기 더 오래걸리긴 해도 더 공간이 많다.
// pointer : rust에는 reference라는 특별한 포인터를 쓴다. 책과 같음 (e.g. 1페이지로 가기, 15페이지로 가기)
//                  reference는 잠깐 빌리는 것과 같음.

fn main() {
	let my_number = 15; // i32
	let single_reference = &my_number; // reference to my_number (&i32)
	let double_reference = &single_reference; // &&i32
	let five_references = &&&&&my_number; // &&&&&i32
	
	// 주의 : && != &
}