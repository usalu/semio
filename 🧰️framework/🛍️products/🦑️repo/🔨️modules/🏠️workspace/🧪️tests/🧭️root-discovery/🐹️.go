// 🐹️ Go side of the root discovery case.
package adapter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	workspace "github.com/usalu/semio/repo/workspace"
	host "semio.tech/repo/test"
)

// region 🔖️Trees

type discoveryTree struct {
	Name         string            `json:"name"`
	Directories  []string          `json:"directories"`
	Files        map[string]string `json:"files"`
	StartAt      string            `json:"startAt"`
	ExpectedRoot string            `json:"expectedRoot"`
}

type configDocument struct {
	Name     string  `json:"name"`
	Document *string `json:"document"`
	Expected string  `json:"expected"`
}

type discoveryFile struct {
	Schema          string            `json:"schema"`
	Trees           []discoveryTree   `json:"trees"`
	Layout          map[string]string `json:"layout"`
	ConfigDocuments []configDocument  `json:"configDocuments"`
}

func loadDiscoveryFile(ctx *host.Context) (discoveryFile, error) {
	data, err := ctx.FixtureBytes("shared://📡️root-discovery-trees.json")
	if err != nil {
		return discoveryFile{}, err
	}
	var file discoveryFile
	err = json.Unmarshal(data, &file)
	return file, err
}

func materialize(base string, tree discoveryTree) error {
	for _, dir := range tree.Directories {
		if err := os.MkdirAll(filepath.Join(base, filepath.FromSlash(dir)), 0o755); err != nil {
			return err
		}
	}
	for path, content := range tree.Files {
		full := filepath.Join(base, filepath.FromSlash(path))
		if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
			return err
		}
		if err := os.WriteFile(full, []byte(content), 0o644); err != nil {
			return err
		}
	}
	return nil
}

func relativeSlash(base string, target string) string {
	resolvedBase, err := filepath.EvalSymlinks(base)
	if err != nil {
		resolvedBase = base
	}
	resolvedTarget, err := filepath.EvalSymlinks(target)
	if err != nil {
		resolvedTarget = target
	}
	relative, err := filepath.Rel(resolvedBase, resolvedTarget)
	if err != nil {
		return "outside"
	}
	slashed := filepath.ToSlash(relative)
	if slashed == ".." || strings.HasPrefix(slashed, "../") {
		return "outside"
	}
	return slashed
}

func renderConfig(config workspace.RepoConfig) string {
	return fmt.Sprintf("session=%t operations=%t plan=%t detail=%s", config.Logging.Session, config.Logging.Operations, config.Logging.Plan, config.Logging.Detail)
}

// endregion 🔖️Trees

// region 🔖️Scenarios

func theFirstMarkerUpwardsWins(ctx *host.Context) (host.Outcome, error) {
	file, err := loadDiscoveryFile(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	roots := make([]string, 0, len(file.Trees))
	for _, tree := range file.Trees {
		base := filepath.Join(ctx.WorkDir, "🌳️"+tree.Name)
		if err := materialize(base, tree); err != nil {
			return host.Outcome{}, err
		}
		found := workspace.FindRepoRoot(filepath.Join(base, filepath.FromSlash(tree.StartAt)))
		roots = append(roots, fmt.Sprintf("%s=%s", tree.Name, relativeSlash(base, found)))
	}
	return host.Outcome{Projection: map[string]any{"roots": roots}}, nil
}

func theLayoutVocabularyIsFixed(ctx *host.Context) (host.Outcome, error) {
	root := filepath.Join(ctx.WorkDir, "🏠️layout")
	if err := os.MkdirAll(root, 0o755); err != nil {
		return host.Outcome{}, err
	}
	layout := []string{
		"semio=" + relativeSlash(root, workspace.SemioDirForRoot(root)),
		"repoMeta=" + relativeSlash(root, workspace.RepoMetaDirForRoot(root)),
		"tickets=" + relativeSlash(root, workspace.TicketsDirForRoot(root)),
		"goals=" + relativeSlash(root, workspace.GoalsDirForRoot(root)),
		"devs=" + relativeSlash(root, workspace.DevsDirForRoot(root)),
		"filesIndex=" + relativeSlash(root, workspace.FilesIndexForRoot(root)),
		"config=" + relativeSlash(root, workspace.RepoMetaPathForRoot(root, workspace.ConfigFileName)),
	}
	return host.Outcome{Projection: map[string]any{"layout": layout}}, nil
}

func settingsFallBackToTheDefaults(ctx *host.Context) (host.Outcome, error) {
	file, err := loadDiscoveryFile(ctx)
	if err != nil {
		return host.Outcome{}, err
	}
	configs := make([]string, 0, len(file.ConfigDocuments))
	for _, document := range file.ConfigDocuments {
		root := filepath.Join(ctx.WorkDir, "📋️"+document.Name)
		metaDir := workspace.RepoMetaDirForRoot(root)
		if err := os.MkdirAll(metaDir, 0o755); err != nil {
			return host.Outcome{}, err
		}
		if document.Document != nil {
			if err := os.WriteFile(filepath.Join(metaDir, workspace.ConfigFileName), []byte(*document.Document), 0o644); err != nil {
				return host.Outcome{}, err
			}
		}
		configs = append(configs, fmt.Sprintf("%s=%s", document.Name, renderConfig(workspace.LoadRepoConfig(root))))
	}
	if strings.TrimSpace(renderConfig(workspace.DefaultRepoConfig())) == "" {
		return host.Outcome{}, fmt.Errorf("the default settings must render")
	}
	return host.Outcome{Projection: map[string]any{"configs": configs}}, nil
}

// endregion 🔖️Scenarios

// region 🔖️Registration

// Adapter is the registration entry point the generated host calls.
func Adapter() *host.Adapter {
	return host.NewAdapter("go").
		Subject("the-first-marker-upwards-wins", theFirstMarkerUpwardsWins).
		Subject("the-layout-vocabulary-is-fixed", theLayoutVocabularyIsFixed).
		Subject("settings-fall-back-to-the-defaults", settingsFallBackToTheDefaults)
}

// endregion 🔖️Registration
