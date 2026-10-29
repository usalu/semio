//go:build windows

package search

import (
	"os"
	"syscall"
	"unsafe"
)

var generationKernel = syscall.NewLazyDLL("kernel32.dll")
var generationLock = generationKernel.NewProc("LockFileEx")
var generationUnlock = generationKernel.NewProc("UnlockFileEx")

func tryGenerationLease(file *os.File, shared bool) (bool, error) {
	flags := uintptr(1)
	if !shared {
		flags |= 2
	}
	overlap := syscall.Overlapped{}
	result, _, err := generationLock.Call(file.Fd(), flags, 0, 1, 0, uintptr(unsafe.Pointer(&overlap)))
	if result != 0 {
		return true, nil
	}
	if err == syscall.Errno(33) || err == syscall.Errno(997) {
		return false, nil
	}
	return false, err
}
func releaseGenerationLease(file *os.File) error {
	overlap := syscall.Overlapped{}
	result, _, err := generationUnlock.Call(file.Fd(), 0, 1, 0, uintptr(unsafe.Pointer(&overlap)))
	if result != 0 {
		return nil
	}
	return err
}
