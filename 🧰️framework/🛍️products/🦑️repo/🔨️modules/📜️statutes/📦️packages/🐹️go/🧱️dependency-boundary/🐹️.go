package statutes

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	regexp "regexp"
	strings "strings"

	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🧱️Dependency Boundary

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

// #endregion 🧱️Dependency Boundary
