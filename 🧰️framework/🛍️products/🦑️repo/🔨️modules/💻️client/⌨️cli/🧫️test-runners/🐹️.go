// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Test runners, external command execution, and LLM/effort/client flag helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
	"time"

	"github.com/usalu/semio/repo/workspace"
)

func uvExists() bool {
	_, err := exec.LookPath("uv")
	return err == nil
}

func detectJSTestRunner(absRoot string, testFilter string) (string, []string) {

	pkgPath := filepath.Join(absRoot, "package.json")
	if FileExists(pkgPath) {
		content, err := ReadTextFile(pkgPath)
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
func runFileTests(filePath string, lang string, testFilter string, cmd *command.Command) error {
	if lang == "" {
		l := GetLanguage(filePath)
		if l != nil {
			lang = l.Name()
		}
	}
	absFilePath := filePath
	if !filepath.IsAbs(absFilePath) {
		absFilePath = filepath.Join(rootDir, filePath)
	}
	bundle := GetBundleByPath(filePath)
	if bundle == nil {
		return fmt.Errorf("no bundle found for file %s", filePath)
	}
	absRoot := filepath.Join(rootDir, bundle.Root)
	relFile, _ := filepath.Rel(absRoot, absFilePath)
	switch lang {
	case "go":
		args := []string{"-v", "-run"}
		if testFilter != "" {
			args = append(args, testFilter)
		} else {
			args = append(args, ".")
		}

		return runCanonicalGoTestDispatch(absRoot, absFilePath, args, cmd)
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

func runSectionTests(filePath string, lang string, section string, cmd *command.Command) error {

	if lang == "" {
		l := GetLanguage(filePath)
		if l != nil {
			lang = l.Name()
		}
	}
	absFilePath := filePath
	if !filepath.IsAbs(absFilePath) {
		absFilePath = filepath.Join(rootDir, filePath)
	}
	bundle := GetBundleByPath(filePath)
	if bundle == nil {
		return fmt.Errorf("no bundle found for file %s", filePath)
	}
	absRoot := filepath.Join(rootDir, bundle.Root)
	switch lang {
	case "go":

		testPattern := collectGoTestsInSection(absFilePath, section)
		if testPattern == "" {
			return fmt.Errorf("no tests found in section %q of %s", section, filePath)
		}
		args := []string{"-v", "-run", testPattern}
		return runCanonicalGoTestDispatch(absRoot, absFilePath, args, cmd)
	default:

		return runFileTests(filePath, lang, "", cmd)
	}
}

// 📑️collectGoTestsInSection holds the data fields for a collectGoTestsInSection record.
func collectGoTestsInSection(filePath string, sectionName string) string {
	content, err := ReadTextFile(filePath)
	if err != nil {
		return ""
	}
	lines := strings.Split(content, "\n")
	flatSection := Flat(sectionName)
	inSection := false
	var testNames []string
	testFuncRe := regexp.MustCompile(`^func (Test\w+)\(`)
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)

		if strings.HasPrefix(trimmed, "//") {
			if idx := strings.Index(trimmed, "#region"); idx >= 0 {
				regionName := strings.TrimSpace(trimmed[idx+len("#region"):])
				_, regionName = extractEntityEmoji(regionName)
				regionName = strings.TrimSpace(regionName)
				if Flat(regionName) == flatSection {
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
			if m := testFuncRe.FindStringSubmatch(line); m != nil {
				testNames = append(testNames, m[1])
			}
		}
	}
	if len(testNames) == 0 {
		return ""
	}
	return "^(" + strings.Join(testNames, "|") + ")$"
}

// 📖️runDefinitionTest holds the data fields for a runDefinitionTest record.
func runDefinitionTest(filePath string, lang string, bundleRoot string, testName string, cmd *command.Command) error {
	if lang == "" {
		l := GetLanguage(filePath)
		if l != nil {
			lang = l.Name()
		}
	}
	if bundleRoot == "" {
		b := GetBundleByPath(filePath)
		if b != nil {
			bundleRoot = b.Root
		}
	}
	absRoot := bundleRoot
	if !filepath.IsAbs(absRoot) {
		absRoot = filepath.Join(rootDir, bundleRoot)
	}

	absFilePath := filePath
	if !filepath.IsAbs(absFilePath) {
		absFilePath = filepath.Join(rootDir, filePath)
	}
	originalName := resolveTestFunctionName(absFilePath, testName)
	if originalName == "" {

		originalName = unflattenTestName(testName)
	}
	switch lang {
	case "go":
		args := []string{"-v", "-run", "^" + originalName + "$"}
		return runCanonicalGoTestDispatch(absRoot, absFilePath, args, cmd)
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
	content, err := ReadTextFile(absFilePath)
	if err != nil {
		return ""
	}
	testFuncRe := regexp.MustCompile(`func ((?:Test|Benchmark|Fuzz)\w+)\(`)
	for _, m := range testFuncRe.FindAllStringSubmatch(content, -1) {
		if Flat(m[1]) == flatName {
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
func runExternalCommand(dir string, name string, args []string, cmd *command.Command) error {
	ctx := cmd.Context()
	if err := ctx.Err(); err != nil {
		return err
	}
	c := workspace.ToolCommand(name, args...)
	c.Dir = dir
	c.Stdout = cmd.OutOrStdout()
	c.Stderr = cmd.ErrOrStderr()
	c.Stdin = os.Stdin
	cmdStr := name
	if len(args) > 0 {
		cmdStr += " " + strings.Join(args, " ")
	}
	fmt.Fprintf(cmd.OutOrStdout(), "Running: %s (in %s)\n", cmdStr, dir)
	if err := c.Start(); err != nil {
		return err
	}
	finished := make(chan error, 1)
	go func() { finished <- c.Wait() }()
	select {
	case err := <-finished:
		return err
	case <-ctx.Done():
		terminateOwnedCommandProcessTree(c.Process.Pid)
		select {
		case <-finished:
		case <-time.After(5 * time.Second):
			_ = c.Process.Kill()
		}
		return ctx.Err()
	}
}

// 🧰️toolchainRoot returns the checkout whose canonical dispatcher runs Go tests: SEMIO_TOOLCHAIN_ROOT when the caller names one, else the bound repository root.
func toolchainRoot() string {
	if explicit := os.Getenv("SEMIO_TOOLCHAIN_ROOT"); explicit != "" {
		return explicit
	}
	return rootDir
}

// 🐹️runCanonicalGoTestDispatch routes native scopes through the registered canonical compiler-input plan.
func runCanonicalGoTestDispatch(moduleRoot string, inputPath string, goArgs []string, cmd *command.Command) error {
	toolchain := toolchainRoot()
	script := filepath.Join(toolchain, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "📦️packages", "🟦️typescript", "📜️script.ts")
	args := []string{script, "go-test", moduleRoot, inputPath}
	args = append(args, goArgs...)
	repo, err := repoExecutable()
	if err != nil {
		return err
	}
	return runExternalCommand(toolchain, repo, args, cmd)
}

// ⬛️testCommand holds the data fields for a testCommand record.
func testCommand(factory EngineFactory, config *Config) *command.Command {
	return &command.Command{
		Use:   "test [testable-id-or-uri]...",
		Short: "Run tests for given entities",
		Long: `Run tests for one or more testable entities identified by their entity IDs or URIs.

With no arguments, runs all tests in the repository.

Supported entity scopes (narrowing order):
  🧰️technology                       - all tests in the technology
  🧰️technology⌨️bundle               - all tests in the bundle
  🧰️technology⌨️bundle🥼️file         - all tests in the test file
  🧰️technology⌨️bundle🥼️file🔖️sec    - all tests in the section
  🧰️technology⌨️bundle🥼️file🔖️sec🧪️fn - a single test function`,
		Args: command.ArbitraryArgs,
		RunE: func(cmd *command.Command, args []string) error {
			scopes := resolveTestScopes(args)
			var firstErr error
			for _, scope := range scopes {
				if err := runTestScope(scope, cmd); err != nil && firstErr == nil {
					firstErr = err
				}
			}
			return firstErr
		},
	}
}

// 🧠️extractLLMFromArgs extracts LLM from command flags and positional arguments.
func extractLLMFromArgs(cmd *command.Command, args []string) (string, []string) {
	if llm, _ := cmd.Flags().GetString("llm"); llm != "" {
		return llm, args
	}

	for _, allowed := range AllowedLLMs {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundLLM := ""
	for _, arg := range args {
		if foundLLM != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := NormalizeLLMSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range AllowedLLMs {
			if strings.Contains(normalized, NormalizeLLMSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundLLM = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundLLM, remaining
}

// 🏋️extractEffortFromArgs extracts reasoning effort from command flags and positional arguments.
func extractEffortFromArgs(cmd *command.Command, args []string) (string, []string) {
	if effort, _ := cmd.Flags().GetString("effort"); effort != "" {
		return effort, args
	}

	for _, allowed := range AllowedEfforts {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundEffort := ""
	for _, arg := range args {
		if foundEffort != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := NormalizeEffortSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range AllowedEfforts {
			if normalized == NormalizeEffortSlug(allowed) || strings.Contains(normalized, NormalizeEffortSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundEffort = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundEffort, remaining
}

// 🧲️extractClientFromArgs holds the data fields for a extractClientFromArgs record.
func extractClientFromArgs(cmd *command.Command, args []string) (string, []string) {

	if client, _ := cmd.Flags().GetString("client"); client != "" {
		return client, args
	}

	for _, allowed := range AllowedClients {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}

	remaining := []string{}
	foundClient := ""
	for _, arg := range args {
		if foundClient != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := NormalizeClientSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range AllowedClients {
			if strings.Contains(normalized, NormalizeClientSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundClient = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundClient, remaining
}

// ➕️addLLMFlags holds the data fields for a addLLMFlags record.
func addLLMFlags(cmd *command.Command) {
	for _, llm := range AllowedLLMs {
		if cmd.Flags().Lookup(llm) == nil {
			cmd.Flags().Bool(llm, false, fmt.Sprintf("Use %s as LLM", llm))
		}
	}
}

// 🏋️addEffortFlags adds boolean flags for all allowed effort values.
func addEffortFlags(cmd *command.Command) {
	for _, effort := range AllowedEfforts {
		if cmd.Flags().Lookup(effort) == nil {
			cmd.Flags().Bool(effort, false, fmt.Sprintf("Use %s as LLM reasoning effort", effort))
		}
	}
}

// 💻️addClientFlags holds the data fields for a addClientFlags record.
func addClientFlags(cmd *command.Command) {
	for _, client := range AllowedClients {
		if cmd.Flags().Lookup(client) == nil {
			cmd.Flags().Bool(client, false, fmt.Sprintf("Use %s as Client", client))
		}
	}
}
