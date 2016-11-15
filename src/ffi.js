const ffi = require('ffi');
const ArrayType = require('ref-array');
const path = require('path');
const ref = require('ref');
const Struct = require('ref-struct');

// const Point = Struct({
// 	x: ref.types.float,
// 	y: ref.types.float
// });

// const PointArray = ArrayType(Point);
// const ArrayOfArray = ArrayType(PointArray);

const lib = ffi.Library('target/release/libffi', {
	// Project Euler Problems
	// 'multiples_of_three_and_five': ['uint64', ['uint64']],
	// 'even_fibonacci_numbers': ['uint64', ['uint64']],
	// 'largest_prime_factor': ['uint64', ['uint64']],
	// 'largest_palindrome_product': ['uint64', ['uint32']],
	// 'smallest_multiple': ['uint64', ['uint64']],
	// 'sum_square_difference': ['uint64', ['uint64']],
	// 'nth_prime': ['uint64', ['uint64']],
	// 'largest_product_in_a_series': ['uint64', ['uint64']],
	// 'special_pythagorean_triplet': ['uint64', []],
	// 'summation_of_primes': ['uint64', ['uint64']],

	// 'highly_divisible_triangular_number': ['uint64', ['uint64']],
	// 'large_sum': ['uint', []],
	// 'longest_collatz_sequence': ['uint64', []],

	// 'power_digit_sum': ['uint64', ['uint64']],


	// 'counting_sundays': ['uint64', []],
	// 'amicable_numbers': ['uint64', ['uint64']],




	// 'digit_fifth_powers': ['uint64', ['uint64']],



	// 'digit_factorials': ['uint64', []],
	// 'pentagon_numbers': ['void', []],


	// 'distinct_powers': ['uint64', ['uint64']],
	// 'consecutive_prime_sum': ['uint64', ['uint64']],
	// 'self_powers': ['uint64', ['uint64']],
	// 'combinatoric_selections': ['uint64', ['uint64']],
	// 'powerful_digit_sum': ['uint64', []],

	// Misc functions
    'are_any_points_in_path': ['bool', ['uint64', 'uint64', 'uint64']],
    'fibonacci': ['uint64', ['uint64']],
    'largest_collatz_sequence': ['uint64', ['uint64']]
});

const executionMap = {
};


function invoke(fn, rgs) {
	const target = executionMap[fn] || lib[fn];
	if(!target) {
		throw new Error(`${fn} is not linked to the library interface`);
	}
	return target(...rgs);
}

const [node, file, fn, ...args] = process.argv;

console.time('invoke');
console.log(invoke(fn, args));
console.timeEnd('invoke');
