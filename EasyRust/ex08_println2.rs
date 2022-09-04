fn main() {
	let my_city = "Seoul";
	let year = 2002;
	let population = 9_987_987;
	
	println!("The city of {} in {} had a population of {}", my_city, year, population);
	println!("The city of {my_city} in {year} had a population of {population}");
	println!("The city of {c} in {y} had a population of {p}", c=my_city, y=year, p=population);
	println!("");
	// string interpolation
	println!("The city of {0} in {1} had a population of {2}. I love {0}!", c=my_city, y=year, p=population);
		// ex. {0} == the first  argument of println
}

/*
출력결과:
The city of Seoul in 2002 had a population of 9987987
The city of Seoul in 2002 had a population of 9987987
The city of Seoul in 2002 had a population of 9987987

The city of Seoul in 2002 had a population of 9987987. I love Seoul!
*/