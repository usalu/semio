// 🆔️ Tests of the artifact identifiers of the temporal, code and goal trees.

package model

import (
	testing "testing"
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
	expected := EmojiText(EmojiYear)
	if id != expected {
		t.Errorf("years id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Year(t *testing.T) {
	id := GetArtifactID("year", map[string]interface{}{"parentId": "", "yy": "26"})
	expected := EmojiText(EmojiYear) + "26"
	if id != expected {
		t.Errorf("year id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Months(t *testing.T) {
	yearId := EmojiText(EmojiYear) + "26"
	id := GetArtifactID("months", map[string]interface{}{"parentId": yearId})
	expected := yearId + EmojiText(EmojiMonth)
	if id != expected {
		t.Errorf("months id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Month(t *testing.T) {
	yearId := EmojiText(EmojiYear) + "26"
	id := GetArtifactID("month", map[string]interface{}{"parentId": yearId, "mm": "02"})
	expected := yearId + EmojiText(EmojiMonth) + "02"
	if id != expected {
		t.Errorf("month id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Days(t *testing.T) {
	monthId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02"
	id := GetArtifactID("days", map[string]interface{}{"parentId": monthId})
	expected := monthId + EmojiText(EmojiDay)
	if id != expected {
		t.Errorf("days id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Day(t *testing.T) {
	monthId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02"
	id := GetArtifactID("day", map[string]interface{}{"parentId": monthId, "dd": "15"})
	expected := monthId + EmojiText(EmojiDay) + "15"
	if id != expected {
		t.Errorf("day id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hours(t *testing.T) {
	dayId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15"
	id := GetArtifactID("hours", map[string]interface{}{"parentId": dayId})
	expected := dayId + EmojiText(EmojiHour)
	if id != expected {
		t.Errorf("hours id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hour(t *testing.T) {
	dayId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15"
	id := GetArtifactID("hour", map[string]interface{}{"parentId": dayId, "hh": "14"})
	expected := dayId + EmojiText(EmojiHour) + "14"
	if id != expected {
		t.Errorf("hour id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minutes(t *testing.T) {
	hourId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14"
	id := GetArtifactID("minutes", map[string]interface{}{"parentId": hourId})
	expected := hourId + EmojiText(EmojiMinute)
	if id != expected {
		t.Errorf("minutes id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minute(t *testing.T) {
	hourId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14"
	id := GetArtifactID("minute", map[string]interface{}{"parentId": hourId, "mm": "33"})
	expected := hourId + EmojiText(EmojiMinute) + "33"
	if id != expected {
		t.Errorf("minute id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Seconds(t *testing.T) {
	minuteId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14" + EmojiText(EmojiMinute) + "33"
	id := GetArtifactID("seconds", map[string]interface{}{"parentId": minuteId})
	expected := minuteId + EmojiText(EmojiSecond)
	if id != expected {
		t.Errorf("seconds id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Second(t *testing.T) {
	minuteId := EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "15" + EmojiText(EmojiHour) + "14" + EmojiText(EmojiMinute) + "33"
	id := GetArtifactID("second", map[string]interface{}{"parentId": minuteId, "ss": "38"})
	expected := minuteId + EmojiText(EmojiSecond) + "38"
	if id != expected {
		t.Errorf("second id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Codebase(t *testing.T) {
	id := GetArtifactID("codebase", map[string]interface{}{"parentId": ""})
	expected := EmojiText(EmojiCodebase)
	if id != expected {
		t.Errorf("codebase id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Technologies(t *testing.T) {
	id := GetArtifactID("technologies", map[string]interface{}{"parentId": ""})
	expected := EmojiText(EmojiTechnologies)
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
		{"user technology", map[string]interface{}{"name": "compose", "kind": "user"}, EmojiText(EmojiTechnologyUser) + "compose"},
		{"infra technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, EmojiText(EmojiTechnologyInfra) + "repo"},
		{"research technology", map[string]interface{}{"name": "coda", "kind": "research"}, EmojiText(EmojiTechnologyResearch) + "coda"},
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
	technologyId := EmojiText(EmojiTechnologyUser) + "compose"
	id := GetArtifactID("bundles", map[string]interface{}{"parentId": technologyId})
	expected := technologyId + EmojiText(EmojiBundles)
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
		{"library bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle", map[string]interface{}{"name": "repo/graphql", "kind": "schema"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"},
		{"ui bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleUI) + "vscode"},
		{"example bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleExample) + "example"},
		{"site bundle", map[string]interface{}{"name": "compose/desktop", "kind": "site"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleSite) + "desktop"},
		{"assets bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, EmojiText(EmojiTechnologyUser) + "asset" + EmojiText(EmojiBundleAssets) + "asset"},
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
		{"root folders", "", EmojiText(EmojiFolders)},
		{"bundle folders", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolders)},
		{"required folder folders", EmojiText(EmojiFolderRequired) + "github", EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFolders)},
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
	bundleId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"org folder under bundle", map[string]interface{}{"path": "compose/js/sketchpad", "name": "sketchpad", "kind": "organization", "parentId": bundleId}, bundleId + EmojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder at root", map[string]interface{}{"path": ".devcontainer", "name": ".devcontainer", "kind": "required", "parentId": ""}, EmojiText(EmojiFolderRequired) + "devcontainer"},
		{"nested folder", map[string]interface{}{"path": "compose/js/sketchpad/pages", "name": "pages", "kind": "organization", "parentId": bundleId + EmojiText(EmojiFolderOrg) + "sketchpad"}, bundleId + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolderOrg) + "pages"},
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
		{"root files", "", EmojiText(EmojiFiles)},
		{"folder files", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFiles)},
		{"required folder files", EmojiText(EmojiFolderRequired) + "github", EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFiles)},
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
	folderId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"code file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code", "parentId": folderId}, folderId + EmojiText(EmojiFileCode) + "design"},
		{"test file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "name": "sketchpad.test.ts", "kind": "lab", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFileLab) + "sketchpadtest"},
		{"config file at root", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "name": "devcontainer.json", "kind": "config", "parentId": EmojiText(EmojiFolderRequired) + "devcontainer"}, EmojiText(EmojiFolderRequired) + "devcontainer" + EmojiText(EmojiFileConfig) + "devcontainer"},
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
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	id := GetArtifactID("line", map[string]interface{}{"parentId": fileId, "line": float64(3872)})
	expected := fileId + EmojiText(EmojiLine) + "3872"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Range(t *testing.T) {
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	id := GetArtifactID("range", map[string]interface{}{"parentId": fileId, "startLine": float64(3872), "endLine": float64(3875)})
	expected := fileId + EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Sections(t *testing.T) {
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	id := GetArtifactID("sections", map[string]interface{}{"parentId": fileId})
	expected := fileId + EmojiText(EmojiSections)
	if id != expected {
		t.Errorf("sections id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Section(t *testing.T) {
	fileId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level section", map[string]interface{}{"name": "State Managment", "parentId": fileId}, fileId + EmojiText(EmojiSection) + "statemanagment"},
		{"nested section", map[string]interface{}{"name": "Store", "parentId": fileId + EmojiText(EmojiSection) + "statemanagment"}, fileId + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("section", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Definitions(t *testing.T) {
	sectionId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"
	id := GetArtifactID("definitions", map[string]interface{}{"parentId": sectionId})
	expected := sectionId + EmojiText(EmojiDefinitions)
	if id != expected {
		t.Errorf("definitions id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Definition(t *testing.T) {
	sectionId := EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"
	id := GetArtifactID("definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": sectionId})
	expected := sectionId + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Goals(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root goals", "", EmojiText(EmojiGoals)},
		{"nested goals", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiGoals)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("goals", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}
