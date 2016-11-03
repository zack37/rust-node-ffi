const ffi = require('ffi');
const path = require('path');

const lib = ffi.Library(path.join(__dirname, 'ffi', 'target', 'release', 'libffi'), {
    'are_any_points_in_path': ['bool', ['float', 'float']],
    'fibonacci': ['int', ['int']],
    'largest_collatz_sequence': ['int', ['int']]
});

const [node, file, fn, ...args] = process.argv;

console.log(`Result of ${fn}: ${lib[fn](...args)}`);