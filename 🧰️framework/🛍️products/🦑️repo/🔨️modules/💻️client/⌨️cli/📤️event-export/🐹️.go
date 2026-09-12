// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Repository snapshot event export.

// #endregion 🧲️Header

package client

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"path/filepath"
	"sort"

	eventstore "github.com/usalu/semio/repo/client/internal/eventstore"
	repopkg "github.com/usalu/semio/repo/go"
)

// #region 📜️Schema

type ExportResult struct {
	Path         string `json:"path"`
	Snapshot     string `json:"snapshot"`
	Technologies int    `json:"technologies"`
	Bundles      int    `json:"bundles"`
	Folders      int    `json:"folders"`
	Files        int    `json:"files"`
	Sections     int    `json:"sections"`
	Definitions  int    `json:"definitions"`
}

// #endregion 📜️Schema

// #region 📤️Export

// 📦️ExportToEventLog writes a deterministic snapshot as append-only events.
func ExportToEventLog(outputPath string, repo RepoContext) (*ExportResult, error) {
	return ExportToEventLogContext(context.Background(), outputPath, repo, nil)
}

// ⏯️ExportToEventLogContext writes an atomic, cancellable event batch with progress.
func ExportToEventLogContext(ctx context.Context, outputPath string, repo RepoContext, progress func(eventstore.Progress)) (*ExportResult, error) {
	if outputPath == "" {
		outputPath = filepath.Join(repo.GetRootDir(), "repo.events.jsonl")
	}
	result := &ExportResult{Path: outputPath}
	inputs := make([]eventstore.Input, 0)
	appendEntity := func(kind, id string, value interface{}) {
		inputs = append(inputs, eventstore.Input{ID: kind + ":" + id, Kind: kind + ".recorded", Data: value})
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	technologies := repo.GetTechnologies()
	result.Technologies = len(technologies)
	for _, value := range technologies {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		appendEntity("technology", value.GetID(), value)
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	bundles := repo.GetBundles()
	result.Bundles = len(bundles)
	for _, value := range bundles {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		appendEntity("bundle", value.GetID(), value)
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	folders := repo.GetFolders()
	result.Folders = len(folders)
	for _, value := range folders {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		appendEntity("folder", value.GetID(), value)
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	files := repo.GetFiles()
	result.Files = len(files)
	for _, value := range files {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		appendEntity("file", value.GetID(), value)
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	sections := repo.GetSections()
	result.Sections = len(sections)
	for _, value := range sections {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		appendEntity("section", value.GetID(), value)
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	definitions := repo.GetDefinitions()
	result.Definitions = len(definitions)
	for _, value := range definitions {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		appendEntity("definition", value.GetID(), value)
	}
	sort.SliceStable(inputs, func(left, right int) bool { return inputs[left].ID < inputs[right].ID })
	snapshotHash := sha256.New()
	for _, input := range inputs {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		data, err := json.Marshal(input.Data)
		if err != nil {
			return nil, err
		}
		fmt.Fprintf(snapshotHash, "%s\x00%s\x00", input.ID, input.Kind)
		snapshotHash.Write(data)
	}
	result.Snapshot = hex.EncodeToString(snapshotHash.Sum(nil))
	for index := range inputs {
		inputs[index].ID = "snapshot:" + result.Snapshot + ":" + inputs[index].ID
	}
	if _, err := (eventstore.Store{Path: outputPath}).Append(ctx, inputs, progress); err != nil {
		return nil, err
	}
	return result, nil
}

// #endregion 📤️Export

// #region 🧰️Tool

// 🔷️ToolExport emits the repository snapshot event stream.
func ToolExport(outputPath string) ToolResult {
	repopkg.Emit(repopkg.EventExportStarting, "repo-cli", repopkg.FilePayload{Path: outputPath})
	output := NewOutput()
	output.Info("\n📦️Exporting repo to event log...")
	result, err := ExportToEventLog(outputPath, NewRepoContext(rootDir))
	if err != nil {
		return toolErrorResult(err)
	}
	repopkg.Emit(repopkg.EventExportEnded, "repo-cli", repopkg.FilePayload{Path: outputPath})
	output.Success(fmt.Sprintf("Exported to: %s", result.Path))
	output.Plain(fmt.Sprintf("  Technologies: %d", result.Technologies))
	output.Plain(fmt.Sprintf("  Bundles: %d", result.Bundles))
	output.Plain(fmt.Sprintf("  Folders: %d", result.Folders))
	output.Plain(fmt.Sprintf("  Files: %d", result.Files))
	output.Plain(fmt.Sprintf("  Sections: %d", result.Sections))
	output.Plain(fmt.Sprintf("  Definitions: %d", result.Definitions))
	return ToolResult{Output: *output, Data: result}
}

// #endregion 🧰️Tool
