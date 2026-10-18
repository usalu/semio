// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧱️tool_codebase carries the codebase, technology, bundle, folder, file, section and definition tool functions of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	filepath "path/filepath"
	sort "sort"
	strings "strings"

	codebasepkg "github.com/usalu/semio/repo/codebase"
	languagespkg "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	treepkg "github.com/usalu/semio/repo/tree"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🏩️Codebase

// 🟫️ToolCodebase MUST complete the operation and return consistent results.
func ToolCodebase() workspace.ToolResult {
	output := workspace.NewOutput()
	ctx := codebasepkg.NewCodebaseContext()

	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := ctx.LoadBreachs(); err != nil {
		return workspace.ToolErrorResult(err)
	}
	if err := ctx.LoadTickets(); err != nil {
		return workspace.ToolErrorResult(err)
	}
	ctx.LoadPolicies()

	codebase := codebasepkg.BuildCodebase(ctx)

	output.Success(fmt.Sprintf("Codebase loaded: %d bundles, %d files, %d breachs",
		len(codebase.Bundles), len(codebase.Files), len(codebase.Breachs)))

	return workspace.ToolResult{Output: *output, Data: codebase}
}

// #endregion 🏩️Codebase

// #region 📋️Tickets

// 🟢️runTechnologyList holds the data fields for a runTechnologyList record.
func runTechnologyList(factory EngineFactory, config Config, cmd *Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "technology list"}

		projChan := make(chan model.Technology)
		go func() {
			ticketspkg.StreamTechnologies(context.Background(), projChan, opts)
		}()

		for p := range projChan {
			data, err := json.Marshal(map[string]interface{}{"technology": p})
			if err != nil {
				continue
			}
			stream <- Event{Kind: KindResult, Command: "technology list", Data: data}
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

// 🌳️runTechnologyTree holds the data fields for a runTechnologyTree record.
func runTechnologyTree(factory EngineFactory, config Config, cmd *Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "technology tree"}

		projChan := make(chan model.Technology)
		go func() {
			ticketspkg.StreamTechnologies(context.Background(), projChan, opts)
		}()

		var technologies []model.Technology
		for p := range projChan {
			technologies = append(technologies, p)
		}
		var events []Event
		for _, p := range technologies {
			data, err := json.Marshal(map[string]interface{}{"technology": p})
			if err != nil {
				continue
			}
			events = append(events, Event{Kind: KindResult, Command: "technology tree", Data: data})
		}
		sort.Slice(events, func(i, j int) bool {
			return formatMarkdownResult(events[i].Command, events[i].Data) < formatMarkdownResult(events[j].Command, events[j].Data)
		})
		for _, ev := range events {
			stream <- ev
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

// 🔖️ToolTechnologyList MUST complete the operation successfully.
func ToolTechnologyList() workspace.ToolResult {
	technologies := codebasepkg.LoadTechnologies()
	sort.Slice(technologies, func(i, j int) bool { return technologies[i].Name < technologies[j].Name })
	var events []Event
	for _, p := range technologies {
		data, _ := json.Marshal(map[string]interface{}{"technology": p})
		events = append(events, Event{Kind: KindResult, Command: "technology list", Data: data})
	}
	return toolResultFromEvents(events, technologies)
}

// 🔖️ToolBundleList MUST complete the operation successfully.
func ToolBundleList() workspace.ToolResult {
	bundles := codebasepkg.LoadBundles()
	sort.Slice(bundles, func(i, j int) bool { return bundles[i].Name < bundles[j].Name })
	var events []Event
	for _, b := range bundles {
		data, _ := json.Marshal(map[string]interface{}{"bundle": b})
		events = append(events, Event{Kind: KindResult, Command: "bundle list", Data: data})
	}
	return toolResultFromEvents(events, bundles)
}

// 🔖️ToolTechnologyTree MUST complete the operation successfully.
func ToolTechnologyTree() workspace.ToolResult {
	return toolResultFromTreeRender(treepkg.TreeNodeTechnology)
}

// 🔖️ToolFolderList MUST complete the operation successfully.
func ToolFolderList(path string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.TrimSuffix(path, "/"))
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	folders, err := workspace.ListDirEntries(absPath, true)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	var relPaths []string
	for _, f := range folders {
		relPaths = append(relPaths, workspace.NormalizePath(filepath.Join(path, f)))
	}
	ignored := workspace.GetGitIgnoredSet(relPaths)
	var filtered []string
	for _, f := range folders {
		relPath := workspace.NormalizePath(filepath.Join(path, f))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, f)
		}
	}
	output.Info(fmt.Sprintf("\n📁️Found %d folders in %s:\n", len(filtered), path))
	for _, f := range filtered {
		output.Plain(fmt.Sprintf("   %s/", f))
	}
	return workspace.ToolResult{Output: *output, Data: filtered}
}

// 🔖️ToolFolderTree MUST complete the operation successfully.
func ToolFolderTree(path string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.TrimSuffix(path, "/"))
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
	}
	output.Info(fmt.Sprintf("\n📁️Folder tree: %s\n", path))
	ticketspkg.PrintTree(output, absPath, "")
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolFileList MUST complete the operation successfully.
func ToolFileList(scopeRaw string) workspace.ToolResult {
	output := workspace.NewOutput()
	scope := workspace.ParseScope(scopeRaw)
	bundles := codebasepkg.GetTechnologies()
	files, err := model.ScopeToFiles(scope, bundles)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	output.Info(fmt.Sprintf("\n📄️Found %d files in scope \"%s\":\n", len(files), scopeRaw))
	for i, f := range files {
		if i >= 50 {
			output.Plain(fmt.Sprintf("   ... and %d more", len(files)-50))
			break
		}
		output.Plain(fmt.Sprintf("   %s", f))
	}
	return workspace.ToolResult{Output: *output, Data: files}
}

// 🔖️ToolFileTree MUST complete the operation successfully.
func ToolFileTree(path string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.TrimSuffix(path, "/"))
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("Path not found: %s", path))
	}
	output.Info(fmt.Sprintf("\n📄️File tree: %s\n", path))
	ticketspkg.PrintTree(output, absPath, "")
	return workspace.ToolResult{Output: *output}
}

// 🔖️ToolSectionTree MUST complete the operation successfully.
func ToolSectionTree(filePath string) workspace.ToolResult {
	output := workspace.NewOutput()
	absPath := filepath.Join(workspace.RootDir, strings.Split(filePath, "#")[0])
	if !workspace.FileExists(absPath) {
		return workspace.ToolErrorMsg(fmt.Sprintf("File not found: %s", filePath))
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return workspace.ToolErrorResult(err)
	}
	sections := languagespkg.ParseSections(content, filePath)
	output.Info(fmt.Sprintf("\n🏷️Sections in %s:\n", filePath))
	var printSection func(s model.Section, prefix string)
	printSection = func(s model.Section, prefix string) {
		output.Plain(fmt.Sprintf("%s└️─️─️ %s (lines %d-%d)", prefix, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, prefix+"    ")
		}
	}
	for _, s := range sections {
		printSection(s, "")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return workspace.ToolResult{Output: *output, Data: sections}
}

// 🔖️ToolDefinitionTree MUST complete the operation successfully.
func ToolDefinitionTree(filePath string) workspace.ToolResult {
	return move.ToolDefinitionList(filePath)
}

// 🔖️ToolUpdateMetabolism MUST complete the operation successfully.
func ToolUpdateMetabolism() workspace.ToolResult {
	output := workspace.NewOutput()
	output.Info("\n🔄️ Running update-metabolism via npx tsx...")
	stdout, stderr, exitCode := workspace.ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
	if exitCode != 0 {
		return workspace.ToolErrorMsg(fmt.Sprintf("%s%s", stdout, stderr))
	}
	output.Success(stdout)
	return workspace.ToolResult{Output: *output}
}

// #endregion 📋️Tickets
