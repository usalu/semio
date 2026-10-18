// 🧬️ Tests of the artifact identifier specification examples.

package model

import (
	testing "testing"

	workspace "github.com/usalu/semio/repo/workspace"
)

func TestSpecExactIDs(t *testing.T) {
	cases := []struct {
		name     string
		kind     string
		data     map[string]interface{}
		expected string
	}{
		{"root", "root", map[string]interface{}{}, ""},
		{"years", "years", map[string]interface{}{"parentId": ""}, "🎆"},
		{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "🎆26"},
		{"months", "months", map[string]interface{}{"parentId": "🎆26"}, "🎆26🌙"},
		{"month 02", "month", map[string]interface{}{"parentId": "🎆26", "mm": "02"}, "🎆26🌙02"},
		{"days", "days", map[string]interface{}{"parentId": "🎆26🌙02"}, "🎆26🌙02☀️"},
		{"day 15", "day", map[string]interface{}{"parentId": "🎆26🌙02", "dd": "15"}, "🎆26🌙02☀️15"},
		{"hours", "hours", map[string]interface{}{"parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⏰"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "🎆26🌙02☀️15", "hh": "14"}, "🎆26🌙02☀️15⏰14"},
		{"minutes", "minutes", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14"}, "🎆26🌙02☀️15⏰14⌚"},
		{"minute 33", "minute", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14", "mm": "33"}, "🎆26🌙02☀️15⏰14⌚33"},
		{"seconds", "seconds", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14⌚33"}, "🎆26🌙02☀️15⏰14⌚33⏱️"},
		{"second 38", "second", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14⌚33", "ss": "38"}, "🎆26🌙02☀️15⏰14⌚33⏱️38"},
		{"technologies", "technologies", map[string]interface{}{"parentId": ""}, "🏗️"},
		{"technology repo", "technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, "🧰repo"},
		{"bundles", "bundles", map[string]interface{}{"parentId": "👤compose"}, "👤compose📦"},
		{"bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, "👤compose📚js"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, "📁"},
		{"bundle folders", "folders", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad"}, "👤compose📚js🗃️sketchpad📁"},
		{"required folder folders", "folders", map[string]interface{}{"parentId": "🛅github"}, "🛅github📁"},
		{"folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": "👤compose📚js"}, "👤compose📚js🗃️sketchpad"},
		{"folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, "🛅devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, "📄"},
		{"folder files", "files", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad"}, "👤compose📚js🗃️sketchpad📄"},
		{"required folder files", "files", map[string]interface{}{"parentId": "🛅github"}, "🛅github📄"},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": "👤compose📚js🗃️sketchpad"}, "👤compose📚js🗃️sketchpad💻design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": "🛅devcontainer"}, "🛅devcontainer⚙️devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design", "line": float64(3872)}, "👤compose📚js🗃️sketchpad💻design📌3872"},
		{"sections in file", "sections", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design"}, "👤compose📚js🗃️sketchpad💻design🔖"},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": "👤compose📚js🗃️sketchpad💻design"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment"},
		{"section Store nested", "section", map[string]interface{}{"name": "Store", "parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🏷️"},
		{"definition createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, "🎯"},
		{"nested goals", "goals", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎯"},
		{"goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": "🎯r26021"}, "🎯r26021🎯runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, "🎫"},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫"},
		{"section tickets", "tickets", map[string]interface{}{"parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🎫"},
		{"ticket", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism"},
		{"draft", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": "🧰repo⌨️client"}, "🧰repo⌨️client📝newarchitecture"},
		{"todo", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore"}, "👤compose📚js🗃️sketchpad💻design🔖statemanagment🔖store🛠️createsketchpadstore📝introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": EmojiText(EmojiFileCode)}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", "policy", map[string]interface{}{"id": "only-one-store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": "💻👮️godfiles",
			"affected": "👤compose📚js🗃️sketchpad💻designstore",
			"lineId":   "📌3872📌3875",
			"secondId": "🎆26🌙02☀️14⏰19⌚07⏱️12",
		}, "💻👮️godfiles🚫👤compose📚js🗃️sketchpad💻designstore🔍📌3872📌3875🎆26🌙02☀️14⏰19⌚07⏱️12"},
		{"contributor", "contributor", map[string]interface{}{"alias": "ueli", "github": "usalu"}, "🧑‍💻ueli"},
		{"checkpoint", "checkpoint", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": "🧑‍💻ueli"}, "🧑‍💻ueli🔀cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      "🎆26🌙02☀️14⏰19⌚07⏱️12",
			"contributorId": "🧑‍💻ueli",
			"entityId":      "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism",
			"kind":          "started",
		}, "🎆26🌙02☀️14⏰19⌚07⏱️12🧑‍💻ueli🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism🌱"},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      "🎆26🌙02☀️14⏰19⌚07⏱️12",
			"contributorId": "🧑‍💻ueli",
			"entityId":      "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism",
			"kind":          "finished",
		}, "🎆26🌙02☀️14⏰19⌚07⏱️12🧑‍💻ueli🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism✅"},
		{"sessions", "sessions", map[string]interface{}{"parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⚪"},
		{"session", "session", map[string]interface{}{"uuid": "e753ed61-e8cc-49b7-88f7-dda53b8d5a15", "parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⚪e753ed61e8cc49b788f7dda53b8d5a15"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID(tc.kind, tc.data)
			if id != tc.expected {
				t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
			}
		})
	}
}

func TestAllSpecIDExamples(t *testing.T) {
	cases := []struct {
		name     string
		kind     string
		data     map[string]interface{}
		expected string
	}{
		{"root id is empty", "root", map[string]interface{}{}, ""},
		{"years under root", "years", map[string]interface{}{"parentId": ""}, "\U0001F386"},
		{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "\U0001F38626"},
		{"months under year", "months", map[string]interface{}{"parentId": "\U0001F38626"}, "\U0001F38626\U0001F319"},
		{"month 02", "month", map[string]interface{}{"parentId": "\U0001F38626", "mm": "02"}, "\U0001F38626\U0001F31902"},
		{"days under month", "days", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay)},
		{"day 15", "day", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902", "dd": "15"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15"},
		{"hours under day", "hours", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F0"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15", "hh": "14"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014"},
		{"minutes under hour", "minutes", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute)},
		{"minute 33", "minute", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014", "mm": "33"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33"},
		{"seconds under minute", "seconds", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33" + EmojiText(EmojiSecond)},
		{"second 38", "second", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33", "ss": "38"}, "\U0001F38626\U0001F31902" + EmojiText(EmojiDay) + "15\u23F014" + EmojiText(EmojiMinute) + "33" + EmojiText(EmojiSecond) + "38"},
		{"technologies under root", "technologies", map[string]interface{}{"parentId": ""}, EmojiText(EmojiTechnologies)},
		{"infra technology repo", "technology", map[string]interface{}{"name": "repo", "kind": "infrastructure"}, EmojiText(EmojiTechnologyInfra) + "repo"},
		{"user technology compose", "technology", map[string]interface{}{"name": "compose", "kind": "user"}, EmojiText(EmojiTechnologyUser) + "compose"},
		{"research technology coda", "technology", map[string]interface{}{"name": "coda", "kind": "research"}, EmojiText(EmojiTechnologyResearch) + "coda"},
		{"bundles under technology", "bundles", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundles)},
		{"library bundle compose/js", "bundle", map[string]interface{}{"name": "compose/js", "kind": "library"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle compose/graphql", "bundle", map[string]interface{}{"name": "compose/graphql", "kind": "schema"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle repo/client", "bundle", map[string]interface{}{"name": "repo/client", "kind": "binary"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"},
		{"ui bundle repo/vscode", "bundle", map[string]interface{}{"name": "repo/vscode", "kind": "ui"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleUI) + "vscode"},
		{"example bundle coda/example", "bundle", map[string]interface{}{"name": "coda/example", "kind": "example"}, EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleExample) + "example"},
		{"assets bundle asset", "bundle", map[string]interface{}{"name": "asset", "kind": "assets"}, EmojiText(EmojiTechnologyUser) + "asset" + EmojiText(EmojiBundleAssets) + "asset"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, EmojiText(EmojiFolders)},
		{"bundle sketchpad folders", "folders", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolders)},
		{"required folder .github folders", "folders", map[string]interface{}{"parentId": EmojiText(EmojiFolderRequired) + "github"}, EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFolders)},
		{"org folder compose/js/sketchpad", "folder", map[string]interface{}{"path": "compose/js/sketchpad", "kind": "organization", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, EmojiText(EmojiFolderRequired) + "devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, EmojiText(EmojiFiles)},
		{"sketchpad files", "files", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFiles)},
		{"github files", "files", map[string]interface{}{"parentId": EmojiText(EmojiFolderRequired) + "github"}, EmojiText(EmojiFolderRequired) + "github" + EmojiText(EmojiFiles)},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "compose/js/sketchpad/Design.tsx", "kind": "code", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": EmojiText(EmojiFolderRequired) + "devcontainer"}, EmojiText(EmojiFolderRequired) + "devcontainer" + EmojiText(EmojiFileConfig) + "devcontainer"},
		{"line 3872", "line", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design", "line": float64(3872)}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiLine) + "3872"},
		{"range 3872-3875", "range", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore", "startLine": float64(3872), "endLine": float64(3875)}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore" + EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875"},
		{"sections in file", "sections", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSections)},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment"},
		{"nested section Store", "section", map[string]interface{}{"name": "Store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitions)},
		{"definition impl createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"},
		{"definition interface", "definition", map[string]interface{}{"name": "IStore", "kind": "interface", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionInterface) + "istore"},
		{"definition constant", "definition", map[string]interface{}{"name": "MAX_SIZE", "kind": "constant", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionConstant) + "maxsize"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, EmojiText(EmojiGoals)},
		{"nested goals under parent", "goals", map[string]interface{}{"parentId": EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiGoals)},
		{"top-level goal", "goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, EmojiText(EmojiGoal) + "r26021"},
		{"nested goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": EmojiText(EmojiGoal) + "r26021"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, EmojiText(EmojiTickets)},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTickets)},
		{"section tickets", "tickets", map[string]interface{}{"parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiTickets)},
		{"ticket Introduce Key Guid Uri Mechanism", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad"}, EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"draft New Architecture", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client"}, EmojiText(EmojiTechnologyInfra) + "repo" + EmojiText(EmojiBundleBinary) + "client" + EmojiText(EmojiDraft) + "newarchitecture"},
		{"todo Introduce Proper Sync Mechanism", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiDefinitionImpl) + "createsketchpadstore" + EmojiText(EmojiTodo) + "introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": EmojiText(EmojiFileCode)}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles"},
		{"specific policy Only One Store", "policy", map[string]interface{}{"id": "only-one-store", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "design" + EmojiText(EmojiSection) + "statemanagment" + EmojiText(EmojiSection) + "store" + EmojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles",
			"affected": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore",
			"lineId":   EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875",
			"secondId": EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
		}, EmojiText(EmojiFileCode) + EmojiText(EmojiPolicy) + "godfiles" + EmojiText(EmojiBreach) + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFileCode) + "designstore" + EmojiText(EmojiBreachScope) + EmojiText(EmojiLine) + "3872" + EmojiText(EmojiLine) + "3875" + EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12"},
		{"contributor usalu", "contributor", map[string]interface{}{"github": "usalu"}, EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "checkpoint", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": EmojiText(EmojiContributor) + "usalu"}, EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiCheckpoint) + "cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "started",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionStarted)},
		{"interaction edited", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "edited",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionEdited)},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "finished",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionFinished)},
		{"interaction restarted", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "restarted",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionRestarted)},
		{"interaction deleted", "interaction", map[string]interface{}{
			"secondId":      EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12",
			"contributorId": EmojiText(EmojiContributor) + "usalu",
			"entityId":      EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "deleted",
		}, EmojiText(EmojiYear) + "26" + EmojiText(EmojiMonth) + "02" + EmojiText(EmojiDay) + "14" + EmojiText(EmojiHour) + "19" + EmojiText(EmojiMinute) + "07" + EmojiText(EmojiSecond) + "12" + EmojiText(EmojiContributor) + "usalu" + EmojiText(EmojiGoal) + "r26021" + EmojiText(EmojiGoal) + "runningsketchpad" + EmojiText(EmojiTicket) + "introducekeyguidurimechanism" + EmojiText(EmojiInteractionDeleted)},
		{"file test kind", "file", map[string]interface{}{"path": "compose/js/sketchpad.test.ts", "kind": "lab", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFileLab) + "sketchpadtest"},
		{"file script kind", "file", map[string]interface{}{"path": "compose/engine/build.ts", "kind": "script", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "engine"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "engine" + EmojiText(EmojiFileScript) + "build"},
		{"file docs kind", "file", map[string]interface{}{"path": "compose/js/README.md", "kind": "docs", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFileDocs) + "readme"},
		{"file asset kind", "file", map[string]interface{}{"path": "compose/js/sketchpad/page/showcase/metabolism.mdx", "kind": "resource", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolderOrg) + "pages" + EmojiText(EmojiFolderOrg) + "showcases"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js" + EmojiText(EmojiFolderOrg) + "sketchpad" + EmojiText(EmojiFolderOrg) + "pages" + EmojiText(EmojiFolderOrg) + "showcases" + EmojiText(EmojiFileResource) + "metabolism"},
		{"file license kind", "file", map[string]interface{}{"path": "compose/go/LICENSE.md", "kind": "license", "parentId": EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "go"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "go" + EmojiText(EmojiFileLicense) + "license"},
		{"site bundle compose/docs", "bundle", map[string]interface{}{"name": "compose/docs", "kind": "site"}, EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleSite) + "docs"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID(tc.kind, tc.data)
			if id != tc.expected {
				t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
			}
		})
	}
}

func TestArtifactIDAndURI(t *testing.T) {
	tests := []struct {
		name    string
		kind    string
		data    map[string]interface{}
		wantID  string
		wantURI string
	}{
		{
			name:    "root",
			kind:    "root",
			data:    map[string]interface{}{},
			wantID:  "",
			wantURI: "repo://root",
		},
		{
			name:    "technologies collection",
			kind:    "technologies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiTechnologies),
			wantURI: "repo://technologies/" + EmojiText(EmojiTechnologies),
		},
		{
			name:    "technology user",
			kind:    "technology",
			data:    map[string]interface{}{"name": "compose", "kind": "user"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose",
			wantURI: "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose",
		},
		{
			name:    "technology infrastructure",
			kind:    "technology",
			data:    map[string]interface{}{"name": "repo", "kind": "infrastructure"},
			wantID:  EmojiText(EmojiTechnologyInfra) + "repo",
			wantURI: "repo://technology/" + EmojiText(EmojiTechnologyInfra) + "repo",
		},
		{
			name:    "technology research",
			kind:    "technology",
			data:    map[string]interface{}{"name": "coda", "kind": "research"},
			wantID:  EmojiText(EmojiTechnologyResearch) + "coda",
			wantURI: "repo://technology/" + EmojiText(EmojiTechnologyResearch) + "coda",
		},
		{
			name:    "bundles collection",
			kind:    "bundles",
			data:    map[string]interface{}{"technologyCode": "compose", "parentId": EmojiText(EmojiTechnologyUser) + "compose"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundles),
			wantURI: "repo://bundles/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundles),
		},
		{
			name:    "bundle library",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/js", "kind": "library"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js",
			wantURI: "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js",
		},
		{
			name:    "bundle example",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "coda/example", "kind": "library"},
			wantID:  EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleLibrary) + "example",
			wantURI: "repo://bundle/" + EmojiText(EmojiTechnologyResearch) + "coda" + EmojiText(EmojiBundleLibrary) + "example",
		},
		{
			name:    "bundle ui",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "compose/desktop", "kind": "ui"},
			wantID:  EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleUI) + "desktop",
			wantURI: "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleUI) + "desktop",
		},
		{
			name:    "folders collection empty",
			kind:    "folders",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiFolders),
			wantURI: "repo://folders/" + EmojiText(EmojiFolders),
		},
		{
			name:    "folders collection with parent",
			kind:    "folders",
			data:    map[string]interface{}{"parentPath": "compose/js/src", "parentId": EmojiText(EmojiFolderOrg) + "src"},
			wantID:  EmojiText(EmojiFolderOrg) + "src" + EmojiText(EmojiFolders),
			wantURI: "repo://folders/" + EmojiText(EmojiFolderOrg) + "src" + EmojiText(EmojiFolders),
		},
		{
			name:    "folder required",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/src", "kind": "required"},
			wantID:  EmojiText(EmojiFolderRequired) + "src",
			wantURI: "repo://folder/" + EmojiText(EmojiFolderRequired) + "src",
		},
		{
			name:    "folder organization",
			kind:    "folder",
			data:    map[string]interface{}{"path": "compose/js/utils", "kind": "organization"},
			wantID:  EmojiText(EmojiFolderOrg) + "utils",
			wantURI: "repo://folder/" + EmojiText(EmojiFolderOrg) + "utils",
		},
		{
			name:    "files collection empty",
			kind:    "files",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiFiles),
			wantURI: "repo://files/" + EmojiText(EmojiFiles),
		},
		{
			name:    "file docs",
			kind:    "file",
			data:    map[string]interface{}{"path": "test.txt", "kind": "docs"},
			wantID:  EmojiText(EmojiFileDocs) + "test",
			wantURI: "repo://file/" + EmojiText(EmojiFileDocs) + "test",
		},
		{
			name:    "file code",
			kind:    "file",
			data:    map[string]interface{}{"path": "main.go", "kind": "code"},
			wantID:  EmojiText(EmojiFileCode) + "main",
			wantURI: "repo://file/" + EmojiText(EmojiFileCode) + "main",
		},
		{
			name:    "file test",
			kind:    "file",
			data:    map[string]interface{}{"path": "compose/js/src/🧪️index.test.ts", "kind": "lab"},
			wantID:  EmojiText(EmojiFileLab) + workspace.Flat("🧪️index.test"),
			wantURI: "repo://file/" + EmojiText(EmojiFileLab) + workspace.Flat("🧪️index.test"),
		},
		{
			name:    "file config",
			kind:    "file",
			data:    map[string]interface{}{"path": "tsconfig.json", "kind": "config"},
			wantID:  EmojiText(EmojiFileConfig) + "tsconfig",
			wantURI: "repo://file/" + EmojiText(EmojiFileConfig) + "tsconfig",
		},
		{
			name:    "file script",
			kind:    "file",
			data:    map[string]interface{}{"path": "build.sh", "kind": "script"},
			wantID:  EmojiText(EmojiFileScript) + "build",
			wantURI: "repo://file/" + EmojiText(EmojiFileScript) + "build",
		},
		{
			name:    "file resource",
			kind:    "file",
			data:    map[string]interface{}{"path": "🖼️logo.png", "kind": "resource"},
			wantID:  EmojiText(EmojiFileResource) + workspace.Flat("🖼️logo"),
			wantURI: "repo://file/" + EmojiText(EmojiFileResource) + workspace.Flat("🖼️logo"),
		},
		{
			name:    "file license",
			kind:    "file",
			data:    map[string]interface{}{"path": "LICENSE.md", "kind": "license"},
			wantID:  EmojiText(EmojiFileLicense) + "license",
			wantURI: "repo://file/" + EmojiText(EmojiFileLicense) + "license",
		},
		{
			name:    "sections collection",
			kind:    "sections",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": EmojiText(EmojiFileCode) + "index"},
			wantID:  EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSections),
			wantURI: "repo://sections/" + EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSections),
		},
		{
			name:    "section",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/Design.tsx#State Management#Design Store"},
			wantID:  BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
			wantURI: "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
		},
		{
			name:    "section single level",
			kind:    "section",
			data:    map[string]interface{}{"path": "compose/js/src/file.ts#Imports"},
			wantID:  BuildSectionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
			wantURI: "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Imports"}),
		},
		{
			name:    "definitions collection",
			kind:    "definitions",
			data:    map[string]interface{}{"filePath": "compose/js/src/index.ts", "parentId": EmojiText(EmojiSection) + "types"},
			wantID:  EmojiText(EmojiSection) + "types" + EmojiText(EmojiDefinitions),
			wantURI: "repo://definitions/" + EmojiText(EmojiSection) + "types" + EmojiText(EmojiDefinitions),
		},
		{
			name:    "definition with id",
			kind:    "definition",
			data:    map[string]interface{}{"id": "compose/js/src/index.ts#MyClass", "kind": "implementation"},
			wantID:  BuildDefinitionID(BuildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
		},
		{
			name:    "definition interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "interface", "filePath": "compose/js/src/file.ts", "sectionPath": "Types", "name": "MyInterface"},
			wantID:  BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
		},
		{
			name:    "definition go type treated as interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "type", "filePath": "repo/client/main.go", "sectionPath": "GraphQL Types#GraphQL Input Types", "name": "TicketCloseInput"},
			wantID:  BuildDefinitionID(BuildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("repo/client/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
		},
		{
			name:    "definition constant",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "constant", "filePath": "compose/js/src/file.ts", "name": "MAX_SIZE"},
			wantID:  BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
			wantURI: "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
		},
		{
			name:    "tickets collection",
			kind:    "tickets",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiTickets),
			wantURI: "repo://tickets/" + EmojiText(EmojiTickets),
		},
		{
			name: "ticket",
			kind: "ticket",
			data: map[string]interface{}{
				"year":  float64(2025),
				"month": float64(2),
				"day":   float64(4),
				"slug":  "test-ticket",
			},
			wantID:  EmojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + EmojiText(EmojiTicket) + "testticket",
		},
		{
			name: "ticket with status",
			kind: "ticket",
			data: map[string]interface{}{
				"year":   float64(2025),
				"month":  float64(2),
				"day":    float64(4),
				"slug":   "test-ticket",
				"status": "open",
			},
			wantID:  EmojiText(EmojiTicket) + "testticket",
			wantURI: "repo://ticket/" + EmojiText(EmojiTicket) + "testticket",
		},
		{
			name:    "goals collection",
			kind:    "goals",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiGoals),
			wantURI: "repo://goals/" + EmojiText(EmojiGoals),
		},
		{
			name:    "goal",
			kind:    "goal",
			data:    map[string]interface{}{"id": "RUNNING-SKETCHPAD", "parentId": ""},
			wantID:  EmojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + EmojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "goal nested",
			kind:    "goal",
			data:    map[string]interface{}{"id": "R26-02/RUNNING-SKETCHPAD", "parentId": EmojiText(EmojiGoal) + "r2602"},
			wantID:  EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "repo://goal/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad",
		},
		{
			name:    "drafts collection",
			kind:    "drafts",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiDrafts),
			wantURI: "repo://drafts/" + EmojiText(EmojiDrafts),
		},
		{
			name:    "draft",
			kind:    "draft",
			data:    map[string]interface{}{"slug": "my-draft"},
			wantID:  EmojiText(EmojiDraft) + "mydraft",
			wantURI: "repo://draft/" + EmojiText(EmojiDraft) + "mydraft",
		},
		{
			name:    "todos collection",
			kind:    "todos",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiTodos),
			wantURI: "repo://todos/" + EmojiText(EmojiTodos),
		},
		{
			name:    "todo",
			kind:    "todo",
			data:    map[string]interface{}{"id": "my-todo"},
			wantID:  EmojiText(EmojiTodo) + "mytodo",
			wantURI: "repo://todo/" + EmojiText(EmojiTodo) + "mytodo",
		},
		{
			name:    "policies collection",
			kind:    "policies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiPolicies),
			wantURI: "repo://policies/" + EmojiText(EmojiPolicies),
		},
		{
			name:    "policy",
			kind:    "policy",
			data:    map[string]interface{}{"id": "/code-hygiene"},
			wantID:  EmojiText(EmojiPolicy) + "codehygiene",
			wantURI: "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene",
		},
		{
			name:    "statutes collection",
			kind:    "statutes",
			data:    map[string]interface{}{},
			wantID:  "",
			wantURI: "repo://statutes",
		},
		{
			name:    "statute",
			kind:    "statute",
			data:    map[string]interface{}{"id": "code/inline-comment"},
			wantID:  "codeinlinecomment",
			wantURI: "repo://statute/codeinlinecomment",
		},
		{
			name:    "contributors collection",
			kind:    "contributors",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiContributors),
			wantURI: "repo://contributors/" + EmojiText(EmojiContributors),
		},
		{
			name:    "contributor",
			kind:    "contributor",
			data:    map[string]interface{}{"github": "usalu"},
			wantID:  EmojiText(EmojiContributor) + "usalu",
			wantURI: "repo://contributor/" + EmojiText(EmojiContributor) + "usalu",
		},
		{
			name:    "checkpoints collection",
			kind:    "checkpoints",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  EmojiText(EmojiCheckpoints),
			wantURI: "repo://checkpoints/" + EmojiText(EmojiCheckpoints),
		},
		{
			name:    "checkpoint",
			kind:    "checkpoint",
			data:    map[string]interface{}{"sha": "abc123"},
			wantID:  EmojiText(EmojiCheckpoint) + "abc123",
			wantURI: "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123",
		},
		{
			name:    "interactions collection",
			kind:    "interactions",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  "",
			wantURI: "repo://interactions",
		},
		{
			name:    "interaction started ticket",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "started", "entityId": EmojiText(EmojiTicket) + "introduceinteractionmechanism"},
			wantID:  EmojiText(EmojiTicket) + "introduceinteractionmechanism" + EmojiText(EmojiInteractionStarted),
			wantURI: "repo://interaction/" + EmojiText(EmojiTicket) + "introduceinteractionmechanism" + EmojiText(EmojiInteractionStarted),
		},
		{
			name:    "interaction finished goal",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "finished", "entityId": EmojiText(EmojiGoal) + "r2602"},
			wantID:  EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished),
			wantURI: "repo://interaction/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotID := GetArtifactID(tt.kind, tt.data)
			if gotID != tt.wantID {
				t.Errorf("GetArtifactID() = %q, want %q", gotID, tt.wantID)
			}
			gotURI := GetArtifactURI(tt.kind, tt.data)
			if gotURI != tt.wantURI {
				t.Errorf("GetArtifactURI() = %q, want %q", gotURI, tt.wantURI)
			}
		})
	}
}
