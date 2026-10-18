// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎨️formatter carries the formatter plans, availability probing and the fallback text formatter.

// #endregion 🧲️Header

package workspace

import (
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
)

// #region 🎨️Formatter

var FormatterBinaryLookup = exec.LookPath

var FormatterCommandRun = runFormatterCommand

// 📋️formatterPlan holds the data fields for a formatterPlan record.
type formatterPlan struct {
	Binary      string
	Args        []string
	requirement []string
}

// 🔸️runFormatterCommand holds the data fields for a runFormatterCommand record.
func runFormatterCommand(binary string, args []string, workDir string) error {
	commandPath := binary
	if strings.Contains(binary, "/") || strings.Contains(binary, "\\") {
		commandPath = filepath.Join(workDir, binary)
	}
	cmd := exec.Command(commandPath, args...)
	cmd.Dir = workDir
	return cmd.Run()
}

// 🗣️formatterPlansForLanguage holds the data fields for a formatterPlansForLanguage record.
func FormatterPlansForLanguage(languageName string, relPath string) []formatterPlan {
	prettierBinary := filepath.Join("node_modules", ".bin", "prettier")
	prettierPlan := formatterPlan{
		Binary:      prettierBinary,
		Args:        []string{"--write", "--ignore-unknown", relPath},
		requirement: []string{prettierBinary, ".prettierrc.json"},
	}
	switch languageName {
	case "typescript", "markdown", "json", "yaml", "graphql", "sql", "toml":
		return []formatterPlan{prettierPlan}
	case "go":
		return []formatterPlan{
			{Binary: "gofmt", Args: []string{"-w", relPath}, requirement: []string{"go.work"}},
		}
	case "python":
		return []formatterPlan{
			{Binary: "uv", Args: []string{"run", "--group", "dev", "ruff", "format", relPath}, requirement: []string{"pyproject.toml"}},
			{Binary: "uv", Args: []string{"run", "--group", "dev", "black", relPath}, requirement: []string{"pyproject.toml"}},
			prettierPlan,
		}
	case "csharp":
		return []formatterPlan{
			{Binary: "dotnet", Args: []string{"format", "Monorepo.sln", "--include", relPath, "--verbosity", "minimal"}, requirement: []string{"Monorepo.sln"}},
			prettierPlan,
		}
	case "rust":
		return []formatterPlan{
			{Binary: "rustfmt", Args: []string{relPath}, requirement: []string{"Cargo.toml"}},
			prettierPlan,
		}
	case "ruby":
		return []formatterPlan{
			{Binary: "rubocop", Args: []string{"-A", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	case "shell":
		return []formatterPlan{
			{Binary: "shfmt", Args: []string{"-w", relPath}, requirement: []string{"package.json"}},
			prettierPlan,
		}
	default:
		return []formatterPlan{prettierPlan}
	}
}

// 🔺️isFormatterPlanAvailable holds the data fields for a isFormatterPlanAvailable record.
func IsFormatterPlanAvailable(plan formatterPlan, workDir string) bool {
	if strings.Contains(plan.Binary, "/") || strings.Contains(plan.Binary, "\\") {
		if !FileExists(filepath.Join(workDir, plan.Binary)) {
			return false
		}
	} else {
		if _, err := FormatterBinaryLookup(plan.Binary); err != nil {
			return false
		}
	}
	for _, rel := range plan.requirement {
		if !FileExists(filepath.Join(workDir, rel)) {
			return false
		}
	}
	return true
}

// 🔻️fallbackFormatText holds the data fields for a fallbackFormatText record.
func FallbackFormatText(content string) string {
	lines := strings.Split(content, "\n")
	for i, line := range lines {
		lines[i] = strings.TrimRight(line, " \t")
	}
	formatted := strings.Join(lines, "\n")
	if formatted == "" {
		return formatted
	}
	if !strings.HasSuffix(formatted, "\n") {
		formatted += "\n"
	}
	return formatted
}

// #endregion 🎨️Formatter
