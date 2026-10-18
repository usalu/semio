// 📑️ Tests of the native docstring autofixes.

package graphql

import (
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestDefinitionNativeDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖️Header\n\n// [💻️src/app.ts](repo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖️Header\n\n// #region 🔖️Functions\n\n// [🔖️src/app.ts#Functions](repo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖️Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := statutes.NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []model.Breach
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "/**") {
		t.Fatal("expected JSDoc opening after autofix")
	}
	if !strings.Contains(fixedContent, "**/") {
		t.Fatal("expected JSDoc closing after autofix")
	}
	if !strings.Contains(fixedContent, " * Does work.") {
		t.Fatal("expected summary line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * doWork MUST be idempotent.") {
		t.Fatal("expected spec line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, "§doWork") {
		t.Fatal("expected identification in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("expected identification emitted as a single JSDoc line after autofix")
	}
	if strings.Contains(fixedContent, " *  * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("did not expect doubled asterisk marker before definition identification after autofix")
	}
	if strings.Contains(fixedContent, " *\n * [🛠️src/app.ts#Functions§doWork](repo://definition/src/app.ts/functions/dowork)") {
		t.Fatal("did not expect an extra blank JSDoc separator before definition identification after autofix")
	}
}

func TestPythonTripleQuoteDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖️Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := statutes.NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []model.Breach
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(fixedContent, `"""Does work.`) {
		t.Fatal("expected triple-quote docstring with summary after autofix")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec line in docstring after autofix")
	}
	if !strings.Contains(fixedContent, "§do_work") {
		t.Fatal("expected identification in docstring after autofix")
	}
	if !strings.Contains(fixedContent, `"""`) {
		t.Fatal("expected closing triple-quote after autofix")
	}
	if strings.Contains(fixedContent, "# Does work.") {
		t.Fatal("# comment should be removed after autofix")
	}
}

func TestPythonTripleQuoteDocstringMerge(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖️Header\n\n# [💻️src/app.py](repo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖️Header\n\n# #region 🔖️Functions\n\n# [🔖️src/app.py#Functions](repo://section/src/app.py/functions)\n\n# Function declarations.\n\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](repo://definition/src/app.py/functions/do_work)\ndef do_work():\n    \"\"\"Does work.\"\"\"\n    pass\n\n# #endregion 🔖️Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: testFile}
	ctx := statutes.NewPolicyContextWithFiles(scope, []model.Bundle{}, []string{testFile})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []model.Breach
	for _, v := range breachs {
		if v.Kind == model.BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach for # comments above existing docstring")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "Does work.") {
		t.Fatal("expected existing summary preserved after merge")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec from # comment merged into docstring")
	}
	if !strings.Contains(fixedContent, "§do_work") {
		t.Fatal("expected identification merged into docstring")
	}
	if strings.Contains(fixedContent, "# do_work MUST") {
		t.Fatal("# comment should be removed after merge autofix")
	}
}
