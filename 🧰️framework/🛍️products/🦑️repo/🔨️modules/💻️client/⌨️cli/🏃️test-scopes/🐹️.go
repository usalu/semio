// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Test scope resolution for the test command.

// #endregion 🧲️Header

package client

import (
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"path/filepath"
	"strings"
)

// 🔭️testScopeKind classifies the granularity of a testable entity scope.
type testScopeKind string

const (
	testScopeAll        testScopeKind = "all"
	testScopeTechnology testScopeKind = "technology"
	testScopeBundle     testScopeKind = "bundle"
	testScopeFile       testScopeKind = "file"
	testScopeSection    testScopeKind = "section"
	testScopeDefinition testScopeKind = "definition"
)

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
		absRoot = filepath.Join(rootDir, absRoot)
	}
	if FileExists(filepath.Join(absRoot, "go.mod")) {
		return "go"
	}
	if FileExists(filepath.Join(absRoot, "Cargo.toml")) {
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
	if FileExists(filepath.Join(absRoot, "package.json")) {
		return "typescript"
	}
	if FileExists(filepath.Join(absRoot, "pyproject.toml")) || FileExists(filepath.Join(absRoot, "requirements.txt")) {
		return "python"
	}
	return ""
}

// 🔗️resolveTestScopes converts entity IDs or URIs to testScope entries.
func resolveTestScopes(ids []string) []testScope {
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
		uri = IdToUri(normalized)
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
			pName := PathFromUriPath(parts[1])
			_ = pkc
			return testScope{Kind: testScopeTechnology, BundleRoot: pName}
		}
		if len(parts) >= 3 && strings.HasPrefix(parts[2], "b/") {
			bRest := strings.TrimPrefix(parts[2], "b/")
			bParts := strings.SplitN(bRest, "/", 3)
			if len(bParts) >= 2 {
				pkc := parts[0]
				pName := PathFromUriPath(parts[1])
				bkc := bParts[0]
				bName := PathFromUriPath(bParts[1])
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
		filePath := PathFromUriPath(fParts[0])
		lang := ""
		if l := GetLanguage(filePath); l != nil {
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
func findBundleByName(name string) *Bundle {
	bundles := LoadBundles()
	for i := range bundles {
		if bundles[i].Name == name || Flat(bundles[i].Name) == Flat(name) {
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
			filePath = filepath.Join(bundleRoot, PathFromUriPath(parts[i+1]))
			scopeKind = testScopeFile
			i++
		} else if parts[i] == "s" && i+1 < len(parts) {
			section = PathFromUriPath(parts[i+1])
			scopeKind = testScopeSection
			i++
		} else if parts[i] == "d" && i+2 < len(parts) {
			testName = PathFromUriPath(parts[i+2])
			scopeKind = testScopeDefinition
			i += 2
		} else if parts[i] == "fd" && i+2 < len(parts) {
			i += 2
		}
	}
	if l := GetLanguage(filePath); l != nil {
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
			section = PathFromUriPath(parts[i+1])
			scopeKind = testScopeSection
			i++
		} else if parts[i] == "d" && i+2 < len(parts) {
			testName = PathFromUriPath(parts[i+2])
			scopeKind = testScopeDefinition
			i += 2
		}
	}
	return testScope{Kind: scopeKind, FilePath: filePath, Section: section, TestName: testName, Language: lang}
}

// 🔶️runTestScope executes tests for a resolved testScope and streams results.
func runTestScope(scope testScope, cmd *command.Command) error {
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
func runAllTests(cmd *command.Command) error {
	bundles := LoadBundles()
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
func runTechnologyTests(technologyName string, cmd *command.Command) error {
	bundles := LoadBundles()
	flatTechnology := Flat(technologyName)
	var firstErr error
	for _, b := range bundles {
		parts := strings.SplitN(b.Name, "/", 2)
		if Flat(parts[0]) != flatTechnology {
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
func runBundleTests(bundleRoot string, lang string, fileFilter string, testFilter string, cmd *command.Command) error {
	absRoot := bundleRoot
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(rootDir, absRoot)
	}
	if lang == "" {
		lang = detectBundleLanguage(bundleRoot)
	}
	switch lang {
	case "go":
		args := []string{}
		if testFilter != "" {
			args = append(args, "-run", testFilter)
		}
		input := "-"
		if fileFilter != "" {
			input = filepath.Join(absRoot, fileFilter)
		}
		return runCanonicalGoTestDispatch(absRoot, input, args, cmd)
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
