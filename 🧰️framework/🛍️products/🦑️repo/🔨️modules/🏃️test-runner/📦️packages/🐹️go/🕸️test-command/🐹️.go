// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🕸️test-command carries the test command scope resolution and the per-scope test runs.

// #endregion 🧲️Header

package testrunner

import (
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	regexp "regexp"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

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

// #endregion 🕸️Test Command
