// 🥾️ Tests of the cli domain: MCP bootstrap assets.

package cli

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	unicode "unicode"
)

func TestMcpBootstrapAssetsStayRepoRelative(t *testing.T) {
	repoRoot := findTestRepoRoot(".")

	cases := []struct {
		name               string
		path               string
		requiredFragments  []string
		forbiddenFragments []string
	}{
		{
			name: "codex uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".codex", "config.toml"),
			requiredFragments: []string{
				`command = "bun"`,
				`args = ["./📜️script.ts", "dev", "mcp", "stdio", "codex"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "cursor uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".cursor", "mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "cursor"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "generic clients use the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "copilot uses the cross-platform repo bootstrap",
			path: filepath.Join(repoRoot, ".vscode", "mcp.json"),
			requiredFragments: []string{
				`"command": "bun"`,
				`"args": ["./📜️script.ts", "dev", "mcp", "stdio", "copilot"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
			},
		},
		{
			name: "devcontainer provisions through the cross-platform workspace setup",
			path: filepath.Join(repoRoot, ".devcontainer", "devcontainer.json"),
			requiredFragments: []string{
				`"postCreateCommand": ["bun", "repo", "run", "workspace:setup"]`,
			},
			forbiddenFragments: []string{
				"repo/client/client",
				"./repo/client/mcp/go",
				"post-create.sh",
			},
		},
		{
			name: "native shell bootstrap builds the repo client from its canonical source",
			path: filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🐚️.sh"),
			requiredFragments: []string{
				"bun repo run workspace:setup",
				"bun repo run @semio-tech/repo-client:build",
			},
			forbiddenFragments: []string{
				"repo/client/client",
				"./repo/client/mcp/go",
				"💻️client",
			},
		},
		{
			name: "native windows bootstrap builds the repo client from its canonical source",
			path: filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🔩️native", "🥾️bootstrap", "🔵️.ps1"),
			requiredFragments: []string{
				`@("repo", "run", "workspace:setup")`,
				`@("repo", "run", "@semio-tech/repo-client:build")`,
			},
			forbiddenFragments: []string{
				"repo/client/client.exe",
				"./repo/client/mcp/go",
				"💻️client",
			},
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			data, err := os.ReadFile(tc.path)
			if err != nil {
				t.Fatalf("failed to read %s: %v", tc.path, err)
			}
			text := string(data)
			compact := func(value string) string {
				return strings.Map(func(character rune) rune {
					if unicode.IsSpace(character) {
						return -1
					}
					return character
				}, value)
			}
			for _, fragment := range tc.requiredFragments {
				if !strings.Contains(compact(text), compact(fragment)) {
					t.Fatalf("expected %s to contain %q", tc.path, fragment)
				}
			}
			for _, fragment := range tc.forbiddenFragments {
				if strings.Contains(text, fragment) {
					t.Fatalf("expected %s to exclude %q", tc.path, fragment)
				}
			}
		})
	}
}
