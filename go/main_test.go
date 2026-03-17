package main

import "testing"

func TestCountPrimes(t *testing.T) {
	tests := []struct {
		limit    int
		expected int
	}{
		{limit: 0, expected: 0},
		{limit: 2, expected: 0},
		{limit: 10, expected: 4},
		{limit: 20, expected: 8},
		{limit: 100, expected: 25},
	}
	for _, tt := range tests {
		actual := CountPrimes(tt.limit)
		if actual != tt.expected {
			t.Errorf("CountPrimes(%d) expected %d, got %d", tt.limit, tt.expected, actual)
		}
	}
}
