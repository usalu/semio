package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"syscall"
	"unicode/utf16"
	"unsafe"
)

type cabinetInfo struct {
	Name                   *uint16
	Size                   uint32
	Error                  uint32
	Date, Time, Attributes uint16
	Target                 [260]uint16
}

func cabinetName(pointer *uint16) (string, error) {
	if pointer == nil {
		return "", errors.New("missing cabinet member")
	}
	chars := make([]uint16, 0, 128)
	for i := uintptr(0); i < 128; i++ {
		c := *(*uint16)(unsafe.Add(unsafe.Pointer(pointer), 2*i))
		if c == 0 {
			return syscall.UTF16ToString(chars), nil
		}
		chars = append(chars, c)
	}
	return "", errors.New("cabinet member too long")
}

func singleLink(file *os.File) error {
	if file == nil {
		return errors.New("missing cabinet target handle")
	}
	var info syscall.ByHandleFileInformation
	if err := syscall.GetFileInformationByHandle(syscall.Handle(file.Fd()), &info); err != nil {
		return err
	}
	if info.NumberOfLinks != 1 {
		return errors.New("cabinet target link count changed")
	}
	return nil
}

func extractCabinets(ctx context.Context, inputs []cabinetInput, plan []sdkFile, stage string, progress func(int, int)) (result []outputFile, failure error) {
	owned := []ownedPath{}
	defer func() {
		if failure != nil {
			failure = errors.Join(failure, cleanupOwned(owned))
		}
	}()
	return extractCabinetsOwned(ctx, inputs, plan, stage, progress, &owned)
}

func extractCabinetsOwned(ctx context.Context, inputs []cabinetInput, plan []sdkFile, stage string, progress func(int, int), records *[]ownedPath) (result []outputFile, failure error) {
	root, err := os.Lstat(stage)
	if err != nil || !root.IsDir() || root.Mode()&os.ModeSymlink != 0 {
		return nil, errors.New("physical stage required")
	}
	if len(inputs) == 0 || len(inputs) > 64 || len(plan) == 0 || len(plan) > 5000 {
		return nil, errors.New("SDK inventory bound exceeded")
	}
	groups := map[string]map[string]sdkFile{}
	ids, paths := map[string]bool{}, map[string]bool{}
	var total int64
	for _, row := range plan {
		path, err := archivePath(row.Path)
		if err != nil || !strings.HasPrefix(path, "sdk/") || !fileID.MatchString(row.ID) || !cabinetID.MatchString(row.Cabinet) || row.Bytes < 0 || row.Bytes > 100000000 || ids[strings.ToLower(row.ID)] || paths[strings.ToLower(path)] {
			return nil, errors.New("unsafe SDK file plan")
		}
		if len(utf16.Encode([]rune(filepath.Join(stage, filepath.FromSlash(path))))) > 256 {
			return nil, errors.New("SDK path budget exceeded")
		}
		ids[strings.ToLower(row.ID)] = true
		paths[strings.ToLower(path)] = true
		total += row.Bytes
		if total > 2000000000 {
			return nil, errors.New("SDK byte bound exceeded")
		}
		if groups[row.Cabinet] == nil {
			groups[row.Cabinet] = map[string]sdkFile{}
		}
		groups[row.Cabinet][row.ID] = row
	}
	names := map[string]cabinetInput{}
	var held []*os.File
	defer func() {
		for _, file := range held {
			file.Close()
		}
	}()
	for _, input := range inputs {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if !cabinetID.MatchString(input.Name) || names[input.Name].Name != "" || groups[input.Name] == nil {
			return nil, errors.New("unexpected cabinet")
		}
		names[input.Name] = input
		file, err := openLocked(input.Path)
		if err != nil {
			return nil, err
		}
		held = append(held, file)
		stat, err := file.Stat()
		if err != nil || stat.Size() != input.Bytes || input.Bytes <= 0 || input.Bytes > 60000000 {
			return nil, errors.New("cabinet extent mismatch")
		}
		digest := sha256.New()
		if _, err := io.Copy(digest, contextReader{ctx, file}); err != nil {
			return nil, err
		}
		if hex.EncodeToString(digest.Sum(nil)) != strings.ToLower(input.SHA256) {
			return nil, errors.New("cabinet digest mismatch")
		}
	}
	if len(names) != len(groups) {
		return nil, errors.New("missing cabinet")
	}
	if err := ensureDirectories(stage, "sdk", records); err != nil {
		return nil, err
	}
	var current map[string]sdkFile
	seen := map[string]bool{}
	var outputs []*os.File
	outputByPath := map[string]*os.File{}
	defer func() {
		for _, file := range outputs {
			file.Close()
		}
	}()
	var callbackErr error
	callback := syscall.NewCallback(func(_ uintptr, notification uint32, first, _ uintptr) uintptr {
		if err := ctx.Err(); err != nil {
			callbackErr = err
			return 0
		}
		if notification == 0x11 {
			info := (*cabinetInfo)(unsafe.Pointer(first))
			name, err := cabinetName(info.Name)
			row, ok := current[name]
			if err != nil || !ok || seen[name] || row.Bytes != int64(info.Size) {
				callbackErr = errors.New("unexpected cabinet member")
				return 0
			}
			parent := filepath.ToSlash(filepath.Dir(filepath.FromSlash(row.Path)))
			if err := ensureDirectories(stage, parent, records); err != nil {
				callbackErr = err
				return 0
			}
			target := filepath.Join(stage, filepath.FromSlash(row.Path))
			pointer, err := syscall.UTF16PtrFromString(target)
			if err != nil {
				callbackErr = err
				return 0
			}
			handle, err := syscall.CreateFile(pointer, 0, syscall.FILE_SHARE_READ|syscall.FILE_SHARE_WRITE, nil, syscall.CREATE_NEW, syscall.FILE_ATTRIBUTE_NORMAL, 0)
			if err != nil {
				callbackErr = err
				return 0
			}
			file := os.NewFile(uintptr(handle), target)
			if file == nil {
				syscall.CloseHandle(handle)
				callbackErr = errors.New("cabinet target open failed")
				return 0
			}
			outputs = append(outputs, file)
			outputByPath[target] = file
			created, err := file.Stat()
			if err != nil {
				callbackErr = err
				return 0
			}
			if err := recordOwned(records, target, created); err != nil {
				callbackErr = err
				return 0
			}
			if err := singleLink(file); err != nil {
				callbackErr = err
				return 0
			}
			encoded, err := syscall.UTF16FromString(target)
			if err != nil || len(encoded) > len(info.Target) {
				callbackErr = errors.New("cabinet path budget exceeded")
				return 0
			}
			copy(info.Target[:], encoded)
			seen[name] = true
			return 1
		}
		if notification == 0x12 {
			callbackErr = errors.New("continued cabinet unsupported")
			return 1
		}
		return 0
	})
	ordered := make([]string, 0, len(names))
	for name := range names {
		ordered = append(ordered, name)
	}
	sort.Strings(ordered)
	library, err := systemDLL("setupapi.dll")
	if err != nil {
		return nil, err
	}
	api := library.NewProc("SetupIterateCabinetW")
	for _, name := range ordered {
		current = groups[name]
		pointer, err := syscall.UTF16PtrFromString(names[name].Path)
		if err != nil {
			return nil, err
		}
		ok, _, callErr := api.Call(uintptr(unsafe.Pointer(pointer)), 0, callback, 0)
		if ok == 0 || callbackErr != nil {
			return nil, errors.Join(callbackErr, callErr)
		}
		if progress != nil {
			progress(len(seen), len(plan))
		}
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if len(seen) != len(plan) {
		return nil, errors.New("missing cabinet members")
	}
	output := make([]outputFile, 0, len(plan))
	for _, row := range plan {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		path := filepath.Join(stage, filepath.FromSlash(row.Path))
		if err := singleLink(outputByPath[path]); err != nil {
			return nil, err
		}
		file, err := openLocked(path)
		if err != nil {
			return nil, err
		}
		stat, err := file.Stat()
		if err != nil || stat.Size() != row.Bytes {
			file.Close()
			return nil, errors.New("SDK file extent mismatch")
		}
		digest := sha256.New()
		_, err = io.Copy(digest, contextReader{ctx, file})
		closeErr := file.Close()
		if err != nil {
			return nil, err
		}
		if closeErr != nil {
			return nil, closeErr
		}
		output = append(output, outputFile{row.Path, row.Bytes, hex.EncodeToString(digest.Sum(nil))})
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	sort.Slice(output, func(i, j int) bool { return output[i].Path < output[j].Path })
	return output, nil
}
