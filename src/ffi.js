const ffi = require('ffi');
const path = require('path');

const lib = ffi.Library(path.resolve(path.join(__dirname, '..', 'target', 'release', 'libffi')), {
    'are_any_points_in_path': ['bool', ['float', 'float']],
    'fibonacci': ['int', ['int']],
    'largest_collatz_sequence': ['int', ['int']]
});

const [node, file, fn, ...args] = process.argv;

function invoke(fn, rgs) {
	return new Promise((resolve, reject) => {
		target = lib[fn];
		if(!target) {
			return reject(new Error(`${fn} is not linked to the library interface`));
		}
		return target.async(...rgs, (err, result) => {
			if(err) {
				return reject(err);
			}
			resolve(result);
		});
	});
}

return invoke(fn, args)
	.then(console.log.bind(console, `Result of ${fn}:`))
	.catch(console.error.bind(console));
