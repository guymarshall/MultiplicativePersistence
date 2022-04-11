use std::collections::HashMap;

fn multiplicative_persistence(user_input: i128) -> HashMap<String, i128> {
	let steps: i128 = 0;
	let numbers: Vec<i128> = Vec::new();
	while user_input > 10 {
		let string_number: String = user_input.to_string();
		let digits: Vec<&str> = string_number.split("").collect();
		let sum: i128 = 1;
		let digits_count = digits.len();

		for number in 0..=digits_count {
			sum *= digits[number].parse::<i128>().unwrap();
		}

		numbers.push(sum);

		steps += 1;
	}

	let mut result_vector: HashMap<String, i128> = HashMap::new();
	result_vector.insert(
		"steps".to_string(), steps
	);
	result_vector.insert(
		"numbers".to_string(), numbers
	);

	return result_vector;
}

fn main() {
	println!("Hello, world!");
	let user_input: i128 = 277777788888899;

	let highest_steps: HashMap<String, i128> = HashMap::from([
		("steps".to_string(), 0),
		("number".to_string(), 0)
	]);
}

/*
for (let i = 77551000000; i < 1000000000000000; i++) {
	// console.log(`${i}: ${multiplicativePersistence(i).steps}`);
	if (multiplicativePersistence(i).steps > highestSteps.steps) {
		highestSteps.steps = multiplicativePersistence(i).steps;
		highestSteps.number = i;
	}
	if (i % 1000000 == 0) {
		console.log(`Upto ${i} so far: ${highestSteps.number}`);
	}
}

console.log(`Highest step count: ${highestSteps.number} at ${highestSteps.steps}`);
*/