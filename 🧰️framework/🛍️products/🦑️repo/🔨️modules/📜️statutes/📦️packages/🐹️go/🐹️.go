// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📜️statutes is the statutes domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package statutes

import (
	"context"
	"strings"
	"github.com/usalu/semio/repo/model"
	"github.com/usalu/semio/repo/workspace"

)

// #region 📜️Statutes

// 🩵️PolicyDef holds the data fields for a policy def record.
type PolicyDef struct {
	ID          string               `json:"id"`
	Name        string               `json:"name"`
	Description string               `json:"description"`
	Scopes      []string             `json:"scopes"`
	Priority    model.BreachPriority `json:"priority"`
	Groups      []model.Territory    `json:"groups"`
	Run         PolicyFunc           `json:"-"`
}

// 🩶️AllKinds MUST include all statutes from the group and its children.
// 🟪️AllKinds returns all statutes associated with the group.
func (p *PolicyDef) AllKinds() []model.Statute {
	var result []model.Statute
	for _, g := range p.Groups {
		result = append(result, g.AllKinds()...)
	}
	return result
}

type PolicyFunc func(ctx *PolicyContext) []model.Breach

// 💿️policies holds the data fields for a policies record.
var policies = []PolicyDef{
	{
		ID:          "code",
		Name:        "Code",
		Description: "Validates source file headers, sections, and comments",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
		Priority:    model.BreachPriorityLow,
		Groups: []model.Territory{
			{
				Name:        "File",
				Description: "File header region breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Groups:      []model.Territory{},
				Kinds: []model.Statute{
					model.BreachCodeFileMissingHeaderRegion,
					model.BreachCodeFileWrongHeaderRegionFormat,
					model.BreachCodeFileMissingContributors,
					model.BreachCodeFileMissingSummary,
					model.BreachCodeFileMissingLicense,
					model.BreachCodeFileWrongLicense,
					model.BreachCodeFileMissingRequirements,
					model.BreachCodeFileMissingDocs,
				},
			},
			{
				Name:        "Section",
				Description: "Section structure breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Groups: []model.Territory{
					{
						Name:        "Wrong Format",
						Description: "Section format breachs",
						Groups: []model.Territory{
							{
								Name:        "Summary",
								Description: "Section summary format breachs",
								Kinds: []model.Statute{
									model.BreachCodeSectionWrongFormatSummaryTooLong,
								},
							},
							{
								Name:        "Requirements",
								Description: "Section requirements format breachs",
								Kinds: []model.Statute{
									model.BreachCodeSectionWrongFormatRequirementsSplitBlock,
								},
							},
						},
						Kinds: []model.Statute{
							model.BreachCodeSectionWrongFormat,
							model.BreachCodeSectionWrongFormatNewlineAfterRegion,
							model.BreachCodeSectionWrongFormatDocs,
						},
					},
				},
				Kinds: []model.Statute{
					model.BreachCodeSectionEmpty,
					model.BreachCodeSectionOrphanDefinition,
					model.BreachCodeSectionMissingStartName,
					model.BreachCodeSectionMissingEndName,
					model.BreachCodeSectionNameMismatch,
					model.BreachCodeSectionMissingSummary,
					model.BreachCodeSectionMissingRequirements,
					model.BreachCodeSectionMissingDocs,
				},
			},
			{
				Name:        "Definition",
				Description: "Definition documentation breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Groups: []model.Territory{
					{
						Name:        "Wrong Format",
						Description: "Definition format breachs",
						Kinds: []model.Statute{
							model.BreachCodeDefWrongFormat,
							model.BreachCodeDefNotNativeDocstring,
						},
					},
				},
				Kinds: []model.Statute{
					model.BreachCodeDefMissingSummary,
					model.BreachCodeDefMissingRequirements,
					model.BreachCodeDefMissingDocs,
				},
			},
			{
				Name:        "Comment",
				Description: "Forbidden comment breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []model.Statute{
					model.BreachCodeCommentInline,
					model.BreachCodeCommentBlock,
					model.BreachCodeCommentJSDoc,
				},
			},
			{
				Name:        "Requirements",
				Description: "Specification content breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []model.Statute{
					model.BreachCodeRequirementsSyntax,
				},
			},
			{
				Name:        "Unicode",
				Description: "Unicode character breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []model.Statute{
					model.BreachCodeUnicodeEmojiVariation,
				},
			},
			{
				Name:        "Docs",
				Description: "Documentation file breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []model.Statute{
					model.BreachCodeDocsMissingReadme,
				},
			},
			{
				Name:        "Rust",
				Description: "Rust-specific section breachs",
				Scopes:      []string{"**/*.rs"},
				Kinds: []model.Statute{
					model.BreachCodeRustRegionComment,
				},
			},
		},
		Run: codePolicy,
	},
	{
		ID:          "dev-docs",
		Name:        "DevDocs",
		Description: "Validates README.md and AGENTS.md documentation structure",
		Scopes:      []string{"README.md", "AGENTS.md"},
		Priority:    model.BreachPriorityLow,
		Groups: []model.Territory{
			{
				Name:        "File",
				Description: "File documentation breachs",
				Scopes:      []string{"AGENTS.md"},
				Kinds: []model.Statute{
					model.BreachDevDocsMissingFile,
					model.BreachDevDocsWrongFilePath,
					model.BreachDevDocsWrongFileName,
					model.BreachDevDocsWrongFileOrder,
				},
			},
			{
				Name:        "Folder",
				Description: "Folder documentation breachs",
				Scopes:      []string{"AGENTS.md"},
				Kinds: []model.Statute{
					model.BreachDevDocsMissingFolder,
					model.BreachDevDocsWrongFolderPath,
					model.BreachDevDocsWrongFolderName,
					model.BreachDevDocsWrongFolderOrder,
				},
			},
			{
				Name:        "Component",
				Description: "Component documentation breachs",
				Scopes:      []string{"README.md"},
				Kinds: []model.Statute{
					model.BreachDevDocsMissingComponent,
					model.BreachDevDocsWrongComponentName,
					model.BreachDevDocsWrongComponentOrder,
				},
			},
		},
		Run: devDocsPolicy,
	},
	{
		ID:          "sketchpad",
		Name:        "Sketchpad",
		Description: "Validates sketchpad imports, state management, and hook patterns",
		Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
		Priority:    model.BreachPriorityHigh,
		Groups: []model.Territory{
			{
				Name:        "Import",
				Description: "Import restriction breachs",
				Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
				Kinds: []model.Statute{
					model.BreachSketchpadImportThirdParty,
				},
			},
			{
				Name:        "State",
				Description: "State management breachs",
				Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
				Kinds: []model.Statute{
					model.BreachSketchpadStateMultipleMachines,
					model.BreachSketchpadStateCreateActor,
					model.BreachSketchpadStateYjsAppState,
					model.BreachSketchpadStateForbiddenStore,
				},
			},
			{
				Name:        "Hooks",
				Description: "Hook pattern breachs",
				Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
				Kinds: []model.Statute{
					model.BreachSketchpadHooksNonTriadic,
				},
			},
		},
		Run: sketchpadPolicy,
	},
	{
		ID:          "repo",
		Name:        "Repo",
		Description: "Validates strict repo command implementation parity and ticket tracking",
		Scopes:      []string{"go/repo/main.go", "js/vscode/package.json", "graphql/repo/🔣️schema.graphql"},
		Priority:    model.BreachPriorityHigh,
		Groups: []model.Territory{
			{
				Name:        "Parity",
				Description: "Command parity breachs",
				Scopes:      []string{"go/repo/main.go", "js/vscode/package.json", "graphql/repo/🔣️schema.graphql"},
				Kinds: []model.Statute{
					model.BreachRepoMissingCommand,
					model.BreachRepoMissingTicketTracking,
				},
			},
		},
		Run: repoPolicy,
	},
	{
		ID:          "system",
		Name:        "System",
		Description: "Enforces repository dashboard authority over developer configuration",
		Scopes:      []string{"**/*"},
		Priority:    model.BreachPriorityHigh,
		Groups:      []model.Territory{{Name: "Developer Configuration", Description: "Editor-owned configuration is forbidden", Kinds: []model.Statute{model.BreachSystemIdeConfiguration}}},
		Run:         SystemPolicy,
	},
	{
		ID:          "folder",
		Name:        "Folder",
		Description: "Validates folder structure and detects illegal empty folders",
		Scopes:      []string{"**/*"},
		Priority:    model.BreachPriorityLow,
		Groups: []model.Territory{
			{
				Name:        "Name",
				Description: "Folder emoji identity breaches",
				Kinds: []model.Statute{
					model.BreachFolderNameMissingEmoji,
					model.BreachFolderNameGenericEmoji,
					model.BreachFolderNameNoncanonicalEmojiPresentation,
					model.BreachFolderNameWhitespaceAfterEmoji,
					model.BreachFolderNameEmojiNotUnique,
					model.BreachFolderNameMultipleEmojis,
				},
			},
			{
				Name:        "Illegal",
				Description: "Illegal folder breachs",
				Groups: []model.Territory{
					{
						Name:        "Empty",
						Description: "Empty folders that should be removed",
						Kinds: []model.Statute{
							model.BreachFolderIllegalEmpty,
						},
					},
				},
			},
		},
		Run: folderPolicy,
	},
	{
		ID:          "file",
		Name:        "File",
		Description: "Validates file existence against the godfile allowlist",
		Scopes:      []string{"**/*"},
		Priority:    model.BreachPriorityHigh,
		Groups: []model.Territory{
			{
				Name:        "Name",
				Description: "File emoji identity breaches",
				Kinds: []model.Statute{
					model.BreachFileNameMissingEmoji,
					model.BreachFileNameGenericEmoji,
					model.BreachFileNameNoncanonicalEmojiPresentation,
					model.BreachFileNameWhitespaceAfterEmoji,
					model.BreachFileNameEmojiNotUnique,
					model.BreachFileNameMultipleEmojis,
					model.BreachFileNameReservedEmoji,
				},
			},
			{
				Name:        "Illegal",
				Description: "Illegal file breachs",
				Groups: []model.Territory{
					{
						Name:        "Use Godfile",
						Description: "Files not listed in .🧬semio/🦑️repo/📁️files.json godfile",
						Kinds: []model.Statute{
							model.BreachFileIllegalUseGodfile,
						},
					},
				},
			},
		},
		Run: filePolicy,
	},
	{
		ID:          "compose",
		Name:        "Compose",
		Description: "Validates that compose.* files are self-contained and do not import UI dependencies",
		Scopes:      []string{"**/*.{ts,tsx}"},
		Priority:    model.BreachPriorityHigh,
		Groups: []model.Territory{
			{
				Name:        "Import",
				Description: "Import restriction breachs",
				Groups: []model.Territory{
					{
						Name:        "No Ui Dependency",
						Description: "compose.* files must not import from UI dependencies",
						Kinds: []model.Statute{
							model.BreachComposeNoUiDependency,
						},
					},
				},
			},
		},
		Run: composePolicy,
	},
	{
		ID:          "dependency-boundary",
		Name:        "Dependency Boundary",
		Description: "Third-party libraries must only be imported behind adapter boundaries",
		Scopes:      []string{"**/*"},
		Priority:    model.BreachPriorityHigh,
		Groups: []model.Territory{
			{
				Name:        "Import",
				Description: "Direct third-party import breachs",
				Scopes:      []string{"**/*"},
				Kinds: []model.Statute{
					model.BreachDependencyBoundaryDirectImport,
				},
			},
		},
		Run: dependencyBoundaryPolicy,
	},
}

// 🎯FindPolicy MUST return nil when no match is found.
// 🔎FindPolicy searches for and returns the matching policy.
func FindPolicy(id string) (PolicyDef, bool) {
	for _, p := range policies {
		if p.ID == id {
			return p, true
		}
	}
	return PolicyDef{}, false
}

// 🏪GetPolicies MUST return the stored value without modification.
// 📦GetPolicies returns the policies of the value.
func GetPolicies() []PolicyDef {
	return policies
}

// 📪StreamPolicies MUST emit all matching entries and close the channel when done.
// 🔍StreamPolicies streams the policies over a channel with optional filtering.
func StreamPolicies(ctx context.Context, out chan<- PolicyDef, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	for _, p := range policies {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !model.MatchesFilter(p.ID, options) && !model.MatchesFilter(p.Name, options) {
				continue
			}
			if !model.MatchesQuery(p.ID+" "+p.Name+" "+p.Description, options) {
				continue
			}
			if len(options.IncludePolicies) > 0 {
				found := false
				for _, id := range options.IncludePolicies {
					if p.ID == id || strings.HasPrefix(p.ID, id+":") {
						found = true
						break
					}
				}
				if !found {
					continue
				}
			}
			if len(options.ExcludePolicies) > 0 {
				excluded := false
				for _, id := range options.ExcludePolicies {
					if p.ID == id || strings.HasPrefix(p.ID, id+":") {
						excluded = true
						break
					}
				}
				if excluded {
					continue
				}
			}
			out <- p
		}
	}
	return nil
}

// ✔️CheckPolicies MUST run all applicable policies and aggregate breachs.
// ✔️CheckPolicies validates the policies and returns any breachs.
func CheckPolicies(scope workspace.Scope, bundles []model.Bundle, policyIDs []string) ([]model.Breach, error) {
	ctx := NewPolicyContext(scope, bundles)
	return CheckPoliciesWithContext(ctx, policyIDs)
}

// 🟨CheckPoliciesWithContext MUST run all applicable policies and aggregate breachs.
// 🔑CheckPoliciesWithContext validates the policies with context and returns any breachs.
func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]model.Breach, error) {
	var breachs []model.Breach
	var policiesToRun []PolicyDef
	if len(policyIDs) > 0 {
		for _, p := range policies {
			for _, id := range policyIDs {
				if p.ID == id {
					policiesToRun = append(policiesToRun, p)
					break
				}
			}
		}
	} else {
		for _, p := range policies {
			if matchesScope(p.Scopes, ctx.Scope) {
				policiesToRun = append(policiesToRun, p)
			}
		}
	}
	for _, policy := range policiesToRun {
		policyBreachs := policy.Run(ctx)
		breachs = append(breachs, policyBreachs...)
	}
	return breachs, nil
}

// 🔭matchesScope holds the data fields for a matchesScope record.
func matchesScope(policyScopes []string, targetScope workspace.Scope) bool {
	for _, pattern := range policyScopes {
		if pattern == "*" || pattern == "**/*" {
			return true
		}
		if strings.HasPrefix(pattern, "compose") {
			if targetScope.Kind == workspace.ScopeRepo || (targetScope.Kind == workspace.ScopeTechnology && strings.HasPrefix(targetScope.TechnologyName, pattern)) {
				return true
			}
		}
		if targetScope.Kind == workspace.ScopeRepo && strings.HasPrefix(pattern, "**/*.") {
			return true
		}
		if targetScope.FilePath != "" {
			normalizedTarget := workspace.NormalizePath(targetScope.FilePath)
			normalizedPattern := workspace.NormalizePath(pattern)
			if matched, _ := workspace.Match(normalizedPattern, normalizedTarget); matched {
				return true
			}
		}
	}
	return false
}

// 🔬️AnalyzeFile MUST return a non-nil error when the operation fails.
func AnalyzeFile(filePath string, bundles []model.Bundle) ([]model.Breach, error) {
	scope := workspace.Scope{
		Kind:     workspace.ScopeFile,
		FilePath: filePath,
	}
	files := model.FilterConsideredFiles([]string{filePath})
	files = model.FilterGitIgnored(files)
	ctx := NewPolicyContextWithFiles(scope, bundles, files)
	return CheckPoliciesWithContext(ctx, nil)
}

// 🔶️GetRegisteredPolicies MUST retrieve the requested value or return an error.
// 🔖️GetRegisteredPolicies retrieves and returns the registered policies.
func GetRegisteredPolicies() []PolicyDef {
	return GetPolicies()
}

// #endregion 📜️Statutes
