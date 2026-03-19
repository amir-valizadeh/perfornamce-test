#include <iostream>
#include <chrono>
#include <cassert>

int countPrimes(int limit) {
    int count = 0;
    for (int i = 2; i < limit; i++) {
        bool is_prime = true;
        for (int j = 2; j <= i / 2; j++) {
            if (i % j == 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) count++;
    }
    return count;
}

#ifdef TEST
int main() {
    assert(countPrimes(0) == 0);
    assert(countPrimes(2) == 0);
    assert(countPrimes(10) == 4);
    assert(countPrimes(20) == 8);
    assert(countPrimes(100) == 25);
    std::cout << "C++ tests passed.\n";
    return 0;
}
#else
int main() {
    auto start = std::chrono::high_resolution_clock::now();
    int count = countPrimes(300000);
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double, std::milli> ms = end - start;
    std::cout << count << " - " << ms.count() << "ms\n";
    return 0;
}
#endif
