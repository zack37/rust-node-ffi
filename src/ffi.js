const ffi = require('ffi');
const path = require('path');

const lib = ffi.Library(path.resolve(path.join(__dirname, '..', 'target', 'release', 'libffi')), {
    'are_any_points_in_path': ['bool', ['float', 'float']],
    'fibonacci': ['int', ['int']],
    'largest_collatz_sequence': ['int', ['int']]
});

const [node, file, fn, ...args] = process.argv;

function invoke(fn, rgs) {
	target = lib[fn];
	if(!target) {
		throw new Error(`${fn} is not linked to the library interface`);
	}
	return target(...rgs);
}

console.log(`Result of ${fn}: ${invoke(fn, args)}`);