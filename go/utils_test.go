package aoc_2021

import "testing"

func TestReadInputLines(t *testing.T) {
	testCases := []struct {
		path        string
		expectedErr bool
	}{
		{"../inputs/1.txt", false},
		{"../inputs/none.txt", true},
	}

	for _, tc := range testCases {
		_, err := ReadInputLines(tc.path)
		if tc.expectedErr && err == nil {
			t.Errorf("expected error when reading %s", tc.path)
		}
	}
}

func TestLinesToInts(t *testing.T) {
	testCases := []struct {
		lines       []string
		expectedErr bool
	}{
		{[]string{"123", "456", "789"}, false},
		{[]string{"foo", "bar", "baz"}, true},
		{[]string{"123", "bar", "789"}, true},
	}

	for _, tc := range testCases {
		ints, err := LinesToInts(tc.lines)
		if tc.expectedErr && err == nil {
			t.Errorf("expected error, got: %v", ints)
		}
	}
}
