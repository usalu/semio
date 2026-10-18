// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// File header IDs, file kind derivation, and kind emoji.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestFileHeaderId(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"config json", "tsconfig.json", emojiText(EmojiFileConfig) + "tsconfig"},
		{"docs md", "README.md", emojiText(EmojiFileDocs) + "readme"},
		{"script sh", "build.sh", emojiText(EmojiFileScript) + "build"},
		{"script bash", "deploy.bash", emojiText(EmojiFileScript) + "deploy"},
		{"script ps1", "setup.ps1", emojiText(EmojiFileScript) + "setup"},
		{"resource png", "🖼️logo.png", emojiText(EmojiFileResource) + Flat("🖼️logo")},
		{"license", "LICENSE.md", emojiText(EmojiFileLicense) + "license"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := FileHeaderId(tt.path)
			if got != tt.want {
				t.Errorf("FileHeaderId(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}

	t.Run("shebang ts file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "tools/build.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env tsx\nconsole.log('build');\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := emojiText(EmojiFolderOrg) + "tools" + emojiText(EmojiFileScript) + Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("shebang py file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "scripts/run.py"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env python3\nprint('hello')\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := emojiText(EmojiFolderOrg) + "scripts" + emojiText(EmojiFileScript) + Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("code ts without shebang stays code", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "src/index.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("export const x = 1;\n"), 0644)

		got := FileHeaderId(filePath)
		baseName := filepath.Base(filePath)
		want := emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + Flat(strings.TrimSuffix(baseName, filepath.Ext(baseName)))
		if got != want {
			t.Errorf("FileHeaderId(%q) without shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("nonexistent code file stays code", func(t *testing.T) {
		got := FileHeaderId("nonexistent/file.ts")
		want := emojiText(EmojiFolderOrg) + "nonexistent" + emojiText(EmojiFileCode) + Flat("file")
		if got != want {
			t.Errorf("FileHeaderId for nonexistent file = %q, want %q", got, want)
		}
	})
}

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
		{"lab", "lab", emojiText(EmojiFileLab)},
		{"script", "script", "\U0001F4DC"},
		{"docs", "docs", "\U0001F4C3"},
		{"config", "config", "\u2699\uFE0F"},
		{"resource", "resource", "\U0001F4BE"},
		{"template", "template", emojiText(EmojiFileTemplate)},
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
