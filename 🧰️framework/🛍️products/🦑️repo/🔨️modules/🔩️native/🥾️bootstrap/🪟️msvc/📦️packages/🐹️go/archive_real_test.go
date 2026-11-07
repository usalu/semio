package main

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedMsvcArchives(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("verified publisher payloads not provided")
	}
	hashes := []string{"03f56d24dd23314467144060c8ee70ca0fdd79a901b60e7d2f92f9cbe8d042cf", "40bc5dd011e16ee46698a8e36e71f4c85ee34dbe938368b295ed27690c569860", "428bcda4a867056ab696aeca95b1057f5282aa9a8121481e7787fae7548efde2", "254cbe1d5d5ac71ef7aebecba9980d530d3b3f63984d43bd5aca66b22d19ae30", "2ed606cdc1cb7e5d311ec19364b7df9e9932817ebfed761ae358cd438936dd57"}
	sizes := []int64{28454839, 239072, 2057481, 51736641, 28628657}
	inputs := make([]archiveInput, len(hashes))
	for i := range inputs {
		inputs[i] = archiveInput{filepath.Join(root, fmt.Sprintf("msvc-%d.vsix", i)), hashes[i], sizes[i]}
	}
	rows, err := extractArchives(context.Background(), inputs, t.TempDir(), nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(rows) != 494 {
		t.Fatalf("got %d files", len(rows))
	}
}
