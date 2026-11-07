package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/signal"
	"path/filepath"
)

func extract(ctx context.Context, recipePath string, request extractionRequest) (failure error) {
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()
	var progressErr error
	recipe, err := readBounded(recipePath, 100000)
	if err != nil {
		return err
	}
	catalog, err := readBounded(filepath.Join(request.InputsRoot, "bazel-msvc-manifest.json"), 25000000)
	if err != nil {
		return err
	}
	plan, err := makeInputPlan(recipe, catalog, request.InputsRoot)
	if err != nil {
		return err
	}
	progress := func(phase string) func(int, int) {
		return func(done, total int) {
			if progressErr == nil {
				progressErr = json.NewEncoder(os.Stdout).Encode(map[string]any{"phase": phase, "done": done, "total": total})
				if progressErr != nil {
					cancel()
				}
			}
		}
	}
	sdk, err := loadSdkPlan(ctx, plan, progress("metadata"))
	if err != nil {
		return err
	}
	if err := preflight(ctx, plan, sdk, request.StageRoot, int64(len(catalog))); err != nil {
		return err
	}
	owned := []ownedPath{}
	defer func() {
		if failure != nil {
			failure = errors.Join(failure, cleanupOwned(owned))
		}
	}()
	archives, err := extractArchivesOwned(ctx, plan.archives, request.StageRoot, progress("compiler"), &owned)
	if err != nil {
		return err
	}
	files, err := extractCabinetsOwned(ctx, plan.cabinets, sdk, request.StageRoot, progress("sdk"), &owned)
	if err != nil {
		return err
	}
	inventory, err := canonicalManifest(append(archives, files...))
	if err != nil {
		return err
	}
	if sha(inventory) != inventoryDigest {
		return errors.New("provider inventory digest mismatch")
	}
	if err := ctx.Err(); err != nil {
		return err
	}
	marker, err := os.OpenFile(filepath.Join(request.StageRoot, ".semio-inventory.json"), os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
	if err != nil {
		return err
	}
	markerInfo, err := marker.Stat()
	if err != nil {
		marker.Close()
		return err
	}
	if err := recordOwned(&owned, marker.Name(), markerInfo); err != nil {
		marker.Close()
		return err
	}
	written, writeErr := marker.Write(inventory)
	if writeErr == nil && written != len(inventory) {
		writeErr = io.ErrShortWrite
	}
	closeErr := marker.Close()
	if writeErr != nil || closeErr != nil {
		return errors.Join(writeErr, closeErr)
	}
	return ctx.Err()
}

func main() {
	if len(os.Args) != 4 || (os.Args[1] != "extract" && os.Args[1] != "acquire") || os.Args[2] != "--recipe" || !filepath.IsAbs(os.Args[3]) {
		fmt.Fprintln(os.Stderr, "usage: msvc acquire|extract --recipe <absolute path>")
		os.Exit(2)
	}
	data, err := io.ReadAll(io.LimitReader(os.Stdin, 4097))
	if err == nil {
		var request extractionRequest
		request, err = parseRequest(data)
		if err == nil {
			ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
			defer stop()
			if os.Args[1] == "acquire" {
				err = acquire(ctx, os.Args[3], request, func(done, total int) {
					if json.NewEncoder(os.Stdout).Encode(map[string]any{"phase": "acquire", "done": done, "total": total}) != nil {
						stop()
					}
				})
			} else {
				err = extract(ctx, os.Args[3], request)
			}
		}
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
