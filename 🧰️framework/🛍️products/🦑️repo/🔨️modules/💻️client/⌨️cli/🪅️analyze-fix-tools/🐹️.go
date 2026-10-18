// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Analyze and fix commands and policy tools.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	repopkg "github.com/usalu/semio/repo/go"
	"strings"
	"time"
)

// 💿️printGQL holds the data fields for a printGQL record.
func printGQL(query string, variables map[string]interface{}) error {
	result, err := gql(query, variables)
	if err != nil {
		return err
	}
	fmt.Println(result)
	return nil
}

// 🔬️analyzeCmd holds the data fields for a analyzeCmd record.
var analyzeCmd = &command.Command{
	Use:   "analyze [scope]",
	Short: "Analyze codebase for breachs",
	RunE: func(cmd *command.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		if scope == nil {
			ctx := NewCodebaseContext()
			ctx.LoadBundles()
			if err := ctx.LoadFiles(); err != nil {
				return err
			}
			if err := ctx.LoadBreachs(); err != nil {
				return err
			}
			if err := ctx.LoadTickets(); err != nil {
				return err
			}
			ctx.LoadPolicies()
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return printGQL(`
			query Analyze($scope: String) {
				analyze(scope: $scope) {
					breachs {
						id
						summary
						scope
						line
						column
						excerpt
						kind { id priority autofixable reason solution }
					}
					metrics { total autofixable byPriority { high medium low } }
				}
			}
		`, variables)
	},
}

// 🔧️Fix command implementation for automatic policy breach repair.
// 🔧️autofixCmd holds the data fields for a autofixCmd record.
var autofixCmd = &command.Command{
	Use:   "autofix [scope]",
	Short: "Apply autofixes for breachs",
	RunE: func(cmd *command.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return printGQL(`
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

// 🔬️ToolAnalyze MUST complete the operation successfully.
func ToolAnalyze(scopeRaw string, policyIDs []string) ToolResult {
	repopkg.Emit(repopkg.EventAnalyzeStarting, "repo-cli", repopkg.FolderPayload{Path: scopeRaw})
	scope := ParseScope(scopeRaw)
	bundles := GetTechnologies()
	breachs, err := CheckPolicies(scope, bundles, policyIDs)
	if err != nil {
		return ToolResult{Error: err.Error()}
	}

	byPriority := make(map[string]int)
	for range breachs {

	}

	report := AnalyzeReport{
		Second:  time.Now().Format(time.RFC3339),
		Status:  "success",
		Scope:   scopeRaw,
		Breachs: breachs,
		Summary: Summary{
			Total:      len(breachs),
			ByPriority: byPriority,
		},
	}
	output := NewOutput()
	repopkg.Emit(repopkg.EventAnalyzeEnded, "repo-cli", repopkg.FolderPayload{Path: scopeRaw})
	bytes, _ := json.MarshalIndent(report, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: report}
}

// 🔧️ToolFix MUST complete the operation successfully.
func ToolFix(scopeRaw string) ToolResult {
	repopkg.Emit(repopkg.EventFixStarting, "repo-cli", repopkg.FolderPayload{Path: scopeRaw})
	ctx := NewRepoContext(rootDir)
	res, err := ctx.Fix(&scopeRaw)
	if err != nil {
		return ToolResult{Error: err.Error()}
	}
	output := NewOutput()
	repopkg.Emit(repopkg.EventFixEnded, "repo-cli", repopkg.FolderPayload{Path: scopeRaw})
	bytes, _ := json.MarshalIndent(res, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: res}
}

// 📜️ToolPolicyList MUST complete the operation successfully.
// Uses registered policy metadata only — avoids building the monorepo tree.
func ToolPolicyList() ToolResult {
	allPolicies := GetRegisteredPolicies()
	var sb strings.Builder
	for _, p := range allPolicies {
		pData := map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description}
		sb.WriteString(renderEntityMarkdown("policy", pData) + "\n")
	}
	output := NewOutput()
	output.Plain(sb.String())
	return ToolResult{Output: *output, Data: allPolicies}
}

// 🌳️ToolPolicyTree MUST complete the operation successfully.
func ToolPolicyTree() ToolResult {
	allPolicies := GetRegisteredPolicies()
	var sb strings.Builder
	for _, p := range allPolicies {
		pData := map[string]interface{}{"id": p.ID, "name": p.Name, "description": p.Description}
		sb.WriteString(renderEntityMarkdown("policy", pData) + "\n")
		for _, k := range p.AllKinds() {
			meta := k.Info()
			vkData := map[string]interface{}{"id": string(k), "description": meta.Reason}
			sb.WriteString("  " + renderEntityMarkdown("statute", vkData) + "\n")
		}
	}
	treeOutput := NewOutput()
	treeOutput.Plain(sb.String())
	return ToolResult{Output: *treeOutput, Data: allPolicies}
}

// ✔️ToolPolicyCheck MUST complete the operation successfully.
func ToolPolicyCheck(policyID, scopeRaw string) ToolResult {
	return ToolAnalyze(scopeRaw, []string{policyID})
}

// 📋️ToolPolicyBreachList MUST complete the operation successfully.
func ToolPolicyBreachList(policyID string) ToolResult {

	return ToolAnalyze("compose", []string{policyID})
}
