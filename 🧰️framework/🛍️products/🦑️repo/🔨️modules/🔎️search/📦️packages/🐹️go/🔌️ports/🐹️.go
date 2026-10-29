package search

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"sync"
	"time"
)

type GenerationLease interface{ Close() error }
type generationLease struct {
	file *os.File
	once sync.Once
	err  error
}

// 🔒️ Acquires a persistent-inode kernel lease whose ownership ends when the process exits.
func AcquireGenerationLease(ctx context.Context, path string, shared bool, progress func(Progress)) (GenerationLease, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if err := ordinarySearchDirectory(filepath.Dir(path)); err != nil {
		return nil, err
	}
	info, err := os.Lstat(path)
	var file *os.File
	if os.IsNotExist(err) {
		file, err = os.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_RDWR, 0o600)
		if os.IsExist(err) {
			info, err = os.Lstat(path)
		}
	}
	if file == nil && err == nil {
		if !info.Mode().IsRegular() || info.Mode()&os.ModeSymlink != 0 {
			return nil, errors.New("ordinary search lease file required")
		}
		file, err = os.OpenFile(path, os.O_RDWR, 0)
	}
	if err != nil {
		return nil, err
	}
	if file == nil {
		return nil, errors.New("search lease file required")
	}
	defer func() {
		if file != nil {
			_ = file.Close()
		}
	}()
	fail := func(err error) (GenerationLease, error) {
		closing := file.Close()
		file = nil
		return nil, errors.Join(err, closing)
	}
	opened, err := file.Stat()
	if err != nil {
		return fail(err)
	}
	current, err := os.Lstat(path)
	if err != nil {
		return fail(err)
	}
	if !opened.Mode().IsRegular() || !current.Mode().IsRegular() || current.Mode()&os.ModeSymlink != 0 || !os.SameFile(opened, current) {
		return fail(errors.New("search lease identity changed"))
	}
	for attempt := 1; ; attempt++ {
		if err = ctx.Err(); err != nil {
			return fail(err)
		}
		acquired, err := tryGenerationLease(file, shared)
		if err != nil {
			return fail(err)
		}
		if acquired {
			named, namedErr := os.Lstat(path)
			if namedErr != nil || !named.Mode().IsRegular() || named.Mode()&os.ModeSymlink != 0 || !os.SameFile(opened, named) {
				return fail(errors.Join(namedErr, errors.New("search lease identity changed while waiting"), releaseGenerationLease(file)))
			}
			if err = ctx.Err(); err != nil {
				_ = releaseGenerationLease(file)
				return fail(err)
			}
			owner := &generationLease{file: file}
			file = nil
			return owner, nil
		}
		report(progress, Progress{Current: attempt, Step: "waiting-lock"})
		timer := time.NewTimer(200 * time.Millisecond)
		select {
		case <-ctx.Done():
			timer.Stop()
			return fail(ctx.Err())
		case <-timer.C:
		}
	}
}
func (lease *generationLease) Close() error {
	lease.once.Do(func() { lease.err = errors.Join(releaseGenerationLease(lease.file), lease.file.Close()) })
	return lease.err
}
