// 🔬️ Tests of the statutes domain, split out of the pre-split godfile suite.

package statutes

import (
	testing "testing"

	model "github.com/usalu/semio/repo/model"
)

// #region 🔬️Policy Registry Tests

func TestPolicyStatutesHaveMetadata(t *testing.T) {
	for _, policy := range GetPolicies() {
		for _, statute := range policy.AllKinds() {
			info, ok := model.StatuteInfoTable[statute]
			if !ok {
				t.Fatalf("policy %q statute %q is missing metadata", policy.ID, statute)
			}
			if info.Kind != statute {
				t.Fatalf("policy %q statute %q metadata kind = %q", policy.ID, statute, info.Kind)
			}
		}
	}
}

func TestPolicyDefAllKinds(t *testing.T) {
	t.Run("groups collect all nested kinds", func(t *testing.T) {
		p := PolicyDef{
			ID:          "test",
			Name:        "Test",
			Description: "Test policy",
			Scopes:      []string{"**/*"},
			Groups: []model.Territory{
				{
					Name:   "File",
					Scopes: []string{"**/*.ts"},
					Kinds:  []model.Statute{model.BreachCodeFileMissingHeaderRegion, model.BreachCodeFileMissingSummary},
				},
				{
					Name:   "Section",
					Scopes: []string{"**/*.ts"},
					Kinds:  []model.Statute{model.BreachCodeSectionEmpty},
				},
			},
			Run: func(ctx *PolicyContext) []model.Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 3 {
			t.Fatalf("expected 3 kinds, got %d", len(kinds))
		}
	})
	t.Run("empty groups returns empty", func(t *testing.T) {
		p := PolicyDef{
			ID:     "empty",
			Name:   "Empty",
			Scopes: []string{"**/*"},
			Run:    func(ctx *PolicyContext) []model.Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
}

func TestRegisteredPoliciesHaveGroups(t *testing.T) {
	policies := GetRegisteredPolicies()
	for _, p := range policies {
		if len(p.Groups) == 0 {
			t.Errorf("policy %s has no groups", p.ID)
		}
		kinds := p.AllKinds()
		if len(kinds) == 0 {
			t.Errorf("policy %s has no statutes", p.ID)
		}
	}
}

func TestFolderPolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("folder")
	if !found {
		t.Fatal("folder policy not registered")
	}
	if policy.Name != "Folder" {
		t.Errorf("expected policy name Folder, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == model.BreachFolderIllegalEmpty {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("folder policy should contain BreachFolderIllegalEmpty kind")
	}
}

func TestFilePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("file")
	if !found {
		t.Fatal("file policy not registered")
	}
	if policy.Name != "File" {
		t.Errorf("expected policy name File, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == model.BreachFileIllegalUseGodfile {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("file policy should contain BreachFileIllegalUseGodfile kind")
	}
}

// #endregion 🔬️Policy Registry Tests
