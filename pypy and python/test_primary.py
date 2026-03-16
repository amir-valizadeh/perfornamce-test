import unittest
from primary import count_primes

class TestPrimary(unittest.TestCase):
    def test_count_primes(self):
        self.assertEqual(count_primes(0), 0)
        self.assertEqual(count_primes(2), 0)
        self.assertEqual(count_primes(10), 4)
        self.assertEqual(count_primes(20), 8)
        self.assertEqual(count_primes(100), 25)

if __name__ == '__main__':
    unittest.main()
