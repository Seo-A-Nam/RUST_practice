// & immutable reference / shared reference (안전하게 공유 가능)
// &mut mutable reference / unique reference (하나만 쓸 수 있음 -- 하나만 바꿀 수 있게 함)

fn main() {
	let mut my_number1 = 9;
	let mut my_number2 = 9;
	let num_ref = &mut my_number1; // num_ref가 my_number 데이터 바꿀 수 있음
	let temp_ref = &mut &mut my_number2;
	
	// num_ref = 10; -- 에러! : 포인터가 아닌 값 자체를 봐야함
	*num_ref = 10;
	**temp_ref = 12;
	println!("Number is now my_nubmer1 = {}, my_number2 = {}", my_number1, my_number2);
	
}

/* 
** 왜 mutable reference와 immutable reference를 같이 못쓰는 가???
-- ppt 만드는 상황을 상상하기

1. only one mutable reference
  ppt를 만드는 데 상사한테 컴퓨터 빌려주고 피드백 받기 (내 컴퓨터 안전 -- 나랑 상사가 동시에 쓸 수 없으니까)
  상사가 내 컴퓨터 돌려주면 내가 그걸 받고서 바꿀 수 있음
  
2. only immutable references
  내가 100명 앞에서 ppt를 실제로 발표를 하는 데, 남들은 그 데이터를 볼 수만 있음.
  자기네들이 본 그 데이터를 적어놓을 수 있고 자기맘대로 바꿀 수 있는데, 내 컴퓨터를 직접 만질 수는 없으니 내 데이터는 바뀌지 않아 안전함!

3. 문제 나는 상황
  내 컴퓨터가 mutable이라 상사랑 공유하는 컴퓨터임.
  근데 내가 ppt 발표 중인데 상사가 알아서 막 내 ppt 바꾸기 시작하면 문제가 나겠죠?
  이런 경우에는 위험한 상황이라, Rust에서 막아주게 됨!
*/