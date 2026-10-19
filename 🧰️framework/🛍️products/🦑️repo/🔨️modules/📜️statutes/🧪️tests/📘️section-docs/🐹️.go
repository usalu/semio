package statutes

import (
	os "os"
	filepath "path/filepath"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// #region 📘️Section Docs Tests

func TestSectionMissingSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[model.Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[model.BreachCodeSectionMissingSummary] == 0 {
		t.Fatal("expected section missing summary breach")
	}
}

func TestSectionWithSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeSectionMissingSummary {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestDefinitionMissingSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Function declarations.\n\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[model.Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[model.BreachCodeDefMissingSummary] == 0 {
		t.Fatal("expected definition missing summary breach")
	}
}

func TestDefinitionWithSummaryAndRequirements(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Function declarations.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefMissingSummary || v.Kind == model.BreachCodeDefMissingRequirements {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestSectionDocLinesExemptsDocComments(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// 💻️src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n// Function declarations.\n// Functions MUST be exported.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []model.Bundle{}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeCommentInline {
			t.Fatalf("section doc comment wrongly flagged as inline at line %d", v.Line)
		}
	}
}

func TestDefinitionNativeDocstring(t *testing.T) {
	tests := []struct {
		name         string
		file         string
		content      string
		expectBreach bool
	}{
		{
			name:         "TypeScript // comments should flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n",
			expectBreach: true,
		},
		{
			name:         "TypeScript JSDoc should NOT flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n",
			expectBreach: false,
		},
		{
			name:         "Go // comments should NOT flag breach (native format)",
			file:         "src/app.go",
			content:      "package main\n\n// #region 🔖️Header\n\n// [💻️src/app.go](repo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.go#Functions](repo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](repo://definition/src/app.go/functions/dowork)\nfunc DoWork() {}\n\n// #endregion 🔖️Functions\n",
			expectBreach: false,
		},
		{
			name:         "Python # comments should flag breach (should use triple-quote docstring)",
			file:         "src/app.py",
			content:      "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖️Functions\n",
			expectBreach: true,
		},
		{
			name:         "Python triple-quote docstring should NOT flag breach",
			file:         "src/app.py",
			content:      "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖️Functions\n",
			expectBreach: false,
		},
		{
			name:         "CSharp // comments should flag breach (should use ///)",
			file:         "src/App.cs",
			content:      "// #region 🔖️Header\n\n// [💻️src/App.cs](repo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/App.cs#Types](repo://section/src/app.cs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/App.cs#Types§AppState](repo://definition/src/app.cs/type/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖️Types\n",
			expectBreach: true,
		},
		{
			name:         "CSharp /// comments should NOT flag breach",
			file:         "src/App.cs",
			content:      "// #region 🔖️Header\n\n// [💻️src/App.cs](repo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/App.cs#Types](repo://section/src/app.cs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/App.cs#Types§AppState](repo://definition/src/app.cs/type/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖️Types\n",
			expectBreach: false,
		},
		{
			name:         "Rust // comments should flag breach (should use ///)",
			file:         "src/lib.rs",
			content:      "// #region 🔖️Header\n\n// [💻️src/lib.rs](repo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/lib.rs#Types](repo://section/src/lib.rs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/lib.rs#Types§AppState](repo://definition/src/lib.rs/type/appstate)\npub struct AppState {}\n\n// #endregion 🔖️Types\n",
			expectBreach: true,
		},
		{
			name:         "Rust /// comments should NOT flag breach",
			file:         "src/lib.rs",
			content:      "// #region 🔖️Header\n\n// [💻️src/lib.rs](repo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Types\n\n// [🔖️src/lib.rs#Types](repo://section/src/lib.rs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/lib.rs#Types§AppState](repo://definition/src/lib.rs/type/appstate)\npub struct AppState {}\n\n// #endregion 🔖️Types\n",
			expectBreach: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			sandbox.BindRoot(t, tmpDir)
			dir := filepath.Dir(filepath.Join(tmpDir, tt.file))
			os.MkdirAll(dir, 0o755)
			absPath := filepath.Join(tmpDir, tt.file)
			if err := workspace.WriteTextFile(absPath, tt.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: tt.file}
			ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{tt.file})
			breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			hasBreach := false
			for _, v := range breachs {
				if v.Kind == model.BreachCodeDefNotNativeDocstring {
					hasBreach = true
					break
				}
			}
			if tt.expectBreach && !hasBreach {
				t.Fatal("expected DefNotNativeDocstring breach but got none")
			}
			if !tt.expectBreach && hasBreach {
				t.Fatal("unexpected DefNotNativeDocstring breach")
			}
		})
	}
}

func TestPythonTripleQuoteDocstringExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖️Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeCommentBlock {
			t.Fatalf("Python triple-quote docstring should not be flagged as block comment at line %d", v.Line)
		}
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			t.Fatalf("Python triple-quote docstring should not flag DefNotNativeDocstring at line %d", v.Line)
		}
	}
}

func TestDefinitionJSDocExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == model.BreachCodeCommentJSDoc {
			t.Fatalf("definition JSDoc should not be flagged as comment breach at line %d", v.Line)
		}
		if v.Kind == model.BreachCodeCommentBlock {
			t.Fatalf("definition JSDoc should not be flagged as block comment breach at line %d", v.Line)
		}
	}
}

// #endregion 📘️Section Docs Tests
