// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Artifact IDs for time, codebase, technology, bundle, folder, and file nodes.

// #endregion 🧲️Header

package client

import (
	"testing"
)

// 🏺️#region 🪨️Entity ID
func TestGetArtifactID_Root(t *testing.T) {
	id := GetArtifactID("root", map[string]interface{}{})
	if id != "" {
		t.Errorf("root id: expected empty, got %q", id)
	}
}

func TestGetArtifactID_Years(t *testing.T) {
	id := GetArtifactID("years", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiYear)
	if id != expected {
		t.Errorf("years id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Year(t *testing.T) {
	id := GetArtifactID("year", map[string]interface{}{"parentId": "", "yy": "26"})
	expected := emojiText(EmojiYear) + "26"
	if id != expected {
		t.Errorf("year id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Months(t *testing.T) {
	yearId := emojiText(EmojiYear) + "26"
	id := GetArtifactID("months", map[string]interface{}{"parentId": yearId})
	expected := yearId + emojiText(EmojiMonth)
	if id != expected {
		t.Errorf("months id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Month(t *testing.T) {
	yearId := emojiText(EmojiYear) + "26"
	id := GetArtifactID("month", map[string]interface{}{"parentId": yearId, "mm": "02"})
	expected := yearId + emojiText(EmojiMonth) + "02"
	if id != expected {
		t.Errorf("month id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Days(t *testing.T) {
	monthId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02"
	id := GetArtifactID("days", map[string]interface{}{"parentId": monthId})
	expected := monthId + emojiText(EmojiDay)
	if id != expected {
		t.Errorf("days id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Day(t *testing.T) {
	monthId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02"
	id := GetArtifactID("day", map[string]interface{}{"parentId": monthId, "dd": "15"})
	expected := monthId + emojiText(EmojiDay) + "15"
	if id != expected {
		t.Errorf("day id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hours(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	id := GetArtifactID("hours", map[string]interface{}{"parentId": dayId})
	expected := dayId + emojiText(EmojiHour)
	if id != expected {
		t.Errorf("hours id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hour(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	id := GetArtifactID("hour", map[string]interface{}{"parentId": dayId, "hh": "14"})
	expected := dayId + emojiText(EmojiHour) + "14"
	if id != expected {
		t.Errorf("hour id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minutes(t *testing.T) {
	hourId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14"
	id := GetArtifactID("minutes", map[string]interface{}{"parentId": hourId})
	expected := hourId + emojiText(EmojiMinute)
	if id != expected {
		t.Errorf("minutes id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minute(t *testing.T) {
	hourId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14"
	id := GetArtifactID("minute", map[string]interface{}{"parentId": hourId, "mm": "33"})
	expected := hourId + emojiText(EmojiMinute) + "33"
	if id != expected {
		t.Errorf("minute id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Seconds(t *testing.T) {
	minuteId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14" + emojiText(EmojiMinute) + "33"
	id := GetArtifactID("seconds", map[string]interface{}{"parentId": minuteId})
	expected := minuteId + emojiText(EmojiSecond)
	if id != expected {
		t.Errorf("seconds id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Second(t *testing.T) {
	minuteId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14" + emojiText(EmojiMinute) + "33"
	id := GetArtifactID("second", map[string]interface{}{"parentId": minuteId, "ss": "38"})
	expected := minuteId + emojiText(EmojiSecond) + "38"
	if id != expected {
		t.Errorf("second id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Codebase(t *testing.T) {
	id := GetArtifactID("codebase", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiCodebase)
	if id != expected {
		t.Errorf("codebase id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Technologies(t *testing.T) {
	id := GetArtifactID("technologies", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiTechnologies)
	if id != expected {
		t.Errorf("technologies id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Technology(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"user technology", map[string]interface{}{"name": "compose", "kind": "user"}, emojiText(EmojiTechnologyUser) + "compose"},
		{"infra technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, emojiText(EmojiTechnologyInfra) + "repo"},
		{"research technology", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiTechnologyResearch) + "coda"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("technology", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Bundles(t *testing.T) {
	technologyId := emojiText(EmojiTechnologyUser) + "compose"
	id := GetArtifactID("bundles", map[string]interface{}{"parentId": technologyId})
	expected := technologyId + emojiText(EmojiBundles)
	if id != expected {
		t.Errorf("bundles id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Bundle(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"library bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle", map[string]interface{}{"name": "repo/graphql", "kind": "schema"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleBinary) + "client"},
		{"ui bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, emojiText(EmojiTechnologyInfra) + "repo" + emojiText(EmojiBundleUI) + "vscode"},
		{"example bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, emojiText(EmojiTechnologyResearch) + "coda" + emojiText(EmojiBundleExample) + "example"},
		{"site bundle", map[string]interface{}{"name": "compose/desktop", "kind": "site"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleSite) + "desktop"},
		{"assets bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, emojiText(EmojiTechnologyUser) + "asset" + emojiText(EmojiBundleAssets) + "asset"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("bundle", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Folders(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root folders", "", emojiText(EmojiFolders)},
		{"bundle folders", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolders)},
		{"required folder folders", emojiText(EmojiFolderRequired) + "github", emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFolders)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("folders", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Folder(t *testing.T) {
	bundleId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"org folder under bundle", map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization", "parentId": bundleId}, bundleId + emojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder at root", map[string]interface{}{"path": ".devcontainer", "name": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
		{"nested folder", map[string]interface{}{"path": "compose/js/sketchpad/pages", "name": "pages", "kind": "organization", "parentId": bundleId + emojiText(EmojiFolderOrg) + "sketchpad"}, bundleId + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("folder", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Files(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root files", "", emojiText(EmojiFiles)},
		{"folder files", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFiles)},
		{"required folder files", emojiText(EmojiFolderRequired) + "github", emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFiles)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("files", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_File(t *testing.T) {
	folderId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"code file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code", "parentId": folderId}, folderId + emojiText(EmojiFileCode) + "design"},
		{"test file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "name": "sketchpad.test.ts", "kind": "lab", "parentId": emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileLab) + "sketchpadtest"},
		{"config file at root", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "name": "devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainer"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("file", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Line(t *testing.T) {
	fileId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"
	id := GetArtifactID("line", map[string]interface{}{"parentId": fileId, "line": float64(3872)})
	expected := fileId + emojiText(EmojiLine) + "3872"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Range(t *testing.T) {
	fileId := emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "design"
	id := GetArtifactID("range", map[string]interface{}{"parentId": fileId, "startLine": float64(3872), "endLine": float64(3875)})
	expected := fileId + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}
