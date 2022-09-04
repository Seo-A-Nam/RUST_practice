/*
	primitive type(기본 타입)
	integer는 정수, 
	
	+ plus sign
	- minus sign
		** i8, i16, i32, i64, i128 and isize. // signed Integer (음수 양수 둘 다 있음)
		** u8, u16, u32, u64, and usize. // Usigned (양수만 존재)
	
	* bits = 8 bit = 1 bytes
	 
	* size = computer architecture에 따른 것
	* isize -> 32비트 -> i32
	* usize -> 64비트 -> u64
*/

fn main() {
	// type interface
	let temp   = 100; // i32가 default
	let my_number: u8 = 100; // 255가 최대 범위
	let my_other_number = 100; // 원래 default는 i32 타입
	let my_thrid_number = my_number + my_other_number;
		// ** 주의 : () = u8 + default 이니까 default를 u8로 가정해서 my_thrid_number을 u8로 캐스팅함
		// 만일 default가 아닌 서로 다른 타입을 연산하려고 하면 타입에러 남! (만일 저기서 my_other_nubmer의 타입을 u16으로 했다면 에러 낫을 것)
}