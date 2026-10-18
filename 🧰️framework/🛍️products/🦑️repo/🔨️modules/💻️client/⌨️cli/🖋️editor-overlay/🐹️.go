// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Editor surface overlays and system policy.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"
)

// 📝️EditorSurface is the editor state of one host: workspace settings and recommended extensions.
type EditorSurface struct {
	Settings   map[string]interface{}
	Extensions []string
}

// 🧩️EditorOverlay declares what the devcontainer does not inherit from the host editor files and what only it adds (`.devcontainer/editor-overlay.json`).
type EditorOverlay struct {
	Settings struct {
		Omit []string               `json:"omit"`
		Set  map[string]interface{} `json:"set"`
	} `json:"settings"`
	Extensions struct {
		Omit []string `json:"omit"`
		Add  []string `json:"add"`
	} `json:"extensions"`
}

// 🧮️DeriveContainerEditor derives the devcontainer editor surface from the host canonical surface and the declared overlay:
// the host entries minus the omitted ones, `${workspaceFolder}` rewritten to `${containerWorkspaceFolder}`, then the overlay's own settings and extensions.
func DeriveContainerEditor(canonical EditorSurface, overlay EditorOverlay) EditorSurface {
	settings := map[string]interface{}{}
	encoded, err := json.Marshal(canonical.Settings)
	if err == nil {
		_ = json.Unmarshal([]byte(strings.ReplaceAll(string(encoded), "${workspaceFolder}", "${containerWorkspaceFolder}")), &settings)
	}
	for _, key := range overlay.Settings.Omit {
		delete(settings, key)
	}
	for key, value := range overlay.Settings.Set {
		settings[key] = value
	}
	var extensions []string
	seen := map[string]struct{}{}
	for _, id := range append(slices.DeleteFunc(slices.Clone(canonical.Extensions), func(id string) bool { return slices.Contains(overlay.Extensions.Omit, id) }), overlay.Extensions.Add...) {
		if _, ok := seen[id]; !ok {
			seen[id] = struct{}{}
			extensions = append(extensions, id)
		}
	}
	return EditorSurface{Settings: settings, Extensions: extensions}
}

// 📦️stripJSONComments removes // and /* */ comments outside strings so JSONC files parse as JSON.
func stripJSONComments(data []byte) []byte {
	out := make([]byte, 0, len(data))
	for index := 0; index < len(data); index++ {
		switch {
		case data[index] == '"':
			start := index
			for index++; index < len(data) && data[index] != '"'; index++ {
				if data[index] == '\\' {
					index++
				}
			}
			out = append(out, data[start:min(index+1, len(data))]...)
		case data[index] == '/' && index+1 < len(data) && data[index+1] == '/':
			for index < len(data) && data[index] != '\n' {
				index++
			}
			out = append(out, '\n')
		case data[index] == '/' && index+1 < len(data) && data[index+1] == '*':
			end := strings.Index(string(data[index+2:]), "*/")
			if end < 0 {
				return out
			}
			index += end + 3
		default:
			out = append(out, data[index])
		}
	}
	return out
}

// 📦️readJSONC parses a JSONC file into `out`.
func readJSONC(path string, out interface{}) bool {
	data, err := os.ReadFile(path)
	if err != nil {
		return false
	}
	return json.Unmarshal(stripJSONComments(data), out) == nil
}

// 📖️ReadHostEditor reads the host canonical editor surface (`.vscode/settings.json`, `.vscode/extensions.json`).
func ReadHostEditor(rootDir string) (EditorSurface, bool) {
	var surface EditorSurface
	var extensions struct {
		Recommendations []interface{} `json:"recommendations"`
	}
	if !readJSONC(filepath.Join(rootDir, ".vscode", "settings.json"), &surface.Settings) || !readJSONC(filepath.Join(rootDir, ".vscode", "extensions.json"), &extensions) {
		return surface, false
	}
	surface.Extensions = extensionIDsFromJSONList(extensions.Recommendations)
	return surface, true
}

// 📖️ReadContainerEditor reads the editor surface checked into `.devcontainer/devcontainer.json`.
func ReadContainerEditor(rootDir string) (EditorSurface, bool) {
	var devcontainer struct {
		Customizations struct {
			Vscode struct {
				Settings   map[string]interface{} `json:"settings"`
				Extensions []interface{}          `json:"extensions"`
			} `json:"vscode"`
		} `json:"customizations"`
	}
	if !readJSONC(filepath.Join(rootDir, ".devcontainer", "devcontainer.json"), &devcontainer) {
		return EditorSurface{}, false
	}
	return EditorSurface{Settings: devcontainer.Customizations.Vscode.Settings, Extensions: extensionIDsFromJSONList(devcontainer.Customizations.Vscode.Extensions)}, true
}

// 📖️ReadEditorOverlay reads `.devcontainer/editor-overlay.json`; a missing file declares no overlay.
func ReadEditorOverlay(rootDir string) EditorOverlay {
	var overlay EditorOverlay
	readJSONC(filepath.Join(rootDir, ".devcontainer", "editor-overlay.json"), &overlay)
	return overlay
}

// 📦️extensionIDsFromJSONList normalizes a JSON string list of extension ids.
func extensionIDsFromJSONList(values []interface{}) []string {
	if len(values) == 0 {
		return nil
	}
	ids := make([]string, 0, len(values))
	for _, value := range values {
		id, ok := value.(string)
		if !ok || id == "" {
			continue
		}
		ids = append(ids, id)
	}
	return ids
}

// ⚖️systemPolicy requires the devcontainer editor block to equal its derivation from the host editor files and the declared overlay.
func systemPolicy(ctx *PolicyContext) []Breach {
	var breachs []Breach
	canonical, hasCanonical := ReadHostEditor(ctx.RootDir)
	actual, hasContainer := ReadContainerEditor(ctx.RootDir)
	if !hasCanonical || !hasContainer {
		return ctx.FilterIgnored(breachs)
	}
	expected := DeriveContainerEditor(canonical, ReadEditorOverlay(ctx.RootDir))
	if !reflect.DeepEqual(expected.Settings, actual.Settings) {
		breachs = append(breachs, ctx.CreateBreach(
			".devcontainer/devcontainer.json customizations.vscode.settings must equal .vscode/settings.json plus .devcontainer/editor-overlay.json",
			BreachSystemDevcontainerVscodeSettingsDrift,
			".devcontainer/devcontainer.json", 1, 0, ""))
	}
	if !slices.Equal(expected.Extensions, actual.Extensions) {
		breachs = append(breachs, ctx.CreateBreach(
			".devcontainer/devcontainer.json customizations.vscode.extensions must equal .vscode/extensions.json plus .devcontainer/editor-overlay.json",
			BreachSystemDevcontainerVscodeExtensionsDrift,
			".devcontainer/devcontainer.json", 1, 0, ""))
	}
	return ctx.FilterIgnored(breachs)
}
