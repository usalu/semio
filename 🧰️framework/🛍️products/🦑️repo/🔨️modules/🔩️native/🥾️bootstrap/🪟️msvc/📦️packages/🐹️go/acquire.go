package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"time"
)

func publisherURL(raw string) error {
	u, err := url.Parse(raw)
	if err != nil || u.Scheme != "https" || u.Host != "download.visualstudio.microsoft.com" || u.User != nil || u.Fragment != "" {
		return errors.New("untrusted publisher URL")
	}
	return nil
}

func acquirePayload(ctx context.Context, client *http.Client, source, path string, size int64, digest string) (failure error) {
	return acquirePayloadOwned(ctx, client, source, path, size, digest, nil)
}

func acquirePayloadOwned(ctx context.Context, client *http.Client, source, path string, size int64, digest string, register func(os.FileInfo) error) (failure error) {
	if size <= 0 || size > 60000000 {
		return errors.New("invalid payload extent")
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, source, nil)
	if err != nil {
		return err
	}
	req.Header.Set("Accept-Encoding", "identity")
	response, err := client.Do(req)
	if err != nil {
		return err
	}
	defer response.Body.Close()
	if response.StatusCode != http.StatusOK || response.ContentLength >= 0 && response.ContentLength != size {
		return errors.New("unexpected publisher response")
	}
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
	if err != nil {
		return err
	}
	info, err := file.Stat()
	if err != nil {
		file.Close()
		return err
	}
	defer func() {
		if failure != nil {
			current, err := os.Lstat(path)
			if err == nil && os.SameFile(info, current) {
				failure = errors.Join(failure, os.Remove(path))
			} else {
				failure = errors.Join(failure, errors.New("acquisition output identity changed"))
			}
		}
	}()
	if register != nil {
		if err := register(info); err != nil {
			file.Close()
			return err
		}
	}
	hash := sha256.New()
	n, err := io.Copy(io.MultiWriter(file, hash), io.LimitReader(response.Body, size+1))
	closeErr := file.Close()
	if err != nil || closeErr != nil {
		return errors.Join(err, closeErr)
	}
	if n != size || hex.EncodeToString(hash.Sum(nil)) != digest {
		return errors.New("publisher payload integrity mismatch")
	}
	return ctx.Err()
}

func acquire(ctx context.Context, recipePath string, request extractionRequest, progress func(int, int)) (failure error) {
	recipe, err := readBounded(recipePath, 100000)
	if err != nil {
		return err
	}
	if sha(recipe) != recipeDigest {
		return errors.New("untrusted recipe")
	}
	var document recipeDocument
	if err := json.Unmarshal(recipe, &document); err != nil {
		return err
	}
	if err := publisherURL(document.CatalogURL); err != nil {
		return err
	}
	root, err := os.Lstat(request.InputsRoot)
	if err != nil || !root.IsDir() || root.Mode()&os.ModeSymlink != 0 {
		return errors.New("physical inputs root required")
	}
	entries, err := os.ReadDir(request.InputsRoot)
	if err != nil || len(entries) != 0 {
		return errors.New("empty inputs root required")
	}
	client := &http.Client{Timeout: 10 * time.Minute, CheckRedirect: func(req *http.Request, via []*http.Request) error {
		if len(via) >= 5 {
			return errors.New("too many redirects")
		}
		return publisherURL(req.URL.String())
	}}
	owned := []ownedPath{}
	defer func() {
		if failure != nil {
			failure = errors.Join(failure, cleanupOwned(owned))
		}
	}()
	get := func(source, path string, size int64, digest string) error {
		if err := publisherURL(source); err != nil {
			return err
		}
		return acquirePayloadOwned(ctx, client, source, path, size, digest, func(info os.FileInfo) error { return recordOwned(&owned, path, info) })
	}
	catalogPath := filepath.Join(request.InputsRoot, "bazel-msvc-manifest.json")
	if err := get(document.CatalogURL, catalogPath, 18343591, catalogDigest); err != nil {
		return err
	}
	catalog, err := readBounded(catalogPath, 25000000)
	if err != nil {
		return err
	}
	plan, err := makeInputPlan(recipe, catalog, request.InputsRoot)
	if err != nil {
		return err
	}
	inputs := append(append([]archiveInput{}, plan.archives...), plan.msis...)
	for _, cab := range plan.cabinets {
		inputs = append(inputs, cab.archiveInput)
	}
	if len(inputs) != len(plan.urls) {
		return errors.New("payload plan mismatch")
	}
	for i, input := range inputs {
		if err := get(plan.urls[i], input.Path, input.Bytes, input.SHA256); err != nil {
			return fmt.Errorf("payload %d: %w", i, err)
		}
		if progress != nil {
			progress(i+1, len(inputs))
		}
		if err := ctx.Err(); err != nil {
			return err
		}
	}
	return nil
}
