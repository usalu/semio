package main

import (
	"errors"
	"os"
	"syscall"
)

func openLocked(path string) (*os.File, error) {
	before, err := os.Lstat(path)
	if err != nil {
		return nil, err
	}
	if !before.Mode().IsRegular() {
		return nil, errors.New("regular payload required")
	}
	name, err := syscall.UTF16PtrFromString(path)
	if err != nil {
		return nil, err
	}
	handle, err := syscall.CreateFile(name, syscall.GENERIC_READ, syscall.FILE_SHARE_READ, nil, syscall.OPEN_EXISTING, syscall.FILE_ATTRIBUTE_NORMAL, 0)
	if err != nil {
		return nil, err
	}
	file := os.NewFile(uintptr(handle), path)
	if file == nil {
		syscall.CloseHandle(handle)
		return nil, errors.New("payload open failed")
	}
	after, err := file.Stat()
	if err != nil || !os.SameFile(before, after) {
		file.Close()
		return nil, errors.New("payload identity changed")
	}
	return file, nil
}
