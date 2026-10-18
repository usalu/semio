package statutes

import (
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

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
