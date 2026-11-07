package main

import (
	"context"
	"errors"
	"strings"
)

func loadSdkPlan(ctx context.Context, inputs inputPlan, progress func(int, int)) ([]sdkFile, error) {
	if len(inputs.msis) != 8 || len(inputs.tableHashes) != 8 || len(inputs.cabinets) != 34 {
		return nil, errors.New("incomplete SDK inputs")
	}
	files := []sdkFile{}
	ids, paths := map[string]bool{}, map[string]bool{}
	for i, input := range inputs.msis {
		tables, err := readMsi(ctx, input)
		if err != nil {
			return nil, err
		}
		encoded, err := canonicalTables(tables)
		if err != nil {
			return nil, err
		}
		if sha(encoded) != inputs.tableHashes[i] {
			return nil, errors.New("MSI table digest mismatch")
		}
		rows, err := sdkFiles(tables.directories, tables.components, tables.files, tables.media)
		if err != nil {
			return nil, err
		}
		for _, row := range rows {
			id, path := strings.ToLower(row.ID), strings.ToLower(row.Path)
			if ids[id] || paths[path] {
				return nil, errors.New("overlapping MSI output")
			}
			ids[id], paths[path] = true, true
			files = append(files, row)
		}
		if progress != nil {
			progress(i+1, len(inputs.msis))
		}
		if err := ctx.Err(); err != nil {
			return nil, err
		}
	}
	if len(files) != 4453 {
		return nil, errors.New("SDK file count mismatch")
	}
	return files, nil
}
