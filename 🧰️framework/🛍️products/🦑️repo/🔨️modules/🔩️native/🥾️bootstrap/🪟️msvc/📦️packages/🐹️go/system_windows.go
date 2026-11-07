package main

import (
	"errors"
	"path/filepath"
	"syscall"
	"unsafe"
)

func systemDLL(name string) (*syscall.LazyDLL, error) {
	if name != "msi.dll" && name != "setupapi.dll" {
		return nil, errors.New("unsupported system library")
	}
	buffer := make([]uint16, 32768)
	kernel := syscall.NewLazyDLL("kernel32.dll")
	n, _, callErr := kernel.NewProc("GetSystemDirectoryW").Call(uintptr(unsafe.Pointer(&buffer[0])), uintptr(len(buffer)))
	if n == 0 || n >= uintptr(len(buffer)) {
		return nil, callErr
	}
	directory := syscall.UTF16ToString(buffer[:n])
	if !filepath.IsAbs(directory) {
		return nil, errors.New("absolute system directory required")
	}
	return syscall.NewLazyDLL(filepath.Join(directory, name)), nil
}
