#both python and pypy have a same syntax

count = 0
is_prime = False
for c in range(2, 3_00_000):
    is_prime = True
    for inner_c in range(2, int(c / 2 + 1)):
        if c % inner_c == 0:
            is_prime = False
            break

    if is_prime:
        count += 1

print(count)
