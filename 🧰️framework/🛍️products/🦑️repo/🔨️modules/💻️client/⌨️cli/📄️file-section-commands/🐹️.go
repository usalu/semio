// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// File, section, definition, move, integrate, and extract commands.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"os"
	"path/filepath"
)

// ◾fileCommand holds the data fields for a fileCommand record.
func fileCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "file", Short: "File management commands"}
	createCmd := &command.Command{
		Use:   "create",
		Short: "Create a file",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
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
	moveCmd := &command.Command{
		Use:   "move",
		Short: "Move a file",
		Args:  command.MaximumNArgs(2),
		RunE: func(cmd *command.Command, args []string) error {
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
	deleteCmd := &command.Command{
		Use:   "delete",
		Short: "Delete a file",
		Args:  command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
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
func sectionCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "section", Short: "Section management commands"}
	createCmd := &command.Command{
		Use:   "create",
		Short: "Create a section",
		Args:  command.MaximumNArgs(3),
		RunE: func(cmd *command.Command, args []string) error {
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
	moveCmd := &command.Command{
		Use:   "move",
		Short: "Move a section",
		Args:  command.MaximumNArgs(3),
		RunE: func(cmd *command.Command, args []string) error {
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
	deleteCmd := &command.Command{
		Use:   "delete",
		Short: "Delete a section",
		Args:  command.MaximumNArgs(2),
		RunE: func(cmd *command.Command, args []string) error {
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
	integrateCmd := &command.Command{
		Use:   "integrate",
		Short: "Integrate source code into a target file section",
		Args:  command.MaximumNArgs(4),
		RunE: func(cmd *command.Command, args []string) error {
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

	extractCmd := &command.Command{
		Use:   "extract",
		Short: "Extract a section from a source file into a target file",
		Args:  command.MaximumNArgs(3),
		RunE: func(cmd *command.Command, args []string) error {
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
func definitionCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{Use: "definition", Short: "Definition management commands"}
	listCmd := &command.Command{
		Use:     "list",
		Aliases: []string{"tree"},
		Short:   "List definitions",
		Args:    command.MaximumNArgs(1),
		RunE: func(cmd *command.Command, args []string) error {
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

				defChan := make(chan Definition)
				go func() {
					StreamDefinitions(context.Background(), path, defChan, opts)
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

// 🚚️moveCommand holds the data fields for a moveCommand record.
func moveCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "move <source> <target>",
		Short: "Move an artifact from source to target",
		Long:  "Move an artifact (file, folder, section) between locations. Supports cross-kind moves: file→section calls integrate, section→file calls extract.",
		Args:  command.ExactArgs(2),
		RunE: func(cmd *command.Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			source := ParseArtifactRef(args[0])
			target := ParseArtifactRef(args[1])

			var result ToolResult

			switch {
			case source.Kind == "file" && target.Kind == "file":
				result = ToolFileMove(source.Path, target.Path)
			case source.Kind == "folder" && target.Kind == "folder":
				result = ToolFolderMove(source.Path, target.Path)
			case source.Kind == "section" && target.Kind == "section" && source.Path == target.Path:
				if len(source.SectionParts) == 0 || len(target.SectionParts) == 0 {
					return fmt.Errorf("missing section path")
				}
				oldName := ResolveSectionName(source.Path, source.SectionParts[len(source.SectionParts)-1])
				newName := ResolveSectionName(target.Path, target.SectionParts[len(target.SectionParts)-1])
				result = ToolSectionMove(source.Path, oldName, newName)
			case source.Kind == "file" && target.Kind == "section":
				targetFile := target.Path
				var sectionName, parentSection string
				if len(target.SectionParts) > 0 {
					sectionName = ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
				}
				if len(target.SectionParts) > 1 {
					parentSection = ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
				}
				result = ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
				if result.Error == "" {
					absSource := filepath.Join(rootDir, source.Path)
					os.Remove(absSource)
					RemoveAgentsDocsEntry(source.Path)
				}
			case source.Kind == "section" && target.Kind == "file":
				if len(source.SectionParts) == 0 {
					return fmt.Errorf("missing section path")
				}
				sourceFile := source.Path
				sectionName := ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
				result = ToolExtract(sourceFile, sectionName, target.Path)
			default:
				return fmt.Errorf("unsupported move: %s → %s", source.Kind, target.Kind)
			}

			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
}

func integrateCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "integrate [source] [target]",
		Short: "Integrate source code into a target file section",
		Args:  command.MaximumNArgs(2),
		RunE: func(cmd *command.Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			if len(args) == 2 {
				source := ParseArtifactRef(args[0])
				target := ParseArtifactRef(args[1])

				if source.Kind == "file" && target.Kind == "section" {
					targetFile := target.Path
					var sectionName, parentSection string
					if len(target.SectionParts) > 0 {
						sectionName = ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
					}
					if len(target.SectionParts) > 1 {
						parentSection = ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
					}
					result := ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
					if result.Error != "" {
						return fmt.Errorf("%s", result.Error)
					}
					for _, line := range result.Output.Lines {
						fmt.Println(line.Text)
					}
					return nil
				}
			}

			file, _ := cmd.Flags().GetString("file")
			targetFile, _ := cmd.Flags().GetString("target-file")
			targetSection, _ := cmd.Flags().GetString("target-section")
			parentSection, _ := cmd.Flags().GetString("parent-section")

			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if file == "" || targetFile == "" || targetSection == "" {
				return fmt.Errorf("missing file, target-file, or target-section")
			}

			result := ToolIntegrate(file, targetSection, targetFile, parentSection)
			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
	cmd.Flags().String("file", "", "Source file path")
	cmd.Flags().String("target-file", "", "Target file path")
	cmd.Flags().String("target-section", "", "Target section name")
	cmd.Flags().String("parent-section", "", "Parent section name")
	return cmd
}

// ◼extractCommand holds the data fields for a extractCommand record.
func extractCommand(factory EngineFactory, config *Config) *command.Command {
	cmd := &command.Command{
		Use:   "extract [source] [target]",
		Short: "Extract a section from a source file into a target file",
		Args:  command.MaximumNArgs(2),
		RunE: func(cmd *command.Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			if len(args) == 2 {
				source := ParseArtifactRef(args[0])
				target := ParseArtifactRef(args[1])

				if source.Kind == "section" && target.Kind == "file" {
					if len(source.SectionParts) == 0 {
						return fmt.Errorf("missing section path")
					}
					sourceFile := source.Path
					sectionName := ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
					result := ToolExtract(sourceFile, sectionName, target.Path)
					if result.Error != "" {
						return fmt.Errorf("%s", result.Error)
					}
					for _, line := range result.Output.Lines {
						fmt.Println(line.Text)
					}
					return nil
				}
			}

			file, _ := cmd.Flags().GetString("file")
			section, _ := cmd.Flags().GetString("section")
			targetFile, _ := cmd.Flags().GetString("target-file")

			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if file == "" || section == "" || targetFile == "" {
				return fmt.Errorf("missing file, section, or target-file")
			}

			result := ToolExtract(file, section, targetFile)
			if result.Error != "" {
				return fmt.Errorf("%s", result.Error)
			}
			for _, line := range result.Output.Lines {
				fmt.Println(line.Text)
			}
			return nil
		},
	}
	cmd.Flags().String("file", "", "Source file path")
	cmd.Flags().String("section", "", "Section name")
	cmd.Flags().String("parent-section", "", "Parent section name")
	cmd.Flags().String("target-file", "", "Target file path")
	return cmd
}
