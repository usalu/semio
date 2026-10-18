// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Policy registry and streaming.

// #endregion 🧲️Header

package client

import (
	"context"
	"strings"
)

type PolicyFunc func(ctx *PolicyContext) []Breach

// 💿️policies holds the data fields for a policies record.
var policies = []PolicyDef{
	{
		ID:          "code",
		Name:        "Code",
		Description: "Validates source file headers, sections, and comments",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
		Priority:    BreachPriorityLow,
		Groups: []Territory{
			{
				Name:        "File",
				Description: "File header region breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Groups:      []Territory{},
				Kinds: []Statute{
					BreachCodeFileMissingHeaderRegion,
					BreachCodeFileWrongHeaderRegionFormat,
					BreachCodeFileMissingContributors,
					BreachCodeFileMissingSummary,
					BreachCodeFileMissingLicense,
					BreachCodeFileWrongLicense,
					BreachCodeFileMissingRequirements,
					BreachCodeFileMissingDocs,
				},
			},
			{
				Name:        "Section",
				Description: "Section structure breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Groups: []Territory{
					{
						Name:        "Wrong Format",
						Description: "Section format breachs",
						Groups: []Territory{
							{
								Name:        "Summary",
								Description: "Section summary format breachs",
								Kinds: []Statute{
									BreachCodeSectionWrongFormatSummaryTooLong,
								},
							},
							{
								Name:        "Requirements",
								Description: "Section requirements format breachs",
								Kinds: []Statute{
									BreachCodeSectionWrongFormatRequirementsSplitBlock,
								},
							},
						},
						Kinds: []Statute{
							BreachCodeSectionWrongFormat,
							BreachCodeSectionWrongFormatNewlineAfterRegion,
							BreachCodeSectionWrongFormatDocs,
						},
					},
				},
				Kinds: []Statute{
					BreachCodeSectionEmpty,
					BreachCodeSectionOrphanDefinition,
					BreachCodeSectionMissingStartName,
					BreachCodeSectionMissingEndName,
					BreachCodeSectionNameMismatch,
					BreachCodeSectionMissingSummary,
					BreachCodeSectionMissingRequirements,
					BreachCodeSectionMissingDocs,
				},
			},
			{
				Name:        "Definition",
				Description: "Definition documentation breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Groups: []Territory{
					{
						Name:        "Wrong Format",
						Description: "Definition format breachs",
						Kinds: []Statute{
							BreachCodeDefWrongFormat,
							BreachCodeDefNotNativeDocstring,
						},
					},
				},
				Kinds: []Statute{
					BreachCodeDefMissingSummary,
					BreachCodeDefMissingRequirements,
					BreachCodeDefMissingDocs,
				},
			},
			{
				Name:        "Comment",
				Description: "Forbidden comment breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []Statute{
					BreachCodeCommentInline,
					BreachCodeCommentBlock,
					BreachCodeCommentJSDoc,
				},
			},
			{
				Name:        "Requirements",
				Description: "Specification content breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []Statute{
					BreachCodeRequirementsSyntax,
				},
			},
			{
				Name:        "Unicode",
				Description: "Unicode character breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []Statute{
					BreachCodeUnicodeEmojiVariation,
				},
			},
			{
				Name:        "Docs",
				Description: "Documentation file breachs",
				Scopes:      []string{"**/*.{ts,tsx,py,cs,go,rs}"},
				Kinds: []Statute{
					BreachCodeDocsMissingReadme,
				},
			},
			{
				Name:        "Rust",
				Description: "Rust-specific section breachs",
				Scopes:      []string{"**/*.rs"},
				Kinds: []Statute{
					BreachCodeRustRegionComment,
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
		Priority:    BreachPriorityLow,
		Groups: []Territory{
			{
				Name:        "File",
				Description: "File documentation breachs",
				Scopes:      []string{"AGENTS.md"},
				Kinds: []Statute{
					BreachDevDocsMissingFile,
					BreachDevDocsWrongFilePath,
					BreachDevDocsWrongFileName,
					BreachDevDocsWrongFileOrder,
				},
			},
			{
				Name:        "Folder",
				Description: "Folder documentation breachs",
				Scopes:      []string{"AGENTS.md"},
				Kinds: []Statute{
					BreachDevDocsMissingFolder,
					BreachDevDocsWrongFolderPath,
					BreachDevDocsWrongFolderName,
					BreachDevDocsWrongFolderOrder,
				},
			},
			{
				Name:        "Component",
				Description: "Component documentation breachs",
				Scopes:      []string{"README.md"},
				Kinds: []Statute{
					BreachDevDocsMissingComponent,
					BreachDevDocsWrongComponentName,
					BreachDevDocsWrongComponentOrder,
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
		Priority:    BreachPriorityHigh,
		Groups: []Territory{
			{
				Name:        "Import",
				Description: "Import restriction breachs",
				Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
				Kinds: []Statute{
					BreachSketchpadImportThirdParty,
				},
			},
			{
				Name:        "State",
				Description: "State management breachs",
				Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
				Kinds: []Statute{
					BreachSketchpadStateMultipleMachines,
					BreachSketchpadStateCreateActor,
					BreachSketchpadStateYjsAppState,
					BreachSketchpadStateForbiddenStore,
				},
			},
			{
				Name:        "Hooks",
				Description: "Hook pattern breachs",
				Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
				Kinds: []Statute{
					BreachSketchpadHooksNonTriadic,
				},
			},
		},
		Run: sketchpadPolicy,
	},
	{
		ID:          "repo",
		Name:        "Repo",
		Description: "Validates strict repo command implementation parity and ticket tracking",
		Scopes:      []string{"go/repo/main.go", "js/vscode/package.json", "graphql/repo/🔗️.graphql"},
		Priority:    BreachPriorityHigh,
		Groups: []Territory{
			{
				Name:        "Parity",
				Description: "Command parity breachs",
				Scopes:      []string{"go/repo/main.go", "js/vscode/package.json", "graphql/repo/🔗️.graphql"},
				Kinds: []Statute{
					BreachRepoMissingCommand,
					BreachRepoMissingTicketTracking,
				},
			},
		},
		Run: repoPolicy,
	},
	{
		ID:          "system",
		Name:        "System",
		Description: "Validates system configuration files like devcontainer and editor settings",
		Scopes:      []string{".vscode/settings.json", ".vscode/extensions.json", ".devcontainer/devcontainer.json", ".devcontainer/editor-overlay.json"},
		Priority:    BreachPriorityHigh,
		Groups: []Territory{
			{
				Name:        "Devcontainer",
				Description: "Devcontainer configuration breachs",
				Groups: []Territory{
					{
						Name:        "VSCode",
						Description: "The devcontainer editor block must equal its derivation from the host editor files",
						Kinds: []Statute{
							BreachSystemDevcontainerVscodeSettingsDrift,
							BreachSystemDevcontainerVscodeExtensionsDrift,
						},
					},
				},
			},
		},
		Run: systemPolicy,
	},
	{
		ID:          "folder",
		Name:        "Folder",
		Description: "Validates folder structure and detects illegal empty folders",
		Scopes:      []string{"**/*"},
		Priority:    BreachPriorityLow,
		Groups: []Territory{
			{
				Name:        "Name",
				Description: "Folder emoji identity breaches",
				Kinds: []Statute{
					BreachFolderNameMissingEmoji,
					BreachFolderNameGenericEmoji,
					BreachFolderNameNoncanonicalEmojiPresentation,
					BreachFolderNameWhitespaceAfterEmoji,
					BreachFolderNameEmojiNotUnique,
					BreachFolderNameMultipleEmojis,
				},
			},
			{
				Name:        "Illegal",
				Description: "Illegal folder breachs",
				Groups: []Territory{
					{
						Name:        "Empty",
						Description: "Empty folders that should be removed",
						Kinds: []Statute{
							BreachFolderIllegalEmpty,
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
		Priority:    BreachPriorityHigh,
		Groups: []Territory{
			{
				Name:        "Name",
				Description: "File emoji identity breaches",
				Kinds: []Statute{
					BreachFileNameMissingEmoji,
					BreachFileNameGenericEmoji,
					BreachFileNameNoncanonicalEmojiPresentation,
					BreachFileNameWhitespaceAfterEmoji,
					BreachFileNameEmojiNotUnique,
					BreachFileNameMultipleEmojis,
					BreachFileNameReservedEmoji,
				},
			},
			{
				Name:        "Illegal",
				Description: "Illegal file breachs",
				Groups: []Territory{
					{
						Name:        "Use Godfile",
						Description: "Files not listed in .🧬semio/🦑️repo/📁️files.json godfile",
						Kinds: []Statute{
							BreachFileIllegalUseGodfile,
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
		Priority:    BreachPriorityHigh,
		Groups: []Territory{
			{
				Name:        "Import",
				Description: "Import restriction breachs",
				Groups: []Territory{
					{
						Name:        "No Ui Dependency",
						Description: "compose.* files must not import from UI dependencies",
						Kinds: []Statute{
							BreachComposeNoUiDependency,
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
		Priority:    BreachPriorityHigh,
		Groups: []Territory{
			{
				Name:        "Import",
				Description: "Direct third-party import breachs",
				Scopes:      []string{"**/*"},
				Kinds: []Statute{
					BreachDependencyBoundaryDirectImport,
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
func StreamPolicies(ctx context.Context, out chan<- PolicyDef, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	for _, p := range policies {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(p.ID, options) && !matchesFilter(p.Name, options) {
				continue
			}
			if !matchesQuery(p.ID+" "+p.Name+" "+p.Description, options) {
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
