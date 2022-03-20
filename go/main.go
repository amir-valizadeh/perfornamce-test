package main

import (
	"fmt"
)

func main() {
	var number = 0
	var isPrime = false
	for i := 2; i <= 300_000; i++ {
		if i == 0 || i == 1 {
			break
		} else {
			isPrime=true
			for j := 2; j <= i/2; j++ {
				if i%j == 0 {
					isPrime=false
					break
				}
			}
			
		

	}
	if isPrime==true {
		number++
	}
	
}
fmt.Println(number)
}

