let count = 0;
let is_prime = false;

for (let i = 2; i < 3_000_000; i++) {
	is_prime = true;
	for (let j = 2; j < (i / 2 + 1); j++) {
		if (i % j == 0) {
			is_prime = false;
			break;
		}

	}
		if (is_prime) count += 1;

}

console.log({ count });
