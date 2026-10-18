// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧬️mcp_tool_codebase carries the MCP tool handlers of the cli for folders, files, sections, artifact moves, graphql and navigation.

// #endregion 🧲️Header

package cli

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	codebasepkg "github.com/usalu/semio/repo/codebase"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	model "github.com/usalu/semio/repo/model"
	move "github.com/usalu/semio/repo/move"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔌️Mcp Surface

// #region 🪄️Handlers

// 📁️folderCreate holds the data fields for a folderCreate record.
func folderCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFolderCreate(path)
	return toolResultToMCP(result)
}

// 🔸️folderMove holds the data fields for a folderMove record.
func folderMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	result := move.ToolFolderMove(source, target)
	return toolResultToMCP(result)
}

// 🔺️folderDelete holds the data fields for a folderDelete record.
func folderDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFolderDelete(path)
	return toolResultToMCP(result)
}

// 🆕️fileCreate holds the data fields for a fileCreate record.
func fileCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFileCreate(path)
	return toolResultToMCP(result)
}

// 📄️fileMove holds the data fields for a fileMove record.
func fileMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	result := move.ToolFileMove(source, target)
	return toolResultToMCP(result)
}

// 🔻️fileDelete holds the data fields for a fileDelete record.
func fileDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	result := move.ToolFileDelete(path)
	return toolResultToMCP(result)
}

// 📑️sectionCreate holds the data fields for a sectionCreate record.
func sectionCreate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	result := move.ToolSectionCreate(file, section)
	return toolResultToMCP(result)
}

// ⬛️sectionMove holds the data fields for a sectionMove record.
func sectionMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	oldName, err := requireStringArg(args, "old_name")
	if err != nil {
		return nil, err
	}
	newName, err := requireStringArg(args, "new_name")
	if err != nil {
		return nil, err
	}
	result := move.ToolSectionMove(file, oldName, newName)
	return toolResultToMCP(result)
}

// ⬜️sectionDelete holds the data fields for a sectionDelete record.
func sectionDelete(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	result := move.ToolSectionDelete(file, section)
	return toolResultToMCP(result)
}

func sectionIntegrate(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	targetSection, err := requireStringArg(args, "target_section")
	if err != nil {
		return nil, err
	}
	targetFile, err := requireStringArg(args, "target_file")
	if err != nil {
		return nil, err
	}
	targetParentSection, _, err := getStringArg(args, "target_parent_section")
	if err != nil {
		return nil, err
	}
	result := move.ToolIntegrate(source, targetSection, targetFile, targetParentSection)
	return toolResultToMCP(result)
}

// 🧲️sectionExtract holds the data fields for a sectionExtract record.
func sectionExtract(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	sourceFile, err := requireStringArg(args, "source_file")
	if err != nil {
		return nil, err
	}
	sourceSection, err := requireStringArg(args, "source_section")
	if err != nil {
		return nil, err
	}
	targetFile, err := requireStringArg(args, "target_file")
	if err != nil {
		return nil, err
	}
	result := move.ToolExtract(sourceFile, sourceSection, targetFile)
	return toolResultToMCP(result)
}

// 🏺️artifactMove holds the data fields for a artifactMove record.
func artifactMove(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	sourceID, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	targetID, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}

	source := model.ParseArtifactRef(sourceID)
	target := model.ParseArtifactRef(targetID)

	var result workspace.ToolResult

	switch {
	case source.Kind == "file" && target.Kind == "file":
		result = move.ToolFileMove(source.Path, target.Path)
	case source.Kind == "folder" && target.Kind == "folder":
		result = move.ToolFolderMove(source.Path, target.Path)
	case source.Kind == "section" && target.Kind == "section" && source.Path == target.Path:
		if len(source.SectionParts) == 0 || len(target.SectionParts) == 0 {
			return nil, fmt.Errorf("missing section path")
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
			return nil, fmt.Errorf("missing section path")
		}
		sourceFile := source.Path
		sectionName := codebasepkg.ResolveSectionName(sourceFile, source.SectionParts[len(source.SectionParts)-1])
		result = move.ToolExtract(sourceFile, sectionName, target.Path)
	default:
		return nil, fmt.Errorf("unsupported move: %s → %s", source.Kind, target.Kind)
	}

	return toolResultToMCP(result)
}

func graphqlQuery(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	query, err := requireStringArg(args, "query")
	if err != nil {
		return nil, err
	}
	variablesStr, _, err := getStringArg(args, "variables")
	if err != nil {
		return nil, err
	}
	var variables map[string]interface{}
	if variablesStr != "" {
		if err := json.Unmarshal([]byte(variablesStr), &variables); err != nil {
			return nil, fmt.Errorf("invalid variables JSON: %w", err)
		}
	}
	result, gqlErr := graphqlpkg.Gql(query, variables)
	if gqlErr != nil {
		return textResult(fmt.Sprintf(`{"error": %q}`, gqlErr.Error())), nil
	}
	return textResult(result), nil
}

// 🟥️navigateTool holds the data fields for a navigateTool record.
func navigateTool(ctx context.Context, request CallToolRequest) (*CallToolResult, error) {
	args := getArgs(request)
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	var uri string
	if strings.HasPrefix(target, "repo://") {
		uri = target
	} else {
		uri = model.IdToUri(target)
		if uri == "" {
			return nil, fmt.Errorf("could not resolve target: %s", target)
		}
	}
	id := model.UriToId(uri)
	result := map[string]string{"uri": uri, "id": id}
	bytes, _ := json.Marshal(result)
	return textResult(string(bytes)), nil
}

// #endregion 🪄️Handlers

// #endregion 🔌️Mcp Surface
