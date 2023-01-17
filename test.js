const lib = require('.');
const obj1 = lib.printAsNapiObj();
console.log('obj:', obj1, typeof obj1);
const obj2 = lib.printAsValue();
console.log('obj:', obj2, typeof obj2);

const str = lib.printAsString();
console.log('str:', str, typeof str);
