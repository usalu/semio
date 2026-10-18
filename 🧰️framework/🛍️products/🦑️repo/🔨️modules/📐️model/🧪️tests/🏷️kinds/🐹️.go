// 🏷️ Tests of the file and definition kind derivations.

package model

import (
	strings "strings"
	testing "testing"
)

func TestDeriveFileKind(t *testing.T) {
	tests := []struct {
		name string
		file string
		want string
	}{
		{"ts code", "index.ts", FileKindCode},
		{"tsx code", "App.tsx", FileKindCode},
		{"go code", "main.go", FileKindCode},
		{"py code", "compose.py", FileKindCode},
		{"cs code", "Compose.cs", FileKindCode},
		{"rs code", "lib.rs", FileKindCode},
		{"rb code", "app.rb", FileKindCode},
		{"sh script", "build.sh", FileKindScript},
		{"bash script", "deploy.bash", FileKindScript},
		{"zsh script", "setup.zsh", FileKindScript},
		{"fish script", "init.fish", FileKindScript},
		{"bat script", "run.bat", FileKindScript},
		{"cmd script", "build.cmd", FileKindScript},
		{"ps1 script", "setup.ps1", FileKindScript},
		{"psm1 script", "module.psm1", FileKindScript},
		{"test ts", "🧪️index.test.ts", FileKindLab},
		{"test go", "main_test.go", FileKindLab},
		{"spec ts", "app.spec.ts", FileKindLab},
		{"benchmark go", "compose_benchmark.go", FileKindLab},
		{"stories tsx", "Button.stories.tsx", FileKindLab},
		{"json config", "tsconfig.json", FileKindConfig},
		{"yaml config", "config.yaml", FileKindConfig},
		{"toml config", "pyproject.toml", FileKindConfig},
		{"env config", ".env", FileKindConfig},
		{"md docs", "README.md", FileKindDocs},
		{"txt docs", "notes.txt", FileKindDocs},
		{"png resource", "🖼️logo.png", FileKindResource},
		{"svg resource", "icon.svg", FileKindResource},
		{"wasm resource", "module.wasm", FileKindResource},
		{"tpl template", "layout.tpl", FileKindTemplate},
		{"tmpl template", "page.tmpl", FileKindTemplate},
		{"gotmpl template", "header.gotmpl", FileKindTemplate},
		{"mustache template", "view.mustache", FileKindTemplate},
		{"hbs template", "partial.hbs", FileKindTemplate},
		{"jinja2 template", "base.jinja2", FileKindTemplate},
		{"j2 template", "config.j2", FileKindTemplate},
		{"ejs template", "page.ejs", FileKindTemplate},
		{"njk template", "layout.njk", FileKindTemplate},
		{"pug template", "index.pug", FileKindTemplate},
		{"license md", "LICENSE.md", FileKindLicense},
		{"licence txt", "LICENCE.txt", FileKindLicense},
		{"gitignore config", ".gitignore", FileKindConfig},
		{"dockerfile config", "Dockerfile", FileKindConfig},
		{"makefile config", "Makefile", FileKindConfig},
		{"config suffix", "⚙️vite.config.ts", FileKindConfig},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := DeriveFileKind(tt.file)
			if got != tt.want {
				t.Errorf("DeriveFileKind(%q) = %q, want %q", tt.file, got, tt.want)
			}
		})
	}
}

func TestFileKindEmoji(t *testing.T) {
	tests := []struct {
		name  string
		kind  string
		emoji string
	}{
		{"code", "code", "\U0001F4BB"},
		{"lab", "lab", EmojiText(EmojiFileLab)},
		{"script", "script", "\U0001F4DC"},
		{"docs", "docs", "\U0001F4C3"},
		{"config", "config", "\u2699\uFE0F"},
		{"resource", "resource", "\U0001F4BE"},
		{"template", "template", EmojiText(EmojiFileTemplate)},
		{"license", "license", "\u2696\uFE0F"},
		{"unknown", "unknown", ""},
		{"empty", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := map[string]interface{}{"kind": tt.kind}
			got := fileKindEmoji(data)
			if got != tt.emoji {
				t.Errorf("fileKindEmoji(%q) = %q, want %q", tt.kind, got, tt.emoji)
			}
		})
	}
}

// 🧪️#region 🕸️Test Command
func TestIsTestFunctionName(t *testing.T) {

	tests := []struct {
		name     string
		input    string
		expected bool
	}{
		{"Go TestXxx", "TestSomething", true},
		{"Go TestAbc", "TestAbc", true},
		{"Go BenchmarkXxx", "BenchmarkSomething", true},
		{"Go FuzzXxx", "FuzzSomething", true},
		{"Python test_xxx", "test_something", true},
		{"Go Test without capital", "Testnotcapital", false},
		{"plain func", "doSomething", false},
		{"plain func capitalized", "DoSomething", false},
		{"Test alone not enough", "Test", false},
		{"Benchmark alone not enough", "Benchmark", false},
		{"Fuzz alone not enough", "Fuzz", false},
		{"test_ prefix not Go", "test_go_style", true},
		{"lowercase test", "testsomething", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := isTestFunctionName(tt.input)
			if got != tt.expected {
				t.Errorf("isTestFunctionName(%q) = %v, want %v", tt.input, got, tt.expected)
			}
		})
	}
}

func TestDefinitionKindTestEmoji(t *testing.T) {

	emoji := definitionKindEmoji(map[string]interface{}{"kind": string(DefinitionKindTest)})
	expected := EmojiText(EmojiDefinitionTest)
	if emoji != expected {
		t.Errorf("definitionKindEmoji(test) = %q, want %q", emoji, expected)
	}
}

func TestDefinitionKindTestCode(t *testing.T) {

	code := definitionKindCode(map[string]interface{}{"kind": string(DefinitionKindTest)})
	if code != "t" {
		t.Errorf("definitionKindCode(DefinitionKindTest) = %q, want %q", code, "t")
	}

	emoji := definitionEmojiFromCode("t")
	if emoji != EmojiDefinitionTest {
		t.Errorf("definitionEmojiFromCode(t) = %q, want %q", emoji, EmojiDefinitionTest)
	}
}

func TestDefinitionKindTestIsValid(t *testing.T) {
	if !DefinitionKindTest.IsValid() {
		t.Error("DefinitionKindTest.IsValid() should return true")
	}
}

func TestBuildDefinitionIDTestKind(t *testing.T) {

	labFileID := EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client" + EmojiText(EmojiFileLab) + "maintest"

	id := BuildDefinitionID(labFileID, nil, "TestSomething", DefinitionKindImplementation)
	testEmoji := EmojiText(EmojiDefinitionTest)
	implEmoji := EmojiText(EmojiDefinitionImpl)

	if !strings.Contains(id, testEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should contain test emoji %q, got %q", testEmoji, id)
	}
	if strings.Contains(id, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in lab file should NOT contain impl emoji %q, got %q", implEmoji, id)
	}

	id2 := BuildDefinitionID(labFileID, nil, "helperFunc", DefinitionKindImplementation)
	if !strings.Contains(id2, implEmoji) {
		t.Errorf("buildDefinitionID for helperFunc in lab file should contain impl emoji %q, got %q", implEmoji, id2)
	}

	codeFileID := EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client" + EmojiText(EmojiFileCode) + "main"
	id3 := BuildDefinitionID(codeFileID, nil, "TestSomething", DefinitionKindImplementation)
	if !strings.Contains(id3, implEmoji) {
		t.Errorf("buildDefinitionID for TestSomething in code file should contain impl emoji %q, got %q", implEmoji, id3)
	}
}
