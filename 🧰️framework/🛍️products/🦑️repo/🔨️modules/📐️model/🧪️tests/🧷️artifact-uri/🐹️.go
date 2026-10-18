// 🧷️ Tests of the identifier and uri conversions.

package model

import (
	testing "testing"
)

func TestIdToUri(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"technology user", EmojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", EmojiText(EmojiTechnologyInfra) + "repo", "repo://technology/" + EmojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"folder required", EmojiText(EmojiFolderRequired) + "src", "repo://folder/" + EmojiText(EmojiFolderRequired) + "src"},
		{"folder org", EmojiText(EmojiFolderOrg) + "utils", "repo://folder/" + EmojiText(EmojiFolderOrg) + "utils"},
		{"file docs", EmojiText(EmojiFileDocs) + "test", "repo://file/" + EmojiText(EmojiFileDocs) + "test"},
		{"file code", EmojiText(EmojiFileCode) + "main", "repo://file/" + EmojiText(EmojiFileCode) + "main"},
		{"section", EmojiText(EmojiSection), "repo://section/" + EmojiText(EmojiSection)},
		{"section nested", BuildSectionID(BuildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"}), "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/design.tsx", nil), []string{"state managment", "store"})},
		{"definition impl", BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation), "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation)},
		{"ticket", EmojiText(EmojiTicket) + "testticket", "repo://ticket/" + EmojiText(EmojiTicket) + "testticket"},
		{"goal", EmojiText(EmojiGoal) + "r2602runningsketchpad", "repo://goal/" + EmojiText(EmojiGoal) + "r2602runningsketchpad"},
		{"goal nested", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad", "repo://goal/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", EmojiText(EmojiDraft) + "mydraft", "repo://draft/" + EmojiText(EmojiDraft) + "mydraft"},
		{"policy", EmojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", EmojiText(EmojiContributor) + "usalu", "repo://contributor/" + EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", EmojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123"},
		{"interaction started ticket", EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted), "repo://interaction/" + EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted)},
		{"interaction finished goal", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished), "repo://interaction/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionFinished)},
		{"session", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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
		{"technologies", "repo://technologies/" + EmojiText(EmojiTechnologies), EmojiText(EmojiTechnologies)},
		{"technology", "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose", EmojiText(EmojiTechnologyUser) + "compose"},
		{"technology infra", "repo://technology/" + EmojiText(EmojiTechnologyInfra) + "repo", EmojiText(EmojiTechnologyInfra) + "repo"},
		{"bundle", "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
		{"folder", "repo://folder/" + EmojiText(EmojiFolderOrg) + "src", EmojiText(EmojiFolderOrg) + "src"},
		{"file", "repo://file/" + EmojiText(EmojiFileCode) + "test", EmojiText(EmojiFileCode) + "test"},
		{"section", "repo://section/" + BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}), BuildSectionID(BuildFileID("compose/js/src/Design.tsx", nil), []string{"State Management", "Design Store"})},
		{"definition", "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation), BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation)},
		{"definition with section", "repo://definition/" + BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation), BuildDefinitionID(BuildFileID("compose/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation)},
		{"ticket", "repo://ticket/" + EmojiText(EmojiTicket) + "testticket", EmojiText(EmojiTicket) + "testticket"},
		{"goal", "repo://goal/" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "runningsketchpad"},
		{"goal nested", "repo://goal/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiGoal) + "runningsketchpad"},
		{"draft", "repo://draft/" + EmojiText(EmojiDraft) + "mydraft", EmojiText(EmojiDraft) + "mydraft"},
		{"policy", "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene", EmojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", "repo://contributor/" + EmojiText(EmojiContributor) + "usalu", EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123", EmojiText(EmojiCheckpoint) + "abc123"},
		{"interaction", "repo://interaction/" + EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted), EmojiText(EmojiTicket) + "testticket" + EmojiText(EmojiInteractionStarted)},
		{"session", "repo://session/" + EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15", EmojiText(EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"},
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

func TestIdUriRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		id   string
		uri  string
	}{
		{"policy", EmojiText(EmojiPolicy) + "codehygiene", "repo://policy/" + EmojiText(EmojiPolicy) + "codehygiene"},
		{"contributor", EmojiText(EmojiContributor) + "usalu", "repo://contributor/" + EmojiText(EmojiContributor) + "usalu"},
		{"checkpoint", EmojiText(EmojiCheckpoint) + "abc123", "repo://checkpoint/" + EmojiText(EmojiCheckpoint) + "abc123"},
		{"draft", EmojiText(EmojiDraft) + "mydraft", "repo://draft/" + EmojiText(EmojiDraft) + "mydraft"},
		{"section", EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSection) + "imports", "repo://section/" + EmojiText(EmojiFileCode) + "index" + EmojiText(EmojiSection) + "imports"},
		{"file", EmojiText(EmojiFileCode) + "index", "repo://file/" + EmojiText(EmojiFileCode) + "index"},
		{"ticket", EmojiText(EmojiTicket) + "20260115someticket", "repo://ticket/" + EmojiText(EmojiTicket) + "20260115someticket"},
		{"goal", EmojiText(EmojiGoal) + "r2602running", "repo://goal/" + EmojiText(EmojiGoal) + "r2602running"},
		{"interaction goal", EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionStarted), "repo://interaction/" + EmojiText(EmojiGoal) + "r2602" + EmojiText(EmojiInteractionStarted)},
		{"technology", EmojiText(EmojiTechnologyUser) + "compose", "repo://technology/" + EmojiText(EmojiTechnologyUser) + "compose"},
		{"bundle", EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js", "repo://bundle/" + EmojiText(EmojiTechnologyUser) + "compose" + EmojiText(EmojiBundleLibrary) + "js"},
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
