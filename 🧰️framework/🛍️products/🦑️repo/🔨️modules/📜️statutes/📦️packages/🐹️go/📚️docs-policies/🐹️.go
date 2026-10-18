package statutes

import (
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 📚️Docs Policies

// 😀emojiPolicy holds the data fields for a emojiPolicy record.
func emojiPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}

	textDefaultEmojis := []string{
		"\U0001F3D7",
		"\u2328",
		"\U0001F5B1",
		"\U0001F5C3",
		"\u2699",
		"\u2696",
		"\U0001F3F7",
		"\U0001F6E0",
		"\u2702",
		"\U0001F6E1",
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		hasVS15 := strings.Contains(content, "\uFE0E")
		hasMissingVS16 := false
		for _, emoji := range textDefaultEmojis {
			temp := strings.ReplaceAll(content, emoji+"\uFE0F", "")
			if strings.Contains(temp, emoji) {
				hasMissingVS16 = true
				break
			}
		}

		if hasVS15 || hasMissingVS16 {
			lines := strings.Split(content, "\n")
			for i, line := range lines {
				lineHasVS15 := strings.Contains(line, "\uFE0E")
				lineHasMissingVS16 := false
				for _, emoji := range textDefaultEmojis {
					temp := strings.ReplaceAll(line, emoji+"\uFE0F", "")
					if strings.Contains(temp, emoji) {
						lineHasMissingVS16 = true
						break
					}
				}

				if lineHasVS15 || lineHasMissingVS16 {
					breachs = append(breachs, ctx.CreateBreach(
						"Emoji must use colorful variation (VS16) and avoid text variation (VS15)",
						model.BreachCodeUnicodeEmojiVariation,
						file, i+1, 0, strings.TrimSpace(line)))
				}
			}
		}
	}
	return breachs
}

// ▪️docsPolicy holds the data fields for a docsPolicy record.
func docsPolicy(ctx *PolicyContext) []model.Breach {
	if ctx.Scope.Kind == workspace.ScopeFile || ctx.Scope.Kind == workspace.ScopeSection || ctx.Scope.Kind == workspace.ScopeDefinition {
		return nil
	}
	var breachs []model.Breach
	checked := make(map[string]bool)
	for _, bundle := range ctx.Bundles {
		bundleRoot := bundle.Root
		if checked[bundleRoot] {
			continue
		}
		checked[bundleRoot] = true
		readmePath := filepath.Join(bundleRoot, "README.md")
		absPath := filepath.Join(ctx.RootDir, readmePath)
		if _, err := os.Stat(absPath); os.IsNotExist(err) {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Bundle %q is missing README.md with # Summary and # 💯️Requirements sections", bundle.Name),
				model.BreachCodeDocsMissingReadme,
				readmePath, 0, 0, ""))
			continue
		}
		content := ctx.ReadText(readmePath)
		if !strings.Contains(content, "# Summary") {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Bundle %q README.md is missing # Summary section", bundle.Name),
				model.BreachCodeDocsMissingReadme,
				readmePath, 0, 0, ""))
		}
		if !strings.Contains(content, "# 💯️Requirements") {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Bundle %q README.md is missing # 💯️Requirements section", bundle.Name),
				model.BreachCodeDocsMissingReadme,
				readmePath, 0, 0, ""))
		}
	}
	return breachs
}

// ▫️devDocsPolicy holds the data fields for a devDocsPolicy record.
func devDocsPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	agentsContent := ctx.ReadText("AGENTS.md")
	if agentsContent == "" {
		return breachs
	}
	codebaseStart := strings.Index(agentsContent, "\n# Codebase")
	if codebaseStart == -1 {
		return breachs
	}
	codebaseContent := agentsContent[codebaseStart:]
	nextH1 := strings.Index(codebaseContent[1:], "\n# ")
	if nextH1 != -1 {
		codebaseContent = codebaseContent[:nextH1+1]
	}
	fileSectionRegex := regexp.MustCompile(`(?m)^## 📄\x{FE0F}?\s*(.+?)\s*$`)
	folderSectionRegex := regexp.MustCompile(`(?m)^## 📁\x{FE0F}?\s*(.+?)\s*$`)
	fileMatches := fileSectionRegex.FindAllStringSubmatchIndex(codebaseContent, -1)
	folderMatches := folderSectionRegex.FindAllStringSubmatchIndex(codebaseContent, -1)
	var fileSections []struct {
		path string
		line int
		pos  int
	}
	var folderSections []struct {
		path string
		line int
		pos  int
	}
	for _, match := range fileMatches {
		path := codebaseContent[match[2]:match[3]]
		lineNum := strings.Count(agentsContent[:codebaseStart+match[0]], "\n") + 1
		fileSections = append(fileSections, struct {
			path string
			line int
			pos  int
		}{path: path, line: lineNum, pos: match[0]})
	}
	for _, match := range folderMatches {
		path := codebaseContent[match[2]:match[3]]
		lineNum := strings.Count(agentsContent[:codebaseStart+match[0]], "\n") + 1
		folderSections = append(folderSections, struct {
			path string
			line int
			pos  int
		}{path: path, line: lineNum, pos: match[0]})
	}
	for i := 0; i < len(fileSections)-1; i++ {
		if fileSections[i].path > fileSections[i+1].path {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("File section '%s' should come after '%s' (alphabetical order)", fileSections[i].path, fileSections[i+1].path),
				model.BreachDevDocsWrongFileOrder,
				"AGENTS.md", fileSections[i+1].line, 0, ""))
		}
	}
	for i := 0; i < len(folderSections)-1; i++ {
		if folderSections[i].path > folderSections[i+1].path {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Folder section '%s' should come after '%s' (alphabetical order)", folderSections[i].path, folderSections[i+1].path),
				model.BreachDevDocsWrongFolderOrder,
				"AGENTS.md", folderSections[i+1].line, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}

// ◾sketchpadPolicy holds the data fields for a sketchpadPolicy record.
func sketchpadPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	elementsFile := ""
	for _, file := range files {
		if strings.HasSuffix(file, "elements.tsx") {
			elementsFile = file
			break
		}
	}
	thirdPartyPackages := []string{
		"react", "xstate", "@radix-client", "@dnd-kit", "zustand", "immer",
		"framer-motion", "clsx", "tailwind", "three", "@react-three",
	}
	createMachineCount := 0
	for _, file := range files {
		if !strings.HasSuffix(file, ".ts") && !strings.HasSuffix(file, ".tsx") {
			continue
		}
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lines := strings.Split(content, "\n")
		isElementsFile := file == elementsFile
		sections := ctx.Sections(file)
		isStateManagementSection := func(lineNum int) bool {
			for _, section := range sections {
				if strings.Contains(strings.ToLower(section.Name), "state management") ||
					strings.Contains(strings.ToLower(section.Name), "state-management") {
					if lineNum >= section.StartLine && lineNum <= section.EndLine {
						return true
					}
				}
			}
			return false
		}
		for lineNum, line := range lines {
			lineNumber := lineNum + 1
			if !isElementsFile && strings.Contains(line, "import ") {
				for _, pkg := range thirdPartyPackages {
					importPattern := fmt.Sprintf(`from\s+['"]%s`, regexp.QuoteMeta(pkg))
					if matched, _ := regexp.MatchString(importPattern, line); matched {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Third party import '%s' must only be in elements.tsx", pkg),
							model.BreachSketchpadImportThirdParty,
							file, lineNumber, 0, strings.TrimSpace(line)))
						break
					}
				}
			}
			if strings.Contains(line, "createMachine(") || strings.Contains(line, "createMachine<") {
				createMachineCount++
				if createMachineCount > 1 {
					breachs = append(breachs, ctx.CreateBreach(
						"createMachine can only be used once in sketchpad",
						model.BreachSketchpadStateMultipleMachines,
						file, lineNumber, 0, strings.TrimSpace(line)))
				}
			}
			if strings.Contains(line, "createActor(") || strings.Contains(line, "createActor<") {
				breachs = append(breachs, ctx.CreateBreach(
					"createActor is forbidden in sketchpad",
					model.BreachSketchpadStateCreateActor,
					file, lineNumber, 0, strings.TrimSpace(line)))
			}
			storePatterns := []string{"create(", "createStore(", "useStore("}
			for _, pattern := range storePatterns {
				if strings.Contains(line, pattern) && !isStateManagementSection(lineNumber) {
					if strings.Contains(line, "zustand") || strings.Contains(line, "store") {
						breachs = append(breachs, ctx.CreateBreach(
							"Stores outside of State Management sections are forbidden",
							model.BreachSketchpadStateForbiddenStore,
							file, lineNumber, 0, strings.TrimSpace(line)))
					}
				}
			}
		}
	}
	return ctx.FilterIgnored(breachs)
}

// ◽repoPolicy holds the data fields for a repoPolicy record.
func repoPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach

	canonicalCommands := []string{
		"tree",
		"ticket_open", "ticket_close", "ticket_reopen",
		"goal_open", "goal_close", "goal_reopen",
		"draft_create", "draft_delete",
		"section_create", "section_move", "section_delete", "integrate", "extract",
		"fix",
	}

	mainGoPath := "go/repo/main.go"
	mainContent := ctx.ReadText(mainGoPath)
	if mainContent != "" {
		for _, cmd := range canonicalCommands {

			mcpPattern := fmt.Sprintf("mcp.NewTool(\"%s\"", cmd)
			if !strings.Contains(mainContent, mcpPattern) {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Missing MCP registration for %s in go/repo/main.go", cmd),
					model.BreachRepoMissingCommand,
					mainGoPath, 1, 0, cmd))
			}
		}

		trackingTokens := []string{
			"ToolTicketOpen",
			"ToolTicketClose",
		}

		for _, token := range trackingTokens {
			if !strings.Contains(mainContent, token) {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Missing ticket tracking function %s in go/repo/main.go", token),
					model.BreachRepoMissingTicketTracking,
					mainGoPath, 1, 0, token))
			}
		}
	} else {
		breachs = append(breachs, ctx.CreateBreach(
			"Could not read go/repo/main.go for parity check",
			model.BreachRepoMissingCommand,
			mainGoPath, 1, 0, ""))
	}

	return ctx.FilterIgnored(breachs)
}

// #endregion 📚️Docs Policies
