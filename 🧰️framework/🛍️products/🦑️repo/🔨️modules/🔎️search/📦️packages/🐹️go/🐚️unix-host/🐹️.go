//go:build linux || darwin

package search

import (
	"errors"
	"os"
	"syscall"
)

func tryGenerationLease(file *os.File, shared bool) (bool, error) {
	mode := syscall.LOCK_EX
	if shared {
		mode = syscall.LOCK_SH
	}
	err := syscall.Flock(int(file.Fd()), mode|syscall.LOCK_NB)
	if errors.Is(err, syscall.EWOULDBLOCK) || errors.Is(err, syscall.EAGAIN) {
		return false, nil
	}
	return err == nil, err
}
func releaseGenerationLease(file *os.File) error {
	return syscall.Flock(int(file.Fd()), syscall.LOCK_UN)
}
