package main

import (
	"archive/zip"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"io"
	"os"
	"path/filepath"
	"strings"
	"syscall"
	"unsafe"
)

func preflight(ctx context.Context, plan inputPlan, sdk []sdkFile, stage string, catalogBytes int64) error {
	root := filepath.VolumeName(stage) + `\`
	name, err := syscall.UTF16PtrFromString(root)
	if err != nil {
		return err
	}
	kernel := syscall.NewLazyDLL("kernel32.dll")
	var sectors, sectorBytes, freeClusters, totalClusters uint32
	var available uint64
	ok, _, callErr := kernel.NewProc("GetDiskFreeSpaceW").Call(uintptr(unsafe.Pointer(name)), uintptr(unsafe.Pointer(&sectors)), uintptr(unsafe.Pointer(&sectorBytes)), uintptr(unsafe.Pointer(&freeClusters)), uintptr(unsafe.Pointer(&totalClusters)))
	if ok == 0 {
		return callErr
	}
	ok, _, callErr = kernel.NewProc("GetDiskFreeSpaceExW").Call(uintptr(unsafe.Pointer(name)), uintptr(unsafe.Pointer(&available)), 0, 0)
	if ok == 0 {
		return callErr
	}
	cluster := uint64(sectors) * uint64(sectorBytes)
	if cluster == 0 || cluster > 1048576 {
		return errors.New("invalid volume allocation unit")
	}
	var total uint64
	add := func(size int64) error {
		if size < 0 || size > 2000000000 {
			return errors.New("invalid allocation extent")
		}
		rounded := (uint64(size) + cluster - 1) / cluster * cluster
		total += rounded
		if total > 2147483648 {
			return errors.New("native provider exceeds capacity bound")
		}
		return nil
	}
	if err := add(catalogBytes); err != nil {
		return err
	}
	if err := add(783670); err != nil {
		return err
	}
	if err := add(20971520); err != nil {
		return err
	}
	for _, row := range sdk {
		if err := add(row.Bytes); err != nil {
			return err
		}
	}
	all := append(append([]archiveInput{}, plan.archives...), plan.msis...)
	for _, cab := range plan.cabinets {
		all = append(all, cab.archiveInput)
	}
	for _, input := range all {
		if err := add(input.Bytes); err != nil {
			return err
		}
	}
	count := 0
	names := map[string]bool{}
	for _, input := range plan.archives {
		if err := ctx.Err(); err != nil {
			return err
		}
		file, err := openLocked(input.Path)
		if err != nil {
			return err
		}
		stat, err := file.Stat()
		if err != nil || stat.Size() != input.Bytes {
			file.Close()
			return errors.New("archive extent mismatch")
		}
		digest := sha256.New()
		_, err = io.Copy(digest, contextReader{ctx, file})
		if err != nil || hex.EncodeToString(digest.Sum(nil)) != input.SHA256 {
			file.Close()
			return errors.New("archive digest mismatch")
		}
		archive, err := zip.NewReader(file, stat.Size())
		if err != nil {
			file.Close()
			return err
		}
		for _, member := range archive.File {
			if !strings.HasPrefix(member.Name, "Contents/") || member.FileInfo().IsDir() {
				continue
			}
			path, err := archivePath(strings.TrimPrefix(member.Name, "Contents/"))
			if err != nil || names[strings.ToLower(path)] || member.Mode()&os.ModeType != 0 || member.UncompressedSize64 > 100000000 {
				file.Close()
				return errors.New("unsafe archive member")
			}
			names[strings.ToLower(path)] = true
			count++
			if err := add(int64(member.UncompressedSize64)); err != nil {
				file.Close()
				return err
			}
		}
		file.Close()
	}
	if count != 494 || len(sdk) != 4453 {
		return errors.New("provider inventory count mismatch")
	}
	if total > available {
		return errors.New("insufficient volume headroom")
	}
	return json.NewEncoder(os.Stdout).Encode(map[string]any{"phase": "preflight", "bytes": total, "allocationUnit": cluster})
}
