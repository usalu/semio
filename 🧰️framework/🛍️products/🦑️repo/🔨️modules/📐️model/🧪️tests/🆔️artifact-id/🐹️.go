// 🆔️ Tests of the artifact identifiers of the temporal, code and goal trees.

package model

import (
	"encoding/json"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
	"os"
	"path/filepath"
	"strings"
	"sync"
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

type artifactLayoutFixture struct {
	Directories, Files                                              []string
	ProbePath, ExpectedFolderID, RemoveFile, ExpectedAfterRemovalID string
	MaximumWarmAllocations                                          float64
	MaximumDirectoryObservations                                    int
	Boundaries                                                      []ArtifactBoundary
	ExpectedBoundaryID                                              string
	BoundaryCases                                                   []struct {
		Name, Coordinate string
		Boundaries       []ArtifactBoundary
		ExpectedFolderID string
	}
}

func artifactLayoutCase(t *testing.T) (string, artifactLayoutFixture) {
	t.Helper()
	data, err := os.ReadFile(sandbox.OriginPath("..", "..", "🧪️tests", "🧫️fixtures", "🆔️artifact-id", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture artifactLayoutFixture
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	root, err := filepath.EvalSymlinks(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}
	for _, directory := range fixture.Directories {
		if err := os.MkdirAll(filepath.Join(root, filepath.FromSlash(directory)), 0755); err != nil {
			t.Fatal(err)
		}
	}
	for _, file := range fixture.Files {
		if err := os.WriteFile(filepath.Join(root, filepath.FromSlash(file)), []byte("{}"), 0600); err != nil {
			t.Fatal(err)
		}
	}
	return root, fixture
}
func TestArtifactLayoutReusesResolvedAncestorObservations(t *testing.T) {
	root, fixture := artifactLayoutCase(t)
	index := NewArtifactIndex(root, nil)
	if actual := index.BuildFolderID(fixture.ProbePath, nil); actual != fixture.ExpectedFolderID {
		t.Fatalf("folder ID=%q want=%q", actual, fixture.ExpectedFolderID)
	}
	allocations := testing.AllocsPerRun(5, func() { index.BuildFolderID(fixture.ProbePath, nil) })
	if allocations > fixture.MaximumWarmAllocations {
		t.Fatalf("warm identifier repeated ancestor work: allocations=%g maximum=%g", allocations, fixture.MaximumWarmAllocations)
	}
}

func TestArtifactLayoutObservesEachDirectoryOnceAndRefreshesBetweenInvocations(t *testing.T) {
	root, fixture := artifactLayoutCase(t)
	layout := NewArtifactLayout(root, nil)
	original := layout.readDirectory
	counts := map[string]int{}
	layout.readDirectory = func(path string) ([]os.DirEntry, error) { counts[path]++; return original(path) }
	for repeat := 0; repeat < 4; repeat++ {
		if actual := layout.BuildFolderID(fixture.ProbePath, nil); actual != fixture.ExpectedFolderID {
			t.Fatalf("folder=%q", actual)
		}
		layout.BuildFileID(fixture.ProbePath+"/main.go", nil)
		for _, directory := range fixture.Directories {
			layout.FolderKind(directory)
		}
	}
	for path, count := range counts {
		if count > fixture.MaximumDirectoryObservations {
			t.Fatalf("repeated directory observation %s: %d", path, count)
		}
	}
	if err := os.Remove(filepath.Join(root, fixture.RemoveFile)); err != nil {
		t.Fatal(err)
	}
	if actual := layout.BuildFolderID(fixture.ProbePath, nil); actual != fixture.ExpectedFolderID {
		t.Fatal("invocation snapshot changed after observation")
	}
	if actual := NewArtifactLayout(root, nil).BuildFolderID(fixture.ProbePath, nil); actual != fixture.ExpectedAfterRemovalID {
		t.Fatalf("fresh invocation retained stale directory kind: %q", actual)
	}
	bounded := NewArtifactLayout(root, fixture.Boundaries)
	if actual := bounded.BuildFolderID(fixture.ProbePath, nil); actual != fixture.ExpectedBoundaryID {
		t.Fatalf("nearest boundary=%q", actual)
	}
}
func TestArtifactLayoutSharedStreamsRetainOneObservation(t *testing.T) {
	root, fixture := artifactLayoutCase(t)
	layout := NewArtifactLayout(root, nil)
	original := layout.readDirectory
	counts := map[string]int{}
	layout.readDirectory = func(path string) ([]os.DirEntry, error) { counts[path]++; return original(path) }
	var workers sync.WaitGroup
	workers.Add(8)
	for worker := 0; worker < 8; worker++ {
		go func() {
			defer workers.Done()
			for repeat := 0; repeat < 10; repeat++ {
				layout.BuildFileID(fixture.ProbePath+"/main.go", nil)
				layout.BuildFolderID(fixture.ProbePath, nil)
				layout.FolderKind("alpha")
			}
		}()
	}
	workers.Wait()
	for path, count := range counts {
		if count > fixture.MaximumDirectoryObservations {
			t.Fatalf("concurrent repeated observation %s: %d", path, count)
		}
	}
}
func TestArtifactLayoutIgnoresNonregularIndicatorEntries(t *testing.T) {
	root, _ := artifactLayoutCase(t)
	directory := filepath.Join(root, "ordinary")
	if err := os.MkdirAll(filepath.Join(directory, "package.json"), 0755); err != nil {
		t.Fatal(err)
	}
	if actual := NewArtifactLayout(root, nil).FolderKind("ordinary"); actual != FolderKindOrganization {
		t.Fatal("directory was accepted as a manifest file")
	}
	target := filepath.Join(root, "source.json")
	if err := os.WriteFile(target, []byte("{}"), 0600); err != nil {
		t.Fatal(err)
	}
	if err := os.Symlink(target, filepath.Join(directory, "go.mod")); err != nil {
		t.Skipf("symlink capability unavailable: %v", err)
	}
	if actual := NewArtifactLayout(root, nil).FolderKind("ordinary"); actual != FolderKindOrganization {
		t.Fatal("symlink was accepted as a manifest file")
	}
}

func TestArtifactLayoutRootAndBoundaryCoordinates(t *testing.T) {
	root, fixture := artifactLayoutCase(t)
	for _, row := range fixture.BoundaryCases {
		t.Run(row.Name, func(t *testing.T) {
			boundaries := append([]ArtifactBoundary(nil), row.Boundaries...)
			path := fixture.ProbePath
			if row.Coordinate == "absolute" {
				path = filepath.Join(root, filepath.FromSlash(path))
				for index := range boundaries {
					boundaries[index].Root = filepath.Join(root, filepath.FromSlash(boundaries[index].Root))
				}
			}
			layout := NewArtifactLayout(root, boundaries)
			if actual := layout.BuildFolderID(path, nil); actual != row.ExpectedFolderID {
				t.Fatalf("folder=%q want=%q", actual, row.ExpectedFolderID)
			}
			if actual := layout.BuildFileID(filepath.Join(path, "main.go"), nil); !strings.HasPrefix(actual, row.ExpectedFolderID) {
				t.Fatalf("file parent does not retain boundary: %q", actual)
			}
		})
	}
	bundles := []Bundle{{Name: "root/base", Root: ".", Kind: BundleKindLibrary}, {Name: "root/nested", Root: "alpha/beta", Kind: BundleKindLibrary}}
	index := NewArtifactIndex(root, bundles)
	for _, path := range []string{"alpha/beta/file.go", filepath.Join(root, "alpha", "beta", "file.go")} {
		if actual := index.GetBundleByPath(path); actual == nil || actual.Root != "alpha/beta" {
			t.Fatalf("nearest bundle for %q: %+v", path, actual)
		}
	}
	if actual := index.GetBundleByPath("other/file.go"); actual == nil || actual.Root != "." {
		t.Fatalf("root fallback: %+v", actual)
	}
}

func TestArtifactLayoutURIReusesResolvedIdentity(t *testing.T) {
	root, fixture := artifactLayoutCase(t)
	layout := NewArtifactLayout(root, nil)
	folder := layout.BuildFolderID(fixture.ProbePath, nil)
	file := layout.BuildFileID(fixture.ProbePath+"/main.go", nil)
	if actual := ArtifactURIForID("folder", folder); actual != "repo://folder/"+fixture.ExpectedFolderID {
		t.Fatalf("folder URI=%q", actual)
	}
	if actual := ArtifactURIForID("file", file); actual != "repo://file/"+fixture.ExpectedFolderID+EmojiText(EmojiFileCode)+"main" {
		t.Fatalf("file URI=%q", actual)
	}
}
