// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📤️export carries the event-log export of the cli and the export command.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	filepath "path/filepath"

	eventspkg "github.com/usalu/semio/repo/events"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📤️Event Export

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

// 🗂️repoExportSource adapts a RepoContext to the events.ExportSource port.
type repoExportSource struct{ repo model.RepoContext }

// 📋️ExportEntities enumerates every snapshot entity in taxonomy order.
func (source repoExportSource) ExportEntities() []eventspkg.ExportEntity {
	var entities []eventspkg.ExportEntity
	for _, value := range source.repo.GetTechnologies() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "technology", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetBundles() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "bundle", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetFolders() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "folder", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetFiles() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "file", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetSections() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "section", ID: value.GetID(), Value: value})
	}
	for _, value := range source.repo.GetDefinitions() {
		entities = append(entities, eventspkg.ExportEntity{Kind: "definition", ID: value.GetID(), Value: value})
	}
	return entities
}

// 📦️ExportToEventLog writes a deterministic snapshot as append-only events.
func ExportToEventLog(outputPath string, repo model.RepoContext) (*ExportResult, error) {
	return ExportToEventLogContext(context.Background(), outputPath, repo, nil)
}

// ⏯️ExportToEventLogContext writes an atomic, cancellable event batch with progress.
func ExportToEventLogContext(ctx context.Context, outputPath string, repo model.RepoContext, progress func(eventspkg.Progress)) (*ExportResult, error) {
	if outputPath == "" {
		outputPath = filepath.Join(repo.GetRootDir(), "repo.events.jsonl")
	}
	snapshot, err := eventspkg.BuildExportSnapshot(ctx, repoExportSource{repo: repo}.ExportEntities())
	if err != nil {
		return nil, err
	}
	if _, err := (eventspkg.Store{Path: outputPath}).Append(ctx, snapshot.Inputs, progress); err != nil {
		return nil, err
	}
	return &ExportResult{
		Path:         outputPath,
		Snapshot:     snapshot.Snapshot,
		Technologies: snapshot.Counts["technology"],
		Bundles:      snapshot.Counts["bundle"],
		Folders:      snapshot.Counts["folder"],
		Files:        snapshot.Counts["file"],
		Sections:     snapshot.Counts["section"],
		Definitions:  snapshot.Counts["definition"],
	}, nil
}

// 🔷️ToolExport emits the repository snapshot event stream.
func ToolExport(outputPath string) workspace.ToolResult {
	eventspkg.Emit(eventspkg.EventExportStarting, "repo-cli", eventspkg.FilePayload{Path: outputPath})
	output := workspace.NewOutput()
	output.Info("\n📦️Exporting repo to event log...")
	result, err := ExportToEventLog(outputPath, graphqlpkg.NewRepoContext(workspace.GetRootDir()))
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	eventspkg.Emit(eventspkg.EventExportEnded, "repo-cli", eventspkg.FilePayload{Path: outputPath})
	output.Success(fmt.Sprintf("Exported to: %s", result.Path))
	output.Plain(fmt.Sprintf("  Technologies: %d", result.Technologies))
	output.Plain(fmt.Sprintf("  Bundles: %d", result.Bundles))
	output.Plain(fmt.Sprintf("  Folders: %d", result.Folders))
	output.Plain(fmt.Sprintf("  Files: %d", result.Files))
	output.Plain(fmt.Sprintf("  Sections: %d", result.Sections))
	output.Plain(fmt.Sprintf("  Definitions: %d", result.Definitions))
	return workspace.ToolResult{Output: *output, Data: result}
}

// #endregion 📤️Event Export

// #region 🌧️Cli Adapter

// 📤️exportCommand holds the data fields for a exportCommand record.
func exportCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "export [output]",
		Short: "Export repo data to an event log",
		Long:  `Export repo data as deterministic append-only events for replay.`,
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			outputPath := ""
			if len(args) > 0 {
				outputPath = args[0]
			}
			repoRoot := config.Repo
			if repoRoot == "" {
				repoRoot = workspace.FindRepoRoot(".")
			}
			ctx := graphqlpkg.NewRepoContext(repoRoot)
			result, err := ExportToEventLogContext(cmd.Context(), outputPath, ctx, nil)
			if err != nil {
				return err
			}
			jsonBytes, err := json.MarshalIndent(result, "", "  ")
			if err != nil {
				return err
			}
			cmd.Println(string(jsonBytes))
			return nil
		},
	}
}

// #endregion 🌧️Cli Adapter
