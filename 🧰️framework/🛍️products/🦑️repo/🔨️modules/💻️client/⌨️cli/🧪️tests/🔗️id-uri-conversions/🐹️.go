// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// ID and URI conversions for paths, sections, definitions, and statutes.

// #endregion 🧲️Header

package client

import (
	"testing"
)

func TestIdToUri(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"technology user", emojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", emojiText(EmojiTechnologyInfra) + "repo", "repo://technology/" + emojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"folder required", emojiText(EmojiFolderRequired) + "src", "repo://folder/" + emojiText(EmojiFolderRequired) + "src"},
		{"folder org", emojiText(EmojiFolderOrg) + "utils", "repo://folder/" + emojiText(EmojiFolderOrg) + "utils"},
		{"file docs", emojiText(EmojiFileDocs) + "test", "repo://file/" + emojiText(EmojiFileDocs) + "test"},
		{"file code", emojiText(EmojiFileCode) + "main", "repo://file/" + emojiText(EmojiFileCode) + "main"},
		{"section", emojiText(EmojiSection), "repo://section/" + emojiText(EmojiSection)},
		{"section nested", buildSectionID(buildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"}), "repo://section/" + buildSectionID(buildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"})},
		{"definition impl", buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation), "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation)},
		{"ticket", emojiText(EmojiTicket) + "testticket", "repo://ticket/" + emojiText(EmojiTicket) + "testticket"},
		{"goal", emojiText(EmojiGoal) + "r2602runningsketchpad", "repo://goal/" + emojiText(EmojiGoal) + "r2602runningsketchpad"},
		{"goal nested", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", "repo://goal/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", emojiText(EmojiDraft) + "mydraft", "repo://draft/" + emojiText(EmojiDraft) + "mydraft"},
		{"policy", emojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", emojiText(EmojiContributor) + "usalu", "repo://contributor/" + emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", emojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123"},
		{"interaction started ticket", emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted), "repo://interaction/" + emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},
		{"interaction finished goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished), "repo://interaction/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished)},
		{"session", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"empty string", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := IdToUri(tt.id)
			if got != tt.want {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestUriToId(t *testing.T) {
	tests := []struct {
		name string
		uri  string
		want string
	}{
		{"root", "repo://root", ""},
		{"technologies", "repo://technologies/" + emojiText(EmojiTechnologies), emojiText(EmojiTechnologies)},
		{"technology", "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose", emojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", "repo://technology/" + emojiText(EmojiTechnologyInfra) + "repo", emojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
		{"folder", "repo://folder/" + emojiText(EmojiFolderOrg) + "src", emojiText(EmojiFolderOrg) + "src"},
		{"file", "repo://file/" + emojiText(EmojiFileCode) + "test", emojiText(EmojiFileCode) + "test"},
		{"section", "repo://section/" + buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}), buildSectionID(buildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"})},
		{"definition", "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation), buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation)},
		{"definition with section", "repo://definition/" + buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation), buildDefinitionID(buildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation)},
		{"ticket", "repo://ticket/" + emojiText(EmojiTicket) + "testticket", emojiText(EmojiTicket) + "testticket"},
		{"goal", "repo://goal/" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "runningsketchpad"},
		{"goal nested", "repo://goal/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", "repo://draft/" + emojiText(EmojiDraft) + "mydraft", emojiText(EmojiDraft) + "mydraft"},
		{"policy", "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene", emojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", "repo://contributor/" + emojiText(EmojiContributor) + "usalu", emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123", emojiText(EmojiCheckpoint) + "abc123"},
		{"interaction", "repo://interaction/" + emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted), emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},
		{"session", "repo://session/" + emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", emojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
		{"kind only no id", "repo://technologies", ""},
		{"invalid", "https://example.com", ""},
		{"empty", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := UriToId(tt.uri)
			if got != tt.want {
				t.Errorf("UriToId(%q) = %q, want %q", tt.uri, got, tt.want)
			}
		})
	}
}

func TestPathToUriPath(t *testing.T) {
	tests := []struct {
		path string
		want string
	}{
		{"compose/js/src", "compose/js/src"},
		{"repo/client/main.go", "repo/client/main.go"},
		{"test.txt", "test.txt"},
		{"a b/c d", "a%20b/c%20d"},
	}
	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			if got := PathToUriPath(tt.path); got != tt.want {
				t.Errorf("PathToUriPath(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestPathFromUriPath(t *testing.T) {
	tests := []struct {
		uriPath string
		want    string
	}{
		{"compose/js/src", "compose/js/src"},
		{"repo/client/main.go", "repo/client/main.go"},
		{"a%20b/c%20d", "a b/c d"},
	}
	for _, tt := range tests {
		t.Run(tt.uriPath, func(t *testing.T) {
			if got := PathFromUriPath(tt.uriPath); got != tt.want {
				t.Errorf("PathFromUriPath(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestSectionIdValueToUriPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"no hash", "compose/js/src/file.ts", "compose/js/src/file.ts"},
		{"single section", "compose/js/src/file.ts#Imports", "compose/js/src/file.ts/Imports"},
		{"nested sections", "compose/js/src/Design.tsx#State Management#Design Store", "compose/js/src/Design.tsx/State%20Management/Design%20Store"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := SectionIdValueToUriPath(tt.value)
			if got != tt.want {
				t.Errorf("SectionIdValueToUriPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestDefinitionIdValueToUriPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"no hash", "compose/js/src/file.ts", "compose/js/src/file.ts"},
		{"with section and def", "compose/js/src/file.ts#Section§myFunc", "compose/js/src/file.ts/Section/myFunc"},
		{"def only", "compose/js/src/file.ts§myFunc", "compose/js/src/file.ts/myFunc"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := DefinitionIdValueToUriPath(tt.value)
			if got != tt.want {
				t.Errorf("DefinitionIdValueToUriPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestParseSectionUriPath(t *testing.T) {
	tests := []struct {
		name      string
		uriPath   string
		wantFile  string
		wantSlugs []string
	}{
		{"file only", "compose/js/src/file.ts", "compose/js/src/file.ts", nil},
		{"file with sections", "compose/js/src/Design.tsx/State%20Management/Design%20Store", "compose/js/src/Design.tsx", []string{"State%20Management", "Design%20Store"}},
		{"file with one section", "compose/js/src/file.ts/Imports", "compose/js/src/file.ts", []string{"Imports"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotFile, gotSlugs := ParseSectionUriPath(tt.uriPath)
			if gotFile != tt.wantFile {
				t.Errorf("ParseSectionUriPath(%q) filePath = %q, want %q", tt.uriPath, gotFile, tt.wantFile)
			}
			if len(gotSlugs) != len(tt.wantSlugs) {
				t.Errorf("ParseSectionUriPath(%q) slugs len = %d, want %d", tt.uriPath, len(gotSlugs), len(tt.wantSlugs))
			} else {
				for i, s := range gotSlugs {
					if s != tt.wantSlugs[i] {
						t.Errorf("ParseSectionUriPath(%q) slug[%d] = %q, want %q", tt.uriPath, i, s, tt.wantSlugs[i])
					}
				}
			}
		})
	}
}

func TestStatuteIdToUriPath(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"single segment", "code", "code"},
		{"two segments", "code/inline-comment", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdToUriPath(tt.id)
			if got != tt.want {
				t.Errorf("StatuteIdToUriPath(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestStatuteUriPathToId(t *testing.T) {
	tests := []struct {
		name    string
		uriPath string
		want    string
	}{
		{"single segment", "code", "code"},
		{"two segments", "code/inline-comment", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteUriPathToId(tt.uriPath)
			if got != tt.want {
				t.Errorf("StatuteUriPathToId(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestTitleizeSlug(t *testing.T) {
	tests := []struct {
		name string
		slug string
		want string
	}{
		{"single word", "code", "Code"},
		{"two words", "inline-comment", "Inline Comment"},
		{"three words", "missing-region-marker", "Missing Region Marker"},
		{"already titleized", "Code", "Code"},
		{"uppercase input", "CODE", "Code"},
		{"empty", "", ""},
		{"single char", "a", "A"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := TitleizeSlug(tt.slug)
			if got != tt.want {
				t.Errorf("TitleizeSlug(%q) = %q, want %q", tt.slug, got, tt.want)
			}
		})
	}
}

func TestStatutePathToIdValue(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"single segment", "code", "Code"},
		{"two segments", "code/inline-comment", "Code#Inline Comment"},
		{"three segments", "code/file/missing-header-region", "Code#File#Missing Header Region"},
		{"four segments", "code/header/region/nested", "Code#Header#Region#Nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatutePathToIdValue(tt.path)
			if got != tt.want {
				t.Errorf("StatutePathToIdValue(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestStatuteIdValueToPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"single segment", "Code", "code"},
		{"two segments", "Code#Inline Comment", "code/inline-comment"},
		{"three segments", "Code#File#Missing Header Region", "code/file/missing-header-region"},
		{"four segments", "Code#Header#Region#Nested", "code/header/region/nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdValueToPath(tt.value)
			if got != tt.want {
				t.Errorf("StatuteIdValueToPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestStatutePathIdValueRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		path string
	}{
		{"single segment", "code"},
		{"two segments", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			idValue := StatutePathToIdValue(tt.path)
			gotPath := StatuteIdValueToPath(idValue)
			if gotPath != tt.path {
				t.Errorf("round trip failed: path %q -> idValue %q -> path %q", tt.path, idValue, gotPath)
			}
		})
	}
}

func TestIdUriRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		id   string
		uri  string
	}{
		{"policy", emojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + emojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", emojiText(EmojiContributor) + "usalu", "repo://contributor/" + emojiText(EmojiContributor) + "usalu"},
		{"checkpoint", emojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + emojiText(EmojiCheckpoint) + "abc123"},
		{"draft", emojiText(EmojiDraft) + "mydraft", "repo://draft/" + emojiText(EmojiDraft) + "mydraft"},
		{"section", emojiText(EmojiFileCode) + "index" + emojiText(EmojiSection) + "imports", "repo://section/" + emojiText(EmojiFileCode) + "index" + emojiText(EmojiSection) + "imports"},
		{"file", emojiText(EmojiFileCode) + "index", "repo://file/" + emojiText(EmojiFileCode) + "index"},
		{"ticket", emojiText(EmojiTicket) + "20260115someticket", "repo://ticket/" + emojiText(EmojiTicket) + "20260115someticket"},
		{"goal", emojiText(EmojiGoal) + "r2602running", "repo://goal/" + emojiText(EmojiGoal) + "r2602running"},
		{"interaction goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted), "repo://interaction/" + emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted)},
		{"technology", emojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + emojiText(EmojiTechnologyUser) + "compose"},
		{"bundle", emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + emojiText(EmojiTechnologyUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},
	}
	for _, tt := range tests {
		t.Run(tt.name+"_IdToUri", func(t *testing.T) {
			gotUri := IdToUri(tt.id)
			if gotUri != tt.uri {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, gotUri, tt.uri)
			}
		})
		t.Run(tt.name+"_UriToId_then_IdToUri", func(t *testing.T) {
			gotId := UriToId(tt.uri)
			gotUri := IdToUri(gotId)
			if gotUri != tt.uri {
				t.Errorf("IdToUri(UriToId(%q)) = %q, want %q (intermediate id: %q)", tt.uri, gotUri, tt.uri, gotId)
			}
		})
	}
}
