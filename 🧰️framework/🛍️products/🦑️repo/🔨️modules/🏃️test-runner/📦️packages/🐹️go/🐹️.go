// 🏃️ Package testrunner is the Go half of the repo test-runner domain.
//
// It carries the process port every planned invocation goes through and its replaying double,
// alongside the runner behaviour the `go-split` moved out of the pre-split godfile.
package testrunner

import (
	json "encoding/json"
	fmt "fmt"
	fs "io/fs"
	os "os"
	exec "os/exec"
	gopath "path"
	filepath "path/filepath"
	regexp "regexp"
	sort "sort"
	strconv "strconv"
	strings "strings"
	atomic "sync/atomic"
	unicode "unicode"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	todospkg "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region ⚙️Process Port

// 🎬️ProcessRequest is one process a plan wants to run.
type ProcessRequest struct {
	// 🏷️Program is the executable name.
	Program string
	// 📜️Args are the arguments after the program name.
	Args []string
	// 📁️Cwd is the working directory.
	Cwd string
	// 🌱️Env is the environment overlay.
	Env map[string]string
}

// 📤️ProcessOutput is what a process answered.
type ProcessOutput struct {
	// 🔢️Status is the exit status.
	Status int
	// 📝️Stdout is standard output.
	Stdout string
	// 📝️Stderr is standard error.
	Stderr string
}

// ⚙️ProcessRunner is the only door from a plan to an operating system. It is the same shape as the
// `ProcessRunner` trait in the Rust twin and the one 🧩️providers will own for every shelled-out
// command in this repository.
type ProcessRunner interface {
	// ▶️Run executes one process to completion.
	Run(request ProcessRequest) (ProcessOutput, error)
}

// 🎞️RecordedTranscript is one recorded (argv, cwd) → output pair.
type RecordedTranscript struct {
	// 📜️Argv is the argv this transcript answers, program name first.
	Argv []string
	// 📁️Cwd is the working directory it answers for; empty matches any.
	Cwd string
	// 📤️Output is the recorded answer.
	Output ProcessOutput
}

// 🎞️RecordedProcessRunner replays committed transcripts and never touches the machine.
type RecordedProcessRunner struct {
	// 🎞️Transcripts are matched in order.
	Transcripts []RecordedTranscript
}

// ▶️Run answers from the first transcript whose argv matches and whose cwd matches or is empty.
func (runner *RecordedProcessRunner) Run(request ProcessRequest) (ProcessOutput, error) {
	argv := append([]string{request.Program}, request.Args...)
	for _, transcript := range runner.Transcripts {
		if !equalArgv(transcript.Argv, argv) {
			continue
		}
		if transcript.Cwd != "" && normalizeSeparators(transcript.Cwd) != normalizeSeparators(request.Cwd) {
			continue
		}
		return transcript.Output, nil
	}
	return ProcessOutput{}, fmt.Errorf("no recorded transcript for %s", strings.Join(argv, " "))
}

func equalArgv(left []string, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}

func normalizeSeparators(path string) string {
	return strings.ReplaceAll(path, "\\", "/")
}

// #endregion ⚙️Process Port

// #region 🪪️IdentityPending

// 🪪️EntityIdentity is the identifier vocabulary the scope resolver needs from 🪪️identity. It is
// declared here because that module does not export it yet; when it does, this collapses to an alias.
type EntityIdentity interface {
	Flat(text string) string
	PathFromUriPath(uriPath string) string
	IdToUri(id string) string
}

// 🗣️LanguageTable is the file-extension to language mapping the scope resolver needs from 🗣️languages.
type LanguageTable interface {
	LanguageOf(filePath string) (string, bool)
}

// 🪪️PendingIdentity is the ported-from-Go default for both ports.
type PendingIdentity struct{}

// 🔤️Flat keeps alphanumerics and every non-ASCII rune, then lowercases.
func (PendingIdentity) Flat(text string) string { return Flat(text) }

// 🛤️PathFromUriPath decodes %20 in every segment.
func (PendingIdentity) PathFromUriPath(uriPath string) string { return PathFromUriPath(uriPath) }

// 🔗️IdToUri answers the empty string until 🪪️identity owns the entity emoji table.
func (PendingIdentity) IdToUri(string) string { return "" }

// 🗣️LanguageOf answers the language a testable extension maps to.
func (PendingIdentity) LanguageOf(filePath string) (string, bool) {
	return LanguageOfExtension(filePath)
}

// 🔤️Flat keeps alphanumerics and every non-ASCII rune, then lowercases.
func Flat(text string) string {
	var builder strings.Builder
	for _, value := range text {
		if (value >= '0' && value <= '9') || (value >= 'a' && value <= 'z') || (value >= 'A' && value <= 'Z') || value > 0x7F {
			builder.WriteRune(value)
		}
	}
	return strings.ToLower(builder.String())
}

// 🛤️PathFromUriPath decodes %20 per segment.
func PathFromUriPath(uriPath string) string {
	segments := strings.Split(uriPath, "/")
	for index, segment := range segments {
		segments[index] = strings.ReplaceAll(segment, "%20", " ")
	}
	return strings.Join(segments, "/")
}

// 🗣️LanguageOfExtension is the extension table the planner needs, a subset of 🗣️languages limited to
// testable languages.
func LanguageOfExtension(filePath string) (string, bool) {
	lower := strings.ToLower(filePath)
	index := strings.LastIndex(lower, ".")
	if index < 0 {
		return "", false
	}
	switch lower[index+1:] {
	case "go":
		return "go", true
	case "rs":
		return "rust", true
	case "cs":
		return "csharp", true
	case "ts", "tsx":
		return "typescript", true
	case "js", "jsx", "mjs", "cjs":
		return "javascript", true
	case "py":
		return "python", true
	case "rb":
		return "ruby", true
	}
	return "", false
}

// #endregion 🪪️IdentityPending

// #region 🧮️Paths

// 🧭️NormalizeSeparators forward-slashes every path this domain stores or compares.
func NormalizeSeparators(value string) string { return strings.ReplaceAll(value, "\\", "/") }

// 📏️IsAbsolutePath accepts both POSIX roots and Windows drive roots so one fixture runs on every host.
func IsAbsolutePath(value string) bool {
	value = NormalizeSeparators(value)
	if strings.HasPrefix(value, "/") {
		return true
	}
	if len(value) < 3 || value[1] != ':' || value[2] != '/' {
		return false
	}
	return (value[0] >= 'a' && value[0] <= 'z') || (value[0] >= 'A' && value[0] <= 'Z')
}

// 🔗️JoinPath joins and cleans, dropping `.` segments and resolving `..`.
func JoinPath(base string, relative string) string {
	base = NormalizeSeparators(base)
	relative = NormalizeSeparators(relative)
	if base == "" {
		return CleanPath(relative)
	}
	if relative == "" {
		return CleanPath(base)
	}
	return CleanPath(strings.TrimRight(base, "/") + "/" + relative)
}

// 🧹️CleanPath collapses separators and `.`/`..` segments while keeping any root prefix.
func CleanPath(value string) string {
	value = NormalizeSeparators(value)
	prefix, rest := "", value
	if strings.HasPrefix(value, "/") {
		prefix, rest = "/", value[1:]
	} else if IsAbsolutePath(value) {
		prefix, rest = value[:3], value[3:]
	}
	segments := make([]string, 0, 8)
	for _, segment := range strings.Split(rest, "/") {
		switch segment {
		case "", ".":
		case "..":
			if len(segments) > 0 && segments[len(segments)-1] != ".." {
				segments = segments[:len(segments)-1]
			} else if prefix == "" {
				segments = append(segments, "..")
			}
		default:
			segments = append(segments, segment)
		}
	}
	body := strings.Join(segments, "/")
	if prefix != "" {
		return prefix + body
	}
	if body == "" {
		return "."
	}
	return body
}

// 🏷️BaseName is the final segment of a path.
func BaseName(value string) string {
	value = NormalizeSeparators(value)
	trimmed := strings.TrimRight(value, "/")
	if trimmed == "" {
		return value
	}
	if index := strings.LastIndex(trimmed, "/"); index >= 0 {
		return trimmed[index+1:]
	}
	return trimmed
}

// 📁️DirName is everything before the final segment.
func DirName(value string) string {
	cleaned := CleanPath(value)
	index := strings.LastIndex(cleaned, "/")
	if index == 0 {
		return "/"
	}
	if index > 0 {
		return cleaned[:index]
	}
	return "."
}

// ↔️RelativePath is target expressed relative to base, or target unchanged when it is not below it.
func RelativePath(base string, target string) string {
	base = CleanPath(base)
	target = CleanPath(target)
	if base == target {
		return "."
	}
	prefix := strings.TrimRight(base, "/") + "/"
	if strings.HasPrefix(target, prefix) {
		return target[len(prefix):]
	}
	return target
}

// #endregion 🧮️Paths

// #region 🌲️Snapshot

// 🌲️EntryKind says whether a snapshot entry is a file or a directory.
type EntryKind string

// 📄️EntryKindFile is a regular file.
const EntryKindFile EntryKind = "file"

// 📁️EntryKindDirectory is a directory.
const EntryKindDirectory EntryKind = "directory"

// 📄️SnapshotEntry is one path in a planning snapshot, with contents only where a rule reads them.
type SnapshotEntry struct {
	// 🛤️Path is repository-relative or absolute, always forward-slashed.
	Path string `json:"path"`
	// 🌲️Kind is file or directory.
	Kind EntryKind `json:"kind"`
	// 📝️Contents is the text, present only for files a planning rule inspects.
	Contents *string `json:"contents,omitempty"`
}

// 📦️SnapshotBundle is the two bundle fields planning needs, projected from 📐️model's richer Bundle.
type SnapshotBundle struct {
	// 🏷️Name is the technology/bundle name.
	Name string `json:"name"`
	// 🛤️Root is the repository-relative bundle root.
	Root string `json:"root"`
}

// 🌍️FilesystemSnapshot is everything planning is allowed to know about the world.
type FilesystemSnapshot struct {
	// 🏠️Root is the repository root every relative path is joined onto.
	Root string `json:"root"`
	// 🌲️Entries are every path the plan may probe.
	Entries []SnapshotEntry `json:"entries"`
	// 📦️Bundles is the bundle table.
	Bundles []SnapshotBundle `json:"bundles"`
	// 🐍️UvAvailable says whether uv is on PATH.
	UvAvailable bool `json:"uvAvailable"`
}

// 🧭️Absolute is the absolute form of a repository-relative path.
func (snapshot *FilesystemSnapshot) Absolute(path string) string {
	if IsAbsolutePath(path) {
		return CleanPath(path)
	}
	return JoinPath(snapshot.Root, path)
}

// ✅️FileExists is true only for a present regular file.
func (snapshot *FilesystemSnapshot) FileExists(path string) bool {
	wanted := CleanPath(path)
	for _, entry := range snapshot.Entries {
		if entry.Kind == EntryKindFile && CleanPath(entry.Path) == wanted {
			return true
		}
	}
	return false
}

// 📁️DirectoryExists says whether a directory is present.
func (snapshot *FilesystemSnapshot) DirectoryExists(path string) bool {
	wanted := CleanPath(path)
	for _, entry := range snapshot.Entries {
		if entry.Kind == EntryKindDirectory && CleanPath(entry.Path) == wanted {
			return true
		}
	}
	return false
}

// 📝️ReadText answers the contents of a recorded file.
func (snapshot *FilesystemSnapshot) ReadText(path string) (string, bool) {
	wanted := CleanPath(path)
	for _, entry := range snapshot.Entries {
		if entry.Kind != EntryKindFile || CleanPath(entry.Path) != wanted {
			continue
		}
		if entry.Contents == nil {
			return "", false
		}
		return *entry.Contents, true
	}
	return "", false
}

// 🔎️GlobChildren is a non-recursive match against the direct children of a directory.
func (snapshot *FilesystemSnapshot) GlobChildren(directory string, pattern string) []string {
	directory = CleanPath(directory)
	matched := []string{}
	for _, entry := range snapshot.Entries {
		if entry.Kind != EntryKindFile {
			continue
		}
		candidate := CleanPath(entry.Path)
		if DirName(candidate) != directory {
			continue
		}
		if ok, err := gopath.Match(pattern, BaseName(candidate)); err == nil && ok {
			matched = append(matched, candidate)
		}
	}
	sort.Strings(matched)
	return matched
}

// 🚶️WalkFiles is every recorded file at or below root, sorted.
func (snapshot *FilesystemSnapshot) WalkFiles(root string, recursive bool) []string {
	root = CleanPath(root)
	prefix := strings.TrimRight(root, "/") + "/"
	found := []string{}
	for _, entry := range snapshot.Entries {
		if entry.Kind != EntryKindFile {
			continue
		}
		candidate := CleanPath(entry.Path)
		if candidate != root && !strings.HasPrefix(candidate, prefix) {
			continue
		}
		if !recursive && DirName(candidate) != root {
			continue
		}
		found = append(found, candidate)
	}
	sort.Strings(found)
	return found
}

// 📦️BundleByName answers on exact name first, then on flattened equality.
func (snapshot *FilesystemSnapshot) BundleByName(name string) *SnapshotBundle {
	for index := range snapshot.Bundles {
		bundle := &snapshot.Bundles[index]
		if bundle.Name == name || Flat(bundle.Name) == Flat(name) {
			return bundle
		}
	}
	return nil
}

// 📦️BundleByPath answers the longest bundle root that prefixes the path.
func (snapshot *FilesystemSnapshot) BundleByPath(path string) *SnapshotBundle {
	path = CleanPath(NormalizeSeparators(path))
	var best *SnapshotBundle
	bestLength := -1
	for index := range snapshot.Bundles {
		bundle := &snapshot.Bundles[index]
		root := CleanPath(bundle.Root)
		if path != root && !strings.HasPrefix(path, strings.TrimRight(root, "/")+"/") {
			continue
		}
		if len(root) >= bestLength {
			best, bestLength = bundle, len(root)
		}
	}
	return best
}

// 🗣️DetectBundleLanguage probes manifests in a fixed, order-sensitive sequence.
func (snapshot *FilesystemSnapshot) DetectBundleLanguage(bundleRoot string) string {
	absolute := snapshot.Absolute(bundleRoot)
	if snapshot.FileExists(JoinPath(absolute, "go.mod")) {
		return "go"
	}
	if snapshot.FileExists(JoinPath(absolute, "Cargo.toml")) {
		return "rust"
	}
	if len(snapshot.GlobChildren(absolute, "*.csproj")) > 0 {
		return "csharp"
	}
	if len(snapshot.GlobChildren(absolute, "*.sln")) > 0 {
		return "csharp"
	}
	if snapshot.FileExists(JoinPath(absolute, "package.json")) {
		return "typescript"
	}
	if snapshot.FileExists(JoinPath(absolute, "pyproject.toml")) || snapshot.FileExists(JoinPath(absolute, "requirements.txt")) {
		return "python"
	}
	return ""
}

// #endregion 🌲️Snapshot

// #region 🔭️Scope

// 🔭️ScopeKind says how wide a test run reaches.
type ScopeKind string

// 🌍️ScopeKindAll is every bundle with a detectable language.
const ScopeKindAll ScopeKind = "all"

// 🧰️ScopeKindTechnology is every bundle of one technology.
const ScopeKindTechnology ScopeKind = "technology"

// ⌨️ScopeKindBundle is one bundle.
const ScopeKindBundle ScopeKind = "bundle"

// 🥼️ScopeKindFile is one file.
const ScopeKindFile ScopeKind = "file"

// 🔖️ScopeKindSection is one section of one file.
const ScopeKindSection ScopeKind = "section"

// 🧪️ScopeKindDefinition is one test function.
const ScopeKindDefinition ScopeKind = "definition"

// 🧪️TestScope is a resolved scope.
type TestScope struct {
	// 🔭️Kind is the granularity.
	Kind ScopeKind `json:"kind"`
	// 📦️BundleRoot is the bundle root, or the technology name for a technology scope.
	BundleRoot string `json:"bundle_root"`
	// 🥼️FilePath is the file path.
	FilePath string `json:"file_path"`
	// 🔖️Section is the section name.
	Section string `json:"section"`
	// 🧪️TestName is the flat test name.
	TestName string `json:"test_name"`
	// 🗣️Language is the language.
	Language string `json:"language"`
}

// 🌍️AllScope is the scope an empty or unrecognised selector resolves to.
func AllScope() TestScope { return TestScope{Kind: ScopeKindAll} }

// 🔗️ResolveTestScopes answers one `all` scope when no selector is given.
func (snapshot *FilesystemSnapshot) ResolveTestScopes(ids []string, identity EntityIdentity, languages LanguageTable) []TestScope {
	if len(ids) == 0 {
		return []TestScope{AllScope()}
	}
	scopes := make([]TestScope, 0, len(ids))
	for _, raw := range ids {
		scopes = append(scopes, snapshot.ResolveTestScope(raw, identity, languages))
	}
	return scopes
}

// 🔷️ResolveTestScope strips variation selectors, parses `repo://` and follows the `p/` and `f/` routes.
func (snapshot *FilesystemSnapshot) ResolveTestScope(raw string, identity EntityIdentity, languages LanguageTable) TestScope {
	normalized := strings.TrimSpace(strings.Map(stripVariationSelector, raw))
	uri := normalized
	if !strings.HasPrefix(normalized, "repo://") {
		uri = identity.IdToUri(normalized)
	}
	if uri == "" {
		return AllScope()
	}
	path := strings.TrimPrefix(uri, "repo://")
	if rest, ok := strings.CutPrefix(path, "p/"); ok {
		parts := strings.SplitN(rest, "/", 3)
		if len(parts) == 2 {
			return TestScope{Kind: ScopeKindTechnology, BundleRoot: identity.PathFromUriPath(parts[1])}
		}
		if len(parts) >= 3 {
			if bundleRest, ok := strings.CutPrefix(parts[2], "b/"); ok {
				bundleParts := strings.SplitN(bundleRest, "/", 3)
				if len(bundleParts) >= 2 {
					technology := identity.PathFromUriPath(parts[1])
					bundleName := strings.TrimLeft(technology, "@") + "/" + identity.PathFromUriPath(bundleParts[1])
					bundleRoot := bundleName
					if found := snapshot.BundleByName(bundleName); found != nil {
						bundleRoot = found.Root
					}
					if len(bundleParts) == 2 {
						return TestScope{Kind: ScopeKindBundle, BundleRoot: bundleRoot, Language: snapshot.DetectBundleLanguage(bundleRoot)}
					}
					return snapshot.ResolveTestScopeFromBundleSubPath(bundleRoot, bundleParts[2], identity, languages)
				}
			}
		}
	}
	if rest, ok := strings.CutPrefix(path, "f/"); ok {
		parts := strings.SplitN(rest, "/", 2)
		filePath := identity.PathFromUriPath(parts[0])
		language, _ := languages.LanguageOf(filePath)
		if len(parts) == 1 {
			return TestScope{Kind: ScopeKindFile, FilePath: filePath, Language: language}
		}
		return ResolveTestScopeFromFileSubPath(filePath, language, parts[1], identity)
	}
	return AllScope()
}

func stripVariationSelector(value rune) rune {
	if value == '\uFE0E' || value == '\uFE0F' {
		return -1
	}
	return value
}

// 📦️ResolveTestScopeFromBundleSubPath narrows on the `f`, `s`, `d` and `fd` markers in order.
func (snapshot *FilesystemSnapshot) ResolveTestScopeFromBundleSubPath(bundleRoot string, subPath string, identity EntityIdentity, languages LanguageTable) TestScope {
	language := snapshot.DetectBundleLanguage(bundleRoot)
	parts := strings.Split(subPath, "/")
	scope := TestScope{Kind: ScopeKindBundle, BundleRoot: bundleRoot}
	for index := 0; index < len(parts); index++ {
		switch {
		case parts[index] == "f" && index+1 < len(parts):
			scope.FilePath = JoinPath(bundleRoot, identity.PathFromUriPath(parts[index+1]))
			scope.Kind = ScopeKindFile
			index++
		case parts[index] == "s" && index+1 < len(parts):
			scope.Section = identity.PathFromUriPath(parts[index+1])
			scope.Kind = ScopeKindSection
			index++
		case parts[index] == "d" && index+2 < len(parts):
			scope.TestName = identity.PathFromUriPath(parts[index+2])
			scope.Kind = ScopeKindDefinition
			index += 2
		case parts[index] == "fd" && index+2 < len(parts):
			index += 2
		}
	}
	if found, ok := languages.LanguageOf(scope.FilePath); ok {
		language = found
	}
	scope.Language = language
	return scope
}

// 📄️ResolveTestScopeFromFileSubPath narrows a file scope on the `s` and `d` markers.
func ResolveTestScopeFromFileSubPath(filePath string, language string, subPath string, identity EntityIdentity) TestScope {
	parts := strings.Split(subPath, "/")
	scope := TestScope{Kind: ScopeKindFile, FilePath: filePath, Language: language}
	for index := 0; index < len(parts); index++ {
		switch {
		case parts[index] == "s" && index+1 < len(parts):
			scope.Section = identity.PathFromUriPath(parts[index+1])
			scope.Kind = ScopeKindSection
			index++
		case parts[index] == "d" && index+2 < len(parts):
			scope.TestName = identity.PathFromUriPath(parts[index+2])
			scope.Kind = ScopeKindDefinition
			index += 2
		}
	}
	return scope
}

// #endregion 🔭️Scope

// #region 🗺️Planning

// 🏭️Runner is the program a planned invocation runs.
type Runner string

// 🐹️RunnerGo runs `go test`.
const RunnerGo Runner = "go"

// 🦀️RunnerCargo runs `cargo test`.
const RunnerCargo Runner = "cargo"

// 🦀️RunnerCargoNextest runs `cargo nextest run`.
const RunnerCargoNextest Runner = "cargo-nextest"

// 🔷️RunnerDotnet runs `dotnet test`.
const RunnerDotnet Runner = "dotnet"

// 🟦️RunnerNpx runs `npx <js runner>`.
const RunnerNpx Runner = "npx"

// 🟦️RunnerNpm runs `npm test`.
const RunnerNpm Runner = "npm"

// 🐍️RunnerUv runs `uv run pytest`.
const RunnerUv Runner = "uv"

// 🐍️RunnerPytest runs `pytest`.
const RunnerPytest Runner = "pytest"

// 💎️RunnerRspec runs `rspec`.
const RunnerRspec Runner = "rspec"

// 🏷️Program is the executable name a runner invokes.
func (runner Runner) Program() string {
	if runner == RunnerCargoNextest {
		return "cargo"
	}
	return string(runner)
}

// 🎬️RunnerInvocation is one process the plan will run, fully determined before anything executes.
type RunnerInvocation struct {
	// 🏭️Runner is which runner.
	Runner Runner `json:"runner"`
	// 📜️Argv is the full argv, program name first.
	Argv []string `json:"argv"`
	// 📁️Cwd is the absolute working directory.
	Cwd string `json:"cwd"`
	// 🌱️Env is the environment overlay, rendered sorted by key.
	Env map[string]string `json:"env"`
	// 🎯️Filter is the test filter this invocation carries, if any.
	Filter *string `json:"filter,omitempty"`
}

// 🗺️InvocationPlan is an ordered plan plus the refusals the planner reports as errors.
type InvocationPlan struct {
	// 🎬️Invocations are the invocations in execution order.
	Invocations []RunnerInvocation `json:"invocations"`
	// 🚫️Problems is one message per scope the planner refused.
	Problems []string `json:"problems"`
}

// 🗺️NewInvocationPlan is an empty plan whose slices are present rather than null.
func NewInvocationPlan() InvocationPlan {
	return InvocationPlan{Invocations: []RunnerInvocation{}, Problems: []string{}}
}

// ➕️Extend folds another plan in, preserving order.
func (plan *InvocationPlan) Extend(other InvocationPlan) {
	plan.Invocations = append(plan.Invocations, other.Invocations...)
	plan.Problems = append(plan.Problems, other.Problems...)
}

// 🗺️PlanScopes plans every scope in order.
func (snapshot *FilesystemSnapshot) PlanScopes(scopes []TestScope) InvocationPlan {
	plan := NewInvocationPlan()
	for _, scope := range scopes {
		plan.Extend(snapshot.PlanScope(scope))
	}
	return plan
}

// 🗺️PlanScope plans one scope, with the process call replaced by a recorded invocation.
func (snapshot *FilesystemSnapshot) PlanScope(scope TestScope) InvocationPlan {
	switch scope.Kind {
	case ScopeKindTechnology:
		return snapshot.PlanTechnology(scope.BundleRoot)
	case ScopeKindBundle:
		return snapshot.PlanBundle(scope.BundleRoot, scope.Language, "", "")
	case ScopeKindFile:
		return snapshot.PlanFile(scope.FilePath, scope.Language, "")
	case ScopeKindSection:
		return snapshot.PlanSection(scope.FilePath, scope.Language, scope.Section)
	case ScopeKindDefinition:
		return snapshot.PlanDefinition(scope.FilePath, scope.Language, scope.BundleRoot, scope.TestName)
	}
	return snapshot.PlanAll()
}

// 🔹️PlanAll plans every bundle whose language is detectable, in bundle table order.
func (snapshot *FilesystemSnapshot) PlanAll() InvocationPlan {
	plan := NewInvocationPlan()
	for _, bundle := range snapshot.Bundles {
		language := snapshot.DetectBundleLanguage(bundle.Root)
		if language == "" {
			continue
		}
		plan.Extend(snapshot.PlanBundle(bundle.Root, language, "", ""))
	}
	return plan
}

// 🛠️PlanTechnology plans the bundles whose flattened technology segment matches.
func (snapshot *FilesystemSnapshot) PlanTechnology(technologyName string) InvocationPlan {
	wanted := Flat(technologyName)
	plan := NewInvocationPlan()
	for _, bundle := range snapshot.Bundles {
		technology := bundle.Name
		if index := strings.Index(technology, "/"); index >= 0 {
			technology = technology[:index]
		}
		if Flat(technology) != wanted {
			continue
		}
		language := snapshot.DetectBundleLanguage(bundle.Root)
		if language == "" {
			continue
		}
		plan.Extend(snapshot.PlanBundle(bundle.Root, language, "", ""))
	}
	return plan
}

// 🔸️PlanBundle plans one bundle.
func (snapshot *FilesystemSnapshot) PlanBundle(bundleRoot string, language string, fileFilter string, testFilter string) InvocationPlan {
	cwd := snapshot.Absolute(bundleRoot)
	if language == "" {
		language = snapshot.DetectBundleLanguage(bundleRoot)
	}
	switch language {
	case "go":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, "-run", testFilter)
		}
		if fileFilter == "" {
			args = append(args, "./...")
		} else {
			args = append(args, "./"+fileFilter+"/...")
		}
		return singleInvocation(RunnerGo, args, cwd, testFilter)
	case "rust":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, testFilter)
		}
		return singleInvocation(RunnerCargo, args, cwd, testFilter)
	case "csharp":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, "--filter", testFilter)
		}
		return singleInvocation(RunnerDotnet, args, cwd, testFilter)
	case "typescript", "javascript":
		runner, args := snapshot.DetectJSTestRunner(cwd, testFilter)
		return singleInvocation(runner, args, cwd, testFilter)
	case "python":
		args := []string{"run", "pytest"}
		if fileFilter != "" {
			args = append(args, fileFilter)
		}
		if testFilter != "" {
			args = append(args, "-k", testFilter)
		}
		if snapshot.UvAvailable {
			return singleInvocation(RunnerUv, args, cwd, testFilter)
		}
		return singleInvocation(RunnerPytest, args[2:], cwd, testFilter)
	}
	return refusedPlan(fmt.Sprintf("unknown language %q for bundle %s", language, bundleRoot))
}

// 🟦️DetectJSTestRunner lets the manifest text decide, with vitest as the fallback.
func (snapshot *FilesystemSnapshot) DetectJSTestRunner(absoluteRoot string, testFilter string) (Runner, []string) {
	if contents, ok := snapshot.ReadText(JoinPath(absoluteRoot, "package.json")); ok {
		if strings.Contains(contents, "vitest") {
			return RunnerNpx, jsRunnerArgs([]string{"vitest", "run"}, testFilter)
		}
		if strings.Contains(contents, "jest") {
			return RunnerNpx, jsRunnerArgs([]string{"jest"}, testFilter)
		}
		if strings.Contains(contents, "\"test\"") {
			return RunnerNpm, []string{"test"}
		}
	}
	return RunnerNpx, jsRunnerArgs([]string{"vitest", "run"}, testFilter)
}

// 🔺️PlanFile plans one file.
func (snapshot *FilesystemSnapshot) PlanFile(filePath string, language string, testFilter string) InvocationPlan {
	if language == "" {
		language, _ = LanguageOfExtension(filePath)
	}
	absoluteFile := snapshot.Absolute(filePath)
	bundle := snapshot.BundleByPath(filePath)
	if bundle == nil {
		return refusedPlan(fmt.Sprintf("no bundle found for file %s", filePath))
	}
	cwd := JoinPath(snapshot.Root, bundle.Root)
	relativeFile := RelativePath(cwd, absoluteFile)
	switch language {
	case "go":
		args := []string{"test", "-v", "-run"}
		if testFilter == "" {
			args = append(args, ".")
		} else {
			args = append(args, testFilter)
		}
		relativeDir := RelativePath(cwd, DirName(absoluteFile))
		if relativeDir == "." || relativeDir == "" {
			args = append(args, "./...")
		} else {
			args = append(args, "./"+relativeDir+"/...")
		}
		return singleInvocation(RunnerGo, args, cwd, testFilter)
	case "python":
		args := []string{"run", "pytest", relativeFile}
		if testFilter != "" {
			args = append(args, "-k", testFilter)
		}
		if snapshot.UvAvailable {
			return singleInvocation(RunnerUv, args, cwd, testFilter)
		}
		return singleInvocation(RunnerPytest, args[2:], cwd, testFilter)
	case "typescript", "javascript":
		runner, args := snapshot.DetectJSTestRunner(cwd, testFilter)
		return singleInvocation(runner, append(args, relativeFile), cwd, testFilter)
	case "csharp":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, "--filter", testFilter)
		}
		return singleInvocation(RunnerDotnet, args, cwd, testFilter)
	case "rust":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, testFilter)
		}
		return singleInvocation(RunnerCargo, args, cwd, testFilter)
	}
	return refusedPlan(fmt.Sprintf("unsupported language %q for file test", language))
}

// 🔖️PlanSection narrows to a section for Go only; every other language falls back to the file.
func (snapshot *FilesystemSnapshot) PlanSection(filePath string, language string, section string) InvocationPlan {
	if language == "" {
		language, _ = LanguageOfExtension(filePath)
	}
	absoluteFile := snapshot.Absolute(filePath)
	bundle := snapshot.BundleByPath(filePath)
	if bundle == nil {
		return refusedPlan(fmt.Sprintf("no bundle found for file %s", filePath))
	}
	if language != "go" {
		return snapshot.PlanFile(filePath, language, "")
	}
	contents, _ := snapshot.ReadText(absoluteFile)
	pattern := CollectGoTestsInSection(contents, section)
	if pattern == "" {
		return refusedPlan(fmt.Sprintf("no tests found in section %q of %s", section, filePath))
	}
	return singleInvocation(RunnerGo, []string{"test", "-v", "-run", pattern, "./..."}, JoinPath(snapshot.Root, bundle.Root), pattern)
}

// 🧪️PlanDefinition plans one test function.
func (snapshot *FilesystemSnapshot) PlanDefinition(filePath string, language string, bundleRoot string, testName string) InvocationPlan {
	if language == "" {
		language, _ = LanguageOfExtension(filePath)
	}
	if bundleRoot == "" {
		if bundle := snapshot.BundleByPath(filePath); bundle != nil {
			bundleRoot = bundle.Root
		}
	}
	cwd := snapshot.Absolute(bundleRoot)
	contents, _ := snapshot.ReadText(JoinPath(snapshot.Root, filePath))
	original := ResolveTestFunctionName(contents, testName)
	if original == "" {
		original = UnflattenTestName(testName)
	}
	switch language {
	case "go":
		return singleInvocation(RunnerGo, []string{"test", "-v", "-run", "^" + original + "$", "./..."}, cwd, original)
	case "python":
		args := []string{"run", "pytest", "-k", testName}
		if snapshot.UvAvailable {
			return singleInvocation(RunnerUv, args, cwd, testName)
		}
		return singleInvocation(RunnerPytest, args[2:], cwd, testName)
	case "csharp":
		return singleInvocation(RunnerDotnet, []string{"test", "--filter", "FullyQualifiedName~" + original}, cwd, original)
	case "rust":
		return singleInvocation(RunnerCargo, []string{"test", testName}, cwd, testName)
	}
	return snapshot.PlanBundle(bundleRoot, language, "", testName)
}

// ✂️StripLeadingGrapheme drops one leading non-ASCII grapheme cluster — the emoji plus the variation
// selectors, skin-tone modifiers and zero-width-joined runes that belong to it — so that `🚀Gamma`
// and `🚀️Gamma` both name the section `Gamma`.
func StripLeadingGrapheme(value string) string {
	runes := []rune(value)
	index := 0
	for index < len(runes) && runes[index] <= 0x7F {
		index++
	}
	if index >= len(runes) {
		return value
	}
	index++
	for index < len(runes) {
		rune := runes[index]
		joined := rune == 0x200D
		modifier := (rune >= 0xFE00 && rune <= 0xFE0F) || (rune >= 0x1F3FB && rune <= 0x1F3FF) || (rune >= 0x0300 && rune <= 0x036F) || rune == 0x20E3
		if !joined && !modifier {
			break
		}
		index++
		if joined && index < len(runes) {
			index++
		}
	}
	return strings.TrimSpace(string(runes[index:]))
}

// 📑️CollectGoTestsInSection scans region markers and yields an anchored `-run` pattern.
func CollectGoTestsInSection(content string, sectionName string) string {
	flatSection := Flat(sectionName)
	inSection := false
	names := []string{}
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "//") {
			if index := strings.Index(trimmed, "#region"); index >= 0 {
				region := StripLeadingGrapheme(strings.TrimSpace(trimmed[index+len("#region"):]))
				if Flat(region) == flatSection {
					inSection = true
					continue
				} else if inSection {
					inSection = false
				}
			} else if strings.Contains(trimmed, "#endregion") && inSection {
				break
			}
		}
		if inSection {
			if name, ok := goTestFunctionName(line); ok {
				names = append(names, name)
			}
		}
	}
	if len(names) == 0 {
		return ""
	}
	return "^(" + strings.Join(names, "|") + ")$"
}

// 🎯️ResolveTestFunctionName answers the declared name whose flat form matches.
func ResolveTestFunctionName(content string, flatName string) string {
	for _, name := range declaredTestFunctionNames(content) {
		if Flat(name) == flatName {
			return name
		}
	}
	return ""
}

// 🧱️UnflattenTestName re-cases a flat name from the known prefixes.
func UnflattenTestName(value string) string {
	for _, prefix := range []string{"testbenchmark", "testfuzz", "benchmark", "fuzz", "test"} {
		rest, ok := strings.CutPrefix(value, prefix)
		if !ok || rest == "" {
			continue
		}
		return strings.ToUpper(prefix[:1]) + prefix[1:] + strings.ToUpper(rest[:1]) + rest[1:]
	}
	if value == "" {
		return ""
	}
	return strings.ToUpper(value[:1]) + value[1:]
}

func jsRunnerArgs(args []string, testFilter string) []string {
	if testFilter == "" {
		return args
	}
	return append(args, "-t", testFilter)
}

func singleInvocation(runner Runner, args []string, cwd string, filter string) InvocationPlan {
	argv := append([]string{runner.Program()}, args...)
	invocation := RunnerInvocation{Runner: runner, Argv: argv, Cwd: cwd, Env: map[string]string{}}
	if filter != "" {
		value := filter
		invocation.Filter = &value
	}
	return InvocationPlan{Invocations: []RunnerInvocation{invocation}, Problems: []string{}}
}

func refusedPlan(problem string) InvocationPlan {
	return InvocationPlan{Invocations: []RunnerInvocation{}, Problems: []string{problem}}
}

func identifierLength(value string) int {
	for index := 0; index < len(value); index++ {
		letter := value[index]
		if (letter >= '0' && letter <= '9') || (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z') || letter == '_' {
			continue
		}
		return index
	}
	return len(value)
}

func goTestFunctionName(line string) (string, bool) {
	rest, ok := strings.CutPrefix(line, "func ")
	if !ok {
		return "", false
	}
	rest, ok = strings.CutPrefix(rest, "Test")
	if !ok {
		return "", false
	}
	length := identifierLength(rest)
	if strings.HasPrefix(rest[length:], "(") {
		return "Test" + rest[:length], true
	}
	return "", false
}

func declaredTestFunctionNames(content string) []string {
	found := []string{}
	rest := content
	for {
		index := strings.Index(rest, "func ")
		if index < 0 {
			return found
		}
		tail := rest[index+len("func "):]
		for _, prefix := range []string{"Test", "Benchmark", "Fuzz"} {
			after, ok := strings.CutPrefix(tail, prefix)
			if !ok {
				continue
			}
			length := identifierLength(after)
			if strings.HasPrefix(after[length:], "(") {
				found = append(found, prefix+after[:length])
			}
			break
		}
		rest = tail
	}
}

// #endregion 🗺️Planning

// #region ⚙️Execution

// 🛑️CancellationToken is a cooperative cancellation flag shared with whoever drives the run.
type CancellationToken struct {
	flag atomic.Bool
}

// 🆕️NewCancellationToken answers a token that is not cancelled.
func NewCancellationToken() *CancellationToken { return &CancellationToken{} }

// 🛑️Cancel requests cancellation; every later invocation is skipped.
func (token *CancellationToken) Cancel() { token.flag.Store(true) }

// ❓️IsCancelled says whether cancellation was requested.
func (token *CancellationToken) IsCancelled() bool { return token != nil && token.flag.Load() }

// 📣️ProgressEventKind names what the caller is being told.
type ProgressEventKind string

// 🚦️ProgressPlanStarted says the plan is about to start.
const ProgressPlanStarted ProgressEventKind = "plan-started"

// ▶️ProgressInvocationStarted says one invocation is starting.
const ProgressInvocationStarted ProgressEventKind = "invocation-started"

// ⏹️ProgressInvocationFinished says one invocation finished.
const ProgressInvocationFinished ProgressEventKind = "invocation-finished"

// 🛑️ProgressCancelled says cancellation was observed before the next invocation started.
const ProgressCancelled ProgressEventKind = "cancelled"

// 🏁️ProgressPlanFinished says the plan is done.
const ProgressPlanFinished ProgressEventKind = "plan-finished"

// 📣️ProgressEvent is one entry of the progress stream a run reports.
type ProgressEvent struct {
	// 📣️Event is which of the five kinds this is.
	Event ProgressEventKind
	// 🔢️Total is the number of invocations in the plan.
	Total int
	// 🔢️Index is the zero-based position in the plan.
	Index int
	// 📜️Argv is the argv of the invocation.
	Argv []string
	// 🚦️Status is how the invocation ended.
	Status RunStatus
	// 🔢️Failed is the number of failing tests the invocation reported.
	Failed int
	// 🔢️Completed is the number of invocations that had already completed.
	Completed int
}

// 📣️MarshalJSON renders only the fields the event's kind carries.
func (event ProgressEvent) MarshalJSON() ([]byte, error) {
	switch event.Event {
	case ProgressPlanStarted:
		return json.Marshal(map[string]any{"event": string(event.Event), "total": event.Total})
	case ProgressInvocationStarted:
		argv := event.Argv
		if argv == nil {
			argv = []string{}
		}
		return json.Marshal(map[string]any{"event": string(event.Event), "index": event.Index, "argv": argv})
	case ProgressInvocationFinished:
		return json.Marshal(map[string]any{"event": string(event.Event), "index": event.Index, "status": string(event.Status), "failed": event.Failed})
	}
	return json.Marshal(map[string]any{"event": string(event.Event), "completed": event.Completed})
}

// 📊️ExecutionReport is what a whole plan produced.
type ExecutionReport struct {
	// 📊️Outcomes is one outcome per completed invocation, in plan order.
	Outcomes []TestOutcome `json:"outcomes"`
	// 🛑️Cancelled says whether the run stopped early because cancellation was requested.
	Cancelled bool `json:"cancelled"`
	// 🔢️Completed is the number of invocations that completed.
	Completed int `json:"completed"`
	// 🔢️Total is the number of invocations the plan contained.
	Total int `json:"total"`
	// 🚫️Problems are the planning refusals plus any runner that could not be started.
	Problems []string `json:"problems"`
}

// ▶️ExecutePlan runs a plan through the port, reporting progress and honouring cancellation between
// invocations.
func ExecutePlan(plan InvocationPlan, runner ProcessRunner, cancellation *CancellationToken, progress func(ProgressEvent)) ExecutionReport {
	total := len(plan.Invocations)
	report := ExecutionReport{Outcomes: []TestOutcome{}, Total: total, Problems: append([]string{}, plan.Problems...)}
	announce := func(event ProgressEvent) {
		if progress != nil {
			progress(event)
		}
	}
	announce(ProgressEvent{Event: ProgressPlanStarted, Total: total})
	for index, invocation := range plan.Invocations {
		if cancellation.IsCancelled() {
			report.Cancelled = true
			announce(ProgressEvent{Event: ProgressCancelled, Completed: report.Completed})
			return report
		}
		announce(ProgressEvent{Event: ProgressInvocationStarted, Index: index, Argv: invocation.Argv})
		request := ProcessRequest{Args: []string{}, Cwd: invocation.Cwd, Env: invocation.Env}
		if len(invocation.Argv) > 0 {
			request.Program = invocation.Argv[0]
			request.Args = invocation.Argv[1:]
		}
		outcome := TestOutcome{Runner: invocation.Runner, Status: RunStatusNotRun, Tests: []TestCaseOutcome{}}
		if output, err := runner.Run(request); err != nil {
			report.Problems = append(report.Problems, err.Error())
		} else {
			outcome = ParseOutcome(invocation.Runner, output)
		}
		announce(ProgressEvent{Event: ProgressInvocationFinished, Index: index, Status: outcome.Status, Failed: outcome.Totals.Failed})
		report.Outcomes = append(report.Outcomes, outcome)
		report.Completed++
	}
	announce(ProgressEvent{Event: ProgressPlanFinished, Completed: report.Completed})
	return report
}

// #endregion ⚙️Execution

// #region 📊️Parsing

// 🚦️TestStatus is how one test ended.
type TestStatus string

// ✅️TestStatusPassed is a passing test.
const TestStatusPassed TestStatus = "passed"

// ❌️TestStatusFailed is a failing test.
const TestStatusFailed TestStatus = "failed"

// ⏭️TestStatusSkipped is a skipped, ignored or pending test.
const TestStatusSkipped TestStatus = "skipped"

// 🚦️RunStatus is how one invocation ended.
type RunStatus string

// ✅️RunStatusPassed means every test passed.
const RunStatusPassed RunStatus = "passed"

// ❌️RunStatusFailed means at least one test failed, or the runner exited non-zero.
const RunStatusFailed RunStatus = "failed"

// 🛑️RunStatusCancelled means the invocation was cancelled before it produced a verdict.
const RunStatusCancelled RunStatus = "cancelled"

// ⚪️RunStatusNotRun means the invocation never started.
const RunStatusNotRun RunStatus = "not-run"

// 🧪️TestCaseOutcome is one test, normalised across every runner dialect.
type TestCaseOutcome struct {
	// 🏷️Name is the test name as the runner reported it.
	Name string `json:"name"`
	// 📦️Suite is the package, file or suite the runner attributed it to.
	Suite string `json:"suite,omitempty"`
	// 🚦️Status is the verdict.
	Status TestStatus `json:"status"`
	// ⏱️DurationMs is the duration in milliseconds where the runner reports one.
	DurationMs *float64 `json:"duration_ms,omitempty"`
	// 💬️Message is the failure text where the runner reports one.
	Message string `json:"message,omitempty"`
}

// 🔢️TestTotals are counts, always recomputed from the cases so a report's own summary cannot drift.
type TestTotals struct {
	// ✅️Passed is the number of passing tests.
	Passed int `json:"passed"`
	// ❌️Failed is the number of failing tests.
	Failed int `json:"failed"`
	// ⏭️Skipped is the number of skipped tests.
	Skipped int `json:"skipped"`
	// 🔢️Total is the number of reported tests.
	Total int `json:"total"`
}

// 📊️TestOutcome is one invocation's result in the one model every runner folds into.
type TestOutcome struct {
	// 🏭️Runner is which runner produced it.
	Runner Runner `json:"runner"`
	// 🚦️Status is the overall verdict.
	Status RunStatus `json:"status"`
	// 🧪️Tests is every reported test, in report order.
	Tests []TestCaseOutcome `json:"tests"`
	// 🔢️Totals are the counts.
	Totals TestTotals `json:"totals"`
	// 🔢️ExitStatus is the runner exit status where one was observed.
	ExitStatus *int `json:"exit_status,omitempty"`
}

// 📊️ParseOutcome dispatches a runner's native report to its parser.
func ParseOutcome(runner Runner, output ProcessOutput) TestOutcome {
	switch runner {
	case RunnerCargo:
		return ParseCargoTest(output)
	case RunnerCargoNextest:
		return ParseCargoNextest(output)
	case RunnerDotnet:
		return ParseDotnetTest(output)
	case RunnerNpx, RunnerNpm:
		return ParseVitestJSON(output)
	case RunnerUv, RunnerPytest:
		return ParsePytest(output)
	case RunnerRspec:
		return ParseRspecJSON(output)
	}
	return ParseGoTestJSON(output)
}

// 🐹️ParseGoTestJSON reads `go test -json`: one JSON object per line, verdicts on pass, fail and skip.
func ParseGoTestJSON(output ProcessOutput) TestOutcome {
	messages := map[string]string{}
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		var event map[string]any
		if json.Unmarshal([]byte(strings.TrimSpace(line)), &event) != nil || event == nil {
			continue
		}
		action, _ := event["Action"].(string)
		pkg, _ := event["Package"].(string)
		name, ok := event["Test"].(string)
		if !ok {
			continue
		}
		key := pkg + "::" + name
		if action == "output" {
			text, _ := event["Output"].(string)
			messages[key] += text
			continue
		}
		var status TestStatus
		switch action {
		case "pass":
			status = TestStatusPassed
		case "fail":
			status = TestStatusFailed
		case "skip":
			status = TestStatusSkipped
		default:
			continue
		}
		var durationMs *float64
		if elapsed, ok := event["Elapsed"].(float64); ok {
			value := elapsed * 1000
			durationMs = &value
		}
		message := ""
		if status == TestStatusFailed {
			message = strings.TrimRightFunc(messages[key], unicode.IsSpace)
		}
		tests = append(tests, TestCaseOutcome{Name: name, Suite: pkg, Status: status, DurationMs: durationMs, Message: message})
	}
	return finishOutcome(RunnerGo, tests, output)
}

// 🟦️ParseVitestJSON reads vitest's JSON reporter, the shape jest's reporter also emits.
func ParseVitestJSON(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	var report map[string]any
	if json.Unmarshal([]byte(jsonBody(output.Stdout)), &report) == nil {
		files, _ := report["testResults"].([]any)
		for _, entry := range files {
			file, _ := entry.(map[string]any)
			suite, _ := file["name"].(string)
			assertions, _ := file["assertionResults"].([]any)
			for _, item := range assertions {
				assertion, _ := item.(map[string]any)
				name, ok := assertion["fullName"].(string)
				if !ok {
					name, _ = assertion["title"].(string)
				}
				status := TestStatusSkipped
				switch verdict, _ := assertion["status"].(string); verdict {
				case "passed":
					status = TestStatusPassed
				case "failed":
					status = TestStatusFailed
				}
				var durationMs *float64
				if duration, ok := assertion["duration"].(float64); ok {
					value := duration
					durationMs = &value
				}
				failures, _ := assertion["failureMessages"].([]any)
				parts := make([]string, 0, len(failures))
				for _, failure := range failures {
					if text, ok := failure.(string); ok {
						parts = append(parts, text)
					}
				}
				tests = append(tests, TestCaseOutcome{Name: name, Suite: suite, Status: status, DurationMs: durationMs, Message: strings.Join(parts, "\n")})
			}
		}
	}
	return finishOutcome(RunnerNpx, tests, output)
}

// 🐍️ParsePytest reads pytest's verbose terminal report: `<nodeid> <VERDICT>` lines.
func ParsePytest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		trimmed := strings.TrimRightFunc(line, unicode.IsSpace)
		node, tail, ok := strings.Cut(trimmed, " ")
		if !ok || !strings.Contains(node, "::") {
			continue
		}
		verdict := ""
		if fields := strings.Fields(tail); len(fields) > 0 {
			verdict = fields[0]
		}
		var status TestStatus
		switch verdict {
		case "PASSED", "XPASS":
			status = TestStatusPassed
		case "FAILED", "ERROR":
			status = TestStatusFailed
		case "SKIPPED", "XFAIL":
			status = TestStatusSkipped
		default:
			continue
		}
		suite, name, _ := strings.Cut(node, "::")
		tests = append(tests, TestCaseOutcome{Name: name, Suite: suite, Status: status})
	}
	return finishOutcome(RunnerPytest, tests, output)
}

// 🦀️ParseCargoTest reads libtest's text report: `test <path> ... ok|FAILED|ignored`.
func ParseCargoTest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		rest, ok := strings.CutPrefix(strings.TrimSpace(line), "test ")
		if !ok {
			continue
		}
		name, verdict, ok := strings.Cut(rest, " ... ")
		if !ok {
			continue
		}
		var status TestStatus
		switch trimmed := strings.TrimSpace(verdict); {
		case trimmed == "ok":
			status = TestStatusPassed
		case trimmed == "FAILED":
			status = TestStatusFailed
		case strings.HasPrefix(trimmed, "ignored"):
			status = TestStatusSkipped
		default:
			continue
		}
		message := ""
		if status == TestStatusFailed {
			message = cargoFailureBody(output.Stdout, strings.TrimSpace(name))
		}
		tests = append(tests, TestCaseOutcome{Name: strings.TrimSpace(name), Status: status, Message: message})
	}
	return finishOutcome(RunnerCargo, tests, output)
}

// 🦀️ParseCargoNextest reads cargo-nextest's report: `        PASS [   0.012s] <crate> <test>`.
func ParseCargoNextest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range append(textLines(output.Stdout), textLines(output.Stderr)...) {
		verdict, rest, ok := strings.Cut(strings.TrimSpace(line), " ")
		if !ok {
			continue
		}
		var status TestStatus
		switch verdict {
		case "PASS":
			status = TestStatusPassed
		case "FAIL", "TRY":
			status = TestStatusFailed
		case "SKIP":
			status = TestStatusSkipped
		default:
			continue
		}
		rest = strings.TrimSpace(rest)
		closing := strings.Index(rest, "]")
		if closing < 0 {
			continue
		}
		var durationMs *float64
		text := strings.TrimSpace(strings.TrimRight(strings.TrimSpace(strings.TrimLeft(rest[:closing], "[")), "s"))
		if seconds, err := strconv.ParseFloat(text, 64); err == nil {
			value := seconds * 1000
			durationMs = &value
		}
		identifier := strings.TrimSpace(rest[closing+1:])
		suite, name, ok := strings.Cut(identifier, " ")
		if !ok {
			suite, name = "", identifier
		}
		tests = append(tests, TestCaseOutcome{Name: strings.TrimSpace(name), Suite: strings.TrimSpace(suite), Status: status, DurationMs: durationMs})
	}
	return finishOutcome(RunnerCargoNextest, tests, output)
}

// 🔷️ParseDotnetTest reads `dotnet test`'s console logger: `  Passed <name> [1 ms]`.
func ParseDotnetTest(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	for _, line := range textLines(output.Stdout) {
		verdict, rest, ok := strings.Cut(strings.TrimSpace(line), " ")
		if !ok {
			continue
		}
		var status TestStatus
		switch verdict {
		case "Passed":
			status = TestStatusPassed
		case "Failed":
			status = TestStatusFailed
		case "Skipped":
			status = TestStatusSkipped
		default:
			continue
		}
		rest = strings.TrimSpace(rest)
		if rest == "" || strings.HasPrefix(rest, "!") || strings.HasPrefix(rest, "-") {
			continue
		}
		name, durationMs := rest, (*float64)(nil)
		if index := strings.LastIndex(rest, " ["); index >= 0 {
			name = strings.TrimSpace(rest[:index])
			durationMs = parseDotnetDuration(strings.TrimRight(rest[index+2:], "]"))
		}
		tests = append(tests, TestCaseOutcome{Name: name, Status: status, DurationMs: durationMs})
	}
	return finishOutcome(RunnerDotnet, tests, output)
}

// 💎️ParseRspecJSON reads RSpec's `--format json` document.
func ParseRspecJSON(output ProcessOutput) TestOutcome {
	tests := []TestCaseOutcome{}
	var report map[string]any
	if json.Unmarshal([]byte(jsonBody(output.Stdout)), &report) == nil {
		examples, _ := report["examples"].([]any)
		for _, entry := range examples {
			example, _ := entry.(map[string]any)
			name, ok := example["full_description"].(string)
			if !ok {
				name, _ = example["description"].(string)
			}
			status := TestStatusSkipped
			switch verdict, _ := example["status"].(string); verdict {
			case "passed":
				status = TestStatusPassed
			case "failed":
				status = TestStatusFailed
			}
			message := ""
			if exception, ok := example["exception"].(map[string]any); ok {
				message, _ = exception["message"].(string)
			}
			var durationMs *float64
			if runTime, ok := example["run_time"].(float64); ok {
				value := runTime * 1000
				durationMs = &value
			}
			suite, _ := example["file_path"].(string)
			tests = append(tests, TestCaseOutcome{Name: name, Suite: suite, Status: status, DurationMs: durationMs, Message: message})
		}
	}
	return finishOutcome(RunnerRspec, tests, output)
}

func finishOutcome(runner Runner, tests []TestCaseOutcome, output ProcessOutput) TestOutcome {
	totals := TestTotals{Total: len(tests)}
	for _, test := range tests {
		switch test.Status {
		case TestStatusPassed:
			totals.Passed++
		case TestStatusFailed:
			totals.Failed++
		case TestStatusSkipped:
			totals.Skipped++
		}
	}
	status := RunStatusPassed
	if totals.Failed > 0 || output.Status != 0 {
		status = RunStatusFailed
	}
	exitStatus := output.Status
	return TestOutcome{Runner: runner, Status: status, Tests: tests, Totals: totals, ExitStatus: &exitStatus}
}

func textLines(text string) []string {
	if text == "" {
		return nil
	}
	lines := strings.Split(text, "\n")
	if lines[len(lines)-1] == "" {
		lines = lines[:len(lines)-1]
	}
	for index, line := range lines {
		lines[index] = strings.TrimSuffix(line, "\r")
	}
	return lines
}

func jsonBody(text string) string {
	start := strings.Index(text, "{")
	if start < 0 {
		start = 0
	}
	end := len(text)
	if last := strings.LastIndex(text, "}"); last >= 0 {
		end = last + 1
	}
	if start < end {
		return text[start:end]
	}
	return text
}

func parseDotnetDuration(value string) *float64 {
	number, unit, ok := strings.Cut(strings.TrimSpace(value), " ")
	if !ok {
		return nil
	}
	amount, err := strconv.ParseFloat(number, 64)
	if err != nil {
		return nil
	}
	switch unit {
	case "ms":
		return &amount
	case "s":
		scaled := amount * 1000
		return &scaled
	case "us":
		scaled := amount / 1000
		return &scaled
	}
	return nil
}

func cargoFailureBody(stdout string, name string) string {
	header := "---- " + name + " stdout ----"
	index := strings.Index(stdout, header)
	if index < 0 {
		return ""
	}
	rest := stdout[index+len(header):]
	end := strings.Index(rest, "\n----")
	if end < 0 {
		end = len(rest)
	}
	return strings.TrimSpace(rest[:end])
}

// #endregion 📊️Parsing

// #region 🔎️Resolution

// 🧰️PythonTestRunnerBins are the Python test runner executables the resolver recognises.
var PythonTestRunnerBins = []string{"pytest", "py.test", "nosetests", "nose2"}

// 🧰️JavaScriptTestRunnerBins are the JavaScript test runner executables the resolver recognises.
var JavaScriptTestRunnerBins = []string{"jest", "vitest", "mocha", "jasmine", "ava", "tape", "qunit", "karma"}

// 🏷️CommandKind is the tool kind classification restricted to the four verdicts the resolver needs.
type CommandKind string

// 🧪️CommandKindTest is a test command.
const CommandKindTest CommandKind = "test"

// 🔎️CommandKindCodeSearch is a read-only inspection command.
const CommandKindCodeSearch CommandKind = "code-search"

// ✏️CommandKindCodeEdit is a command that writes files.
const CommandKindCodeEdit CommandKind = "code-edit"

// 🖥️CommandKindTerminal is anything else.
const CommandKindTerminal CommandKind = "terminal"

// ✂️SplitCommandSegments splits on `;`, `&&`, `|` and `||` outside quotes.
func SplitCommandSegments(command string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return []string{}
	}
	segments := []string{}
	current := strings.Builder{}
	quote := rune(0)
	runes := []rune(command)
	for index := 0; index < len(runes); index++ {
		value := runes[index]
		if quote != 0 {
			current.WriteRune(value)
			if value == quote {
				quote = 0
			}
			continue
		}
		if value == '\'' || value == '"' {
			quote = value
			current.WriteRune(value)
			continue
		}
		if value == ';' {
			segments = flushSegment(segments, &current)
			continue
		}
		if value == '&' && index+1 < len(runes) && runes[index+1] == '&' {
			segments = flushSegment(segments, &current)
			index++
			continue
		}
		if value == '|' {
			segments = flushSegment(segments, &current)
			if index+1 < len(runes) && runes[index+1] == '|' {
				index++
			}
			continue
		}
		current.WriteRune(value)
	}
	return flushSegment(segments, &current)
}

// 🏷️ClassifyCommandKind answers which of the four kinds a shell command is.
func ClassifyCommandKind(command string) CommandKind {
	trimmed := strings.TrimSpace(command)
	if trimmed == "" {
		return CommandKindTerminal
	}
	if segments := SplitCommandSegments(trimmed); len(segments) > 1 {
		best := CommandKindTerminal
		for _, segment := range segments {
			kind := ClassifyCommandKind(segment)
			if kind == CommandKindTest || kind == CommandKindCodeEdit {
				return kind
			}
			if kind == CommandKindCodeSearch {
				best = CommandKindCodeSearch
			}
		}
		if best != CommandKindTerminal {
			return best
		}
	}
	fields := strings.Fields(trimmed)
	if len(fields) == 0 {
		return CommandKindTerminal
	}
	base := BaseName(fields[0])
	if base == "test" || base == "[" {
		return CommandKindCodeSearch
	}
	if strings.HasPrefix(base, "phpunit") || containsWord(commandTestBins, base) {
		return CommandKindTest
	}
	if strings.Contains(trimmed, "cargo nextest") {
		return CommandKindTest
	}
	for _, candidate := range commandTestNeedles {
		if strings.Contains(trimmed, candidate) {
			return CommandKindTest
		}
	}
	for _, prefix := range commandTestPrefixes {
		if strings.HasPrefix(trimmed, prefix) {
			return CommandKindTest
		}
	}
	if strings.Contains(trimmed, "pytest") || strings.Contains(trimmed, "unittest") || strings.Contains(trimmed, "phpunit") {
		return CommandKindTest
	}
	if containsWord(commandSearchBins, base) {
		return CommandKindCodeSearch
	}
	if containsWord(commandSearchTailBins, base) && !((base == "sed" || base == "awk" || base == "gawk") && strings.Contains(trimmed, "-i")) {
		return CommandKindCodeSearch
	}
	if containsWord(commandEditBins, base) {
		return CommandKindCodeEdit
	}
	if (base == "sed" || base == "awk" || base == "gawk" || base == "mawk" || base == "nawk") && strings.Contains(trimmed, "-i") {
		return CommandKindCodeEdit
	}
	return CommandKindTerminal
}

// 🧲️ExtractTestSegment answers the test segment of a command and the `cd` that precedes it. It is the
// twin of the Rust `extract_test_segment_from_command`, renamed because the 🚚️Split region still
// exports the pre-split `ExtractTestSegmentFromCommand`.
func ExtractTestSegment(command string) (string, string) {
	command = strings.TrimSpace(command)
	if command == "" {
		return "", ""
	}
	segments := SplitCommandSegments(command)
	if len(segments) == 0 {
		segments = []string{command}
	}
	hasComposite := len(segments) > 1
	cwd := ""
	for _, segment := range segments {
		segment = strings.TrimSpace(segment)
		if segment == "" {
			continue
		}
		if rest, ok := strings.CutPrefix(segment, "cd "); ok {
			if fields := strings.Fields(rest); len(fields) > 0 {
				cwd = fields[0]
			}
			continue
		}
		if strings.HasPrefix(segment, "export ") || strings.HasPrefix(segment, "set ") {
			continue
		}
		fields := strings.Fields(segment)
		if len(fields) == 0 {
			continue
		}
		bin := BaseName(fields[0])
		if IsTestCommandSegment(segment, bin) {
			return TrimPipelineTail(segment), cwd
		}
		if (bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun" || bin == "uv") && IsTestCommandSegment(segment, "") {
			return TrimPipelineTail(segment), cwd
		}
	}
	if hasComposite {
		return "", cwd
	}
	return "", ""
}

// 🧪️IsTestCommandSegment says whether one segment of a shell command runs tests.
func IsTestCommandSegment(segment string, bin string) bool {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return false
	}
	fields := strings.Fields(trimmed)
	if bin == "" && len(fields) > 0 {
		bin = BaseName(fields[0])
	}
	if bin == "" {
		return false
	}
	if ClassifyCommandKind(trimmed) == CommandKindTest {
		return true
	}
	if len(fields) < 2 {
		return false
	}
	if (bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun") && ClassifyCommandKind(strings.Join(fields[1:], " ")) == CommandKindTest {
		return true
	}
	if bin == "uv" {
		if fields[1] == "run" && len(fields) > 2 && ClassifyCommandKind(strings.Join(fields[2:], " ")) == CommandKindTest {
			return true
		}
		if ClassifyCommandKind(strings.Join(fields[1:], " ")) == CommandKindTest {
			return true
		}
	}
	return false
}

// ✂️TrimPipelineTail keeps everything up to the first unquoted `|`.
func TrimPipelineTail(segment string) string {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return ""
	}
	current := strings.Builder{}
	quote := rune(0)
	for _, value := range trimmed {
		if quote != 0 {
			current.WriteRune(value)
			if value == quote {
				quote = 0
			}
			continue
		}
		if value == '\'' || value == '"' {
			quote = value
			current.WriteRune(value)
			continue
		}
		if value == '|' {
			break
		}
		current.WriteRune(value)
	}
	return strings.TrimSpace(current.String())
}

// 🔎️ResolveTestFilesFromCommand answers the test files a shell command would exercise.
func (snapshot *FilesystemSnapshot) ResolveTestFilesFromCommand(command string, cwd string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return []string{}
	}
	if cwd == "" {
		cwd = snapshot.Root
	}
	segment, extractedCwd := ExtractTestSegment(command)
	if segment != "" {
		command = segment
	}
	if extractedCwd != "" {
		if IsAbsolutePath(extractedCwd) {
			cwd = CleanPath(extractedCwd)
		} else {
			cwd = JoinPath(cwd, extractedCwd)
		}
	}
	parts := strings.Fields(command)
	if len(parts) == 0 {
		return []string{}
	}
	switch BaseName(parts[0]) {
	case "go":
		return snapshot.ResolveGoTestFiles(parts, cwd)
	case "cargo":
		return snapshot.ResolveCargoTestFiles(cwd)
	case "dotnet":
		return snapshot.ResolveDotnetTestFiles(cwd)
	case "python", "python3":
		return snapshot.ResolvePythonTestFiles(cwd)
	case "pytest", "py.test":
		return snapshot.ResolvePytestFiles(nil, cwd)
	case "uv":
		if len(parts) > 2 && parts[1] == "run" && containsWord(PythonTestRunnerBins, BaseName(parts[2])) {
			return snapshot.ResolvePytestFiles(nil, cwd)
		}
	case "uvx":
		if len(parts) > 1 && containsWord(PythonTestRunnerBins, BaseName(parts[1])) {
			return snapshot.ResolvePytestFiles(nil, cwd)
		}
	case "rspec":
		return snapshot.ResolveRspecFiles(cwd)
	case "npm", "pnpm", "yarn", "jest", "vitest", "mocha", "jasmine", "ava":
		return snapshot.FindJSTestFiles(cwd)
	case "npx", "bunx", "pnpx":
		if len(parts) > 1 && containsWord(JavaScriptTestRunnerBins, BaseName(parts[1])) {
			return snapshot.FindJSTestFiles(cwd)
		}
	case "bun":
		if len(parts) > 1 && parts[1] == "test" {
			return snapshot.FindJSTestFiles(cwd)
		}
	case "bundle":
		if len(parts) > 2 && parts[1] == "exec" && BaseName(parts[2]) == "rspec" {
			return snapshot.ResolveRspecFiles(cwd)
		}
	}
	return []string{}
}

// 🐹️ResolveGoTestFiles answers the package targets after `go test`, with `/...` meaning recursive.
func (snapshot *FilesystemSnapshot) ResolveGoTestFiles(args []string, cwd string) []string {
	targets := []string{}
	for index := 2; index < len(args); index++ {
		if strings.TrimSpace(args[index]) == "" || strings.HasPrefix(args[index], "-") {
			continue
		}
		targets = append(targets, args[index])
	}
	files := []string{}
	if len(targets) == 0 {
		return appendSuffixed(files, snapshot.WalkFiles(cwd, true), "_test.go")
	}
	for _, target := range targets {
		recursive := strings.HasSuffix(target, "/...")
		stripped := strings.TrimSuffix(target, "/...")
		targetPath := cwd
		if stripped != "." && stripped != "./" && stripped != "" {
			targetPath = JoinPath(cwd, stripped)
			if IsAbsolutePath(stripped) {
				targetPath = CleanPath(stripped)
			}
		}
		if snapshot.DirectoryExists(targetPath) {
			files = appendSuffixed(files, snapshot.WalkFiles(targetPath, recursive), "_test.go")
		} else if snapshot.FileExists(targetPath) && strings.HasSuffix(targetPath, "_test.go") && !containsWord(files, targetPath) {
			files = append(files, targetPath)
		}
	}
	return files
}

// 🦀️ResolveCargoTestFiles answers the `.rs` files carrying a test attribute.
func (snapshot *FilesystemSnapshot) ResolveCargoTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if !strings.HasSuffix(path, ".rs") {
			continue
		}
		if text, ok := snapshot.ReadText(path); ok && (strings.Contains(text, "#[test") || strings.Contains(text, "#[cfg(test)")) {
			files = append(files, path)
		}
	}
	return files
}

// 🔷️ResolveDotnetTestFiles answers the `.cs` files carrying a test attribute.
func (snapshot *FilesystemSnapshot) ResolveDotnetTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if !strings.HasSuffix(path, ".cs") {
			continue
		}
		if text, ok := snapshot.ReadText(path); ok && (strings.Contains(text, "[Test") || strings.Contains(text, "[Fact") || strings.Contains(text, "[TestMethod")) {
			files = append(files, path)
		}
	}
	return files
}

// 🐍️ResolvePythonTestFiles answers `test_*.py` and `*_test.py`.
func (snapshot *FilesystemSnapshot) ResolvePythonTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		base := BaseName(path)
		if strings.HasSuffix(path, ".py") && (strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.")) {
			files = append(files, path)
		}
	}
	return files
}

// 🐍️ResolvePytestFiles answers pytest naming plus `conftest.py`, honouring positional targets.
func (snapshot *FilesystemSnapshot) ResolvePytestFiles(args []string, cwd string) []string {
	targets := []string{}
	for index := 0; index < len(args); index++ {
		arg := strings.TrimSpace(args[index])
		if arg == "" {
			continue
		}
		if strings.HasPrefix(arg, "-") {
			if (arg == "-k" || arg == "-m" || arg == "--maxfail" || arg == "-c") && index+1 < len(args) {
				index++
			}
			continue
		}
		targets = append(targets, arg)
	}
	files := []string{}
	if len(targets) == 0 {
		return snapshot.scanPytestFiles(cwd, files)
	}
	for _, target := range targets {
		targetPath := JoinPath(cwd, target)
		if IsAbsolutePath(target) {
			targetPath = CleanPath(target)
		}
		if snapshot.DirectoryExists(targetPath) {
			files = snapshot.scanPytestFiles(targetPath, files)
		} else if snapshot.FileExists(targetPath) && strings.HasSuffix(targetPath, ".py") && isPytestFile(targetPath) && !containsWord(files, targetPath) {
			files = append(files, targetPath)
		}
	}
	return files
}

// 💎️ResolveRspecFiles answers the `_spec.rb` files below the working directory.
func (snapshot *FilesystemSnapshot) ResolveRspecFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if strings.HasSuffix(path, "_spec.rb") {
			files = append(files, path)
		}
	}
	return files
}

// 🟦️FindJSTestFiles answers `test.*`, `*.test.{js,ts}` and `*.spec.{js,ts}`.
func (snapshot *FilesystemSnapshot) FindJSTestFiles(cwd string) []string {
	files := []string{}
	for _, path := range snapshot.WalkFiles(cwd, true) {
		if !strings.HasSuffix(path, ".js") && !strings.HasSuffix(path, ".ts") && !strings.HasSuffix(path, ".jsx") && !strings.HasSuffix(path, ".tsx") {
			continue
		}
		base := BaseName(path)
		if strings.HasPrefix(base, "test.") || strings.HasSuffix(base, ".test.js") || strings.HasSuffix(base, ".test.ts") || strings.HasSuffix(base, ".spec.js") || strings.HasSuffix(base, ".spec.ts") {
			files = append(files, path)
		}
	}
	return files
}

func (snapshot *FilesystemSnapshot) scanPytestFiles(root string, files []string) []string {
	for _, path := range snapshot.WalkFiles(root, true) {
		if strings.HasSuffix(path, ".py") && isPytestFile(path) && !containsWord(files, path) {
			files = append(files, path)
		}
	}
	return files
}

func isPytestFile(path string) bool {
	base := BaseName(path)
	return strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.") || strings.EqualFold(base, "conftest.py")
}

func appendSuffixed(files []string, found []string, suffix string) []string {
	for _, path := range found {
		if strings.HasSuffix(path, suffix) && !containsWord(files, path) {
			files = append(files, path)
		}
	}
	return files
}

func containsWord(values []string, wanted string) bool {
	for _, value := range values {
		if value == wanted {
			return true
		}
	}
	return false
}

func flushSegment(segments []string, current *strings.Builder) []string {
	segment := strings.TrimSpace(current.String())
	current.Reset()
	if segment == "" {
		return segments
	}
	return append(segments, segment)
}

var commandTestBins = []string{"rspec", "pytest", "py.test", "jest", "vitest", "mocha", "gradle", "mvn", "bundle"}

var commandTestNeedles = []string{"pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"}

var commandTestPrefixes = []string{
	"npm test", "npm run test", "pnpm test", "yarn test", "bun test", "go test", "cargo test", "cargo nextest", "make test", "make check", "dotnet test", "swift test", "dart test",
	"flutter test", "mix test", "mvn test", "mvn verify", "gradle test", "./gradlew test", "gradlew test", "cabal test", "stack test", "lein test", "sbt test", "bundle exec rspec",
	"tox", "rspec ",
}

var commandSearchBins = []string{
	"grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind", "locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less", "more", "head", "tail",
	"wc", "file", "stat", "du", "which", "whereis", "type", "command", "hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath", "basename", "dirname", "jq",
	"yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column", "rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env",
}

var commandSearchTailBins = []string{
	"printenv", "set", "export", "pwd", "id", "whoami", "hostname", "uname", "date", "uptime", "free", "df", "ps", "top", "htop", "lsof", "netstat", "ss", "test", "sed", "awk", "gawk",
}

var commandEditBins = []string{
	"rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown", "chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip", "gunzip", "bzip2",
	"bunzip2", "xz", "unxz", "zstd",
}

// #endregion 🔎️Resolution

// #region 🧫️Vectors

// 🧭️DetectionVector is one runner-detection vector.
type DetectionVector struct {
	// 🏷️ID is the scenario-local id.
	ID string `json:"id"`
	// 📦️BundleRoot is the bundle root to probe.
	BundleRoot string `json:"bundleRoot"`
	// 🎯️TestFilter is the filter handed to the JavaScript runner detection.
	TestFilter string `json:"testFilter"`
	// 🗣️ExpectedLanguage is the language detection must answer.
	ExpectedLanguage string `json:"expectedLanguage"`
	// 📜️ExpectedArgv is the JavaScript runner argv detection must answer, where declared.
	ExpectedArgv *[]string `json:"expectedArgv,omitempty"`
}

// 🧭️DetectionVectors is the runner-detection fixture.
type DetectionVectors struct {
	// 🌍️Snapshot is the world the vectors probe.
	Snapshot FilesystemSnapshot `json:"snapshot"`
	// 🧭️Vectors are the vectors.
	Vectors []DetectionVector `json:"vectors"`
}

// 🗺️PlanningVector is one invocation-planning vector.
type PlanningVector struct {
	// 🏷️ID is the scenario-local id.
	ID string `json:"id"`
	// 🔭️Scope is the scope to plan.
	Scope TestScope `json:"scope"`
	// 🗺️Expected is the plan the scope must produce.
	Expected *InvocationPlan `json:"expected,omitempty"`
}

// 🗺️PlanningVectors is the invocation-planning fixture.
type PlanningVectors struct {
	// 🌍️Snapshot is the world the scopes are planned against.
	Snapshot FilesystemSnapshot `json:"snapshot"`
	// 🗺️Vectors are the vectors.
	Vectors []PlanningVector `json:"vectors"`
}

// 🧬️SelectorVectors is the scope-identifier fixture.
type SelectorVectors struct {
	// 🧬️Selectors are raw selectors as a caller would type them.
	Selectors []string `json:"selectors"`
}

// 📊️TranscriptVector is one recorded runner report.
type TranscriptVector struct {
	// 🏷️ID is the scenario-local id.
	ID string `json:"id"`
	// 🏭️Runner is the runner that produced it.
	Runner Runner `json:"runner"`
	// 📤️Output is the recorded report.
	Output ProcessOutput `json:"output"`
}

// 📊️TranscriptVectors is the result-parsing fixture.
type TranscriptVectors struct {
	// 📊️Vectors are the vectors.
	Vectors []TranscriptVector `json:"vectors"`
}

// 🛑️CancellationVectors is the cancellation fixture.
type CancellationVectors struct {
	// 🗺️Plan is the plan to run.
	Plan InvocationPlan `json:"plan"`
	// 🎞️Transcripts are the transcripts that answer it.
	Transcripts []RecordedTranscript `json:"transcripts"`
	// 🛑️CancelAfter says to cancel once this many invocations have completed.
	CancelAfter int `json:"cancelAfter"`
}

// 🧭️ParseDetectionVectors reads a runner-detection fixture.
func ParseDetectionVectors(source []byte) (DetectionVectors, error) {
	vectors := DetectionVectors{}
	err := json.Unmarshal(source, &vectors)
	return vectors, err
}

// 🗺️ParsePlanningVectors reads an invocation-planning fixture.
func ParsePlanningVectors(source []byte) (PlanningVectors, error) {
	vectors := PlanningVectors{}
	err := json.Unmarshal(source, &vectors)
	return vectors, err
}

// 🧬️ParseSelectorVectors reads a scope-identifier fixture.
func ParseSelectorVectors(source []byte) (SelectorVectors, error) {
	vectors := SelectorVectors{}
	err := json.Unmarshal(source, &vectors)
	return vectors, err
}

// 📊️ParseTranscriptVectors reads a result-parsing fixture.
func ParseTranscriptVectors(source []byte) (TranscriptVectors, error) {
	vectors := TranscriptVectors{}
	err := json.Unmarshal(source, &vectors)
	return vectors, err
}

// 🛑️ParseCancellationVectors reads a cancellation fixture.
func ParseCancellationVectors(source []byte) (CancellationVectors, error) {
	vectors := CancellationVectors{}
	err := json.Unmarshal(source, &vectors)
	if vectors.Plan.Invocations == nil {
		vectors.Plan.Invocations = []RunnerInvocation{}
	}
	if vectors.Plan.Problems == nil {
		vectors.Plan.Problems = []string{}
	}
	return vectors, err
}

// 🗺️PlanToJSONText renders a plan as JSON text, with every absent collection rendered as empty.
func PlanToJSONText(plan InvocationPlan) string {
	normalized := InvocationPlan{Invocations: []RunnerInvocation{}, Problems: []string{}}
	normalized.Problems = append(normalized.Problems, plan.Problems...)
	for _, invocation := range plan.Invocations {
		if invocation.Argv == nil {
			invocation.Argv = []string{}
		}
		if invocation.Env == nil {
			invocation.Env = map[string]string{}
		}
		normalized.Invocations = append(normalized.Invocations, invocation)
	}
	return marshalText(normalized)
}

// 🔭️ScopeToJSONText renders a scope as JSON text.
func ScopeToJSONText(scope TestScope) string { return marshalText(scope) }

// 📊️OutcomeToJSONText renders one outcome as JSON text.
func OutcomeToJSONText(outcome TestOutcome) string { return marshalText(normalizedOutcome(outcome)) }

// 📊️ReportToJSONText renders a whole execution report as JSON text.
func ReportToJSONText(report ExecutionReport) string {
	normalized := ExecutionReport{Outcomes: []TestOutcome{}, Cancelled: report.Cancelled, Completed: report.Completed, Total: report.Total, Problems: []string{}}
	normalized.Problems = append(normalized.Problems, report.Problems...)
	for _, outcome := range report.Outcomes {
		normalized.Outcomes = append(normalized.Outcomes, normalizedOutcome(outcome))
	}
	return marshalText(normalized)
}

// 📣️ProgressToJSONText renders a progress event stream as JSON text.
func ProgressToJSONText(events []ProgressEvent) string {
	if events == nil {
		events = []ProgressEvent{}
	}
	return marshalText(events)
}

func normalizedOutcome(outcome TestOutcome) TestOutcome {
	if outcome.Tests == nil {
		outcome.Tests = []TestCaseOutcome{}
	}
	return outcome
}

func marshalText(value any) string {
	encoded, err := json.Marshal(value)
	if err != nil {
		return fmt.Sprintf("{\"error\":%q}", err.Error())
	}
	return string(encoded)
}

// #endregion 🧫️Vectors

// #region 🚚️Split

// #region 🕸️Test Command

// 🔭️testScopeKind classifies the granularity of a testable entity scope.
type testScopeKind string

const testScopeAll testScopeKind = "all"

const testScopeTechnology testScopeKind = "technology"

const testScopeBundle testScopeKind = "bundle"

const testScopeFile testScopeKind = "file"

const testScopeSection testScopeKind = "section"

const testScopeDefinition testScopeKind = "definition"

// 🧪️testScope carries resolved information for running tests within a specific entity boundary.
type testScope struct {
	Kind       testScopeKind
	BundleRoot string
	FilePath   string
	Section    string
	TestName   string
	Language   string
}

// 🗣️detectBundleLanguage returns the primary language of a bundle by inspecting manifest files.
func detectBundleLanguage(bundleRoot string) string {
	absRoot := bundleRoot
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(workspace.RootDir, absRoot)
	}
	if workspace.FileExists(filepath.Join(absRoot, "go.mod")) {
		return "go"
	}
	if workspace.FileExists(filepath.Join(absRoot, "Cargo.toml")) {
		return "rust"
	}
	if _, err := filepath.Glob(filepath.Join(absRoot, "*.csproj")); err == nil {
		matches, _ := filepath.Glob(filepath.Join(absRoot, "*.csproj"))
		if len(matches) > 0 {
			return "csharp"
		}
	}
	if _, err := filepath.Glob(filepath.Join(absRoot, "*.sln")); err == nil {
		matches, _ := filepath.Glob(filepath.Join(absRoot, "*.sln"))
		if len(matches) > 0 {
			return "csharp"
		}
	}
	if workspace.FileExists(filepath.Join(absRoot, "package.json")) {
		return "typescript"
	}
	if workspace.FileExists(filepath.Join(absRoot, "pyproject.toml")) || workspace.FileExists(filepath.Join(absRoot, "requirements.txt")) {
		return "python"
	}
	return ""
}

// 🔗️resolveTestScopes converts entity IDs or URIs to testScope entries.
func ResolveTestScopes(ids []string) []testScope {
	if len(ids) == 0 {
		return []testScope{{Kind: testScopeAll}}
	}
	var scopes []testScope
	for _, raw := range ids {
		scopes = append(scopes, resolveTestScope(raw))
	}
	return scopes
}

// 🔷️resolveTestScope resolves a single entity ID or URI to a testScope.
func resolveTestScope(raw string) testScope {
	normalized := strings.ReplaceAll(raw, "\uFE0E", "")
	normalized = strings.ReplaceAll(normalized, "\uFE0F", "")
	normalized = strings.TrimSpace(normalized)

	uri := normalized
	if !strings.HasPrefix(uri, "repo://") {
		uri = model.IdToUri(normalized)
	}

	if uri == "" {
		return testScope{Kind: testScopeAll}
	}

	p := strings.TrimPrefix(uri, "repo://")

	if strings.HasPrefix(p, "p/") {
		rest := strings.TrimPrefix(p, "p/")
		parts := strings.SplitN(rest, "/", 3)
		if len(parts) == 2 {

			pkc := parts[0]
			pName := workspace.PathFromUriPath(parts[1])
			_ = pkc
			return testScope{Kind: testScopeTechnology, BundleRoot: pName}
		}
		if len(parts) >= 3 && strings.HasPrefix(parts[2], "b/") {
			bRest := strings.TrimPrefix(parts[2], "b/")
			bParts := strings.SplitN(bRest, "/", 3)
			if len(bParts) >= 2 {
				pkc := parts[0]
				pName := workspace.PathFromUriPath(parts[1])
				bkc := bParts[0]
				bName := workspace.PathFromUriPath(bParts[1])
				_ = pkc
				_ = bkc
				bundleName := strings.TrimPrefix(pName, "@") + "/" + bName
				bundle := findBundleByName(bundleName)
				bundleRoot := bundleName
				if bundle != nil {
					bundleRoot = bundle.Root
				}
				if len(bParts) == 2 {
					return testScope{Kind: testScopeBundle, BundleRoot: bundleRoot, Language: detectBundleLanguage(bundleRoot)}
				}

				return resolveTestScopeFromBundleSubPath(bundleRoot, bParts[2])
			}
		}
	}

	if strings.HasPrefix(p, "f/") {
		fRest := strings.TrimPrefix(p, "f/")
		fParts := strings.SplitN(fRest, "/", 2)
		filePath := workspace.PathFromUriPath(fParts[0])
		lang := ""
		if l := languages.GetLanguage(filePath); l != nil {
			lang = l.Name()
		}
		if len(fParts) == 1 {
			return testScope{Kind: testScopeFile, FilePath: filePath, Language: lang}
		}
		return resolveTestScopeFromFileSubPath(filePath, lang, fParts[1])
	}

	return testScope{Kind: testScopeAll}
}

// 💿️findBundleByName holds the data fields for a findBundleByName record.
func findBundleByName(name string) *model.Bundle {
	bundles := codebase.LoadBundles()
	for i := range bundles {
		if bundles[i].Name == name || workspace.Flat(bundles[i].Name) == workspace.Flat(name) {
			return &bundles[i]
		}
	}
	return nil
}

// 📦️resolveTestScopeFromBundleSubPath holds the data fields for a resolveTestScopeFromBundleSubPath record.
func resolveTestScopeFromBundleSubPath(bundleRoot string, subPath string) testScope {
	lang := detectBundleLanguage(bundleRoot)

	// Simplistic: extract file, section, definition
	parts := strings.Split(subPath, "/")
	filePath := ""
	section := ""
	testName := ""
	scopeKind := testScopeBundle
	for i := 0; i < len(parts); i++ {
		if parts[i] == "f" && i+1 < len(parts) {
			filePath = filepath.Join(bundleRoot, workspace.PathFromUriPath(parts[i+1]))
			scopeKind = testScopeFile
			i++
		} else if parts[i] == "s" && i+1 < len(parts) {
			section = workspace.PathFromUriPath(parts[i+1])
			scopeKind = testScopeSection
			i++
		} else if parts[i] == "d" && i+2 < len(parts) {
			testName = workspace.PathFromUriPath(parts[i+2])
			scopeKind = testScopeDefinition
			i += 2
		} else if parts[i] == "fd" && i+2 < len(parts) {
			i += 2
		}
	}
	if l := languages.GetLanguage(filePath); l != nil {
		lang = l.Name()
	}
	return testScope{Kind: scopeKind, BundleRoot: bundleRoot, FilePath: filePath, Section: section, TestName: testName, Language: lang}
}

// 📄️resolveTestScopeFromFileSubPath holds the data fields for a resolveTestScopeFromFileSubPath record.
func resolveTestScopeFromFileSubPath(filePath string, lang string, subPath string) testScope {
	section := ""
	testName := ""
	scopeKind := testScopeFile
	parts := strings.Split(subPath, "/")
	for i := 0; i < len(parts); i++ {
		if parts[i] == "s" && i+1 < len(parts) {
			section = workspace.PathFromUriPath(parts[i+1])
			scopeKind = testScopeSection
			i++
		} else if parts[i] == "d" && i+2 < len(parts) {
			testName = workspace.PathFromUriPath(parts[i+2])
			scopeKind = testScopeDefinition
			i += 2
		}
	}
	return testScope{Kind: scopeKind, FilePath: filePath, Section: section, TestName: testName, Language: lang}
}

// 🔶️runTestScope executes tests for a resolved testScope and streams results.
func RunTestScope(scope testScope, cmd workspace.CommandStreams) error {
	switch scope.Kind {
	case testScopeAll:
		return runAllTests(cmd)
	case testScopeTechnology:
		return runTechnologyTests(scope.BundleRoot, cmd)
	case testScopeBundle:
		return runBundleTests(scope.BundleRoot, scope.Language, "", "", cmd)
	case testScopeFile:
		return runFileTests(scope.FilePath, scope.Language, "", cmd)
	case testScopeSection:
		return runSectionTests(scope.FilePath, scope.Language, scope.Section, cmd)
	case testScopeDefinition:
		return runDefinitionTest(scope.FilePath, scope.Language, scope.BundleRoot, scope.TestName, cmd)
	}
	return nil
}

// 🔹️runAllTests holds the data fields for a runAllTests record.
func runAllTests(cmd workspace.CommandStreams) error {
	bundles := codebase.LoadBundles()
	var firstErr error
	for _, b := range bundles {
		lang := detectBundleLanguage(b.Root)
		if lang == "" {
			continue
		}
		if err := runBundleTests(b.Root, lang, "", "", cmd); err != nil && firstErr == nil {
			firstErr = err
		}
	}
	return firstErr
}

// 🛠️runTechnologyTests holds the data fields for a runTechnologyTests record.
func runTechnologyTests(technologyName string, cmd workspace.CommandStreams) error {
	bundles := codebase.LoadBundles()
	flatTechnology := workspace.Flat(technologyName)
	var firstErr error
	for _, b := range bundles {
		parts := strings.SplitN(b.Name, "/", 2)
		if workspace.Flat(parts[0]) != flatTechnology {
			continue
		}
		lang := detectBundleLanguage(b.Root)
		if lang == "" {
			continue
		}
		if err := runBundleTests(b.Root, lang, "", "", cmd); err != nil && firstErr == nil {
			firstErr = err
		}
	}
	return firstErr
}

// 🔸️runBundleTests holds the data fields for a runBundleTests record.
func runBundleTests(bundleRoot string, lang string, fileFilter string, testFilter string, cmd workspace.CommandStreams) error {
	absRoot := bundleRoot
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(workspace.RootDir, absRoot)
	}
	if lang == "" {
		lang = detectBundleLanguage(bundleRoot)
	}
	switch lang {
	case "go":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, "-run", testFilter)
		}
		if fileFilter != "" {
			args = append(args, "./"+fileFilter+"/...")
		} else {
			args = append(args, "./...")
		}
		return runExternalCommand(absRoot, "go", args, cmd)
	case "rust":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, testFilter)
		}
		return runExternalCommand(absRoot, "cargo", args, cmd)
	case "csharp":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, "--filter", testFilter)
		}
		return runExternalCommand(absRoot, "dotnet", args, cmd)
	case "typescript", "javascript":
		runner, runArgs := detectJSTestRunner(absRoot, testFilter)
		return runExternalCommand(absRoot, runner, runArgs, cmd)
	case "python":
		args := []string{"run", "pytest"}
		if fileFilter != "" {
			args = append(args, fileFilter)
		}
		if testFilter != "" {
			args = append(args, "-k", testFilter)
		}
		if uvExists() {
			return runExternalCommand(absRoot, "uv", args, cmd)
		}
		return runExternalCommand(absRoot, "pytest", args[2:], cmd)
	}
	return fmt.Errorf("unknown language %q for bundle %s", lang, bundleRoot)
}

func uvExists() bool {
	_, err := exec.LookPath("uv")
	return err == nil
}

func detectJSTestRunner(absRoot string, testFilter string) (string, []string) {

	pkgPath := filepath.Join(absRoot, "package.json")
	if workspace.FileExists(pkgPath) {
		content, err := workspace.ReadTextFile(pkgPath)
		if err == nil {
			if strings.Contains(content, "vitest") {
				args := []string{"vitest", "run"}
				if testFilter != "" {
					args = append(args, "-t", testFilter)
				}
				return "npx", args
			}
			if strings.Contains(content, "jest") {
				args := []string{"jest"}
				if testFilter != "" {
					args = append(args, "-t", testFilter)
				}
				return "npx", args
			}

			if strings.Contains(content, `"test"`) {
				args := []string{"test"}
				return "npm", args
			}
		}
	}

	args := []string{"vitest", "run"}
	if testFilter != "" {
		args = append(args, "-t", testFilter)
	}
	return "npx", args
}

// 🔺️runFileTests holds the data fields for a runFileTests record.
func runFileTests(filePath string, lang string, testFilter string, cmd workspace.CommandStreams) error {
	if lang == "" {
		l := languages.GetLanguage(filePath)
		if l != nil {
			lang = l.Name()
		}
	}
	absFilePath := filePath
	if !filepath.IsAbs(absFilePath) {
		absFilePath = filepath.Join(workspace.RootDir, filePath)
	}
	bundle := model.GetBundleByPath(filePath)
	if bundle == nil {
		return fmt.Errorf("no bundle found for file %s", filePath)
	}
	absRoot := filepath.Join(workspace.RootDir, bundle.Root)
	relFile, _ := filepath.Rel(absRoot, absFilePath)
	switch lang {
	case "go":
		args := []string{"test", "-v", "-run"}
		if testFilter != "" {
			args = append(args, testFilter)
		} else {
			args = append(args, ".")
		}

		fileDir := filepath.Dir(absFilePath)
		relDir, _ := filepath.Rel(absRoot, fileDir)
		if relDir == "." || relDir == "" {
			args = append(args, "./...")
		} else {
			args = append(args, "./"+relDir+"/...")
		}
		return runExternalCommand(absRoot, "go", args, cmd)
	case "python":
		args := []string{"run", "pytest", relFile}
		if testFilter != "" {
			args = append(args, "-k", testFilter)
		}
		if uvExists() {
			return runExternalCommand(absRoot, "uv", args, cmd)
		}
		return runExternalCommand(absRoot, "pytest", args[2:], cmd)
	case "typescript", "javascript":
		runner, runArgs := detectJSTestRunner(absRoot, testFilter)
		runArgs = append(runArgs, relFile)
		return runExternalCommand(absRoot, runner, runArgs, cmd)
	case "csharp":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, "--filter", testFilter)
		}
		return runExternalCommand(absRoot, "dotnet", args, cmd)
	case "rust":
		args := []string{"test"}
		if testFilter != "" {
			args = append(args, testFilter)
		}
		return runExternalCommand(absRoot, "cargo", args, cmd)
	}
	return fmt.Errorf("unsupported language %q for file test", lang)
}

func runSectionTests(filePath string, lang string, section string, cmd workspace.CommandStreams) error {

	if lang == "" {
		l := languages.GetLanguage(filePath)
		if l != nil {
			lang = l.Name()
		}
	}
	absFilePath := filePath
	if !filepath.IsAbs(absFilePath) {
		absFilePath = filepath.Join(workspace.RootDir, filePath)
	}
	bundle := model.GetBundleByPath(filePath)
	if bundle == nil {
		return fmt.Errorf("no bundle found for file %s", filePath)
	}
	absRoot := filepath.Join(workspace.RootDir, bundle.Root)
	switch lang {
	case "go":

		testPattern := collectGoTestsInSection(absFilePath, section)
		if testPattern == "" {
			return fmt.Errorf("no tests found in section %q of %s", section, filePath)
		}
		args := []string{"test", "-v", "-run", testPattern, "./..."}
		return runExternalCommand(absRoot, "go", args, cmd)
	default:

		return runFileTests(filePath, lang, "", cmd)
	}
}

// 📑️collectGoTestsInSection reads the file and folds it through the module's one section scanner.
func collectGoTestsInSection(filePath string, sectionName string) string {
	content, err := workspace.ReadTextFile(filePath)
	if err != nil {
		return ""
	}
	return CollectGoTestsInSection(content, sectionName)
}

// 📖️runDefinitionTest holds the data fields for a runDefinitionTest record.
func runDefinitionTest(filePath string, lang string, bundleRoot string, testName string, cmd workspace.CommandStreams) error {
	if lang == "" {
		l := languages.GetLanguage(filePath)
		if l != nil {
			lang = l.Name()
		}
	}
	if bundleRoot == "" {
		b := model.GetBundleByPath(filePath)
		if b != nil {
			bundleRoot = b.Root
		}
	}
	absRoot := bundleRoot
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(workspace.RootDir, bundleRoot)
	}

	originalName := resolveTestFunctionName(filepath.Join(workspace.RootDir, filePath), testName)
	if originalName == "" {

		originalName = unflattenTestName(testName)
	}
	switch lang {
	case "go":
		args := []string{"test", "-v", "-run", "^" + originalName + "$", "./..."}
		return runExternalCommand(absRoot, "go", args, cmd)
	case "python":
		args := []string{"run", "pytest", "-k", testName}
		if uvExists() {
			return runExternalCommand(absRoot, "uv", args, cmd)
		}
		return runExternalCommand(absRoot, "pytest", args[2:], cmd)
	case "csharp":
		args := []string{"test", "--filter", "FullyQualifiedName~" + originalName}
		return runExternalCommand(absRoot, "dotnet", args, cmd)
	case "rust":
		args := []string{"test", testName}
		return runExternalCommand(absRoot, "cargo", args, cmd)
	default:
		return runBundleTests(bundleRoot, lang, "", testName, cmd)
	}
}

// 🎯️resolveTestFunctionName finds the actual function name in a file matching the flat name.
func resolveTestFunctionName(absFilePath string, flatName string) string {
	content, err := workspace.ReadTextFile(absFilePath)
	if err != nil {
		return ""
	}
	testFuncRe := regexp.MustCompile(`func ((?:Test|Benchmark|Fuzz)\w+)\(`)
	for _, m := range testFuncRe.FindAllStringSubmatch(content, -1) {
		if workspace.Flat(m[1]) == flatName {
			return m[1]
		}
	}
	return ""
}

// 🧱️unflattenTestName reconstructs a probable PascalCase function name from a flat name.
func unflattenTestName(flat string) string {
	for _, prefix := range []string{"testbenchmark", "testfuzz", "benchmark", "fuzz", "test"} {
		if strings.HasPrefix(flat, prefix) {
			rest := flat[len(prefix):]
			if len(rest) > 0 {
				return strings.ToUpper(prefix[:1]) + prefix[1:] + strings.ToUpper(rest[:1]) + rest[1:]
			}
		}
	}
	if len(flat) > 0 {
		return strings.ToUpper(flat[:1]) + flat[1:]
	}
	return flat
}

// 🔻️runExternalCommand holds the data fields for a runExternalCommand record.
func runExternalCommand(dir string, name string, args []string, cmd workspace.CommandStreams) error {
	c := exec.Command(name, args...)
	c.Dir = dir
	c.Stdout = cmd.OutOrStdout()
	c.Stderr = cmd.ErrOrStderr()
	c.Stdin = os.Stdin
	cmdStr := name
	if len(args) > 0 {
		cmdStr += " " + strings.Join(args, " ")
	}
	fmt.Fprintf(cmd.OutOrStdout(), "Running: %s (in %s)\n", cmdStr, dir)
	return c.Run()
}

// 😀️headingEmojiRe holds the data fields for a headingEmojiRe record.
var headingEmojiRe = regexp.MustCompile(`^[\x{1F000}-\x{1FFFF}\x{2600}-\x{27FF}\x{FE00}-\x{FE0F}]+\s*`)

// 📑️ExtractMarkdownSection holds the data fields for a ExtractMarkdownSection record.
func ExtractMarkdownSection(content string, sectionName string) string {
	lines := strings.Split(content, "\n")
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	inSection := false
	sectionLevel := 0
	var result []string
	for _, line := range lines {
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(headingEmojiRe.ReplaceAllString(match[2], ""))
			if !inSection && name == sectionName && level == 1 {
				inSection = true
				sectionLevel = level
				continue
			}
			if inSection && level <= sectionLevel {
				break
			}
		}
		if inSection {
			result = append(result, line)
		}
	}
	text := strings.TrimSpace(strings.Join(result, "\n"))
	return text
}

// 🗃️isLicenseText holds the data fields for a isLicenseText record.
func isLicenseText(text string) bool {
	lower := strings.ToLower(text)
	return strings.Contains(lower, "gnu") ||
		strings.Contains(lower, "license") ||
		strings.Contains(lower, "free software") ||
		strings.Contains(lower, "warranty") ||
		strings.Contains(lower, "redistribute") ||
		strings.Contains(lower, "copyright")
}

// 🟨️isHeaderMetaLine holds the data fields for a isHeaderMetaLine record.
func isHeaderMetaLine(text string) bool {
	if strings.HasPrefix(text, "[") && strings.Contains(text, "](") {
		return true
	}
	if strings.HasPrefix(text, "#region") || strings.HasPrefix(text, "#endregion") ||
		strings.HasPrefix(text, "region ") || text == "region" ||
		strings.HasPrefix(text, "endregion") {
		return true
	}
	if len(text) > 0 && text[0] >= '0' && text[0] <= '9' {
		return true
	}
	if strings.HasPrefix(text, "💻️") || strings.HasPrefix(text, "🧰️") || strings.HasPrefix(text, "🔬️") {
		return true
	}
	return false
}

func ExtractFileHeaderSummary(filePath string) string {
	absPath := filepath.Join(workspace.RootDir, filePath)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return ""
	}
	lang := languages.GetLanguage(filePath)
	if lang == nil || !lang.SupportsHeaders() {
		return ""
	}
	prefix := lang.CommentPrefix()
	sections := lang.ParseSections(content)
	var headerSection *model.Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	if headerSection == nil {
		return ""
	}
	lines := strings.Split(content, "\n")
	type commentBlock struct {
		lines []string
	}
	var blocks []commentBlock
	var currentBlock []string
	inBlock := false
	for i := headerSection.StartLine; i < headerSection.EndLine && i <= len(lines); i++ {
		line := strings.TrimSpace(lines[i-1])
		if line == "" {
			if inBlock {
				blocks = append(blocks, commentBlock{lines: currentBlock})
				currentBlock = nil
				inBlock = false
			}
			continue
		}
		if !strings.HasPrefix(line, prefix) {
			continue
		}
		inBlock = true
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		currentBlock = append(currentBlock, commentText)
	}
	if len(currentBlock) > 0 {
		blocks = append(blocks, commentBlock{lines: currentBlock})
	}
	for _, block := range blocks {
		allMeta := true
		anyLicense := false
		hasSpec := false
		for _, l := range block.lines {
			if !isHeaderMetaLine(l) {
				allMeta = false
			}
			if isLicenseText(l) {
				anyLicense = true
			}
			if statutes.IsSpecText(l) {
				hasSpec = true
			}
		}
		if allMeta || anyLicense || hasSpec {
			continue
		}
		return strings.TrimSpace(strings.Join(block.lines, "\n"))
	}
	return ""
}

// 📄️ExtractFileHeaderRequirements holds the data fields for a ExtractFileHeaderRequirements record.
func ExtractFileHeaderRequirements(filePath string) string {
	absPath := filepath.Join(workspace.RootDir, filePath)
	content, err := workspace.ReadTextFile(absPath)
	if err != nil {
		return ""
	}
	lang := languages.GetLanguage(filePath)
	if lang == nil || !lang.SupportsHeaders() {
		return ""
	}
	prefix := lang.CommentPrefix()
	sections := lang.ParseSections(content)
	var headerSection *model.Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	if headerSection == nil {
		return ""
	}
	lines := strings.Split(content, "\n")
	type commentBlock struct {
		lines []string
	}
	var blocks []commentBlock
	var currentBlock []string
	inBlock := false
	for i := headerSection.StartLine; i < headerSection.EndLine && i <= len(lines); i++ {
		line := strings.TrimSpace(lines[i-1])
		if line == "" {
			if inBlock {
				blocks = append(blocks, commentBlock{lines: currentBlock})
				currentBlock = nil
				inBlock = false
			}
			continue
		}
		if !strings.HasPrefix(line, prefix) {
			continue
		}
		inBlock = true
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		currentBlock = append(currentBlock, commentText)
	}
	if len(currentBlock) > 0 {
		blocks = append(blocks, commentBlock{lines: currentBlock})
	}
	var requirementsLines []string
	for _, block := range blocks {
		hasSpec := false
		for _, l := range block.lines {
			if statutes.IsSpecText(l) {
				hasSpec = true
				break
			}
		}
		if hasSpec {
			requirementsLines = append(requirementsLines, block.lines...)
		}
	}
	return strings.TrimSpace(strings.Join(requirementsLines, "\n"))
}

func ExtractSectionLeadComments(content string, section model.Section, prefix string) (requirements string, summary string) {
	lines := strings.Split(content, "\n")
	lowerName := strings.ToLower(section.Name)
	if lowerName == "header" || lowerName == "license" {
		return "", ""
	}
	var specLines []string
	var summaryLines []string
	for i := section.StartLine; i < section.EndLine && i <= len(lines); i++ {
		line := strings.TrimSpace(lines[i-1])
		if line == "" {
			continue
		}
		if !strings.HasPrefix(line, prefix) {
			break
		}
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		if isHeaderMetaLine(commentText) {
			continue
		}
		if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://") {
			continue
		}
		if isLicenseText(commentText) {
			continue
		}
		if statutes.IsSpecText(commentText) {
			specLines = append(specLines, commentText)
		} else {
			summaryLines = append(summaryLines, commentText)
		}
	}
	return strings.TrimSpace(strings.Join(specLines, "\n")), strings.TrimSpace(strings.Join(summaryLines, "\n"))
}

// 📖️ExtractDefinitionDocstring holds the data fields for a ExtractDefinitionDocstring record.
func ExtractDefinitionDocstring(content string, def model.Definition, prefix string) (requirements string, summary string) {
	lines := strings.Split(content, "\n")
	for i := def.StartLine - 2; i >= 0; i-- {
		line := strings.TrimSpace(lines[i])
		if line == "" {
			break
		}
		if !strings.HasPrefix(line, prefix) {
			break
		}
		commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
		if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://") {
			continue
		}
		if statutes.IsSpecText(commentText) {
			requirements = commentText + "\n" + requirements
		} else {
			summary = commentText + "\n" + summary
		}
	}
	return strings.TrimSpace(requirements), strings.TrimSpace(summary)
}

// 🟩️findTechnologyByName holds the data fields for a findTechnologyByName record.
func findTechnologyByName(name string) *model.Technology {
	technologies := codebase.LoadTechnologies()
	for i := range technologies {
		if technologies[i].Name == name {
			return &technologies[i]
		}
	}
	return nil
}

// 🟦️walkTechnologyFiles holds the data fields for a walkTechnologyFiles record.
func walkTechnologyFiles(technology *model.Technology) []model.File {
	var files []model.File
	for _, bundle := range technology.Bundles {
		bundleRoot := filepath.Join(workspace.RootDir, bundle.Root)
		err := filepath.WalkDir(bundleRoot, func(path string, d fs.DirEntry, err error) error {
			if err != nil {
				return nil
			}
			if d.IsDir() {
				name := d.Name()
				if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
					return fs.SkipDir
				}
				return nil
			}
			relPath, _ := filepath.Rel(workspace.RootDir, path)
			relPath = workspace.NormalizePath(relPath)
			if workspace.IsGitIgnored(path) || model.IsGenerated(path) {
				return nil
			}
			kind := model.DeriveFileKind(d.Name())
			bundleID := bundle.GetID()
			files = append(files, model.File{
				ID:        model.BuildFileID(relPath, &bundleID),
				Path:      relPath,
				URI:       model.BuildFileUriFromPath(relPath),
				Name:      d.Name(),
				Extension: filepath.Ext(d.Name()),
				BundleID:  &bundleID,
				Kind:      kind,
			})
			return nil
		})
		_ = err
	}
	return files
}

// 📍️EntityEntry holds the data fields for a EntityEntry record.
type EntityEntry struct {
	ID   string
	URI  string
	Text string
}

func GenerateTechnologyRequirements(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	technologyReadmePath := filepath.Join(workspace.RootDir, technology.Root, "README.md")
	if content, err := workspace.ReadTextFile(technologyReadmePath); err == nil {
		requirements := ExtractMarkdownSection(content, "Requirements")
		if requirements != "" {
			p := &model.Technology{Name: technology.Name, Root: technology.Root, Kind: technology.Kind}
			entries = append(entries, EntityEntry{ID: p.GetID(), URI: p.GetURI(), Text: requirements})
		}
	}
	for _, bundle := range technology.Bundles {
		readmePath := filepath.Join(workspace.RootDir, bundle.Root, "README.md")
		if content, err := workspace.ReadTextFile(readmePath); err == nil {
			requirements := ExtractMarkdownSection(content, "Requirements")
			if requirements != "" {
				entries = append(entries, EntityEntry{ID: bundle.GetID(), URI: bundle.GetURI(), Text: requirements})
			}
		}
		folderReadmes := findFolderReadmes(bundle.Root)
		for _, frp := range folderReadmes {
			if content, err := workspace.ReadTextFile(filepath.Join(workspace.RootDir, frp)); err == nil {
				requirements := ExtractMarkdownSection(content, "Requirements")
				if requirements != "" {
					folderPath := filepath.Dir(frp)
					f := &model.Folder{Path: folderPath, Name: filepath.Base(folderPath)}
					entries = append(entries, EntityEntry{ID: f.GetID(), URI: f.GetURI(), Text: requirements})
				}
			}
		}
	}
	files := walkTechnologyFiles(technology)
	for _, file := range files {
		if file.Kind != model.FileKindCode && file.Kind != model.FileKindScript {
			continue
		}
		headerRequirements := ExtractFileHeaderRequirements(file.Path)
		if headerRequirements != "" {
			entries = append(entries, EntityEntry{ID: file.ID, URI: file.URI, Text: headerRequirements})
		}
		absPath := filepath.Join(workspace.RootDir, file.Path)
		content, err := workspace.ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(file.Path)
		if lang == nil {
			continue
		}
		sections := lang.ParseSections(content)
		prefix := lang.CommentPrefix()
		var walkSections func(secs []model.Section)
		walkSections = func(secs []model.Section) {
			for _, s := range secs {
				if strings.ToLower(s.Name) == "header" {
					walkSections(s.Children)
					continue
				}
				requirements, _ := ExtractSectionLeadComments(content, s, prefix)
				if requirements != "" {
					s.FilePath = file.Path
					entries = append(entries, EntityEntry{ID: s.GetID(), URI: s.GetURI(), Text: requirements})
				}
				walkSections(s.Children)
			}
		}
		walkSections(sections)
		if lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			for _, dr := range defs {
				def := model.Definition{
					Name:      dr.Name,
					Kind:      model.DeriveDefinitionKind(dr.Kind),
					FilePath:  file.Path,
					StartLine: dr.Start,
					EndLine:   dr.End,
				}
				requirements, _ := ExtractDefinitionDocstring(content, def, prefix)
				if requirements != "" {
					entries = append(entries, EntityEntry{ID: def.GetID(), URI: def.GetURI(), Text: requirements})
				}
			}
		}
	}
	var sb strings.Builder
	sb.WriteString("# \U0001F4AF Requirements\n")
	for _, e := range entries {
		sb.WriteString("\n## [" + e.ID + "](" + e.URI + ")\n\n")
		sb.WriteString(e.Text + "\n")
	}
	outputPath := filepath.Join(workspace.RootDir, technology.Root, "SPECS.md")
	return workspace.WriteTextFile(outputPath, sb.String())
}

// 🟪️GenerateTechnologyDocs holds the data fields for a GenerateTechnologyDocs record.
func GenerateTechnologyDocs(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	technologyReadmePath := filepath.Join(workspace.RootDir, technology.Root, "README.md")
	if content, err := workspace.ReadTextFile(technologyReadmePath); err == nil {
		docs := ExtractMarkdownSection(content, "Docs")
		if docs != "" {
			p := &model.Technology{Name: technology.Name, Root: technology.Root, Kind: technology.Kind}
			entries = append(entries, EntityEntry{ID: p.GetID(), URI: p.GetURI(), Text: docs})
		}
	}
	for _, bundle := range technology.Bundles {
		readmePath := filepath.Join(workspace.RootDir, bundle.Root, "README.md")
		if content, err := workspace.ReadTextFile(readmePath); err == nil {
			docs := ExtractMarkdownSection(content, "Docs")
			if docs != "" {
				entries = append(entries, EntityEntry{ID: bundle.GetID(), URI: bundle.GetURI(), Text: docs})
			}
		}
		folderReadmes := findFolderReadmes(bundle.Root)
		for _, frp := range folderReadmes {
			if content, err := workspace.ReadTextFile(filepath.Join(workspace.RootDir, frp)); err == nil {
				docs := ExtractMarkdownSection(content, "Docs")
				if docs != "" {
					folderPath := filepath.Dir(frp)
					f := &model.Folder{Path: folderPath, Name: filepath.Base(folderPath)}
					entries = append(entries, EntityEntry{ID: f.GetID(), URI: f.GetURI(), Text: docs})
				}
			}
		}
	}
	files := walkTechnologyFiles(technology)
	for _, file := range files {
		if file.Kind != model.FileKindCode && file.Kind != model.FileKindScript {
			continue
		}
		summary := ExtractFileHeaderSummary(file.Path)
		if summary != "" {
			entries = append(entries, EntityEntry{ID: file.ID, URI: file.URI, Text: summary})
		}
		absPath := filepath.Join(workspace.RootDir, file.Path)
		content, err := workspace.ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := languages.GetLanguage(file.Path)
		if lang == nil {
			continue
		}
		sections := lang.ParseSections(content)
		prefix := lang.CommentPrefix()
		var walkSections func(secs []model.Section)
		walkSections = func(secs []model.Section) {
			for _, s := range secs {
				if strings.ToLower(s.Name) == "header" {
					walkSections(s.Children)
					continue
				}
				_, summaryText := ExtractSectionLeadComments(content, s, prefix)
				if summaryText != "" {
					s.FilePath = file.Path
					entries = append(entries, EntityEntry{ID: s.GetID(), URI: s.GetURI(), Text: summaryText})
				}
				walkSections(s.Children)
			}
		}
		walkSections(sections)
		if lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			for _, dr := range defs {
				def := model.Definition{
					Name:      dr.Name,
					Kind:      model.DeriveDefinitionKind(dr.Kind),
					FilePath:  file.Path,
					StartLine: dr.Start,
					EndLine:   dr.End,
				}
				_, summaryText := ExtractDefinitionDocstring(content, def, prefix)
				if summaryText != "" {
					entries = append(entries, EntityEntry{ID: def.GetID(), URI: def.GetURI(), Text: summaryText})
				}
			}
		}
	}
	var sb strings.Builder
	sb.WriteString("# \U0001F4DA Docs\n")
	for _, e := range entries {
		sb.WriteString("\n## [" + e.ID + "](" + e.URI + ")\n\n")
		text := e.Text
		if text != "" {
			var filtered []string
			for _, line := range strings.Split(text, "\n") {
				trimmed := strings.TrimSpace(line)
				if strings.HasPrefix(trimmed, "// #region") || strings.HasPrefix(trimmed, "// #endregion") || strings.HasPrefix(trimmed, "#region") || strings.HasPrefix(trimmed, "#endregion") {
					continue
				}
				filtered = append(filtered, line)
			}
			text = strings.TrimSpace(strings.Join(filtered, "\n"))
		}
		if text == "" {
			continue
		}
		sb.WriteString(text + "\n")
	}
	outputPath := filepath.Join(workspace.RootDir, technology.Root, "DOCS.md")
	return workspace.WriteTextFile(outputPath, sb.String())
}

// 🟫️GenerateTechnologyTodos holds the data fields for a GenerateTechnologyTodos record.
func GenerateTechnologyTodos(technologyName string) error {
	technology := findTechnologyByName(technologyName)
	if technology == nil {
		return fmt.Errorf("technology %q not found", technologyName)
	}
	var entries []EntityEntry
	for _, bundle := range technology.Bundles {
		bundleRoot := filepath.Join(workspace.RootDir, bundle.Root)
		todos := todospkg.LocateTodosOnDisk(bundleRoot, todospkg.ScanTodos(todospkg.NewFsTodoTree(bundleRoot)))
		for _, todo := range todos {
			if todo.Location != nil && workspace.IsGitIgnored(todo.Location.FilePath) {
				continue
			}
			if todo.Location != nil && model.IsGenerated(todo.Location.FilePath) {
				continue
			}
			title := todo.Name
			desc := todo.Description
			parentPath := todo.ParentID
			relParent, _ := filepath.Rel(workspace.RootDir, parentPath)
			relParent = workspace.NormalizePath(relParent)
			entityID := relParent
			entityURI := model.BuildFileUriFromPath(relParent)
			if todo.Location != nil {
				relLoc, _ := filepath.Rel(workspace.RootDir, todo.Location.FilePath)
				relLoc = workspace.NormalizePath(relLoc)
				entityID = relLoc
				entityURI = model.BuildFileUriFromPath(relLoc)
			}
			text := "### TODO: " + title
			if desc != "" {
				text += "\n" + desc
			}
			entries = append(entries, EntityEntry{ID: entityID, URI: entityURI, Text: text})
		}
	}
	var sb strings.Builder
	sb.WriteString("# \U0001F533 TODOs\n")
	for _, e := range entries {
		sb.WriteString("\n## [" + e.ID + "](" + e.URI + ")\n\n")
		sb.WriteString(e.Text + "\n")
	}
	outputPath := filepath.Join(workspace.RootDir, technology.Root, "TODOS.md")
	return workspace.WriteTextFile(outputPath, sb.String())
}

// 📁️findFolderReadmes holds the data fields for a findFolderReadmes record.
func findFolderReadmes(bundleRoot string) []string {
	var readmes []string
	absRoot := filepath.Join(workspace.RootDir, bundleRoot)
	filepath.WalkDir(absRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			name := d.Name()
			if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
				return fs.SkipDir
			}
			return nil
		}
		if d.Name() == "README.md" {
			relPath, _ := filepath.Rel(workspace.RootDir, path)
			relPath = workspace.NormalizePath(relPath)
			if relPath != workspace.NormalizePath(filepath.Join(bundleRoot, "README.md")) {
				readmes = append(readmes, relPath)
			}
		}
		return nil
	})
	return readmes
}

// 📬️getBundlesWithOpenTickets holds the data fields for a getBundlesWithOpenTickets record.
func GetBundlesWithOpenTickets() map[string]bool {
	bundlesWithOpenTickets := make(map[string]bool)
	tickets, err := ticketspkg.ListTickets(nil, nil, nil)
	if err != nil {
		return bundlesWithOpenTickets
	}

	for _, t := range tickets {
		if t.Status != model.TicketStatusOpen {
			continue
		}
		for _, iter := range t.Interactions {
			for _, f := range iter.Files {
				parts := strings.Split(f.Path, "/")
				if len(parts) >= 2 && strings.HasPrefix(parts[0], "@") {
					bundleName := parts[0] + "/" + parts[1]
					bundlesWithOpenTickets[bundleName] = true
				}
			}
		}
	}
	return bundlesWithOpenTickets
}

// #endregion 🕸️Test Command

// #region 🖲️Missing Test Functions

// 🧪️pyTestRunnerBins are direct Python test runner binary names.
var PyTestRunnerBins = map[string]bool{
	"pytest": true, "py.test": true, "nosetests": true, "nose2": true,
}

// 🧲️extractTestSegmentFromCommand extracts the test-relevant segment from a command.
func ExtractTestSegmentFromCommand(command string) (string, string) {
	command = strings.TrimSpace(command)
	if command == "" {
		return "", ""
	}
	segments := workspace.SplitCommandSegments(command)
	if len(segments) == 0 {
		segments = []string{command}
	}
	cwd := ""
	hasComposite := len(segments) > 1
	for _, segment := range segments {
		segment = strings.TrimSpace(segment)
		if segment == "" {
			continue
		}
		if strings.HasPrefix(segment, "cd ") {
			parts := strings.Fields(segment)
			if len(parts) >= 2 {
				cwd = parts[1]
			}
			continue
		}
		if strings.HasPrefix(segment, "export ") || strings.HasPrefix(segment, "set ") {
			continue
		}
		fields := strings.Fields(segment)
		if len(fields) == 0 {
			continue
		}
		bin := filepath.Base(fields[0])
		if isTestCommandSegment(segment, bin) {
			return trimPipelineTail(segment), cwd
		}
		if bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun" || bin == "uv" {
			if isTestCommandSegment(segment, "") {
				return trimPipelineTail(segment), cwd
			}
		}
	}
	if hasComposite {
		return "", cwd
	}
	return "", ""
}

func isTestCommandSegment(segment string, bin string) bool {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return false
	}
	if bin == "" {
		fields := strings.Fields(trimmed)
		if len(fields) == 0 {
			return false
		}
		bin = filepath.Base(fields[0])
	}
	if model.ClassifyCommandKind(trimmed) == model.ToolKindTest {
		return true
	}
	fields := strings.Fields(trimmed)
	if len(fields) < 2 {
		return false
	}
	if (bin == "npx" || bin == "pnpm" || bin == "yarn" || bin == "bun") && model.ClassifyCommandKind(strings.Join(fields[1:], " ")) == model.ToolKindTest {
		return true
	}
	if bin == "uv" {
		if fields[1] == "run" && len(fields) > 2 && model.ClassifyCommandKind(strings.Join(fields[2:], " ")) == model.ToolKindTest {
			return true
		}
		if model.ClassifyCommandKind(strings.Join(fields[1:], " ")) == model.ToolKindTest {
			return true
		}
	}
	return false
}

func trimPipelineTail(segment string) string {
	trimmed := strings.TrimSpace(segment)
	if trimmed == "" {
		return ""
	}
	var current strings.Builder
	quote := rune(0)
	for i := 0; i < len(trimmed); i++ {
		ch := rune(trimmed[i])
		if quote != 0 {
			current.WriteRune(ch)
			if ch == quote {
				quote = 0
			}
			continue
		}
		if ch == '\'' || ch == '"' {
			quote = ch
			current.WriteRune(ch)
			continue
		}
		if ch == '|' {
			break
		}
		current.WriteRune(ch)
	}
	return strings.TrimSpace(current.String())
}

// 📄️resolveGoTestFiles resolves test files for Go commands.
func ResolveGoTestFiles(args []string, cwd string) []string {
	var files []string
	seen := map[string]bool{}
	targets := make([]string, 0)
	for i := 2; i < len(args); i++ {
		arg := strings.TrimSpace(args[i])
		if arg == "" || strings.HasPrefix(arg, "-") {
			continue
		}
		targets = append(targets, arg)
	}
	addFile := func(path string) {
		if !seen[path] {
			seen[path] = true
			files = append(files, path)
		}
	}
	scanDir := func(root string, recursive bool) {
		_ = filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if info.IsDir() {
				if !recursive && path != root {
					return filepath.SkipDir
				}
				return nil
			}
			if strings.HasSuffix(path, "_test.go") {
				addFile(path)
			}
			return nil
		})
	}
	if len(targets) == 0 {
		scanDir(cwd, true)
		return files
	}
	for _, target := range targets {
		recursive := strings.HasSuffix(target, "/...")
		targetPath := strings.TrimSuffix(target, "/...")
		if targetPath == "." || targetPath == "./" || targetPath == "" {
			targetPath = cwd
		} else if !filepath.IsAbs(targetPath) {
			targetPath = filepath.Join(cwd, targetPath)
		}
		info, err := os.Stat(targetPath)
		if err != nil {
			continue
		}
		if info.IsDir() {
			scanDir(targetPath, recursive)
			continue
		}
		if strings.HasSuffix(targetPath, "_test.go") {
			addFile(targetPath)
		}
	}
	return files
}

// 🎛️resolveCargoTestFiles resolves test files for Cargo commands.
func ResolveCargoTestFiles(args []string, cwd string) []string {
	var files []string
	// Look for test directories and files
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, ".rs") {
			content, err := os.ReadFile(path)
			if err != nil {
				return nil
			}
			if strings.Contains(string(content), "#[test") || strings.Contains(string(content), "#[cfg(test)") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🔷️resolveDotnetTestFiles resolves test files for .NET commands.
func ResolveDotnetTestFiles(args []string, cwd string) []string {
	var files []string
	// Look for .cs files with test attributes
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, ".cs") {
			content, err := os.ReadFile(path)
			if err != nil {
				return nil
			}
			if strings.Contains(string(content), "[Test") || strings.Contains(string(content), "[Fact") || strings.Contains(string(content), "[TestMethod") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🔶️resolvePythonTestFiles resolves test files for Python commands.
func ResolvePythonTestFiles(args []string, cwd string) []string {
	var files []string
	// Look for test_*.py and *_test.py files
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, ".py") {
			base := filepath.Base(path)
			if strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// 🔹️resolvePytestFiles resolves test files for pytest commands.
func ResolvePytestFiles(args []string, cwd string) []string {
	var files []string
	seen := map[string]bool{}
	addFile := func(path string) {
		if !seen[path] {
			seen[path] = true
			files = append(files, path)
		}
	}
	isPytestFile := func(path string) bool {
		base := filepath.Base(path)
		return strings.HasPrefix(base, "test_") || strings.Contains(base, "_test.") || strings.EqualFold(base, "conftest.py")
	}
	scanDir := func(root string) {
		_ = filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if !info.IsDir() && strings.HasSuffix(path, ".py") && isPytestFile(path) {
				addFile(path)
			}
			return nil
		})
	}
	targets := make([]string, 0)
	for i := 0; i < len(args); i++ {
		arg := strings.TrimSpace(args[i])
		if arg == "" {
			continue
		}
		if strings.HasPrefix(arg, "-") {
			if (arg == "-k" || arg == "-m" || arg == "--maxfail" || arg == "-c") && i+1 < len(args) {
				i++
			}
			continue
		}
		targets = append(targets, arg)
	}
	if len(targets) == 0 {
		scanDir(cwd)
		return files
	}
	for _, target := range targets {
		targetPath := target
		if !filepath.IsAbs(targetPath) {
			targetPath = filepath.Join(cwd, targetPath)
		}
		info, err := os.Stat(targetPath)
		if err != nil {
			continue
		}
		if info.IsDir() {
			scanDir(targetPath)
			continue
		}
		if strings.HasSuffix(targetPath, ".py") && isPytestFile(targetPath) {
			addFile(targetPath)
		}
	}
	return files
}

// 🔸️findJSTestFiles resolves test files for JavaScript test runners.
func FindJSTestFiles(cwd string) []string {
	var files []string
	// Look for test files with common patterns
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && (strings.HasSuffix(path, ".js") || strings.HasSuffix(path, ".ts") || strings.HasSuffix(path, ".jsx") || strings.HasSuffix(path, ".tsx")) {
			base := filepath.Base(path)
			if strings.HasPrefix(base, "test.") || strings.HasSuffix(base, ".test.js") ||
				strings.HasSuffix(base, ".test.ts") || strings.HasSuffix(base, ".spec.js") ||
				strings.HasSuffix(base, ".spec.ts") {
				files = append(files, path)
			}
		}
		return nil
	})
	return files
}

// #endregion 🖲️Missing Test Functions

// #region 💾️Missing Utility Functions

// 🧪️jsTestRunnerBins are JavaScript test runner binary names.
var JsTestRunnerBins = map[string]bool{
	"jest": true, "vitest": true, "mocha": true, "jasmine": true,
	"ava": true, "tape": true, "qunit": true, "karma": true,
}

// 📄️resolveRspecFiles resolves test files for RSpec commands.
func ResolveRspecFiles(args []string, cwd string) []string {
	var files []string
	// Look for _spec.rb files
	_ = filepath.Walk(cwd, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(path, "_spec.rb") {
			files = append(files, path)
		}
		return nil
	})
	return files
}

// 🔺️resolveTestFilesFromCommand resolves test files from a command.
func ResolveTestFilesFromCommand(command string, cwd string) []string {
	command = strings.TrimSpace(command)
	if command == "" {
		return nil
	}
	if cwd == "" {
		cwd = workspace.GetRootDir()
	}
	testSeg, extractedCwd := ExtractTestSegmentFromCommand(command)
	if testSeg != "" {
		command = testSeg
	}
	if extractedCwd != "" {
		if filepath.IsAbs(extractedCwd) {
			cwd = extractedCwd
		} else {
			cwd = filepath.Join(cwd, extractedCwd)
		}
	}
	parts := strings.Fields(command)
	if len(parts) == 0 {
		return nil
	}

	bin := filepath.Base(parts[0])

	switch bin {
	case "go":
		return ResolveGoTestFiles(parts, cwd)
	case "cargo":
		return ResolveCargoTestFiles(parts, cwd)
	case "dotnet":
		return ResolveDotnetTestFiles(parts, cwd)
	case "python", "python3":
		return ResolvePythonTestFiles(parts, cwd)
	case "pytest", "py.test":
		return ResolvePytestFiles(nil, cwd)
	case "uv":
		if len(parts) > 2 && parts[1] == "run" {
			runner := filepath.Base(parts[2])
			if runner == "pytest" || PyTestRunnerBins[runner] {
				return ResolvePytestFiles(nil, cwd)
			}
		}
		return nil
	case "uvx":
		if len(parts) > 1 && PyTestRunnerBins[filepath.Base(parts[1])] {
			return ResolvePytestFiles(nil, cwd)
		}
		return nil
	case "rspec":
		return ResolveRspecFiles(parts, cwd)
	case "npm", "pnpm", "yarn", "jest", "vitest", "mocha", "jasmine", "ava":
		return FindJSTestFiles(cwd)
	case "npx", "bunx", "pnpx":
		if len(parts) > 1 {
			runner := filepath.Base(parts[1])
			if JsTestRunnerBins[runner] {
				return FindJSTestFiles(cwd)
			}
			if runner == "phpunit" {
				return todospkg.ResolvePHPTestFiles(cwd)
			}
		}
		return nil
	case "bun":
		if len(parts) > 1 && parts[1] == "test" {
			return FindJSTestFiles(cwd)
		}
		return nil
	case "bundle":
		if len(parts) > 2 && parts[1] == "exec" && filepath.Base(parts[2]) == "rspec" {
			return ResolveRspecFiles(parts[2:], cwd)
		}
		return nil
	default:
		if strings.HasPrefix(bin, "phpunit") || strings.HasSuffix(bin, "/phpunit") {
			return todospkg.ResolvePHPTestFiles(cwd)
		}
		return nil
	}
}

// #endregion 💾️Missing Utility Functions

// #region 📰️Todos

// 🔬️parseTestInfoFromCommand parses test information from a command string.
// ⏰️Returns a slice of test names/patterns and a timeout duration string.
func ParseTestInfoFromCommand(command string) ([]string, string) {
	segment, _ := ExtractTestSegmentFromCommand(command)
	if segment != "" {
		command = segment
	}
	command = strings.TrimSpace(command)
	fields := strings.Fields(command)
	var tests []string
	timeout := ""

	if strings.Contains(command, "-timeout ") {
		parts := strings.Split(command, "-timeout ")
		if len(parts) > 1 {
			timeoutPart := strings.Fields(parts[1])[0]
			if strings.HasSuffix(timeoutPart, "s") {
				timeout = strings.TrimSuffix(timeoutPart, "s")
			} else if strings.HasSuffix(timeoutPart, "m") {
				mins := strings.TrimSuffix(timeoutPart, "m")
				if minsInt, err := strconv.Atoi(mins); err == nil {
					timeout = strconv.Itoa(minsInt * 60)
				}
			} else {
				timeout = timeoutPart
			}
		}
	} else if strings.Contains(command, "--timeout ") {
		parts := strings.Split(command, "--timeout ")
		if len(parts) > 1 {
			timeout = strings.Fields(parts[1])[0]
		}
	}
	flagValue := func(flags ...string) string {
		for i := 0; i < len(fields); i++ {
			for _, flag := range flags {
				if fields[i] == flag && i+1 < len(fields) {
					return fields[i+1]
				}
				prefix := flag + "="
				if strings.HasPrefix(fields[i], prefix) {
					return strings.TrimPrefix(fields[i], prefix)
				}
			}
		}
		return ""
	}
	joinAfterMarker := func(marker string) string {
		for i := 0; i < len(fields); i++ {
			if fields[i] == marker && i+1 < len(fields) {
				return fields[i+1]
			}
		}
		return ""
	}

	switch {
	case strings.HasPrefix(command, "go test"):
		if value := flagValue("-run"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "pytest"), strings.HasPrefix(command, "python -m pytest"), strings.HasPrefix(command, "python3 -m pytest"), strings.HasPrefix(command, "uv run pytest"), strings.HasPrefix(command, "uvx pytest"):
		if value := flagValue("-k"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "jest"), strings.HasPrefix(command, "npx jest"), strings.HasPrefix(command, "bunx jest"), strings.HasPrefix(command, "pnpx jest"):
		if value := flagValue("-t", "--testNamePattern"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "mocha"), strings.HasPrefix(command, "npx mocha"), strings.HasPrefix(command, "bunx mocha"), strings.HasPrefix(command, "pnpx mocha"):
		if value := flagValue("--grep"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "cargo test"):
		if value := joinAfterMarker("--"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "cargo nextest"):
		tests = []string{""}
	case strings.HasPrefix(command, "dotnet test"):
		if value := flagValue("--filter"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.HasPrefix(command, "rspec"), strings.HasPrefix(command, "bundle exec rspec"):
		if value := flagValue("--example"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.Contains(command, "vitest"):
		if value := flagValue("-t", "--testNamePattern"); value != "" {
			tests = []string{value}
		} else {
			tests = []string{""}
		}
	case strings.Contains(command, "phpunit"), strings.Contains(command, "npm test"), strings.Contains(command, "pnpm test"), strings.Contains(command, "yarn test"), strings.Contains(command, "bun test"):
		tests = []string{""}
	default:
		tests = []string{""}
	}

	return tests, timeout
}

// #endregion 📰️Todos

// #endregion 🚚️Split
