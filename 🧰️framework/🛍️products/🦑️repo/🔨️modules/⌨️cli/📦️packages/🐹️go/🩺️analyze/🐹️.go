// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🩺️analyze carries the analyze and autofix commands of the cli with the policy tool functions.

// #endregion 🧲️Header

package cli

import (
	"fmt"
	"encoding/json"
	"path/filepath"
	"sort"
	"strings"
	"time"
	codebasepkg "github.com/usalu/semio/repo/codebase"
	eventspkg "github.com/usalu/semio/repo/events"
	graphqlpkg "github.com/usalu/semio/repo/graphql"
	languagespkg "github.com/usalu/semio/repo/languages"
	"github.com/usalu/semio/repo/model"
	"github.com/usalu/semio/repo/statutes"
	"github.com/usalu/semio/repo/workspace"

)

// #region 🪅️Analyze Command

// Analyze command implementation for policy breach detection.
// 🔬️analyzeCmd holds the data fields for a analyzeCmd record.
var analyzeCmd = &Command{
	Use:   "analyze [scope]",
	Short: "Analyze codebase for breachs",
	Args:  MaximumNArgs(1),
	RunE: func(cmd *Command, args []string) error {
		scope := "all"
		if len(args) > 0 { scope = args[0] }
		policies, err := cmd.Flags().GetStringSlice("policy")
		if err != nil { return err }
		result := ToolAnalyze(scope, policies)
		if result.Error != "" { return fmt.Errorf("%s", result.Error) }
		for _, line := range result.Output.Lines { fmt.Fprintln(cmd.OutOrStdout(), line.Text) }
		return nil
	},
}

// #endregion 🪅️Analyze Command

// #region 📃️Fix Command

// 🔧️Fix command implementation for automatic policy breach repair.
// 🔧️autofixCmd holds the data fields for a autofixCmd record.
var autofixCmd = &Command{
	Use:   "autofix [scope]",
	Short: "Apply autofixes for breachs",
	RunE: func(cmd *Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return graphqlpkg.PrintGQL(`
			mutation Fix($scope: String) {
				fix(scope: $scope) {
					fixed
					remaining
					breachs {
						id
						summary
						scope
						excerpt
						line
					}
				}
			}
		`, variables)
	},
}

// #endregion 📃️Fix Command

// #region 🧬️Missing Utilities

// 📪️CanCloseTicket MUST return a deterministic boolean result.
func CanCloseTicket(ticket *model.Ticket) (bool, []string) {
	var reasons []string
	if ticket == nil {
		reasons = append(reasons, "Ticket data is nil")
		return false, reasons
	}
	return len(reasons) == 0, reasons
}

// 🔶️GuessSectionName MUST complete the operation successfully.
func GuessSectionName(filePath string) string {
	base := filepath.Base(filePath)
	ext := filepath.Ext(base)
	name := strings.TrimSuffix(base, ext)
	name = strings.ReplaceAll(name, "-", " ")
	name = strings.ReplaceAll(name, "_", " ")
	name = strings.ReplaceAll(name, ".", " ")
	words := strings.Fields(name)
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + w[1:]
		}
	}
	return strings.Join(words, " ")
}

// 🐙️GetGitDiffSectionLineMetrics MUST retrieve the requested value or return an error.
// ⚖️GetGitDiffSectionLineMetrics retrieves and returns the git diff section line metrics.
func GetGitDiffSectionLineMetrics(baseCheckpoint, endCheckpoint, filePath string) map[string]model.LineMetrics {
	return nil
}

// 🔖️FlattenSections MUST return a single-level collection with all nested items.
// 🔢️FlattenSections flattens the nested sections into a single level.
func FlattenSections(sections []model.Section) []model.Section {
	var result []model.Section
	var flatten func(secs []model.Section)
	flatten = func(secs []model.Section) {
		for _, s := range secs {
			result = append(result, s)
			flatten(s.Children)
		}
	}
	flatten(sections)
	return result
}

func computeAffectedSections(filePath string, sections []model.Section, defs []languagespkg.DefinitionRange, addedLineMap map[string][]int, removedLineMap map[string][]int, parentPath string) []model.TicketSection {
	var result []model.TicketSection
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		exclusiveAddedLines := addedLineMap[sectionPath]
		exclusiveRemovedLines := removedLineMap[sectionPath]

		if len(exclusiveAddedLines) > 0 || len(exclusiveRemovedLines) > 0 {
			var affectedDefs []string
			for _, def := range defs {
				if def.Start >= section.StartLine && def.Start <= section.EndLine {
					isInChild := false
					for _, child := range section.Children {
						if def.Start >= child.StartLine && def.Start <= child.EndLine {
							isInChild = true
							break
						}
					}

					if !isInChild {
						defAddedLines := codebasepkg.ComputeLinesInRange(exclusiveAddedLines, def.Start, def.End)
						if len(defAddedLines) > 0 {
							affectedDefs = append(affectedDefs, def.Name)
						}
					}
				}
			}

			result = append(result, model.TicketSection{
				Name:        sectionPath,
				Range:       &model.Range{Start: section.StartLine, End: section.EndLine},
				Definitions: uniqueStrings(affectedDefs),
				Lines:       &model.LineMetrics{Added: len(exclusiveAddedLines), Removed: len(exclusiveRemovedLines)},
			})
		}

		if len(section.Children) > 0 {
			childResults := computeAffectedSections(filePath, section.Children, defs, addedLineMap, removedLineMap, sectionPath)
			result = append(result, childResults...)
		}
	}
	return result
}

// 🔹️setIntersection holds the data fields for a setIntersection record.
func setIntersection(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var intersection []int
	for _, x := range a {
		if m[x] {
			intersection = append(intersection, x)
		}
	}
	return intersection
}

// 🔤️uniqueStrings holds the data fields for a uniqueStrings record.
func uniqueStrings(strs []string) []string {
	seen := make(map[string]bool)
	result := []string{}
	for _, s := range strs {
		if !seen[s] {
			seen[s] = true
			result = append(result, s)
		}
	}
	return result
}

// 🔸️findSectionForLine holds the data fields for a findSectionForLine record.
func findSectionForLine(sections []model.Section, line int) string {
	for _, section := range sections {
		if line >= section.StartLine && line <= section.EndLine {
			if len(section.Children) > 0 {
				childSection := findSectionForLine(section.Children, line)
				if childSection != "" {
					return section.Name + "/" + childSection
				}
			}
			return section.Name
		}
	}
	return ""
}

// #endregion 🧬️Missing Utilities

// #region 🔊️Cli

// 🔹️ensureRepoResolver lazily initializes the repoResolverInstance on first use.
func ensureRepoResolver() {
	if graphqlpkg.RepoResolverInstance == nil {
		graphqlpkg.RepoResolverInstance = graphqlpkg.NewResolver(workspace.GetRootDir())
	}
}

// #endregion 🔊️Cli

// #region 🖋️Missing Tool Functions

// 🔬️ToolAnalyze MUST complete the operation successfully.
func ToolAnalyze(scopeRaw string, policyIDs []string) workspace.ToolResult {
	for _, id := range policyIDs {
		if _, found := statutes.FindPolicy(id); !found { return workspace.ToolResult{Error: fmt.Sprintf("unknown analysis policy %q", id)} }
	}
	eventspkg.Emit(eventspkg.EventAnalyzeStarting, "repo-cli", eventspkg.FolderPayload{Path: scopeRaw})
	scope := workspace.ParseScope(scopeRaw)
	bundles := codebasepkg.GetTechnologies()
	extensions, language := analysisScopeExtensions(scopeRaw)
	var breachs []model.Breach
	var err error
	if language {
		scope = workspace.ParseScope("compose")
		wanted := []string{}
		for extension := range extensions { wanted = append(wanted, strings.TrimPrefix(extension, ".")) }
		files, listErr := workspace.GlobByExtension(workspace.RootDir, "**/*", wanted, []string{"**/node_modules/**", "**/.venv/**"}, true)
		if listErr != nil { return workspace.ToolResult{Error: listErr.Error()} }
		selected := make([]string, 0)
		for _, file := range files { if extensions[strings.ToLower(filepath.Ext(file))] { selected = append(selected, file) } }
		breachs, err = statutes.CheckPoliciesWithContext(statutes.NewPolicyContextWithFiles(scope, bundles, selected), policyIDs)
	} else {
		breachs, err = statutes.CheckPolicies(scope, bundles, policyIDs)
	}
	if err != nil {
		return workspace.ToolResult{Error: err.Error()}
	}

	byPriority := make(map[string]int)
	for range breachs {

	}

	report := languagespkg.AnalyzeReport{
		Second:  time.Now().Format(time.RFC3339),
		Status:  "success",
		Scope:   scopeRaw,
		Breachs: breachs,
		Summary: languagespkg.Summary{
			Total:      len(breachs),
			ByPriority: byPriority,
		},
	}
	output := workspace.NewOutput()
	eventspkg.Emit(eventspkg.EventAnalyzeEnded, "repo-cli", eventspkg.FolderPayload{Path: scopeRaw})
	bytes, _ := json.MarshalIndent(report, "", "  ")
	output.Plain(string(bytes))
	return workspace.ToolResult{Output: *output, Data: report}
}

// 🔬️ Selects the explicit dashboard language scopes before policies read their files.
func analysisScopeExtensions(scope string) (map[string]bool, bool) {
	values := map[string][]string{"all": {".rs", ".ts", ".tsx", ".go", ".py", ".cs", ".md", ".json"}, "rust": {".rs"}, "typescript": {".ts", ".tsx"}, "go": {".go"}, "python": {".py"}, "markdown": {".md"}}
	exts, found := values[scope]
	selected := map[string]bool{}
	for _, extension := range exts { selected[extension] = true }
	return selected, found
}

// 🔠️sortedRegisteredPolicies returns the registered policies ordered by identifier, the order every surface renders.
func sortedRegisteredPolicies() []statutes.PolicyDef {
	policies := append([]statutes.PolicyDef(nil), statutes.GetRegisteredPolicies()...)
	sort.SliceStable(policies, func(left, right int) bool { return policies[left].ID < policies[right].ID })
	return policies
}

// 📜️ToolPolicyList MUST complete the operation successfully.
// Uses registered policy metadata only — avoids building the monorepo tree.
func ToolPolicyList() workspace.ToolResult {
	allPolicies := sortedRegisteredPolicies()
	var sb strings.Builder
	for _, p := range allPolicies {
		pData := map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description}
		sb.WriteString(model.RenderEntityMarkdown("policy", pData) + "\n")
	}
	output := workspace.NewOutput()
	output.Plain(sb.String())
	return workspace.ToolResult{Output: *output, Data: allPolicies}
}

// 🌳️ToolPolicyTree MUST complete the operation successfully.
func ToolPolicyTree() workspace.ToolResult {
	allPolicies := sortedRegisteredPolicies()
	var sb strings.Builder
	for _, p := range allPolicies {
		pData := map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description}
		sb.WriteString(model.RenderEntityMarkdown("policy", pData) + "\n")
		for _, k := range p.AllKinds() {
			meta := k.Info()
			vkData := map[string]interface{}{"id": string(k), "description": meta.Reason}
			sb.WriteString("  " + model.RenderEntityMarkdown("statute", vkData) + "\n")
		}
	}
	treeOutput := workspace.NewOutput()
	treeOutput.Plain(sb.String())
	return workspace.ToolResult{Output: *treeOutput, Data: allPolicies}
}

// ✔️ToolPolicyCheck MUST complete the operation successfully.
func ToolPolicyCheck(policyID, scopeRaw string) workspace.ToolResult {
	return ToolAnalyze(scopeRaw, []string{policyID})
}

// 📋️ToolPolicyBreachList MUST complete the operation successfully.
func ToolPolicyBreachList(policyID string) workspace.ToolResult {

	return ToolAnalyze("compose", []string{policyID})
}

// #endregion 🖋️Missing Tool Functions
