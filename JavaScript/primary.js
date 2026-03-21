/**
 * Counts prime numbers up to the given limit using O(n^2) complexity.
 * @param {number} limit 
 * @returns {number} number of primes
 */
function countPrimes(limit) {
    let count = 0;
    for (let i = 2; i < limit; i++) {
        let is_prime = true;
        for (let j = 2; j < (i / 2 + 1); j++) {
            if (i % j === 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) count += 1;
    }
    return count;
}

if (require.main === module) {
    console.log({ count: countPrimes(300000) });
}

module.exports = { countPrimes };
