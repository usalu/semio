package main

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"syscall"
	"unsafe"
)

type msiTables struct {
	files              [][5]string
	components         [][2]string
	directories, media [][3]string
}

func canonicalTables(t msiTables) ([]byte, error) {
	var data bytes.Buffer
	encoder := json.NewEncoder(&data)
	encoder.SetEscapeHTML(false)
	err := encoder.Encode(map[string]any{"components": t.components, "directories": t.directories, "files": t.files, "media": t.media})
	return data.Bytes(), err
}

func msiRows(ctx context.Context, db uint32, sql string, width int) ([][]string, error) {
	api, err := systemDLL("msi.dll")
	if err != nil {
		return nil, err
	}
	query, err := syscall.UTF16PtrFromString(sql)
	if err != nil {
		return nil, err
	}
	var view uint32
	code, _, _ := api.NewProc("MsiDatabaseOpenViewW").Call(uintptr(db), uintptr(unsafe.Pointer(query)), uintptr(unsafe.Pointer(&view)))
	if code != 0 {
		return nil, fmt.Errorf("MSI view: %d", code)
	}
	defer api.NewProc("MsiCloseHandle").Call(uintptr(view))
	code, _, _ = api.NewProc("MsiViewExecute").Call(uintptr(view), 0)
	if code != 0 {
		return nil, fmt.Errorf("MSI execute: %d", code)
	}
	rows := [][]string{}
	for {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		var record uint32
		code, _, _ = api.NewProc("MsiViewFetch").Call(uintptr(view), uintptr(unsafe.Pointer(&record)))
		if code == 259 {
			break
		}
		if code != 0 {
			return nil, fmt.Errorf("MSI fetch: %d", code)
		}
		row := make([]string, width)
		for i := range row {
			size := uint32(4096)
			buffer := make([]uint16, size)
			code, _, _ = api.NewProc("MsiRecordGetStringW").Call(uintptr(record), uintptr(i+1), uintptr(unsafe.Pointer(&buffer[0])), uintptr(unsafe.Pointer(&size)))
			if code != 0 {
				api.NewProc("MsiCloseHandle").Call(uintptr(record))
				return nil, fmt.Errorf("MSI field: %d", code)
			}
			row[i] = syscall.UTF16ToString(buffer)
		}
		api.NewProc("MsiCloseHandle").Call(uintptr(record))
		rows = append(rows, row)
		if len(rows) > 5000 {
			return nil, errors.New("MSI row bound exceeded")
		}
	}
	return rows, nil
}

func readMsi(ctx context.Context, input archiveInput) (msiTables, error) {
	var result msiTables
	file, err := openLocked(input.Path)
	if err != nil {
		return result, err
	}
	defer file.Close()
	stat, err := file.Stat()
	if err != nil || stat.Size() != input.Bytes || input.Bytes <= 0 || input.Bytes > 2000000 {
		return result, errors.New("MSI extent mismatch")
	}
	digest := sha256.New()
	if _, err := io.Copy(digest, contextReader{ctx, file}); err != nil {
		return result, err
	}
	if hex.EncodeToString(digest.Sum(nil)) != input.SHA256 {
		return result, errors.New("MSI digest mismatch")
	}
	name, err := syscall.UTF16PtrFromString(input.Path)
	if err != nil {
		return result, err
	}
	api, err := systemDLL("msi.dll")
	if err != nil {
		return result, err
	}
	var db uint32
	code, _, _ := api.NewProc("MsiOpenDatabaseW").Call(uintptr(unsafe.Pointer(name)), 0, uintptr(unsafe.Pointer(&db)))
	if code != 0 {
		return result, fmt.Errorf("MSI open: %d", code)
	}
	defer api.NewProc("MsiCloseHandle").Call(uintptr(db))
	files, err := msiRows(ctx, db, "SELECT `File`, `Component_`, `FileName`, `FileSize`, `Sequence` FROM `File`", 5)
	if err != nil {
		return result, err
	}
	for _, row := range files {
		result.files = append(result.files, [5]string(row))
	}
	components, err := msiRows(ctx, db, "SELECT `Component`, `Directory_` FROM `Component`", 2)
	if err != nil {
		return result, err
	}
	for _, row := range components {
		result.components = append(result.components, [2]string(row))
	}
	dirs, err := msiRows(ctx, db, "SELECT `Directory`, `Directory_Parent`, `DefaultDir` FROM `Directory`", 3)
	if err != nil {
		return result, err
	}
	for _, row := range dirs {
		result.directories = append(result.directories, [3]string(row))
	}
	media, err := msiRows(ctx, db, "SELECT `DiskId`, `LastSequence`, `Cabinet` FROM `Media`", 3)
	if err != nil {
		return result, err
	}
	for _, row := range media {
		result.media = append(result.media, [3]string(row))
	}
	return result, nil
}
