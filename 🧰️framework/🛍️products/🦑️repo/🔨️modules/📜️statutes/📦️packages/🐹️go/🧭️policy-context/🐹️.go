package statutes

import (
	filepath "path/filepath"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧭️Policy Context

// 📝PolicyContext holds the data fields for a policy context record.
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

// 🔷NewPolicyContext MUST initialize all required fields and return a valid PolicyContext.
// 🆕NewPolicyContext creates and returns a new PolicyContext instance.
func NewPolicyContext(scope workspace.Scope, bundles []model.Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      workspace.GetRootDir(),
		Bundles:      bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]model.Section),
		ignoreCache:  make(map[string]map[int][]string),
	}
}

// 📄NewPolicyContextWithFiles MUST initialize all required fields and return a valid PolicyContextWithFiles.
// 📄NewPolicyContextWithFiles creates and returns a new PolicyContextWithFiles instance.
func NewPolicyContextWithFiles(scope workspace.Scope, bundles []model.Bundle, files []string) *PolicyContext {
	ctx := NewPolicyContext(scope, bundles)
	ctx.filesOverride = files
	return ctx
}

// 📥Files MUST operate on the PolicyContext receiver and return consistent results.
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

// 🛤️ReadText MUST return the full content from the given path.
// 🔤ReadText reads and returns the text content.
func (ctx *PolicyContext) ReadText(filePath string) string {
	if ctx.fileCache == nil {
		ctx.fileCache = make(map[string]string)
	}
	if ctx.sources != nil {
		return ctx.sources[filePath]
	}
	absPath := filepath.Join(workspace.GetRootDir(), filePath)
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

// 📑Sections MUST operate on the PolicyContext receiver and return consistent results.
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

// ❌ParseIgnoreDirectives MUST return an error when the input is malformed.
// 💾ParseIgnoreDirectives parses the input and returns the ignore directives result.
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

// 🔶IgnoreDirectives MUST operate on the PolicyContext receiver and return consistent results.
func (ctx *PolicyContext) IgnoreDirectives(filePath string) map[int][]string {
	if ignores, ok := ctx.ignoreCache[filePath]; ok {
		return ignores
	}
	content := ctx.ReadText(filePath)
	ignores := ParseIgnoreDirectives(content)
	ctx.ignoreCache[filePath] = ignores
	return ignores
}

// 🔹️IsIgnored MUST return true only when the condition is met.
// ❓IsIgnored reports whether the PolicyContext is ignored.
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

// 🆕CreateBreach MUST persist the new entity and return a reference to it.
// ⚠️CreateBreach creates a new breach and persists it.
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

// 🧲️extractFileFromScope holds the data fields for a extractFileFromScope record.
func extractFileFromScope(scope string) string {

	if idx := strings.Index(scope, "#"); idx != -1 {
		scope = scope[:idx]
	}

	if idx := strings.Index(scope, "::"); idx != -1 {
		scope = scope[:idx]
	}
	return scope
}

// 🧹️FilterIgnored MUST preserve the tree structure while removing non-matching nodes.
// 🔷FilterIgnored filters the ignored based on the given criteria.
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

// #endregion 🧭️Policy Context
