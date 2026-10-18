// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚚️move carries the move, integrate, extract and rename commands of the cli.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"

	codebasepkg "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🕸️Test Command

// 🚚️moveCommand holds the data fields for a moveCommand record.
func moveCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "move <source> <target>",
		Short: "Move an artifact from source to target",
		Long:  "Move an artifact (file, folder, section) between locations. Supports cross-kind moves: file→section calls integrate, section→file calls extract.",
		Args:  ExactArgs(2),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			source := model.ParseArtifactRef(args[0])
			target := model.ParseArtifactRef(args[1])

			var result workspace.ToolResult

			switch {
			case source.Kind == "file" && target.Kind == "file":
				result = move.ToolFileMove(source.Path, target.Path)
			case source.Kind == "folder" && target.Kind == "folder":
				result = move.ToolFolderMove(source.Path, target.Path)
			case source.Kind == "section" && target.Kind == "section" && source.Path == target.Path:
				if len(source.SectionParts) == 0 || len(target.SectionParts) == 0 {
					return fmt.Errorf("missing section path")
				}
				oldName := codebasepkg.ResolveSectionName(source.Path, source.SectionParts[len(source.SectionParts)-1])
				newName := codebasepkg.ResolveSectionName(target.Path, target.SectionParts[len(target.SectionParts)-1])
				result = move.ToolSectionMove(source.Path, oldName, newName)
			case source.Kind == "file" && target.Kind == "section":
				targetFile := target.Path
				var sectionName, parentSection string
				if len(target.SectionParts) > 0 {
					sectionName = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
				}
				if len(target.SectionParts) > 1 {
					parentSection = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
				}
				result = move.ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
				if result.Error == "" {
					absSource := filepath.Join(workspace.RootDir, source.Path)
					os.Remove(absSource)
					ticketspkg.RemoveAgentsDocsEntry(source.Path)
				}
			case source.Kind == "section" && target.Kind == "file":
				if len(source.SectionParts) == 0 {
					return fmt.Errorf("missing section path")
				}
				sourceFile := source.Path
				sectionName := codebasepkg.ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
				result = move.ToolExtract(sourceFile, sectionName, target.Path)
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

func integrateCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "integrate [source] [target]",
		Short: "Integrate source code into a target file section",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			if len(args) == 2 {
				source := model.ParseArtifactRef(args[0])
				target := model.ParseArtifactRef(args[1])

				if source.Kind == "file" && target.Kind == "section" {
					targetFile := target.Path
					var sectionName, parentSection string
					if len(target.SectionParts) > 0 {
						sectionName = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-1])
					}
					if len(target.SectionParts) > 1 {
						parentSection = codebasepkg.ResolveSectionName(targetFile, target.SectionParts[len(target.SectionParts)-2])
					}
					result := move.ToolIntegrate(source.Path, sectionName, targetFile, parentSection)
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

			result := move.ToolIntegrate(file, targetSection, targetFile, parentSection)
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
func extractCommand(factory EngineFactory, config *Config) *Command {
	cmd := &Command{
		Use:   "extract [source] [target]",
		Short: "Extract a section from a source file into a target file",
		Args:  MaximumNArgs(2),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}

			if len(args) == 2 {
				source := model.ParseArtifactRef(args[0])
				target := model.ParseArtifactRef(args[1])

				if source.Kind == "section" && target.Kind == "file" {
					if len(source.SectionParts) == 0 {
						return fmt.Errorf("missing section path")
					}
					sourceFile := source.Path
					sectionName := codebasepkg.ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
					result := move.ToolExtract(sourceFile, sectionName, target.Path)
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

			result := move.ToolExtract(file, section, targetFile)
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

// #endregion 🕸️Test Command

// #region 🔤️Rename

// 🔤️renameCommand returns a command command that renames a token across the repo in all case variants.
func renameCommand(factory EngineFactory, config *Config) *Command {
	return &Command{
		Use:   "rename <old> <new> [scope]",
		Short: "Rename a token across non-gitignored files (all case variants)",
		Long:  "Rewrite UPPER, Title and lower case variants of <old> to <new> in every non-gitignored file's contents and filenames (including folder names) under the optional scope directory (default: repo root). Example: repo rename model representation compose.",
		Args:  RangeArgs(2, 3),
		RunE: func(cmd *Command, args []string) error {
			_, err := factory(*config)
			if err != nil {
				return err
			}
			scope := ""
			if len(args) == 3 {
				scope = args[2]
			}
			result := move.ToolRename(args[0], args[1], scope)
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

// #endregion 🔤️Rename
