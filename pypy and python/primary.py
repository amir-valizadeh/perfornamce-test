def count_primes(limit):
    count = 0
    for c in range(2, limit):
        is_prime = True
        for inner_c in range(2, int(c / 2 + 1)):
            if c % inner_c == 0:
                is_prime = False
                break
        if is_prime:
            count += 1
    return count

if __name__ == '__main__':
    print(count_primes(300_000))
