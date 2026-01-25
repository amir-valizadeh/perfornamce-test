package main

import (
	"fmt"
)

func CountPrimes(limit int) int {
	var number = 0
	var isPrime = false
	for i := 2; i < limit; i++ {
		if i == 0 || i == 1 {
			continue
		} else {
			isPrime = true
			for j := 2; j <= i/2; j++ {
				if i%j == 0 {
					isPrime = false
					break
				}
			}
		}
		if isPrime {
			number++
		}
	}
	return number
}

func main() {
	fmt.Println(CountPrimes(300000))
}
