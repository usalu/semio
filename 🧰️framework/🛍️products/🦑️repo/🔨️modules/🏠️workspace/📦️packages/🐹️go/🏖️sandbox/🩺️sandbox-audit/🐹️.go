// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🩺️sandbox-audit explains why a running test binary does not run against a root it owns and finds the test sources that locate the checkout.

// #endregion 🧲️Header

package sandbox

import (
	bufio "bufio"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"

	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🩺️Sandbox Audit

// 🧐️unownedRootLookups are the source spellings through which a test locates the checkout it runs in instead of a root it owns.
var unownedRootLookups = []string{"os.Getwd(", "runtime.Caller(", `FindRepoRoot(".")`, `FindRepoRoot("")`}

// 🔬️UnownedRootLookups scans the Go test sources of every case directory below testsDir, skipping the named guard case and the language-agnostic host adapters (package adapter), and returns one "case:line: spelling" entry per test source line that locates the real checkout.
func UnownedRootLookups(testsDir string, guardCase string) []string {
	findings := []string{}
	cases, err := os.ReadDir(testsDir)
	if err != nil {
		return []string{fmt.Sprintf("%s: %v", testsDir, err)}
	}
	for _, entry := range cases {
		if !entry.IsDir() || entry.Name() == guardCase {
			continue
		}
		source := filepath.Join(testsDir, entry.Name(), "🐹️.go")
		file, err := os.Open(source)
		if err != nil {
			continue
		}
		scanner := bufio.NewScanner(file)
		scanner.Buffer(make([]byte, 0, 1<<20), 1<<24)
		hostAdapter := false
		for line := 1; scanner.Scan(); line++ {
			text := scanner.Text()
			trimmed := strings.TrimSpace(text)
			if strings.HasPrefix(trimmed, "package ") {
				hostAdapter = trimmed == "package adapter"
			}
			if hostAdapter || strings.HasPrefix(trimmed, "//") {
				continue
			}
			for _, spelling := range unownedRootLookups {
				if strings.Contains(text, spelling) {
					findings = append(findings, fmt.Sprintf("%s:%d: %s", entry.Name(), line, spelling))
				}
			}
		}
		file.Close()
	}
	return findings
}

// 🩺️Problems reports why the running test binary does not run against a root it owns: the bound root, the working directory or the test sources of the module under test point at the checkout the binary was started in.
func Problems(guardCase string) []string {
	problems := []string{}
	if root == "" {
		return []string{"the test binary does not run through sandbox.Run"}
	}
	if bound, ok := workspace.LookupRootDir(); !ok || workspace.CanonicalPath(bound) != workspace.CanonicalPath(root) {
		problems = append(problems, fmt.Sprintf("the bound root %q is not the sandbox root %q", bound, root))
	}
	if cwd, err := os.Getwd(); err != nil || !PathWithin(root, cwd) {
		problems = append(problems, fmt.Sprintf("the working directory %q left the sandbox root %q", cwd, root))
	}
	testsDir := filepath.Join(origin, "🧪️tests")
	if info, err := os.Stat(testsDir); err != nil || !info.IsDir() {
		testsDir = filepath.Join(origin, "..", "..", "🧪️tests")
	}
	return append(problems, UnownedRootLookups(testsDir, guardCase)...)
}

// #endregion 🩺️Sandbox Audit
