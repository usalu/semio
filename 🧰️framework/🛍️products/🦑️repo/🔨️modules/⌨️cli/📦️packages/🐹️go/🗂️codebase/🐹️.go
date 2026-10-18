// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️codebase carries the bundle, folder, file, section, definition and mermaid commands of the cli.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	testrunner "github.com/usalu/semio/repo/testrunner"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	treepkg "github.com/usalu/semio/repo/tree"
)

// #region 🕸️Test Command

// 📦️bundleCommand holds the data fields for a bundleCommand record.
func bundleCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "bundle", Short: "Bundle management commands"}
	listCmd := &Command{
		Use:   "list",
		Short: "List bundles",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			statusFilter := getStatusFilter(cmd)
			var bundlesWithOpenTickets map[string]bool
			if statusFilter != nil {
				bundlesWithOpenTickets = testrunner.GetBundlesWithOpenTickets()
			}

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle list"}

				bundleChan := make(chan model.Bundle)
				go func() {
					ticketspkg.StreamBundles(context.Background(), bundleChan, opts)
				}()

				for b := range bundleChan {

					if statusFilter != nil {
						hasOpenTicket := bundlesWithOpenTickets[b.Name]
						if *statusFilter == "open" && !hasOpenTicket {
							continue
						}
						if *statusFilter == "closed" && hasOpenTicket {
							continue
						}
					}

					data, err := json.Marshal(map[string]interface{}{"bundle": b})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "bundle list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	bindStatusFlags(listCmd)

	treeCmd := &Command{
		Use:   "tree",
		Short: "Show bundle tree",
		RunE: func(cmd *Command, args []string) error {
			opts := getStreamOptions(cmd)
			statusFilter := getStatusFilter(cmd)
			var bundlesWithOpenTickets map[string]bool
			if statusFilter != nil {
				bundlesWithOpenTickets = testrunner.GetBundlesWithOpenTickets()
			}
			_ = bundlesWithOpenTickets

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle tree"}

				bundleChan := make(chan model.Bundle)
				go func() {
					ticketspkg.StreamBundles(context.Background(), bundleChan, opts)
				}()

				var bundles []model.Bundle
				for b := range bundleChan {
					bundles = append(bundles, b)
				}

				sort.Slice(bundles, func(i, j int) bool {
					return bundles[i].Name < bundles[j].Name
				})

				if config.IsMarkdown() {
					var sb strings.Builder
					for _, b := range bundles {
						bMap := map[string]interface{}{}
						bBytes, _ := json.Marshal(b)
						json.Unmarshal(bBytes, &bMap)
						line := model.RenderEntityMarkdown("bundle", bMap)
						sb.WriteString(line + "\n")
					}
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "bundle tree", Data: data}
				} else {
					stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: "."}
					for i, b := range bundles {
						connector := "├️─️─️ "
						if i == len(bundles)-1 {
							connector = "└️─️─️ "
						}
						stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: connector + b.Name}
					}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)
	bindStatusFlags(treeCmd)

	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

// 💠️folderCommand holds the data fields for a folderCommand record.
func folderCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "folder", Short: "Folder management commands"}
	createCmd := &Command{
		Use:   "create",
		Short: "Create a folder",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderCreate($path: String!) { folderCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "Folder path")
	moveCmd := &Command{
		Use:   "move",
		Short: "Move a folder",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FolderMove($src: String!, $dst: String!) { folderMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &Command{
		Use:   "delete",
		Short: "Delete a folder",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderDelete($path: String!) { folderDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "Folder path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

// ◾fileCommand holds the data fields for a fileCommand record.
func fileCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "file", Short: "File management commands"}
	createCmd := &Command{
		Use:   "create",
		Short: "Create a file",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileCreate($path: String!) { fileCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "File path")
	moveCmd := &Command{
		Use:   "move",
		Short: "Move a file",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FileMove($src: String!, $dst: String!) { fileMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &Command{
		Use:   "delete",
		Short: "Delete a file",
		Args:  MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileDelete($path: String!) { fileDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "File path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

// ◽sectionCommand holds the data fields for a sectionCommand record.
func sectionCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "section", Short: "Section management commands"}
	createCmd := &Command{
		Use:   "create",
		Short: "Create a section",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			parent, _ := cmd.Flags().GetString("parent")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if parent == "" && len(args) > 2 {
				parent = args[2]
			}
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			if parent != "" {
				variables["parent"] = parent
			}
			query := `mutation SectionCreate($file: String!, $name: String!, $parent: String) { sectionCreate(file: $file, name: $name, parent: $parent) { id name range { start end } } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("file", "", "File path")
	createCmd.Flags().String("name", "", "Section name")
	createCmd.Flags().String("parent", "", "Parent section")
	moveCmd := &Command{
		Use:   "move",
		Short: "Move a section",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			oldName, _ := cmd.Flags().GetString("old")
			newName, _ := cmd.Flags().GetString("new")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if oldName == "" && len(args) > 1 {
				oldName = args[1]
			}
			if newName == "" && len(args) > 2 {
				newName = args[2]
			}
			if file == "" || oldName == "" || newName == "" {
				return fmt.Errorf("missing file or names")
			}
			variables := map[string]interface{}{"file": file, "oldName": oldName, "newName": newName}
			query := `mutation SectionMove($file: String!, $oldName: String!, $newName: String!) { sectionMove(file: $file, oldName: $oldName, newName: $newName) { id name range { start end } } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("file", "", "File path")
	moveCmd.Flags().String("old", "", "Old section name")
	moveCmd.Flags().String("new", "", "New section name")
	deleteCmd := &Command{
		Use:   "delete",
		Short: "Delete a section",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			query := `mutation SectionDelete($file: String!, $name: String!) { sectionDelete(file: $file, name: $name) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("file", "", "File path")
	deleteCmd.Flags().String("name", "", "Section name")
	integrateCmd := &Command{
		Use:   "integrate",
		Short: "Integrate source code into a target file section",
		Args:  MaximumNArgs(4),
		RunE: func(cmd *Command, args []string) error {
			source, _ := cmd.Flags().GetString("source")
			targetSection, _ := cmd.Flags().GetString("target-section")
			targetFile, _ := cmd.Flags().GetString("target-file")
			targetParent, _ := cmd.Flags().GetString("target-parent")
			if source == "" && len(args) > 0 {
				source = args[0]
			}
			if targetSection == "" && len(args) > 1 {
				targetSection = args[1]
			}
			if targetFile == "" && len(args) > 2 {
				targetFile = args[2]
			}
			if targetParent == "" && len(args) > 3 {
				targetParent = args[3]
			}
			if source == "" || targetSection == "" || targetFile == "" {
				return fmt.Errorf("missing source, target section, or target file")
			}
			variables := map[string]interface{}{
				"source":        source,
				"targetSection": targetSection,
				"targetFile":    targetFile,
			}
			if targetParent != "" {
				variables["targetParent"] = targetParent
			}
			query := `mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) { integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) { id } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	integrateCmd.Flags().String("source", "", "Source file path")
	integrateCmd.Flags().String("target-section", "", "Target section name")
	integrateCmd.Flags().String("target-file", "", "Target file path")
	integrateCmd.Flags().String("target-parent", "", "Target parent section name")

	extractCmd := &Command{
		Use:   "extract",
		Short: "Extract a section from a source file into a target file",
		Args:  MaximumNArgs(3),
		RunE: func(cmd *Command, args []string) error {
			sourceFile, _ := cmd.Flags().GetString("source-file")
			sourceSection, _ := cmd.Flags().GetString("source-section")
			targetFile, _ := cmd.Flags().GetString("target-file")

			if sourceFile == "" && len(args) > 0 {
				sourceFile = args[0]
			}
			if sourceSection == "" && len(args) > 1 {
				sourceSection = args[1]
			}
			if targetFile == "" && len(args) > 2 {
				targetFile = args[2]
			}

			if sourceFile == "" || sourceSection == "" || targetFile == "" {
				return fmt.Errorf("missing source file, source section, or target file")
			}

			variables := map[string]interface{}{
				"sourceFile":    sourceFile,
				"sourceSection": sourceSection,
				"targetFile":    targetFile,
			}
			query := `mutation Extract($sourceFile: String!, $sourceSection: String!, $targetFile: String!) { extract(sourceFile: $sourceFile, sourceSection: $sourceSection, targetFile: $targetFile) { id } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	extractCmd.Flags().String("source-file", "", "Source file path")
	extractCmd.Flags().String("source-section", "", "Source section name")
	extractCmd.Flags().String("target-file", "", "Target file path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(integrateCmd)
	root.AddCommand(extractCmd)
	return root
}

// ◻definitionCommand holds the data fields for a definitionCommand record.
func definitionCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{Use: "definition", Short: "Definition management commands"}
	listCmd := &Command{
		Use:     "list",
		Aliases: []string{"tree"},
		Short:   "List definitions",
		Args:    MaximumNArgs(1),
		RunE: func(cmd *Command, args []string) error {
			path, _ := cmd.Flags().GetString("file")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing file")
			}

			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "definition list"}

				defChan := make(chan model.Definition)
				go func() {
					ticketspkg.StreamDefinitions(context.Background(), path, defChan, opts)
				}()

				for d := range defChan {
					data, err := json.Marshal(map[string]interface{}{"definition": d})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "definition list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().String("file", "", "File path")
	bindStreamFlags(listCmd)
	root.AddCommand(listCmd)
	return root
}

// #endregion 🕸️Test Command

// #region ⏲️Mermaid

func mermaidCommand(factory EngineFactory, config *Config) *Command {
	root := &Command{
		Use:   "mermaid <visualization>",
		Short: "Generate mermaid diagram strings",
	}
	root.AddCommand(&Command{
		Use:   "loc-by-technologies-bundles-folders-files",
		Short: "LOC treemap grouped by technology, bundle, folder, file",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), treepkg.MermaidLocByTechnologiesBundlesFoldersFiles())
			return nil
		},
	})
	root.AddCommand(&Command{
		Use:   "loc-by-contributors",
		Short: "LOC treemap grouped by contributor (via git blame)",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), treepkg.MermaidLocByContributors())
			return nil
		},
	})
	root.AddCommand(&Command{
		Use:   "loc-by-language",
		Short: "LOC treemap grouped by programming language",
		Args:  NoArgs,
		RunE: func(cmd *Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), treepkg.MermaidLocByLanguage())
			return nil
		},
	})
	return root
}

// #endregion ⏲️Mermaid
