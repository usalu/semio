// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗺️planning carries the runners, invocation plans and the planning of every test scope.

// #endregion 🧲️Header

package testrunner

import (
	fmt "fmt"
	strings "strings"
)

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
