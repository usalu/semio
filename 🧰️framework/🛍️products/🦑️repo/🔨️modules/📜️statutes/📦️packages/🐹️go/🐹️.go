// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📜️statutes is the statutes domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package statutes

import (
	bytes "bytes"
	gzip "compress/gzip"
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	fmt "fmt"
	io "io"
	fs "io/fs"
	rand "math/rand"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strings "strings"
	unicode "unicode"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

// #region 🎽️Languages

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

// #endregion 🎽️Languages

// #region 📦️Utils

// 🔧️runFormatterAfterAutofix holds the data fields for a runFormatterAfterAutofix record.
func RunFormatterAfterAutofix(relPath string, language languages.LanguagePlugin) error {
	absPath := filepath.Join(workspace.RootDir, relPath)
	langName := ""
	if language != nil {
		langName = language.Name()
	}
	plans := workspace.FormatterPlansForLanguage(langName, relPath)
	for _, plan := range plans {
		if !workspace.IsFormatterPlanAvailable(plan, workspace.RootDir) {
			continue
		}
		if err := workspace.FormatterCommandRun(plan.Binary, plan.Args, workspace.RootDir); err == nil {
			return nil
		}
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return err
	}
	formatted := workspace.FallbackFormatText(content)
	if formatted == content {
		return nil
	}
	return workspace.WriteTextFile(absPath, formatted)
}

// ⬛️runFormatterForFile runs the language-appropriate formatter on the given file.
// 🖊️Unlike runFormatterAfterAutofix, this only runs the formatter with no fallback.
func RunFormatterForFile(relPath string) {
	lang := languages.GetLanguage(relPath)
	langName := ""
	if lang != nil {
		langName = lang.Name()
	}
	plans := workspace.FormatterPlansForLanguage(langName, relPath)
	for _, plan := range plans {
		if !workspace.IsFormatterPlanAvailable(plan, workspace.RootDir) {
			continue
		}
		if err := workspace.FormatterCommandRun(plan.Binary, plan.Args, workspace.RootDir); err == nil {
			return
		}
	}
}

// #endregion 📦️Utils

// #region Policies

type PolicyFunc func(ctx *PolicyContext) []model.Breach

// ­ƒÆ┐️policies holds the data fields for a policies record.
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
		Description: "Validates system configuration files like devcontainer and editor settings",
		Scopes:      []string{".vscode/settings.json", ".vscode/extensions.json", ".devcontainer/devcontainer.json"},
		Priority:    model.BreachPriorityHigh,
		Groups: []model.Territory{
			{
				Name:        "Devcontainer",
				Description: "Devcontainer configuration breachs",
				Groups: []model.Territory{
					{
						Name:        "VSCode",
						Description: "VSCode settings and extensions must be inside devcontainer.json",
						Kinds: []model.Statute{
							model.BreachSystemDevcontainerVscodeSettingsOutside,
							model.BreachSystemDevcontainerVscodeExtensionsOutside,
						},
					},
				},
			},
		},
		Run: SystemPolicy,
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

// ­ƒÄ»FindPolicy MUST return nil when no match is found.
// ­ƒöÄFindPolicy searches for and returns the matching policy.
func FindPolicy(id string) (PolicyDef, bool) {
	for _, p := range policies {
		if p.ID == id {
			return p, true
		}
	}
	return PolicyDef{}, false
}

// ­ƒÅ¬GetPolicies MUST return the stored value without modification.
// ­ƒôªGetPolicies returns the policies of the value.
func GetPolicies() []PolicyDef {
	return policies
}

// ­ƒô¬StreamPolicies MUST emit all matching entries and close the channel when done.
// ­ƒöìStreamPolicies streams the policies over a channel with optional filtering.
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

// ­ƒôØPolicyContext holds the data fields for a policy context record.
type PolicyContext struct {
	Scope              workspace.Scope
	RootDir            string
	Bundles            []model.Bundle
	fileCache          map[string]string
	sectionCache       map[string][]model.Section
	ignoreCache        map[string]map[int][]string
	specLineCache      map[string]map[int]bool
	sectionDocCache    map[string]map[int]bool
	definitionDocCache map[string]map[int]bool
	filesOverride      []string
	sources            map[string]string
}

// ­ƒöÀNewPolicyContext MUST initialize all required fields and return a valid PolicyContext.
// ­ƒåòNewPolicyContext creates and returns a new PolicyContext instance.
func NewPolicyContext(scope workspace.Scope, bundles []model.Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      workspace.RootDir,
		Bundles:      bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]model.Section),
		ignoreCache:  make(map[string]map[int][]string),
	}
}

// ­ƒôäNewPolicyContextWithFiles MUST initialize all required fields and return a valid PolicyContextWithFiles.
// ­ƒôäNewPolicyContextWithFiles creates and returns a new PolicyContextWithFiles instance.
func NewPolicyContextWithFiles(scope workspace.Scope, bundles []model.Bundle, files []string) *PolicyContext {
	ctx := NewPolicyContext(scope, bundles)
	ctx.filesOverride = files
	return ctx
}

// ­ƒôÑFiles MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) Files() ([]string, error) {
	if ctx.filesOverride != nil {
		return ctx.filesOverride, nil
	}
	files, err := model.ScopeToFiles(ctx.Scope, ctx.Bundles)
	if err != nil {
		return nil, err
	}
	ctx.filesOverride = files
	return files, nil
}

// ­ƒøñ´©ÅReadText MUST return the full content from the given path.
// ­ƒöñReadText reads and returns the text content.
func (ctx *PolicyContext) ReadText(filePath string) string {
	if ctx.fileCache == nil {
		ctx.fileCache = make(map[string]string)
	}
	if ctx.sources != nil {
		return ctx.sources[filePath]
	}
	absPath := filepath.Join(workspace.RootDir, filePath)
	if content, ok := ctx.fileCache[absPath]; ok {
		return content
	}
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		ctx.fileCache[absPath] = ""
		return ""
	}
	ctx.fileCache[absPath] = content
	return content
}

// ­ƒôæSections MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) Sections(filePath string) []model.Section {
	if ctx.sectionCache == nil {
		ctx.sectionCache = make(map[string][]model.Section)
	}
	if sections, ok := ctx.sectionCache[filePath]; ok {
		return sections
	}
	content := ctx.ReadText(filePath)
	sections := languages.ParseSections(content, filePath)
	ctx.sectionCache[filePath] = sections
	return sections
}

// ÔØîParseIgnoreDirectives MUST return an error when the input is malformed.
// ­ƒÆ¥ParseIgnoreDirectives parses the input and returns the ignore directives result.
func ParseIgnoreDirectives(content string) map[int][]string {
	result := make(map[int][]string)
	lines := strings.Split(content, "\n")
	ignorePrefix := "// compose-ignore-"
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, ignorePrefix) {
			pattern := strings.TrimPrefix(trimmed, ignorePrefix)

			patterns := strings.Split(pattern, ",")
			for _, p := range patterns {
				p = strings.TrimSpace(p)
				if p != "" {
					result[i+1] = append(result[i+1], p)
				}
			}
		}
	}
	return result
}

// ­ƒöÂIgnoreDirectives MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) IgnoreDirectives(filePath string) map[int][]string {
	if ignores, ok := ctx.ignoreCache[filePath]; ok {
		return ignores
	}
	content := ctx.ReadText(filePath)
	ignores := ParseIgnoreDirectives(content)
	ctx.ignoreCache[filePath] = ignores
	return ignores
}

// ­ƒö╣️IsIgnored MUST return true only when the condition is met.
// ÔØôIsIgnored reports whether the PolicyContext is ignored.
func (ctx *PolicyContext) IsIgnored(filePath string, breachLine int, kind model.Statute) bool {
	return IsIgnored(ctx.IgnoreDirectives(filePath), breachLine, kind)
}

// 🙈️IsIgnored MUST report true only when a directive above the line names a prefix of the statute.
// 🙈️IsIgnored decides suppression from already parsed directives, without touching the filesystem.
func IsIgnored(directives map[int][]string, breachLine int, kind model.Statute) bool {
	kindStr := string(kind)
	for ignoreLine, patterns := range directives {
		if breachLine > ignoreLine && breachLine <= ignoreLine+100 {
			for _, pattern := range patterns {
				if strings.HasPrefix(kindStr, pattern) {
					return true
				}
			}
		}
	}
	return false
}

// ­ƒåòCreateBreach MUST persist the new entity and return a reference to it.
// ÔÜá´©ÅCreateBreach creates a new breach and persists it.
func (ctx *PolicyContext) CreateBreach(summary string, kind model.Statute, scope string, line int, col int, excerpt string) model.Breach {
	return model.Breach{
		ID:      model.BuildBreachID(scope, line, col),
		Summary: summary,
		Kind:    kind,
		Scope:   scope,
		Line:    line,
		Column:  col,
		Excerpt: excerpt,
	}
}

// ­ƒº▓️extractFileFromScope holds the data fields for a extractFileFromScope record.
func extractFileFromScope(scope string) string {

	if idx := strings.Index(scope, "#"); idx != -1 {
		scope = scope[:idx]
	}

	if idx := strings.Index(scope, "::"); idx != -1 {
		scope = scope[:idx]
	}
	return scope
}

// ­ƒº╣️FilterIgnored MUST preserve the tree structure while removing non-matching nodes.
// ­ƒöÀFilterIgnored filters the ignored based on the given criteria.
func (ctx *PolicyContext) FilterIgnored(breachs []model.Breach) []model.Breach {
	var result []model.Breach
	for _, v := range breachs {
		file := extractFileFromScope(v.Scope)
		if !ctx.IsIgnored(file, v.Line, v.Kind) {
			result = append(result, v)
		}
	}
	return result
}

// ­ƒº®specKeywordPattern holds the data fields for a specKeywordPattern record.
var specKeywordPattern = regexp.MustCompile(`\b(MUST(\s+NOT)?|SHOULD(\s+NOT)?|SHALL(\s+NOT)?|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b`)

// ­ƒö©isSpecText holds the data fields for a isSpecText record.
func IsSpecText(text string) bool {
	return specKeywordPattern.MatchString(text)
}

// ­ƒôÉspecImplSyntaxBacktick holds the data fields for a specImplSyntaxBacktick record.
var specImplSyntaxBacktick = regexp.MustCompile("`[^`]+`")

// ÔÜíspecImplSyntaxFuncCall holds the data fields for a specImplSyntaxFuncCall record.
var specImplSyntaxFuncCall = regexp.MustCompile(`[A-Za-z_]\w*\.\w+\(|[A-Z]\w+\(`)

func hasImplementationSyntax(text string) (bool, string) {
	if specImplSyntaxBacktick.MatchString(text) {
		return true, "backtick-wrapped code"
	}
	if specImplSyntaxFuncCall.MatchString(text) {
		match := specImplSyntaxFuncCall.FindString(text)
		if match != "MUST(" && match != "SHOULD(" && match != "SHALL(" && match != "MAY(" {
			return true, "function/method call: " + match
		}
	}
	return false, ""
}

// ­ƒö║️SpecLines MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) SpecLines(filePath string) map[int]bool {
	if ctx.specLineCache == nil {
		ctx.specLineCache = make(map[string]map[int]bool)
	}
	if cached, ok := ctx.specLineCache[filePath]; ok {
		return cached
	}
	result := make(map[int]bool)
	content := ctx.ReadText(filePath)
	if content == "" {
		ctx.specLineCache[filePath] = result
		return result
	}
	language := languages.GetLanguage(filePath)
	if language == nil {
		ctx.specLineCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	sections := ctx.Sections(filePath)
	prefix := language.CommentPrefix()
	var markSectionSpecLines func(s model.Section)
	markSectionSpecLines = func(s model.Section) {
		if strings.ToLower(s.Name) == "header" {
			for _, child := range s.Children {
				markSectionSpecLines(child)
			}
			return
		}
		inLeadBlock := true
		for i := s.StartLine + 1; i < s.EndLine && i <= len(lines); i++ {
			line := strings.TrimSpace(lines[i-1])
			if line == "" {
				continue
			}
			isComment := strings.HasPrefix(line, prefix)
			if isComment && inLeadBlock {
				commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
				if IsSpecText(commentText) {
					result[i] = true
				}
			} else if !isComment {
				inLeadBlock = false
			}
			if matched, _ := language.PolicySectionStartMatch(lines[i-1]); matched {
				break
			}
		}
		for _, child := range s.Children {
			markSectionSpecLines(child)
		}
	}
	for _, s := range sections {
		markSectionSpecLines(s)
	}
	ctx.specLineCache[filePath] = result
	return result
}

// ­ƒö╗️IsSpecLine MUST return true only when the condition is met.
// ­ƒÉÖIsSpecLine reports whether the PolicyContext is spec line.
func (ctx *PolicyContext) IsSpecLine(filePath string, lineNum int) bool {
	return ctx.SpecLines(filePath)[lineNum]
}

// Ô¼øIsSpecBlock MUST return true only when the condition is met.
// ­ƒöÆIsSpecBlock reports whether the PolicyContext is spec block.
func (ctx *PolicyContext) IsSpecBlock(filePath string, startLine, endLine int, lines []string) bool {
	for i := startLine; i <= endLine && i <= len(lines); i++ {
		if IsSpecText(lines[i-1]) {
			return true
		}
	}
	return false
}

// Ô¼£SectionDocLines MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) SectionDocLines(filePath string) map[int]bool {
	if ctx.sectionDocCache == nil {
		ctx.sectionDocCache = make(map[string]map[int]bool)
	}
	if cached, ok := ctx.sectionDocCache[filePath]; ok {
		return cached
	}
	result := make(map[int]bool)
	content := ctx.ReadText(filePath)
	if content == "" {
		ctx.sectionDocCache[filePath] = result
		return result
	}
	language := languages.GetLanguage(filePath)
	if language == nil {
		ctx.sectionDocCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	sections := ctx.Sections(filePath)
	prefix := language.CommentPrefix()
	var markSectionDocLines func(s model.Section)
	markSectionDocLines = func(s model.Section) {
		if strings.ToLower(s.Name) == "header" {
			for _, child := range s.Children {
				markSectionDocLines(child)
			}
			return
		}
		var blockLines []int
		foundFirstComment := false
		for i := s.StartLine + 1; i < s.EndLine && i <= len(lines); i++ {
			line := strings.TrimSpace(lines[i-1])
			if line == "" {
				if !foundFirstComment {
					continue
				}
				break
			}
			if !strings.HasPrefix(line, prefix) {
				break
			}
			foundFirstComment = true
			blockLines = append(blockLines, i)
			commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
			if IsSpecText(commentText) {
			}
		}
		for _, ln := range blockLines {
			result[ln] = true
		}
		for _, child := range s.Children {
			markSectionDocLines(child)
		}
	}
	for _, s := range sections {
		markSectionDocLines(s)
	}
	if language.SupportsDefinitions() {
		parsedDefs := language.ParseDefinitions(content, lines)
		for _, def := range parsedDefs {
			for lineIndex := def.Start - 2; lineIndex >= 0; lineIndex-- {
				line := strings.TrimSpace(lines[lineIndex])
				if line == "" {
					break
				}
				if !strings.HasPrefix(line, prefix) {
					break
				}
				result[lineIndex+1] = true
			}
		}
	}
	ctx.sectionDocCache[filePath] = result
	return result
}

// ­ƒƒÑIsSectionDocLine MUST return true only when the condition is met.
// ­ƒö╣️IsSectionDocLine reports whether the PolicyContext is section doc line.
func (ctx *PolicyContext) IsSectionDocLine(filePath string, lineNum int) bool {
	return ctx.SectionDocLines(filePath)[lineNum]
}

// ­ƒôûDefinitionDocLines MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) DefinitionDocLines(filePath string) map[int]bool {
	if ctx.definitionDocCache == nil {
		ctx.definitionDocCache = make(map[string]map[int]bool)
	}
	if cached, ok := ctx.definitionDocCache[filePath]; ok {
		return cached
	}
	result := make(map[int]bool)
	content := ctx.ReadText(filePath)
	if content == "" {
		ctx.definitionDocCache[filePath] = result
		return result
	}
	language := languages.GetLanguage(filePath)
	if language == nil {
		ctx.definitionDocCache[filePath] = result
		return result
	}
	if !language.SupportsDefinitions() {
		ctx.definitionDocCache[filePath] = result
		return result
	}
	lines := strings.Split(content, "\n")
	prefix := language.CommentPrefix()
	defs := language.ParseDefinitions(content, lines)
	extras := language.ExtraOrphanDefinitions(lines)
	allDefs := append(defs, extras...)
	for _, d := range allDefs {
		for lineIndex := d.Start - 2; lineIndex >= 0; lineIndex-- {
			line := strings.TrimSpace(lines[lineIndex])
			if line == "" {
				break
			}
			if strings.HasPrefix(line, prefix) {
				result[lineIndex+1] = true
				continue
			}
			if language.Name() == "typescript" {
				if strings.HasPrefix(line, "/*") && strings.HasSuffix(line, "*/") && !strings.HasPrefix(line, "/**") {
					break
				}
				if strings.HasSuffix(line, "**/") || strings.HasSuffix(line, "*/") || strings.HasPrefix(line, "* ") || line == "*" || strings.HasPrefix(line, "/**") {
					result[lineIndex+1] = true
					if strings.HasPrefix(line, "/**") {
						break
					}
					continue
				}
			}
			if (language.Name() == "csharp" || language.Name() == "rust") && strings.HasPrefix(line, "///") {
				result[lineIndex+1] = true
				continue
			}
			break
		}
		if language.Name() == "python" {
			parenDepth := 0
			for _, ch := range lines[d.Start-1] {
				if ch == '(' {
					parenDepth++
				}
				if ch == ')' {
					parenDepth--
				}
			}
			bodyStart := d.Start
			if parenDepth > 0 {
				for scanIdx := d.Start; scanIdx < len(lines) && scanIdx < d.Start+15; scanIdx++ {
					for _, ch := range lines[scanIdx] {
						if ch == '(' {
							parenDepth++
						}
						if ch == ')' {
							parenDepth--
						}
					}
					if parenDepth <= 0 {
						bodyStart = scanIdx + 1
						break
					}
				}
			}
			for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
				trimmed := strings.TrimSpace(lines[bodyIdx])
				if trimmed == "" {
					continue
				}
				if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
					quote := `"""`
					if strings.HasPrefix(trimmed, `'''`) {
						quote = `'''`
					}
					result[bodyIdx+1] = true
					afterOpen := strings.TrimPrefix(trimmed, quote)
					if strings.Index(afterOpen, quote) < 0 {
						for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
							sline := strings.TrimSpace(lines[scanIdx])
							result[scanIdx+1] = true
							if sline == quote || strings.HasSuffix(sline, quote) {
								break
							}
						}
					}
				}
				break
			}
		}
	}
	ctx.definitionDocCache[filePath] = result
	return result
}

// ­ƒƒºIsDefinitionDocLine MUST return true only when the condition is met.
// ÔûÂ´©ÅIsDefinitionDocLine reports whether the PolicyContext is definition doc line.
func (ctx *PolicyContext) IsDefinitionDocLine(filePath string, lineNum int) bool {
	return ctx.DefinitionDocLines(filePath)[lineNum]
}

func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

// Ô£ö´©ÅCheckPolicies MUST run all applicable policies and aggregate breachs.
// Ô£ö´©ÅCheckPolicies validates the policies and returns any breachs.
func CheckPolicies(scope workspace.Scope, bundles []model.Bundle, policyIDs []string) ([]model.Breach, error) {
	ctx := NewPolicyContext(scope, bundles)
	return CheckPoliciesWithContext(ctx, policyIDs)
}

// ­ƒƒ¿CheckPoliciesWithContext MUST run all applicable policies and aggregate breachs.
// ­ƒöæCheckPoliciesWithContext validates the policies with context and returns any breachs.
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

// ­ƒö¡matchesScope holds the data fields for a matchesScope record.
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

// ­ƒƒ®headerPolicy holds the data fields for a headerPolicy record.
func headerPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	agplMarkers := []string{"GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		sections := ctx.Sections(file)
		var headerSection *model.Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection == nil {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing header region in %s", file),
				model.BreachCodeFileMissingHeaderRegion,
				file, 0, 0, ""))
			continue
		}
		headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
		headerLines := strings.Split(headerContent, "\n")
		contributorPattern := regexp.MustCompile(`\d{4}\s+[\w\s]+<[\w.@-]+>`)
		hasContributors := false
		for _, line := range headerLines {
			if contributorPattern.MatchString(line) {
				hasContributors = true
				break
			}
		}
		if !hasContributors {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing contributors in header of %s", file),
				model.BreachCodeFileMissingContributors,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		}
		hasLicense := false
		for _, marker := range agplMarkers {
			if strings.Contains(headerContent, marker) {
				hasLicense = true
				break
			}
		}
		if !hasLicense {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing license in header of %s", file),
				model.BreachCodeFileMissingLicense,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		} else {
			wrongLicenses := []string{"MIT", "Apache", "BSD"}
			hasWrongLicense := false
			for _, wrong := range wrongLicenses {
				if strings.Contains(headerContent, wrong) {
					hasWrongLicense = true
					break
				}
			}
			if strings.Contains(headerContent, "GPL") && !strings.Contains(headerContent, "AGPL") && !strings.Contains(headerContent, "LGPL") {
				hasWrongLicense = true
			}
			if hasWrongLicense {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Wrong license in header of %s", file),
					model.BreachCodeFileWrongLicense,
					fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
			}
		}
		commentPrefix := language.CommentPrefix()
		hasSummary := false
		seenLicense := false
		licenseEnd := false
		for _, line := range headerLines {
			trimmed := strings.TrimSpace(line)
			if trimmed == "" {
				if seenLicense && !licenseEnd {
					licenseEnd = true
				}
				continue
			}
			if strings.HasPrefix(trimmed, commentPrefix+" #region") || strings.HasPrefix(trimmed, commentPrefix+" #endregion") {
				continue
			}
			if !strings.HasPrefix(trimmed, commentPrefix) {
				continue
			}
			commentText := strings.TrimSpace(strings.TrimPrefix(trimmed, commentPrefix))
			if commentText == "" {
				if seenLicense && !licenseEnd {
					licenseEnd = true
				}
				continue
			}

			if contributorPattern.MatchString(line) {
				continue
			}
			isLicenseLine := false
			for _, marker := range agplMarkers {
				if strings.Contains(line, marker) {
					seenLicense = true
					isLicenseLine = true
					break
				}
			}
			if isLicenseLine {
				continue
			}
			if !licenseEnd && seenLicense && (strings.Contains(commentText, "without") || strings.Contains(commentText, "program") || strings.Contains(commentText, "License") || strings.Contains(commentText, "http") || strings.Contains(commentText, "version") || strings.Contains(commentText, "terms") || strings.Contains(commentText, "WARRANTY") || strings.Contains(commentText, "PURPOSE") || strings.Contains(commentText, "General Public") || strings.Contains(commentText, "redistribute") || strings.Contains(commentText, "published") || strings.Contains(commentText, "Foundation") || strings.Contains(commentText, "received")) {
				continue
			}
			if IsSpecText(commentText) {
				continue
			}
			if strings.HasPrefix(commentText, "TODO:") {
				continue
			}
			hasSummary = true
			break
		}
		if !hasSummary && !IsTestOrBenchmarkFile(file) {
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Missing summary in header of %s", file),
				model.BreachCodeFileMissingSummary,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}

// 🧪️isTestOrBenchmarkFile reports whether a path conventionally contains tests or benchmarks.
func IsTestOrBenchmarkFile(file string) bool {
	if model.DeriveFileKind(filepath.Base(file)) == model.FileKindLab {
		return true
	}
	normalized := strings.ToLower(filepath.ToSlash(file))
	if strings.Contains(normalized, "/tests/") || strings.Contains(normalized, ".tests/") ||
		strings.Contains(normalized, "/test/") || strings.Contains(normalized, ".test/") ||
		strings.Contains(normalized, "/benchmark/") || strings.Contains(normalized, ".benchmark/") {
		return true
	}
	name := filepath.Base(normalized)
	return strings.HasSuffix(name, "_test.go") ||
		strings.HasSuffix(name, ".test.ts") ||
		strings.HasSuffix(name, ".test.tsx") ||
		strings.HasSuffix(name, ".test.js") ||
		strings.HasSuffix(name, ".test.jsx") ||
		strings.HasSuffix(name, ".spec.ts") ||
		strings.HasSuffix(name, ".spec.tsx") ||
		strings.HasSuffix(name, ".spec.js") ||
		strings.HasSuffix(name, ".spec.jsx") ||
		strings.HasPrefix(name, "test_") ||
		strings.Contains(name, "benchmark")
}

// ­ƒôñisExportedDefinition holds the data fields for a isExportedDefinition record.
func isExportedDefinition(name string, line string, langName string) bool {
	return true
}

// 🧿definitionModifierWords are the words a TypeScript definition head may carry before `function` or `class`.
var definitionModifierWords = []string{"async ", "abstract ", "declare ", "default "}

// ✂️stripDefinitionModifierWords strips leading modifier words as whole space-separated prefixes, never as a character cutset.
func stripDefinitionModifierWords(line string) string {
	for stripped := true; stripped; {
		stripped = false
		for _, word := range definitionModifierWords {
			if strings.HasPrefix(line, word) {
				line = line[len(word):]
				stripped = true
			}
		}
	}
	return line
}

// ­ƒƒªrequiresDefinitionRequirements holds the data fields for a requiresDefinitionRequirements record.
func requiresDefinitionRequirements(line string, langName string) bool {
	trimmed := strings.TrimSpace(line)
	switch langName {
	case "typescript":
		noExport := strings.TrimPrefix(trimmed, "export ")
		noExport = stripDefinitionModifierWords(noExport)
		return strings.HasPrefix(noExport, "function ") || strings.HasPrefix(noExport, "class ")
	case "go":
		return strings.HasPrefix(trimmed, "func ")
	case "python":
		return strings.HasPrefix(trimmed, "def ") || strings.HasPrefix(trimmed, "class ") || strings.HasPrefix(trimmed, "async def ")
	case "csharp":
		return !strings.Contains(trimmed, " interface ") && !strings.Contains(trimmed, " enum ")
	case "rust":
		noPrefix := strings.TrimPrefix(trimmed, "pub ")
		if strings.HasPrefix(noPrefix, "(") {
			idx := strings.Index(noPrefix, ") ")
			if idx >= 0 {
				noPrefix = strings.TrimSpace(noPrefix[idx+2:])
			}
		}
		return strings.HasPrefix(noPrefix, "fn ") || strings.HasPrefix(noPrefix, "struct ") || strings.HasPrefix(noPrefix, "impl ") || strings.HasPrefix(noPrefix, "trait ")
	}
	return true
}

// ­ƒƒ¬sectionPolicy holds the data fields for a sectionPolicy record.
func sectionPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsSections() {
			continue
		}
		lines := strings.Split(content, "\n")
		type stackItem struct {
			name string
			line int
		}
		var stack []stackItem
		for i, line := range lines {
			lineNum := i + 1
			line = strings.TrimSuffix(line, "\r")
			if matched, name := language.PolicySectionStartMatch(line); matched {
				if name == "" {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Missing section name at %s:%d", file, lineNum),
						model.BreachCodeSectionMissingStartName,
						file, lineNum, 0, strings.TrimSpace(line)))
				}
				stack = append(stack, stackItem{name: name, line: lineNum})
				continue
			}
			if matched, endName := language.PolicySectionEndMatch(line); matched {
				if len(stack) > 0 {
					open := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if open.name != "" {
						if endName == "" {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Missing end section name at %s:%d", file, lineNum),
								model.BreachCodeSectionMissingEndName,
								file, lineNum, 0, strings.TrimSpace(line)))
						} else if endName != open.name {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								model.BreachCodeSectionNameMismatch,
								file, lineNum, 0, fmt.Sprintf("Start: \"%s\" at line %d, End: \"%s\"", open.name, open.line, endName)))
						}
					}
				}
			}
		}
		sections := ctx.Sections(file)
		commentPrefix := language.CommentPrefix()
		var checkSection func(s model.Section, parentPath string)
		checkSection = func(s model.Section, parentPath string) {
			sectionPath := s.Name
			if parentPath != "" {
				sectionPath = parentPath + "#" + s.Name
			}
			sectionContent := content[s.StartIndex:s.EndIndex]
			sectionLines := strings.Split(sectionContent, "\n")
			nonEmpty := 0
			for _, line := range sectionLines[1 : len(sectionLines)-1] {
				trimmed := strings.TrimSpace(line)
				if trimmed != "" && !strings.HasPrefix(trimmed, "//") && !strings.HasPrefix(trimmed, "#") {
					nonEmpty++
				}
			}
			isExempt := s.Name == "Header"
			if nonEmpty == 0 && len(s.Children) == 0 && !isExempt {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Empty section \"%s\" in %s", s.Name, file),
					model.BreachCodeSectionEmpty,
					fmt.Sprintf("%s#%s", file, sectionPath), s.StartLine, 0, ""))
			}
			if s.StartLine < len(lines) && strings.TrimSpace(lines[s.StartLine]) == "" {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Blank line after region start marker in section \"%s\" in %s:%d", s.Name, file, s.StartLine+1),
					model.BreachCodeSectionWrongFormatNewlineAfterRegion,
					fmt.Sprintf("%s#%s", file, sectionPath), s.StartLine+1, 0, ""))
			}
			if !isExempt && s.Name != "" && !IsTestOrBenchmarkFile(file) {
				hasSummary := false
				for i := 1; i < len(sectionLines)-1; i++ {
					line := strings.TrimSpace(sectionLines[i])
					if line == "" {
						continue
					}
					if matched, _ := language.PolicySectionStartMatch(line); matched {
						break
					}
					if matched, _ := language.PolicySectionEndMatch(line); matched {
						break
					}
					if !strings.HasPrefix(line, commentPrefix) {
						break
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, commentPrefix))
					if commentText == "" {
						continue
					}
					hasSummary = true
				}
				if !hasSummary {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Section \"%s\" is missing a summary comment in %s", s.Name, file),
						model.BreachCodeSectionMissingSummary,
						fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, 0, ""))
				}
			}
			for _, child := range s.Children {
				checkSection(child, sectionPath)
			}
		}
		for _, s := range sections {
			checkSection(s, "")
		}
		covered := make([]bool, len(lines))
		var markCovered func(s model.Section)
		markCovered = func(s model.Section) {
			start := s.StartLine
			if start < 1 {
				start = 1
			}
			end := s.EndLine
			if end < start {
				end = start
			}
			if end > len(lines) {
				end = len(lines)
			}
			for lineIndex := start; lineIndex <= end; lineIndex++ {
				covered[lineIndex-1] = true
			}
			for _, child := range s.Children {
				markCovered(child)
			}
		}
		for _, s := range sections {
			markCovered(s)
		}
		type lineRange struct {
			start int
			end   int
		}
		type defRange struct {
			name  string
			kind  string
			start int
			end   int
		}
		type orphanRangeInfo struct {
			start          int
			end            int
			firstLine      string
			isCommentBlock bool
		}
		orphanLines := make([]bool, len(lines))
		for i, line := range lines {
			if covered[i] {
				continue
			}
			line = strings.TrimSuffix(line, "\r")
			if strings.TrimSpace(line) == "" {
				continue
			}
			if i == 0 && strings.HasPrefix(strings.TrimSpace(line), "#!") {
				continue
			}
			if startMatched, _ := language.PolicySectionStartMatch(line); startMatched {
				continue
			}
			if endMatched, _ := language.PolicySectionEndMatch(line); endMatched {
				continue
			}
			orphanLines[i] = true
		}
		var orphanRanges []lineRange
		inOrphan := false
		startLine := 0
		for i := 0; i < len(orphanLines); i++ {
			if orphanLines[i] {
				if !inOrphan {
					inOrphan = true
					startLine = i + 1
				}
			} else if inOrphan {
				orphanRanges = append(orphanRanges, lineRange{start: startLine, end: i})
				inOrphan = false
			}
		}
		if inOrphan {
			orphanRanges = append(orphanRanges, lineRange{start: startLine, end: len(lines)})
		}
		var defRanges []defRange
		var realDefRanges []defRange
		defExcerpts := make(map[string]string)
		if language.SupportsDefinitions() {
			parsedDefs := language.ParseDefinitions(content, lines)
			for _, def := range parsedDefs {
				defRanges = append(defRanges, defRange{name: def.Name, kind: def.Kind, start: def.Start, end: def.End})
				realDefRanges = append(realDefRanges, defRange{name: def.Name, kind: def.Kind, start: def.Start, end: def.End})
				defExcerpts[def.Name] = def.Excerpt
			}
		}
		extraDefs := language.ExtraOrphanDefinitions(lines)
		for _, def := range extraDefs {
			defRanges = append(defRanges, defRange{name: def.Name, start: def.Start, end: def.End})
			defExcerpts[def.Name] = def.Excerpt
		}
		var orphanInfos []orphanRangeInfo
		for _, orphanRange := range orphanRanges {
			firstLine := ""
			isCommentBlock := true
			for lineIndex := orphanRange.start; lineIndex <= orphanRange.end; lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex-1], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				if firstLine == "" {
					firstLine = strings.TrimSpace(line)
				}
				if !strings.HasPrefix(strings.TrimSpace(line), commentPrefix) {
					isCommentBlock = false
				}
			}
			orphanInfos = append(orphanInfos, orphanRangeInfo{
				start:          orphanRange.start,
				end:            orphanRange.end,
				firstLine:      firstLine,
				isCommentBlock: isCommentBlock,
			})
			if isCommentBlock {
				name := fmt.Sprintf("comment-block-%d", orphanRange.start)
				defRanges = append(defRanges, defRange{name: name, start: orphanRange.start, end: orphanRange.end})
				defExcerpts[name] = firstLine
			}
		}
		reportedDefs := make(map[string]bool)
		skipOrphans := IsTestOrBenchmarkFile(file)
		for _, orphanRange := range orphanInfos {
			if skipOrphans {
				continue
			}
			matched := false
			for _, defRange := range defRanges {
				if orphanRange.start <= defRange.end && orphanRange.end >= defRange.start {
					if !reportedDefs[defRange.name] {
						reportedDefs[defRange.name] = true
						excerpt := defRange.name
						if value, ok := defExcerpts[defRange.name]; ok && value != "" {
							excerpt = value
						}
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Orphan definition outside sections at %s:%d", file, defRange.start),
							model.BreachCodeSectionOrphanDefinition,
							fmt.Sprintf("%s::%s", file, defRange.name),
							defRange.start, 0,
							excerpt))
					}
					matched = true
				}
			}
			if matched {
				continue
			}
			name := fmt.Sprintf("orphan-block-%d", orphanRange.start)
			breachs = append(breachs, ctx.CreateBreach(
				fmt.Sprintf("Orphan definition outside sections at %s:%d", file, orphanRange.start),
				model.BreachCodeSectionOrphanDefinition,
				fmt.Sprintf("%s::%s", file, name),
				orphanRange.start, 0,
				orphanRange.firstLine))
		}
		var findSectionPathForLine func(secs []model.Section, lineNum int) string
		findSectionPathForLine = func(secs []model.Section, lineNum int) string {
			for _, s := range secs {
				if lineNum < s.StartLine || lineNum > s.EndLine {
					continue
				}
				childPath := findSectionPathForLine(s.Children, lineNum)
				if childPath != "" {
					return s.Name + "#" + childPath
				}
				return s.Name
			}
			return ""
		}
		for _, def := range realDefRanges {
			isTestFile := IsTestOrBenchmarkFile(file)
			if isTestFile {
				break
			}
			defLine := ""
			if def.start-1 >= 0 && def.start-1 < len(lines) {
				defLine = lines[def.start-1]
			}
			if !isExportedDefinition(def.name, defLine, language.Name()) {
				continue
			}
			hasSummary := false
			hasRequirements := false
			isNativeDocstring := false
			langName := language.Name()
			if langName == "typescript" {
				prevIdx := def.start - 2
				if prevIdx >= 0 {
					prevLine := strings.TrimSpace(lines[prevIdx])
					if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
						isNativeDocstring = true
						for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
							sline := strings.TrimSpace(lines[scanIdx])
							isOpenLine := strings.HasPrefix(sline, "/**")
							content := sline
							if isOpenLine {
								content = strings.TrimPrefix(content, "/**")
							} else if strings.HasPrefix(content, "* ") {
								content = content[2:]
							} else if content == "*" || content == "*/" || content == "**/" {
								if isOpenLine {
									break
								}
								continue
							}
							content = strings.TrimSpace(content)
							if strings.HasSuffix(content, "**/") || strings.HasSuffix(content, "*/") {
								content = strings.TrimSpace(strings.TrimSuffix(strings.TrimSuffix(content, "**/"), "*/"))
							}
							if content == "" {
								if isOpenLine {
									break
								}
								continue
							}
							if strings.HasPrefix(content, "* ") {
								content = strings.TrimSpace(content[2:])
							}
							if IsSpecText(content) {
								hasRequirements = true
							} else {
								hasSummary = true
							}
							if isOpenLine {
								break
							}
						}
					}
				}
			}
			if langName == "csharp" || langName == "rust" {
				prevIdx := def.start - 2
				if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(lines[prevIdx]), "///") {
					isNativeDocstring = true
					for lineIndex := prevIdx; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if !strings.HasPrefix(line, "///") {
							break
						}
						commentText := strings.TrimSpace(strings.TrimPrefix(line, "///"))
						if commentText == "" {
							continue
						}
						commentText = strings.TrimPrefix(commentText, "<summary>")
						commentText = strings.TrimSuffix(commentText, "</summary>")
						commentText = strings.TrimSpace(commentText)
						if commentText == "<remarks>" || commentText == "</remarks>" || commentText == "" {
							continue
						}
						if IsSpecText(commentText) {
							hasRequirements = true
						} else {
							hasSummary = true
						}
					}
				}
			}
			if !isNativeDocstring && langName == "python" {
				defLineRaw := lines[def.start-1]
				parenDepth := 0
				for _, ch := range defLineRaw {
					if ch == '(' {
						parenDepth++
					}
					if ch == ')' {
						parenDepth--
					}
				}
				bodyStart := def.start
				if parenDepth > 0 {
					for scanIdx := def.start; scanIdx < len(lines) && scanIdx < def.start+15; scanIdx++ {
						for _, ch := range lines[scanIdx] {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						if parenDepth <= 0 {
							bodyStart = scanIdx + 1
							break
						}
					}
				}
				for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
					trimmed := strings.TrimSpace(lines[bodyIdx])
					if trimmed == "" {
						continue
					}
					if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
						isNativeDocstring = true
						quote := `"""`
						if strings.HasPrefix(trimmed, `'''`) {
							quote = `'''`
						}
						afterOpen := strings.TrimPrefix(trimmed, quote)
						closeIdx := strings.Index(afterOpen, quote)
						if closeIdx >= 0 {
							content := strings.TrimSpace(afterOpen[:closeIdx])
							if content != "" {
								if IsSpecText(content) {
									hasRequirements = true
								} else {
									hasSummary = true
								}
							}
						} else {
							firstContent := strings.TrimSpace(afterOpen)
							if firstContent != "" {
								if IsSpecText(firstContent) {
									hasRequirements = true
								} else {
									hasSummary = true
								}
							}
							for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
								sline := strings.TrimSpace(lines[scanIdx])
								if sline == quote {
									break
								}
								if strings.HasSuffix(sline, quote) {
									content := strings.TrimSpace(strings.TrimSuffix(sline, quote))
									if content != "" {
										if IsSpecText(content) {
											hasRequirements = true
										} else {
											hasSummary = true
										}
									}
									break
								}
								if sline == "" {
									continue
								}
								if IsSpecText(sline) {
									hasRequirements = true
								} else {
									hasSummary = true
								}
							}
						}
					}
					break
				}
			}
			if langName == "python" && isNativeDocstring {
				for lineIndex := def.start - 2; lineIndex >= 0; lineIndex-- {
					line := strings.TrimSpace(lines[lineIndex])
					if line == "" {
						break
					}
					if !strings.HasPrefix(line, commentPrefix) {
						break
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, commentPrefix))
					if commentText != "" {
						isNativeDocstring = false
						break
					}
				}
			}
			if !isNativeDocstring {
				if langName == "go" {
					isNativeDocstring = true
				}
				for lineIndex := def.start - 2; lineIndex >= 0; lineIndex-- {
					line := strings.TrimSpace(lines[lineIndex])
					if line == "" {
						break
					}
					if !strings.HasPrefix(line, commentPrefix) {
						break
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, commentPrefix))
					if commentText == "" {
						continue
					}
					if IsSpecText(commentText) {
						hasRequirements = true
					} else {
						hasSummary = true
					}
				}
			}
			if !isNativeDocstring && (hasSummary || hasRequirements) {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Definition \"%s\" is not using native docstring format in %s:%d", def.name, file, def.start),
					model.BreachCodeDefNotNativeDocstring,
					fmt.Sprintf("%s::%s", file, def.name), def.start, 0, def.name))
			}
			if !hasSummary {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Definition \"%s\" is missing a summary comment in %s:%d", def.name, file, def.start),
					model.BreachCodeDefMissingSummary,
					fmt.Sprintf("%s::%s", file, def.name), def.start, 0, def.name))
			}
			if !hasRequirements && requiresDefinitionRequirements(defLine, language.Name()) {
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Definition \"%s\" is missing spec comments in %s:%d", def.name, file, def.start),
					model.BreachCodeDefMissingRequirements,
					fmt.Sprintf("%s::%s", file, def.name), def.start, 0, def.name))
			}
		}
	}
	return ctx.FilterIgnored(breachs)
}

// ­ƒƒ½commentPolicy holds the data fields for a commentPolicy record.
func commentPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsComments() {
			continue
		}

		lines := strings.Split(content, "\n")
		langBreachs := language.ScanComments(ctx, file, content, lines)
		breachs = append(breachs, langBreachs...)
	}
	return ctx.FilterIgnored(breachs)
}

// ­ƒÆátruncate holds the data fields for a truncate record.
func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen]
}

// ­ƒö│️requirementsPolicy holds the data fields for a requirementsPolicy record.
func requirementsPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := languages.GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		lines := strings.Split(content, "\n")
		sections := ctx.Sections(file)
		prefix := language.CommentPrefix()
		var headerSection *model.Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection != nil {
			requirementsChildFound := false
			for _, child := range headerSection.Children {
				if strings.ToLower(child.Name) == "requirements" {
					requirementsChildFound = true
					for i := child.StartLine + 1; i < child.EndLine && i <= len(lines); i++ {
						line := strings.TrimSpace(lines[i-1])
						if line == "" {
							continue
						}
						commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
						if commentText == "" {
							continue
						}
						if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
							breachs = append(breachs, ctx.CreateBreach(
								fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
								model.BreachCodeRequirementsSyntax,
								fmt.Sprintf("%s#Header/Requirements", file), i, 0, commentText))
						}
					}
					break
				}
			}
			if !requirementsChildFound {
				for i := headerSection.StartLine + 1; i < headerSection.EndLine && i <= len(lines); i++ {
					line := strings.TrimSpace(lines[i-1])
					if line == "" {
						continue
					}
					if !strings.HasPrefix(line, prefix) {
						continue
					}
					commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
					if commentText == "" {
						continue
					}
					if !IsSpecText(commentText) {
						continue
					}
					if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
						breachs = append(breachs, ctx.CreateBreach(
							fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
							model.BreachCodeRequirementsSyntax,
							fmt.Sprintf("%s#Header", file), i, 0, commentText))
					}
				}
			}
		}
		var checkSectionRequirements func(s model.Section)
		checkSectionRequirements = func(s model.Section) {
			if strings.ToLower(s.Name) == "header" {
				return
			}
			for i := s.StartLine + 1; i < s.EndLine && i <= len(lines); i++ {
				line := strings.TrimSpace(lines[i-1])
				if line == "" {
					continue
				}
				if !strings.HasPrefix(line, prefix) {
					break
				}
				commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
				if !IsSpecText(commentText) {
					break
				}
				if hasSyntax, reason := hasImplementationSyntax(commentText); hasSyntax {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("Spec contains implementation syntax in %s:%d (%s)", file, i, reason),
						model.BreachCodeRequirementsSyntax,
						fmt.Sprintf("%s#%s", file, s.Name), i, 0, commentText))
				}
			}
			for _, child := range s.Children {
				checkSectionRequirements(child)
			}
		}
		for _, s := range sections {
			checkSectionRequirements(s)
		}
	}
	return ctx.FilterIgnored(breachs)
}

// ­ƒö▓️codePolicy holds the data fields for a codePolicy record.
func codePolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	breachs = append(breachs, headerPolicy(ctx)...)
	breachs = append(breachs, sectionPolicy(ctx)...)
	breachs = append(breachs, commentPolicy(ctx)...)
	breachs = append(breachs, requirementsPolicy(ctx)...)
	breachs = append(breachs, emojiPolicy(ctx)...)
	breachs = append(breachs, docsPolicy(ctx)...)
	return breachs
}

// ­ƒÿÇemojiPolicy holds the data fields for a emojiPolicy record.
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

// Ôû¬´©ÅdocsPolicy holds the data fields for a docsPolicy record.
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

// Ôû½´©ÅdevDocsPolicy holds the data fields for a devDocsPolicy record.
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
	fileSectionRegex := regexp.MustCompile(`(?m)^## ­ƒôä\s*(.+?)\s*$`)
	folderSectionRegex := regexp.MustCompile(`(?m)^## ­ƒôü\s*(.+?)\s*$`)
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

// Ôù¥sketchpadPolicy holds the data fields for a sketchpadPolicy record.
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

// Ôù¢repoPolicy holds the data fields for a repoPolicy record.
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

// 📦️readDevcontainerVscodeExtensions returns extension ids from devcontainer customizations.vscode.extensions.
func ReadDevcontainerVscodeExtensions(rootDir string) []string {
	devcontainerPath := filepath.Join(rootDir, ".devcontainer", "devcontainer.json")
	dcData, err := os.ReadFile(devcontainerPath)
	if err != nil {
		return nil
	}
	var devcontainer map[string]interface{}
	if err := json.Unmarshal(dcData, &devcontainer); err != nil {
		return nil
	}
	customizations, _ := devcontainer["customizations"].(map[string]interface{})
	if customizations == nil {
		return nil
	}
	vscodeCustom, _ := customizations["vscode"].(map[string]interface{})
	if vscodeCustom == nil {
		return nil
	}
	rawExtensions, _ := vscodeCustom["extensions"].([]interface{})
	return extensionIDsFromJSONList(rawExtensions)
}

// 📦️readWorkspaceExtensionRecommendations returns recommendation ids from .vscode/extensions.json when present.
func ReadWorkspaceExtensionRecommendations(rootDir string) ([]string, bool) {
	extensionsPath := filepath.Join(rootDir, ".vscode", "extensions.json")
	extData, err := os.ReadFile(extensionsPath)
	if err != nil {
		return nil, false
	}
	var extFile map[string]interface{}
	if err := json.Unmarshal(extData, &extFile); err != nil {
		return nil, false
	}
	rawRecommendations, _ := extFile["recommendations"].([]interface{})
	return extensionIDsFromJSONList(rawRecommendations), true
}

// 📦️extensionIDsFromJSONList normalizes a JSON string list of extension ids.
func extensionIDsFromJSONList(values []interface{}) []string {
	if len(values) == 0 {
		return nil
	}
	ids := make([]string, 0, len(values))
	for _, value := range values {
		id, ok := value.(string)
		if !ok || id == "" {
			continue
		}
		ids = append(ids, id)
	}
	return ids
}

// 📦️missingWorkspaceExtensionRecommendations lists devcontainer extensions absent from workspace recommendations.
func missingWorkspaceExtensionRecommendations(devcontainerExtensions, workspaceRecommendations []string) []string {
	if len(devcontainerExtensions) == 0 {
		return nil
	}
	recommended := map[string]struct{}{}
	for _, id := range workspaceRecommendations {
		recommended[id] = struct{}{}
	}
	var missing []string
	for _, id := range devcontainerExtensions {
		if _, ok := recommended[id]; !ok {
			missing = append(missing, id)
		}
	}
	return missing
}

// 📦️mergeWorkspaceExtensionRecommendations builds a deduplicated recommendation list (devcontainer first, then extras).
func MergeWorkspaceExtensionRecommendations(devcontainerExtensions, workspaceRecommendations []string) []string {
	merged := make([]string, 0, len(devcontainerExtensions)+len(workspaceRecommendations))
	seen := map[string]struct{}{}
	appendUnique := func(ids []string) {
		for _, id := range ids {
			if id == "" {
				continue
			}
			if _, ok := seen[id]; ok {
				continue
			}
			seen[id] = struct{}{}
			merged = append(merged, id)
		}
	}
	appendUnique(devcontainerExtensions)
	appendUnique(workspaceRecommendations)
	return merged
}

// 📦️writeWorkspaceExtensionRecommendations writes .vscode/extensions.json for host editor recommendations.
func WriteWorkspaceExtensionRecommendations(rootDir string, recommendations []string) error {
	extensionsPath := filepath.Join(rootDir, ".vscode", "extensions.json")
	if err := os.MkdirAll(filepath.Dir(extensionsPath), 0755); err != nil {
		return err
	}
	rawRecommendations := make([]interface{}, len(recommendations))
	for i, id := range recommendations {
		rawRecommendations[i] = id
	}
	payload := map[string]interface{}{
		"recommendations":         rawRecommendations,
		"unwantedRecommendations": []interface{}{},
	}
	out, err := json.MarshalIndent(payload, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(extensionsPath, append(out, '\n'), 0644)
}

// Ôù╗️´©ÅsystemPolicy holds the data fields for a systemPolicy record.
func SystemPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	settingsPath := filepath.Join(ctx.RootDir, ".vscode", "settings.json")
	if _, err := os.Stat(settingsPath); err == nil {
		breachs = append(breachs, ctx.CreateBreach(
			"VSCode settings.json must be inside .devcontainer/devcontainer.json customizations.vscode.settings",
			model.BreachSystemDevcontainerVscodeSettingsOutside,
			".vscode/settings.json", 1, 0, ""))
	}
	devcontainerExtensions := ReadDevcontainerVscodeExtensions(ctx.RootDir)
	if len(devcontainerExtensions) > 0 {
		workspaceRecommendations, hasWorkspaceRecommendations := ReadWorkspaceExtensionRecommendations(ctx.RootDir)
		missing := missingWorkspaceExtensionRecommendations(devcontainerExtensions, workspaceRecommendations)
		if !hasWorkspaceRecommendations || len(missing) > 0 {
			breachs = append(breachs, ctx.CreateBreach(
				".vscode/extensions.json must recommend every extension from devcontainer customizations.vscode.extensions for host editors (Cursor reads this file)",
				model.BreachSystemDevcontainerVscodeExtensionsOutside,
				".vscode/extensions.json", 1, 0, ""))
		}
	}
	return ctx.FilterIgnored(breachs)
}

// 🔏️pathEmojiEntry is one language-neutral file or directory identity fact.
type pathEmojiEntry struct {
	Path     string `json:"path"`
	NodeKind string `json:"nodeKind"`
	Reserved bool   `json:"reserved,omitempty"`
}

// ⚠️pathEmojiFinding is one deterministic naming statute result.
type pathEmojiFinding struct {
	Kind    string `json:"kind"`
	Path    string `json:"path"`
	Sibling string `json:"sibling,omitempty"`
	Emoji   string `json:"emoji,omitempty"`
}

// 🪞️foldPathEmojiIdentity removes presentation selectors from a logical emoji identity.
func foldPathEmojiIdentity(value string) string {
	return strings.ReplaceAll(strings.ReplaceAll(value, "\uFE0E", ""), "\uFE0F", "")
}

// 😀️leadingPathEmojiIdentity extracts every contiguous leading emoji grapheme.
func leadingPathEmojiIdentity(value string) (string, string, string) {
	first, rest := model.ExtractEntityEmoji(value)
	if first == "" {
		return "", value, ""
	}
	identity := first
	for {
		next, remaining := model.ExtractEntityEmoji(rest)
		if next == "" {
			break
		}
		identity += next
		rest = remaining
	}
	return identity, rest, first
}

// ⚖️pathEmojiStatuteFindings evaluates the shared file/folder sibling namespace.
func pathEmojiStatuteFindings(entries []pathEmojiEntry, genericEmojiIdentities []string) []pathEmojiFinding {
	generic := map[string]bool{}
	for _, emoji := range genericEmojiIdentities {
		generic[foldPathEmojiIdentity(emoji)] = true
	}
	sort.Slice(entries, func(i, j int) bool { return entries[i].Path < entries[j].Path })
	seen := map[string]pathEmojiEntry{}
	findings := []pathEmojiFinding{}
	for _, entry := range entries {
		name := entry.Path
		if index := strings.LastIndex(name, "/"); index >= 0 {
			name = name[index+1:]
		}
		identity, rest, first := leadingPathEmojiIdentity(name)
		reservedDocumentation := rest == "README" || strings.HasPrefix(rest, "README.") || rest == "LICENSE" || strings.HasPrefix(rest, "LICENSE.") || rest == "AGENTS.md"
		if entry.NodeKind == "file" && reservedDocumentation {
			if identity != "" {
				findings = append(findings, pathEmojiFinding{Kind: "reserved-emoji", Path: entry.Path, Emoji: identity})
			}
			continue
		}
		if entry.Reserved {
			continue
		}
		if identity == "" {
			findings = append(findings, pathEmojiFinding{Kind: "missing", Path: entry.Path})
			continue
		}
		multiple := identity != first
		for index := range rest {
			if emoji, _ := model.ExtractEntityEmoji(rest[index:]); emoji != "" {
				multiple = true
				break
			}
		}
		if multiple {
			findings = append(findings, pathEmojiFinding{Kind: "multiple", Path: entry.Path, Emoji: identity})
		}
		if generic[foldPathEmojiIdentity(first)] {
			findings = append(findings, pathEmojiFinding{Kind: "generic", Path: entry.Path, Emoji: first})
		}
		if (model.EmojiText(first) != strings.ReplaceAll(first, "\uFE0E", "") || strings.Contains(first, "\u20E3")) && !strings.Contains(first, "\uFE0F") {
			findings = append(findings, pathEmojiFinding{Kind: "presentation", Path: entry.Path, Emoji: first})
		}
		if rest != "" {
			firstRest, _ := utf8DecodeRuneInString(rest)
			if unicode.IsSpace(firstRest) {
				findings = append(findings, pathEmojiFinding{Kind: "spacing", Path: entry.Path, Emoji: identity})
			}
		}
		parent := ""
		if index := strings.LastIndex(entry.Path, "/"); index >= 0 {
			parent = entry.Path[:index]
		}
		key := parent + "\x00" + foldPathEmojiIdentity(first)
		if previous, ok := seen[key]; ok {
			findings = append(findings, pathEmojiFinding{Kind: "duplicate", Path: entry.Path, Sibling: previous.Path, Emoji: foldPathEmojiIdentity(first)})
		} else {
			seen[key] = entry
		}
	}
	return findings
}

// 🔡️utf8DecodeRuneInString isolates the standard-library decoding call for path spacing checks.
func utf8DecodeRuneInString(value string) (rune, int) {
	for _, character := range value {
		return character, len(string(character))
	}
	return 0, 0
}

type pathEmojiFixedContract struct {
	PathPattern string `json:"pathPattern"`
	Descendants string `json:"descendants"`
	Scope       struct {
		Kind                     string `json:"kind"`
		EcosystemID              string `json:"ecosystemId"`
		FixedFilenameContractID  string `json:"fixedFilenameContractId"`
		FixedDirectoryContractID string `json:"fixedDirectoryContractId"`
		Path                     string `json:"path"`
	} `json:"scope"`
}

type pathEmojiTaxonomy struct {
	PathEmojiPolicy struct {
		GenericEmojiIdentities        []string `json:"genericEmojiIdentities"`
		ReservedSubtreeDirectoryNames []string `json:"reservedSubtreeDirectoryNames"`
	} `json:"pathEmojiPolicy"`
	FixedFilenameContracts  map[string]pathEmojiFixedContract `json:"fixedFilenameContracts"`
	FixedDirectoryContracts map[string]pathEmojiFixedContract `json:"fixedDirectoryContracts"`
	PathExclusions          map[string]struct {
		Path string `json:"path"`
	} `json:"pathExclusions"`
}

func pathEmojiPackageContext(root, path string) (bool, string) {
	parts := strings.Split(path, "/")
	parent := parts[:len(parts)-1]
	packagesIndex := -1
	for index := len(parent) - 1; index >= 0; index-- {
		_, rest, first := leadingPathEmojiIdentity(parent[index])
		if rest == "packages" && foldPathEmojiIdentity(first) == foldPathEmojiIdentity("📦️") {
			packagesIndex = index
			break
		}
	}
	ecosystemID := ""
	if packagesIndex >= 0 && packagesIndex+1 < len(parent) {
		ecosystemID = parent[packagesIndex+1]
	}
	parentPath := filepath.Join(root, filepath.FromSlash(strings.Join(parent, "/")))
	adjacentEcosystem := ""
	for _, manifest := range []struct{ name, ecosystemID string }{{"Cargo.toml", "🦀️rust"}, {"go.mod", "🐹️go"}, {"package.json", "🟦️typescript"}} {
		_, manifestError := os.Lstat(filepath.Join(parentPath, manifest.name))
		if filepath.Base(path) == manifest.name || manifestError == nil {
			adjacentEcosystem = manifest.ecosystemID
			break
		}
	}
	if ecosystemID == "" {
		ecosystemID = adjacentEcosystem
	}
	packageRoot := packagesIndex == len(parent)-2 || adjacentEcosystem != ""
	return packageRoot, ecosystemID
}

func pathEmojiContractApplies(root, path string, contract pathEmojiFixedContract, taxonomy pathEmojiTaxonomy) bool {
	matched, err := workspace.Match(contract.PathPattern, path)
	if err != nil || !matched {
		return false
	}
	switch contract.Scope.Kind {
	case "exact-path":
		return path == contract.Scope.Path
	case "repository-root":
		return !strings.Contains(path, "/")
	case "package-root":
		packageRoot, ecosystemID := pathEmojiPackageContext(root, path)
		return packageRoot && ecosystemID == contract.Scope.EcosystemID
	case "sibling-fixed-filename-contract":
		sibling, ok := taxonomy.FixedFilenameContracts[contract.Scope.FixedFilenameContractID]
		if !ok {
			return false
		}
		siblingPath := filepath.Join(root, filepath.FromSlash(filepath.ToSlash(filepath.Dir(path))), filepath.Base(sibling.PathPattern))
		_, siblingError := os.Lstat(siblingPath)
		return siblingError == nil
	case "fixed-directory-contract":
		parent, ok := taxonomy.FixedDirectoryContracts[contract.Scope.FixedDirectoryContractID]
		return ok && pathEmojiContractApplies(root, filepath.ToSlash(filepath.Dir(path)), parent, taxonomy)
	default:
		return true
	}
}

func pathEmojiInventory(ctx *PolicyContext) ([]pathEmojiEntry, []string, error) {
	data, err := os.ReadFile(filepath.Join(ctx.RootDir, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json"))
	if err != nil {
		return nil, nil, err
	}
	var taxonomy pathEmojiTaxonomy
	if err := json.Unmarshal(data, &taxonomy); err != nil {
		return nil, nil, err
	}
	output, err := exec.Command("git", "-C", ctx.RootDir, "ls-files", "-co", "--exclude-standard", "-z").Output()
	if err != nil {
		return nil, nil, err
	}
	rawFiles := strings.Split(string(output), "\x00")
	files := []string{}
	directories := map[string]bool{}
	for _, rawFile := range rawFiles {
		if rawFile == "" {
			continue
		}
		file := strings.TrimSuffix(workspace.NormalizePath(rawFile), "/")
		if file == "" {
			continue
		}
		info, statError := os.Lstat(filepath.Join(ctx.RootDir, filepath.FromSlash(file)))
		if statError != nil {
			continue
		}
		if info.IsDir() {
			directories[file] = true
		} else {
			files = append(files, file)
		}
		for parent := filepath.ToSlash(filepath.Dir(file)); parent != "." && parent != ""; parent = filepath.ToSlash(filepath.Dir(parent)) {
			directories[parent] = true
		}
	}
	reservedSubtrees := map[string]bool{}
	for _, name := range taxonomy.PathEmojiPolicy.ReservedSubtreeDirectoryNames {
		reservedSubtrees[foldPathEmojiIdentity(name)] = true
	}
	isExcluded := func(path string) bool {
		for _, exclusion := range taxonomy.PathExclusions {
			if strings.HasPrefix(path+"/", exclusion.Path) {
				return true
			}
		}
		return false
	}
	underReserved := func(path string) bool {
		parts := strings.Split(path, "/")
		for _, part := range parts[:len(parts)-1] {
			_, rest, _ := leadingPathEmojiIdentity(part)
			if reservedSubtrees[foldPathEmojiIdentity(part)] || reservedSubtrees[foldPathEmojiIdentity(rest)] {
				return true
			}
		}
		return false
	}
	isReservedSubtreeName := func(path string) bool {
		name := path
		if index := strings.LastIndex(name, "/"); index >= 0 {
			name = name[index+1:]
		}
		_, rest, _ := leadingPathEmojiIdentity(name)
		return reservedSubtrees[foldPathEmojiIdentity(name)] || reservedSubtrees[foldPathEmojiIdentity(rest)]
	}
	contractReserved := func(path string, contracts map[string]pathEmojiFixedContract) bool {
		for _, contract := range contracts {
			if pathEmojiContractApplies(ctx.RootDir, path, contract, taxonomy) {
				return true
			}
		}
		return false
	}
	fixedReservedRoots := []string{}
	for directory := range directories {
		for _, contract := range taxonomy.FixedDirectoryContracts {
			if contract.Descendants == "reserved" && pathEmojiContractApplies(ctx.RootDir, directory, contract, taxonomy) {
				fixedReservedRoots = append(fixedReservedRoots, directory)
				break
			}
		}
	}
	underFixedReserved := func(path string) bool {
		for _, reservedRoot := range fixedReservedRoots {
			if strings.HasPrefix(path, reservedRoot+"/") {
				return true
			}
		}
		return false
	}
	entries := []pathEmojiEntry{}
	for directory := range directories {
		if !isExcluded(directory) {
			entries = append(entries, pathEmojiEntry{Path: directory, NodeKind: "directory", Reserved: underReserved(directory) || underFixedReserved(directory) || isReservedSubtreeName(directory) || contractReserved(directory, taxonomy.FixedDirectoryContracts)})
		}
	}
	for _, file := range files {
		file = workspace.NormalizePath(file)
		if file != "" && !isExcluded(file) {
			entries = append(entries, pathEmojiEntry{Path: file, NodeKind: "file", Reserved: underReserved(file) || underFixedReserved(file) || contractReserved(file, taxonomy.FixedFilenameContracts)})
		}
	}
	return entries, taxonomy.PathEmojiPolicy.GenericEmojiIdentities, nil
}

func pathEmojiBreaches(ctx *PolicyContext, nodeKind string) []model.Breach {
	entries, generic, err := pathEmojiInventory(ctx)
	if err != nil {
		return nil
	}
	byPath := map[string]string{}
	for _, entry := range entries {
		byPath[entry.Path] = entry.NodeKind
	}
	kinds := map[string]map[string]model.Statute{
		"directory": {"missing": model.BreachFolderNameMissingEmoji, "generic": model.BreachFolderNameGenericEmoji, "presentation": model.BreachFolderNameNoncanonicalEmojiPresentation, "spacing": model.BreachFolderNameWhitespaceAfterEmoji, "duplicate": model.BreachFolderNameEmojiNotUnique, "multiple": model.BreachFolderNameMultipleEmojis},
		"file":      {"missing": model.BreachFileNameMissingEmoji, "generic": model.BreachFileNameGenericEmoji, "presentation": model.BreachFileNameNoncanonicalEmojiPresentation, "spacing": model.BreachFileNameWhitespaceAfterEmoji, "duplicate": model.BreachFileNameEmojiNotUnique, "multiple": model.BreachFileNameMultipleEmojis, "reserved-emoji": model.BreachFileNameReservedEmoji},
	}
	breachs := []model.Breach{}
	for _, finding := range pathEmojiStatuteFindings(entries, generic) {
		if byPath[finding.Path] != nodeKind {
			continue
		}
		summary := "Path \"" + finding.Path + "\" breaches the " + finding.Kind + " emoji statute"
		breachs = append(breachs, ctx.CreateBreach(summary, kinds[nodeKind][finding.Kind], finding.Path, 0, 0, finding.Path))
	}
	return breachs
}

// ­ƒôüfolderPolicy holds the data fields for a folderPolicy record.
func folderPolicy(ctx *PolicyContext) []model.Breach {
	if ctx.Scope.Kind != workspace.ScopeRepo {
		return nil
	}
	var breachs []model.Breach
	breachs = append(breachs, pathEmojiBreaches(ctx, "directory")...)
	excludePrefixes := []string{
		".git/",
		".🧬semio/🦑️repo/",
		"node_modules/",
		".venv/",
		".nx/",
	}

	err := filepath.WalkDir(ctx.RootDir, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() {
			return nil
		}
		relPath, _ := filepath.Rel(ctx.RootDir, path)
		relPath = workspace.NormalizePath(relPath)
		if relPath == "." {
			return nil
		}
		for _, prefix := range excludePrefixes {
			if strings.HasPrefix(relPath+"/", prefix) {
				return filepath.SkipDir
			}
		}

		if workspace.IsIgnoredByGitignore(filepath.Join(ctx.RootDir, relPath)) {
			return filepath.SkipDir
		}

		f, readErr := os.Open(path)
		if readErr != nil {
			return nil
		}
		defer f.Close()
		_, readErr = f.Readdirnames(1)
		if readErr == io.EOF {
			breachs = append(breachs, ctx.CreateBreach(
				"Empty folder \""+relPath+"\" must be removed",
				model.BreachFolderIllegalEmpty,
				relPath+"/", 0, 0, relPath))
		}
		return nil
	})
	_ = err
	return breachs
}

// Ôù╝️´©ÅGodfile holds the data fields for a Godfile record.
type Godfile struct {
	Exact map[string]bool
	Globs []string
}

// ­ƒöÁisGlobPattern holds the data fields for a isGlobPattern record.
func isGlobPattern(value string) bool {
	return strings.ContainsAny(value, "*?[{")
}

// ­ƒö┤️loadGodfile holds the data fields for a loadGodfile record.
func loadGodfile() (*Godfile, error) {
	godfilePath := filepath.Join(workspace.GetRepoMetaDir(), "📁️files.json")
	data, err := os.ReadFile(godfilePath)
	if err != nil {
		return nil, err
	}
	var files []string
	if err := json.Unmarshal(data, &files); err != nil {
		return nil, err
	}
	result := &Godfile{
		Exact: make(map[string]bool, len(files)),
	}
	for _, f := range files {
		normalized := workspace.NormalizePath(f)
		if isGlobPattern(normalized) {
			result.Globs = append(result.Globs, normalized)
			continue
		}
		result.Exact[normalized] = true
	}
	return result, nil
}

// ­ƒƒágodfileMatchesPath holds the data fields for a godfileMatchesPath record.
func godfileMatchesPath(godfile *Godfile, relPath string) bool {
	relPath = workspace.NormalizeRepoPath(relPath)
	if godfile.Exact[relPath] {
		return true
	}
	for _, pattern := range godfile.Globs {
		matched, err := workspace.Match(pattern, relPath)
		if err != nil {
			continue
		}
		if matched {
			return true
		}
	}
	return false
}

// ­ƒƒífilePolicy holds the data fields for a filePolicy record.
func filePolicy(ctx *PolicyContext) []model.Breach {
	if ctx.Scope.Kind != workspace.ScopeRepo {
		return nil
	}
	var breachs []model.Breach
	breachs = append(breachs, pathEmojiBreaches(ctx, "file")...)
	godfile, err := loadGodfile()
	if err != nil {
		return breachs
	}
	_ = filepath.WalkDir(workspace.RootDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			if workspace.IsRepoExcludedPath(path) || strings.HasPrefix(d.Name(), ".git") || d.Name() == "node_modules" || d.Name() == ".venv" {
				return filepath.SkipDir
			}
			return nil
		}
		if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
			return nil
		}
		relPath := workspace.NormalizeRepoPath(path)
		if !godfileMatchesPath(godfile, relPath) {
			breachs = append(breachs, ctx.CreateBreach(
				"File \""+relPath+"\" is not listed in .🧬semio/🦑️repo/📁️files.json",
				model.BreachFileIllegalUseGodfile,
				relPath, 0, 0, relPath))
		}
		return nil
	})
	return breachs
}

// ­ƒƒócomposePolicy validates that compose.* files do not import UI dependencies.
func composePolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	uiPackages := []string{
		"clsx", "tailwind-merge", "tailwind", "tailwindcss",
		"three", "@react-three",
		"react", "react-dom",
		"framer-motion",
		"@radix-ui", "@radix-client",
		"@dnd-kit",
		"zustand",
		"elements/ui",
	}
	for _, file := range files {
		baseName := filepath.Base(file)
		if !strings.HasPrefix(baseName, "compose.") {
			continue
		}
		if !strings.HasSuffix(file, ".ts") && !strings.HasSuffix(file, ".tsx") {
			continue
		}
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lines := strings.Split(content, "\n")
		for lineNum, line := range lines {
			lineNumber := lineNum + 1
			if !strings.Contains(line, "import ") {
				continue
			}
			for _, pkg := range uiPackages {
				importPattern := fmt.Sprintf(`from\s+['"].*%s`, regexp.QuoteMeta(pkg))
				if matched, _ := regexp.MatchString(importPattern, line); matched {
					breachs = append(breachs, ctx.CreateBreach(
						fmt.Sprintf("UI dependency '%s' is not allowed in compose.* files", pkg),
						model.BreachComposeNoUiDependency,
						file, lineNumber, 0, strings.TrimSpace(line)))
					break
				}
			}
		}
	}
	return ctx.FilterIgnored(breachs)
}

// 🔌️dependencyBoundaryPolicy flags third-party imports outside adapter boundaries.
func dependencyBoundaryPolicy(ctx *PolicyContext) []model.Breach {
	var breachs []model.Breach
	files, err := ctx.Files()
	if err != nil {
		return breachs
	}
	for _, file := range files {
		if dependencyBoundarySkipFile(file) {
			continue
		}
		_, thirdParty := loadThirdPartyDepsForFile(file)
		if len(thirdParty) == 0 {
			continue
		}
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		if dependencyBoundaryFileIsAdapter(file, content) {
			continue
		}
		lang := languages.GetLanguage(file)
		if lang == nil {
			continue
		}
		importLines, _ := lang.ExtractImports(content)
		for lineIdx, impLine := range importLines {
			lineNumber := dependencyBoundaryImportLineNumber(content, impLine, lineIdx+1)
			for _, spec := range parseThirdPartyImportSpecs(impLine, file) {
				if !dependencyBoundaryIsThirdParty(spec, thirdParty) {
					continue
				}
				breachs = append(breachs, ctx.CreateBreach(
					fmt.Sprintf("Direct import of third-party %q must live in an adapter module", spec),
					model.BreachDependencyBoundaryDirectImport,
					file, lineNumber, 0, strings.TrimSpace(impLine)))
			}
		}
	}
	return ctx.FilterIgnored(breachs)
}

func dependencyBoundarySkipFile(file string) bool {
	n := workspace.NormalizePath(file)
	if strings.Contains(n, "/node_modules/") || strings.Contains(n, "\\node_modules\\") {
		return true
	}
	if strings.Contains(n, "/.🧬semio/") || strings.Contains(n, "/dist/") || strings.Contains(n, "/target/") {
		return true
	}
	if strings.Contains(n, "/pkg/") && (strings.HasSuffix(n, "_bg.js") || strings.HasSuffix(n, ".wasm")) {
		return true
	}
	base := filepath.Base(n)
	if strings.HasSuffix(base, ".gen.ts") {
		return true
	}
	if strings.Contains(base, ".test.") || strings.HasSuffix(base, "_test.go") || strings.HasSuffix(base, "Tests.cs") {
		return true
	}
	switch filepath.Ext(n) {
	case ".json", ".md", ".yaml", ".yml", ".toml", ".lock", ".svg", ".png", ".jpg", ".woff", ".woff2":
		return true
	}
	return false
}

func dependencyBoundaryFileIsAdapter(file, content string) bool {
	n := strings.ToLower(workspace.NormalizePath(file))
	if strings.Contains(n, "/adapters/") || strings.Contains(n, "\\adapters\\") {
		return true
	}
	if strings.Contains(n, "/external_adapters") || strings.HasSuffix(n, "external_adapters.rs") {
		return true
	}
	if strings.Contains(n, "-transport.") || strings.HasSuffix(n, ".🟦️worker.ts") || strings.Contains(n, "kit-store.worker") {
		return true
	}
	base := strings.ToLower(filepath.Base(n))
	if strings.Contains(base, "adapter") {
		return true
	}
	lower := strings.ToLower(content)
	for _, marker := range []string{
		"//#region 🔌️adapter", "// #region 🔌️adapter",
		"# #region 🔌️adapter", "#region 🔌️adapter",
		"//#region 🔌️adapters", "// #region 🔌️adapters",
		"# #region 🔌️adapters",
		"//#region 🌐️rswasmtransport", "// #region 🌐️rswasmtransport",
		"pub mod adapters", "mod adapters ",
	} {
		if strings.Contains(lower, marker) {
			return true
		}
	}
	return false
}

func loadThirdPartyDepsForFile(file string) (manifestDir string, deps map[string]struct{}) {
	deps = make(map[string]struct{})
	dir := workspace.NormalizePath(filepath.Dir(file))
	if dir == "." {
		dir = ""
	}
	seen := make(map[string]struct{})
	for {
		if _, ok := seen[dir]; ok {
			break
		}
		seen[dir] = struct{}{}
		root := filepath.Join(workspace.RootDir, dir)
		if workspace.FileExists(filepath.Join(root, "package.json")) {
			dependencyBoundaryMergePackageJSON(filepath.Join(root, "package.json"), deps)
			return dir, deps
		}
		if workspace.FileExists(filepath.Join(root, "Cargo.toml")) {
			dependencyBoundaryMergeCargoToml(filepath.Join(root, "Cargo.toml"), deps)
			return dir, deps
		}
		if workspace.FileExists(filepath.Join(root, "go.mod")) {
			dependencyBoundaryMergeGoMod(filepath.Join(root, "go.mod"), deps)
			return dir, deps
		}
		if workspace.FileExists(filepath.Join(root, "pyproject.toml")) {
			dependencyBoundaryMergePyProject(filepath.Join(root, "pyproject.toml"), deps)
			return dir, deps
		}
		csprojs, _ := filepath.Glob(filepath.Join(root, "*.csproj"))
		if len(csprojs) > 0 {
			dependencyBoundaryMergeCsproj(csprojs[0], deps)
			return dir, deps
		}
		if dir == "" {
			break
		}
		parent := workspace.NormalizePath(filepath.Dir(dir))
		if parent == "." {
			parent = ""
		}
		if parent == dir {
			break
		}
		dir = parent
	}
	return "", deps
}

func dependencyBoundaryMergePackageJSON(path string, deps map[string]struct{}) {
	data, err := os.ReadFile(path)
	if err != nil {
		return
	}
	var raw map[string]json.RawMessage
	if json.Unmarshal(data, &raw) != nil {
		return
	}
	for _, key := range []string{"dependencies", "devDependencies", "peerDependencies", "optionalDependencies"} {
		if block, ok := raw[key]; ok {
			var m map[string]string
			if json.Unmarshal(block, &m) == nil {
				for name, ver := range m {
					if dependencyBoundaryIsInternalNpm(name, ver) {
						continue
					}
					deps[name] = struct{}{}
					if strings.HasPrefix(name, "@") {
						parts := strings.SplitN(name, "/", 2)
						if len(parts) == 2 {
							deps[parts[0]+"/"+parts[1]] = struct{}{}
						}
					}
				}
			}
		}
	}
}

func dependencyBoundaryIsInternalNpm(name, ver string) bool {
	if strings.HasPrefix(name, "@compose/") || strings.HasPrefix(name, "@ui/") || strings.HasPrefix(name, "@cad/") ||
		strings.HasPrefix(name, "@puzzle/") || strings.HasPrefix(name, "@framework/") || strings.HasPrefix(name, "@repo/") ||
		strings.HasPrefix(name, "@elements/") || strings.HasPrefix(name, "@coda/") {
		return true
	}
	return strings.HasPrefix(ver, "workspace:") || ver == "link:" || strings.HasPrefix(ver, "file:")
}

func dependencyBoundaryMergeCargoToml(path string, deps map[string]struct{}) {
	data, err := workspace.ReadTextFile(path)
	if err != nil {
		return
	}
	inDeps := false
	for _, line := range strings.Split(data, "\n") {
		trim := strings.TrimSpace(line)
		if strings.HasPrefix(trim, "[") {
			inDeps = strings.HasPrefix(trim, "[dependencies]") || strings.HasPrefix(trim, "[dev-dependencies]") ||
				strings.HasPrefix(trim, "[target.") && strings.Contains(trim, ".dependencies]")
		}
		if !inDeps || trim == "" || strings.HasPrefix(trim, "#") {
			continue
		}
		if strings.Contains(trim, "=") && !strings.HasPrefix(trim, "[") {
			name := strings.TrimSpace(strings.SplitN(trim, "=", 2)[0])
			if name != "" && name != "path" {
				deps[name] = struct{}{}
			}
		}
	}
}

func dependencyBoundaryMergeGoMod(path string, deps map[string]struct{}) {
	data, err := workspace.ReadTextFile(path)
	if err != nil {
		return
	}
	inRequire := false
	for _, line := range strings.Split(data, "\n") {
		trim := strings.TrimSpace(line)
		if strings.HasPrefix(trim, "require (") {
			inRequire = true
			continue
		}
		if inRequire && trim == ")" {
			inRequire = false
			continue
		}
		if strings.HasPrefix(trim, "require ") && !strings.Contains(trim, "// indirect") {
			fields := strings.Fields(trim)
			if len(fields) >= 2 && !strings.HasPrefix(fields[1], "github.com/usalu/semio") {
				deps[fields[1]] = struct{}{}
			}
		}
		if inRequire {
			fields := strings.Fields(trim)
			if len(fields) >= 1 && !strings.HasPrefix(fields[0], "module") {
				deps[fields[0]] = struct{}{}
			}
		}
	}
}

func dependencyBoundaryMergePyProject(path string, deps map[string]struct{}) {
	data, err := workspace.ReadTextFile(path)
	if err != nil {
		return
	}
	inDeps := false
	for _, line := range strings.Split(data, "\n") {
		trim := strings.TrimSpace(line)
		if strings.HasPrefix(trim, "[") {
			inDeps = strings.Contains(trim, "dependencies") || strings.Contains(trim, "optional-dependencies")
		}
		if inDeps && strings.Contains(trim, "=") && !strings.HasPrefix(trim, "[") {
			name := strings.TrimSpace(strings.SplitN(trim, "=", 2)[0])
			if name != "" {
				deps[strings.ReplaceAll(name, "\"", "")] = struct{}{}
			}
		}
	}
}

func dependencyBoundaryMergeCsproj(path string, deps map[string]struct{}) {
	data, err := workspace.ReadTextFile(path)
	if err != nil {
		return
	}
	re := regexp.MustCompile(`<PackageReference\s+Include="([^"]+)"`)
	for _, m := range re.FindAllStringSubmatch(data, -1) {
		if len(m) > 1 {
			deps[m[1]] = struct{}{}
		}
	}
}

func parseThirdPartyImportSpecs(importLine, file string) []string {
	var specs []string
	ext := strings.ToLower(filepath.Ext(file))
	switch ext {
	case ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs":
		re := regexp.MustCompile(`(?:from|import)\s+['"]([^'"]+)['"]`)
		for _, m := range re.FindAllStringSubmatch(importLine, -1) {
			if len(m) > 1 {
				specs = append(specs, m[1])
			}
		}
	case ".go":
		re := regexp.MustCompile(`"([^"]+)"`)
		for _, m := range re.FindAllStringSubmatch(importLine, -1) {
			if len(m) > 1 && !strings.HasPrefix(m[1], "github.com/usalu/semio") {
				specs = append(specs, m[1])
			}
		}
	case ".py":
		re := regexp.MustCompile(`^(?:from|import)\s+([a-zA-Z0-9_\.]+)`)
		if m := re.FindStringSubmatch(strings.TrimSpace(importLine)); len(m) > 1 {
			specs = append(specs, strings.SplitN(m[1], ".", 2)[0])
		}
	case ".cs":
		re := regexp.MustCompile(`using\s+([A-Za-z0-9_.]+)`)
		for _, m := range re.FindAllStringSubmatch(importLine, -1) {
			if len(m) > 1 {
				specs = append(specs, m[1])
			}
		}
	case ".rs":
		re := regexp.MustCompile(`(?:use|extern\s+crate)\s+([a-zA-Z0-9_:]+)`)
		for _, m := range re.FindAllStringSubmatch(importLine, -1) {
			if len(m) > 1 {
				crate := strings.SplitN(m[1], "::", 2)[0]
				specs = append(specs, crate)
			}
		}
	}
	return specs
}

func dependencyBoundaryIsThirdParty(spec string, deps map[string]struct{}) bool {
	if spec == "" || strings.HasPrefix(spec, ".") || strings.HasPrefix(spec, "/") {
		return false
	}
	if _, ok := deps[spec]; ok {
		return true
	}
	if strings.HasPrefix(spec, "@") {
		parts := strings.SplitN(spec, "/", 2)
		if len(parts) == 2 {
			if _, ok := deps[parts[0]+"/"+parts[1]]; ok {
				return true
			}
		}
	}
	for dep := range deps {
		if strings.HasPrefix(spec, dep) || strings.HasPrefix(dep, spec) {
			return true
		}
		if strings.Contains(dep, "/") && strings.HasPrefix(spec, strings.SplitN(dep, "/", 2)[0]) {
			return true
		}
	}
	// Rust/Python root module heuristic
	root := spec
	if i := strings.IndexAny(spec, "./:"); i > 0 {
		root = spec[:i]
	}
	if i := strings.Index(spec, "::"); i > 0 {
		root = spec[:i]
	}
	if _, ok := deps[root]; ok {
		return true
	}
	return false
}

func dependencyBoundaryImportLineNumber(content, importLine string, fallback int) int {
	idx := strings.Index(content, importLine)
	if idx < 0 {
		return fallback
	}
	return strings.Count(content[:idx], "\n") + 1
}

// #endregion Policies

// #region 🧷️BreachCache

// 🧷️BreachCacheEnvelope is the on-disk JSON shape of one breach-cache document: the entity the
// analysis ran over, the script that produced it and the breaches it found.
type BreachCacheEnvelope struct {
	EntityID string         `json:"entityId"`
	Script   string         `json:"script"`
	Breachs  []model.Breach `json:"breachs"`
}

// 🔐️SHA256Hex returns the lowercase hexadecimal SHA-256 digest of one byte string.
func SHA256Hex(data []byte) string {
	digest := sha256.Sum256(data)
	return hex.EncodeToString(digest[:])
}

// 🗜️GzipEncode compresses one payload into a single gzip member.
func GzipEncode(data []byte) []byte {
	var out bytes.Buffer
	writer := gzip.NewWriter(&out)
	if _, err := writer.Write(data); err != nil {
		return nil
	}
	if err := writer.Close(); err != nil {
		return nil
	}
	return out.Bytes()
}

// 📂️GzipDecode MUST return a non-nil error when the member is not a readable gzip stream.
// 📂️GzipDecode inflates one gzip member back into the payload it carries.
func GzipDecode(data []byte) ([]byte, error) {
	reader, err := gzip.NewReader(bytes.NewReader(data))
	if err != nil {
		return nil, err
	}
	defer func() { _ = reader.Close() }()
	payload, err := io.ReadAll(reader)
	if err != nil {
		return nil, err
	}
	return payload, nil
}

// 📄️EncodeBreachCacheJSON MUST return a non-nil error when the document cannot be encoded.
// 📄️EncodeBreachCacheJSON returns the canonical JSON encoding of one breach-cache document:
// declaration order, no HTML escaping and no trailing newline, so the digest is the same everywhere.
func EncodeBreachCacheJSON(envelope BreachCacheEnvelope) (string, error) {
	var out bytes.Buffer
	encoder := json.NewEncoder(&out)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(envelope); err != nil {
		return "", err
	}
	return strings.TrimSuffix(out.String(), "\n"), nil
}

// 📄️ParseBreachCache MUST return a non-nil error when the document is not one envelope.
// 📄️ParseBreachCache returns the breach-cache document one canonical JSON encoding carries.
func ParseBreachCache(document string) (BreachCacheEnvelope, error) {
	var envelope BreachCacheEnvelope
	if err := json.Unmarshal([]byte(document), &envelope); err != nil {
		return BreachCacheEnvelope{}, err
	}
	if envelope.Breachs == nil {
		envelope.Breachs = []model.Breach{}
	}
	return envelope, nil
}

// 🔐️BreachCacheDigest MUST return a non-nil error when the document cannot be encoded.
// 🔐️BreachCacheDigest returns the digest that keys one breach-cache document, over its
// canonical JSON encoding.
func BreachCacheDigest(envelope BreachCacheEnvelope) (string, error) {
	encoded, err := EncodeBreachCacheJSON(envelope)
	if err != nil {
		return "", err
	}
	return SHA256Hex([]byte(encoded)), nil
}

// 🗜️EncodeBreachCache MUST return a non-nil error when the document cannot be encoded.
// 🗜️EncodeBreachCache returns the compressed breach-cache member: gzip over the canonical
// JSON encoding.
func EncodeBreachCache(envelope BreachCacheEnvelope) ([]byte, error) {
	encoded, err := EncodeBreachCacheJSON(envelope)
	if err != nil {
		return nil, err
	}
	return GzipEncode([]byte(encoded)), nil
}

// 📂️DecodeBreachCache MUST return a non-nil error when the member is not one envelope.
// 📂️DecodeBreachCache returns the breach-cache document one compressed member carries.
func DecodeBreachCache(data []byte) (BreachCacheEnvelope, error) {
	payload, err := GzipDecode(data)
	if err != nil {
		return BreachCacheEnvelope{}, err
	}
	return ParseBreachCache(string(payload))
}

func LoadBreachsFromCache(repoRoot string) ([]model.Breach, error) {
	dir := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "breaches")
	entries, err := os.ReadDir(dir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil
		}
		return nil, err
	}
	var all []model.Breach
	for _, e := range entries {
		if e.IsDir() || !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		data, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			return nil, err
		}
		env, err := ParseBreachCache(string(data))
		if err != nil {
			return nil, err
		}
		all = append(all, env.Breachs...)
	}
	return all, nil
}

func BreachMatchesScope(b *model.Breach, s workspace.Scope) bool {
	bp := workspace.NormalizePath(b.Scope)
	switch s.Kind {
	case workspace.ScopeRepo:
		return true
	case workspace.ScopeFile:
		fp := workspace.NormalizePath(s.FilePath)
		return bp == fp || strings.HasPrefix(bp, fp+"#") || strings.HasPrefix(bp, fp+"::")
	case workspace.ScopeFolder:
		fp := strings.TrimSuffix(workspace.NormalizePath(s.FilePath), "/")
		if fp == "" {
			return true
		}
		return bp == fp || strings.HasPrefix(bp, fp+"/")
	case workspace.ScopeTechnology:
		tn := strings.TrimSuffix(workspace.NormalizePath(s.TechnologyName), "/")
		if tn == "" {
			return true
		}
		return bp == tn || strings.HasPrefix(bp, tn+"/")
	case workspace.ScopeSection, workspace.ScopeDefinition:
		fp := workspace.NormalizePath(s.FilePath)
		return strings.HasPrefix(bp, fp)
	default:
		return true
	}
}

// #endregion 🧷️BreachCache

// #region 🔊️Cli

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

// #endregion 🔊️Cli

// #endregion 🚚️Split

// #region 🗃️Sources

// 📄️SourceFile is one repository-relative source the analyzer reads without touching the filesystem.
type SourceFile struct {
	Path    string
	Content string
}

// 🗃️SourceSet is the in-memory workspace port every analyzer entry point runs over.
type SourceSet struct {
	files      []SourceFile
	directives map[string]map[int][]string
}

// 🆕️NewSourceSet MUST parse the ignore directives of every file once and return a usable set.
// 🆕️NewSourceSet creates an in-memory source set from the given files.
func NewSourceSet(files []SourceFile) *SourceSet {
	directives := make(map[string]map[int][]string, len(files))
	for _, file := range files {
		directives[file.Path] = ParseIgnoreDirectives(file.Content)
	}
	return &SourceSet{files: files, directives: directives}
}

// 📄️Files MUST return the analysed files in the order they were given.
func (s *SourceSet) Files() []SourceFile {
	return s.files
}

// 🙈️Directives MUST return the parsed ignore directives of one file.
func (s *SourceSet) Directives(path string) map[int][]string {
	return s.directives[path]
}

// 🧭️PolicyContext MUST return a context that reads only from this source set.
func (s *SourceSet) PolicyContext() *PolicyContext {
	paths := make([]string, 0, len(s.files))
	sources := make(map[string]string, len(s.files))
	for _, file := range s.files {
		paths = append(paths, file.Path)
		sources[file.Path] = file.Content
	}
	ctx := NewPolicyContextWithFiles(workspace.Scope{Kind: workspace.ScopeRepo}, nil, paths)
	ctx.sources = sources
	return ctx
}

// 🔍️Analyze MUST run the header, section and requirements policies in that order.
// 🔍️Analyze returns every breach the ported policies detect over the given sources.
func Analyze(sources *SourceSet) []model.Breach {
	ctx := sources.PolicyContext()
	breachs := []model.Breach{}
	breachs = append(breachs, headerPolicy(ctx)...)
	breachs = append(breachs, sectionPolicy(ctx)...)
	breachs = append(breachs, requirementsPolicy(ctx)...)
	return breachs
}

// 🎛️AnalyzePolicies MUST run exactly the named policy functions, in the order they are named.
// 🎛️AnalyzePolicies narrows analysis to the policy identifiers the caller names.
func AnalyzePolicies(sources *SourceSet, policyIDs []string) []model.Breach {
	ctx := sources.PolicyContext()
	breachs := []model.Breach{}
	for _, id := range policyIDs {
		switch id {
		case "code/header":
			breachs = append(breachs, headerPolicy(ctx)...)
		case "code/section":
			breachs = append(breachs, sectionPolicy(ctx)...)
		case "code/comment":
			breachs = append(breachs, commentPolicy(ctx)...)
		case "code/requirements":
			breachs = append(breachs, requirementsPolicy(ctx)...)
		case "code/emoji":
			breachs = append(breachs, emojiPolicy(ctx)...)
		case "code/docs":
			breachs = append(breachs, docsPolicy(ctx)...)
		}
	}
	return breachs
}

// 🔖️IsAutofixable MUST report the catalog verdict for the statute.
func IsAutofixable(kind model.Statute) bool {
	return kind.Info().Autofixable
}

// #endregion 🗃️Sources

// #region 🩹️Autofix

// 🩹️FixedFile is the outcome of applying every autofix transformation to one source.
type FixedFile struct {
	Path    string
	Content string
	Fixed   []model.Statute
}

// 🩹️Autofix MUST leave the content untouched when nothing is repairable.
// 🩹️Autofix applies every autofixable transformation to one source and reports the statutes it repaired.
func Autofix(source SourceFile) FixedFile {
	content := source.Content
	fixed := []model.Statute{}
	if next, ok := fixNewlineAfterRegion(source.Path, content); ok && next != content {
		content = next
		fixed = append(fixed, model.BreachCodeSectionWrongFormatNewlineAfterRegion)
	}
	return FixedFile{Path: source.Path, Content: content, Fixed: fixed}
}

// 🧹️fixNewlineAfterRegion drops every blank line that follows a region start marker.
func fixNewlineAfterRegion(path string, content string) (string, bool) {
	language := languages.GetLanguage(path)
	if language == nil || !language.SupportsSections() {
		return content, false
	}
	lines := strings.Split(content, "\n")
	drop := make([]bool, len(lines))
	for index, raw := range lines {
		line := strings.TrimSuffix(raw, "\r")
		if matched, _ := language.PolicySectionStartMatch(line); !matched {
			continue
		}
		for next := index + 1; next < len(lines) && strings.TrimSpace(lines[next]) == ""; next++ {
			drop[next] = true
		}
	}
	kept := make([]string, 0, len(lines))
	for index, line := range lines {
		if !drop[index] {
			kept = append(kept, line)
		}
	}
	return strings.Join(kept, "\n"), true
}

// #endregion 🩹️Autofix
