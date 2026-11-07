package main

import (
	"archive/zip"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"unicode/utf16"
)

type archiveInput struct {
	Path, SHA256 string
	Bytes        int64
}
type outputFile struct {
	Path   string `json:"path"`
	Bytes  int64  `json:"bytes"`
	SHA256 string `json:"sha256"`
}
type plannedArchiveFile struct {
	file *zip.File
	path string
}
type ownedPath struct {
	path string
	info os.FileInfo
}

func recordOwned(owned *[]ownedPath, path string, info os.FileInfo) error {
	*owned = append(*owned, ownedPath{path, info})
	current, err := os.Lstat(path)
	if err != nil || !os.SameFile(info, current) {
		return fmt.Errorf("owned output changed: %s", path)
	}
	return nil
}

func cleanupOwned(owned []ownedPath) error {
	var failure error
	for i := len(owned) - 1; i >= 0; i-- {
		current, err := os.Lstat(owned[i].path)
		if errors.Is(err, os.ErrNotExist) {
			continue
		}
		if err != nil || !os.SameFile(current, owned[i].info) {
			failure = errors.Join(failure, fmt.Errorf("owned output changed: %s", owned[i].path))
			continue
		}
		failure = errors.Join(failure, os.Remove(owned[i].path))
	}
	return failure
}

type contextReader struct {
	context context.Context
	reader  io.Reader
}

func (r contextReader) Read(p []byte) (int, error) {
	if err := r.context.Err(); err != nil {
		return 0, err
	}
	return r.reader.Read(p)
}

func ensureDirectories(root, relative string, owned *[]ownedPath) error {
	current := root
	for _, part := range strings.Split(relative, "/") {
		if part == "" {
			continue
		}
		current = filepath.Join(current, part)
		made := false
		if err := os.Mkdir(current, 0700); err != nil {
			if !errors.Is(err, os.ErrExist) {
				return err
			}
		} else {
			made = true
		}
		info, err := os.Lstat(current)
		if err != nil || !info.IsDir() || info.Mode()&os.ModeSymlink != 0 {
			return errors.New("nonphysical stage directory")
		}
		if made {
			*owned = append(*owned, ownedPath{current, info})
		}
	}
	return nil
}

func extractArchives(ctx context.Context, inputs []archiveInput, stage string, progress func(int, int)) (result []outputFile, failure error) {
	owned := []ownedPath{}
	defer func() {
		if failure != nil {
			failure = errors.Join(failure, cleanupOwned(owned))
		}
	}()
	return extractArchivesOwned(ctx, inputs, stage, progress, &owned)
}

func extractArchivesOwned(ctx context.Context, inputs []archiveInput, stage string, progress func(int, int), records *[]ownedPath) (result []outputFile, failure error) {
	info, err := os.Lstat(stage)
	if err != nil || !info.IsDir() || info.Mode()&os.ModeSymlink != 0 {
		return nil, errors.New("physical stage required")
	}
	entries, err := os.ReadDir(stage)
	if err != nil {
		return nil, err
	}
	if len(entries) != 0 {
		return nil, errors.New("empty stage required")
	}
	var held []*os.File
	defer func() {
		for _, file := range held {
			file.Close()
		}
	}()
	planned := []plannedArchiveFile{}
	names := map[string]bool{}
	var total uint64
	for _, input := range inputs {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		file, err := openLocked(input.Path)
		if err != nil {
			return nil, err
		}
		held = append(held, file)
		stat, err := file.Stat()
		if err != nil || stat.Size() != input.Bytes || input.Bytes <= 0 || input.Bytes > 60000000 {
			return nil, errors.New("archive extent mismatch")
		}
		digest := sha256.New()
		if _, err := io.Copy(digest, contextReader{ctx, file}); err != nil {
			return nil, err
		}
		if hex.EncodeToString(digest.Sum(nil)) != strings.ToLower(input.SHA256) {
			return nil, errors.New("archive digest mismatch")
		}
		archive, err := zip.NewReader(file, stat.Size())
		if err != nil {
			return nil, err
		}
		for _, member := range archive.File {
			if !strings.HasPrefix(member.Name, "Contents/") || member.FileInfo().IsDir() {
				continue
			}
			name, err := archivePath(strings.TrimPrefix(member.Name, "Contents/"))
			if err != nil {
				return nil, err
			}
			key := strings.ToLower(name)
			if names[key] || member.Mode()&os.ModeType != 0 || member.UncompressedSize64 > 100000000 {
				return nil, errors.New("unsafe archive member")
			}
			names[key] = true
			target := filepath.Join(stage, filepath.FromSlash(name))
			if len(utf16.Encode([]rune(target))) > 256 {
				return nil, errors.New("archive path budget exceeded")
			}
			total += member.UncompressedSize64
			if total > 2000000000 || len(planned) >= 5000 {
				return nil, errors.New("archive inventory budget exceeded")
			}
			planned = append(planned, plannedArchiveFile{member, name})
		}
	}
	output := make([]outputFile, 0, len(planned))
	for _, item := range planned {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		parent := filepath.ToSlash(filepath.Dir(filepath.FromSlash(item.path)))
		if parent == "." {
			parent = ""
		}
		if err := ensureDirectories(stage, parent, records); err != nil {
			return nil, err
		}
		source, err := item.file.Open()
		if err != nil {
			return nil, err
		}
		target := filepath.Join(stage, filepath.FromSlash(item.path))
		destination, err := os.OpenFile(target, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
		if err != nil {
			source.Close()
			return nil, err
		}
		created, err := destination.Stat()
		if err != nil {
			destination.Close()
			source.Close()
			return nil, err
		}
		if err := recordOwned(records, target, created); err != nil {
			destination.Close()
			source.Close()
			return nil, err
		}
		digest := sha256.New()
		count, copyErr := io.Copy(io.MultiWriter(destination, digest), io.LimitReader(contextReader{ctx, source}, int64(item.file.UncompressedSize64)+1))
		sourceErr := source.Close()
		destinationErr := destination.Close()
		if copyErr != nil {
			return nil, copyErr
		}
		if sourceErr != nil {
			return nil, sourceErr
		}
		if destinationErr != nil {
			return nil, destinationErr
		}
		if count != int64(item.file.UncompressedSize64) {
			return nil, fmt.Errorf("archive member extent mismatch: %s", item.path)
		}
		output = append(output, outputFile{item.path, count, hex.EncodeToString(digest.Sum(nil))})
		if progress != nil {
			progress(len(output), len(planned))
		}
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	sort.Slice(output, func(i, j int) bool { return output[i].Path < output[j].Path })
	return output, nil
}
