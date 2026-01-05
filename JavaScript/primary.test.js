const assert = require('assert');
const { countPrimes } = require('./primary.js');

function runTests() {
    assert.strictEqual(countPrimes(10), 4);
    assert.strictEqual(countPrimes(20), 8);
    assert.strictEqual(countPrimes(100), 25);
    console.log("JavaScript tests passed.");
}

runTests();
