package launcher

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"unicode/utf16"
)

// 📦️WorkspaceLinksPolicy bounds the dependency-free publication of authored local packages.
type WorkspaceLinksPolicy struct {
	Version              int    `json:"version"`
	Directory            string `json:"directory"`
	MaximumPathLength    int    `json:"maximumPathLength"`
	MaximumWorkspaces    int    `json:"maximumWorkspaces"`
	MaximumManifestBytes int64  `json:"maximumManifestBytes"`
	PackageNamePattern   string `json:"packageNamePattern"`
	ReservedNamePattern  string `json:"reservedNamePattern"`
	Managed              struct {
		SourceSchema     string `json:"sourceSchema"`
		ManagerDirectory string `json:"managerDirectory"`
		BudgetPolicy     string `json:"budgetPolicy"`
		NativeSchema     string `json:"nativeSchema"`
	} `json:"managed"`
	name, reserved *regexp.Regexp
	metadataRoot   string
}

// 🔗️WorkspacePackageLink names a local package and its bounded source and resolver destination.
type WorkspacePackageLink struct{ Name, Source, Destination string }

// 📖️ReadWorkspaceLinksPolicy reads the neutral contract before importing JavaScript workspace packages.
func ReadWorkspaceLinksPolicy(root string) (WorkspaceLinksPolicy, error) {
	var schema struct {
		Defs struct {
			Protocol struct {
				Const WorkspaceLinksPolicy `json:"const"`
			} `json:"Protocol"`
		} `json:"$defs"`
	}
	if err := readWorkspaceManifest(filepath.Join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️workspaces/🧬️schema/🔣️.json"), 1<<20, &schema); err != nil {
		return WorkspaceLinksPolicy{}, err
	}
	policy := schema.Defs.Protocol.Const
	policy.metadataRoot = root
	if policy.Version != 1 || policy.Directory != "node_modules" || policy.MaximumPathLength != 256 || policy.MaximumWorkspaces < 1 || policy.MaximumWorkspaces > 4096 || policy.MaximumManifestBytes < 1 || policy.MaximumManifestBytes > 1<<20 {
		return policy, fmt.Errorf("bounded workspace link protocol required")
	}
	var err error
	if policy.name, err = regexp.Compile(policy.PackageNamePattern); err != nil || policy.PackageNamePattern == "" {
		return policy, fmt.Errorf("portable workspace package-name pattern required")
	}
	if policy.reserved, err = regexp.Compile(policy.ReservedNamePattern); err != nil || policy.ReservedNamePattern == "" {
		return policy, fmt.Errorf("portable workspace reserved-name pattern required")
	}
	return policy, nil
}

func readWorkspaceManifest(file string, maximum int64, into any) error {
	if len(utf16.Encode([]rune(file))) > 256 {
		return fmt.Errorf("workspace metadata exceeds the host path contract: %s", file)
	}
	before, err := os.Lstat(file)
	if err != nil {
		return err
	}
	if !before.Mode().IsRegular() {
		return fmt.Errorf("physical workspace manifest required: %s", file)
	}
	reader, err := os.Open(file)
	if err != nil {
		return err
	}
	defer reader.Close()
	facts, err := reader.Stat()
	if err != nil {
		return err
	}
	if !facts.Mode().IsRegular() || !os.SameFile(before, facts) || facts.Size() > maximum {
		return fmt.Errorf("bounded workspace manifest required: %s", file)
	}
	raw, err := io.ReadAll(io.LimitReader(reader, maximum+1))
	if err != nil {
		return err
	}
	if int64(len(raw)) > maximum {
		return fmt.Errorf("workspace manifest exceeds its byte budget: %s", file)
	}
	after, err := os.Lstat(file)
	if err != nil {
		return err
	}
	if !after.Mode().IsRegular() || !os.SameFile(facts, after) || facts.Size() != after.Size() || !facts.ModTime().Equal(after.ModTime()) {
		return fmt.Errorf("workspace manifest changed while being read: %s", file)
	}
	return json.Unmarshal(raw, into)
}

// 🧭️PlanWorkspacePackageLink rejects host-dependent aliases and paths before filesystem mutation.
func PlanWorkspacePackageLink(root, workspace, name string, policy WorkspaceLinksPolicy) (WorkspacePackageLink, error) {
	plan := WorkspacePackageLink{Name: name}
	if policy.name == nil || policy.reserved == nil || !policy.name.MatchString(name) || policy.reserved.MatchString(name) {
		return plan, fmt.Errorf("portable workspace package name required: %s", name)
	}
	if workspace == "" || strings.ContainsAny(workspace, "\\:*?<>\"|") || strings.HasPrefix(workspace, "/") || policy.reserved.MatchString(workspace) {
		return plan, fmt.Errorf("relative literal workspace path required: %s", workspace)
	}
	for _, value := range workspace {
		if value < 32 {
			return plan, fmt.Errorf("workspace control characters are forbidden")
		}
	}
	for _, segment := range strings.Split(workspace, "/") {
		if segment == "" || segment == "." || segment == ".." || strings.TrimRight(segment, " .") != segment {
			return plan, fmt.Errorf("portable workspace path component required: %s", workspace)
		}
	}
	absolute, err := filepath.Abs(root)
	if err != nil {
		return plan, err
	}
	plan.Source = filepath.Join(absolute, filepath.FromSlash(workspace))
	plan.Destination = filepath.Join(absolute, policy.Directory, filepath.FromSlash(name))
	for _, path := range []string{filepath.Join(plan.Source, "package.json"), plan.Destination, filepath.Join(filepath.Dir(plan.Destination), ".repo-link-000000000000")} {
		if len(utf16.Encode([]rune(path))) > policy.MaximumPathLength {
			return plan, fmt.Errorf("workspace link exceeds the host path contract: %s", path)
		}
	}
	return plan, nil
}

func workspaceDirectoryWithin(root, directory string) error {
	current := directory
	for {
		facts, err := os.Stat(current)
		if err == nil {
			if !facts.IsDir() {
				return fmt.Errorf("workspace directory required: %s", current)
			}
			real, err := filepath.EvalSymlinks(current)
			if err != nil {
				return err
			}
			relative, err := filepath.Rel(canonicalRoot(root), canonicalRoot(real))
			if err != nil || relative == ".." || strings.HasPrefix(relative, ".."+string(os.PathSeparator)) {
				return fmt.Errorf("workspace directory escapes its root: %s", directory)
			}
			return nil
		}
		if !os.IsNotExist(err) {
			return err
		}
		parent := filepath.Dir(current)
		if parent == current {
			return err
		}
		current = parent
	}
}

func workspaceDirectoryLink(path string, facts os.FileInfo) bool {
	if facts.Mode()&os.ModeSymlink != 0 {
		return true
	}
	if facts.Mode()&os.ModeIrregular != 0 {
		_, err := os.Readlink(path)
		return err == nil
	}
	return false
}

func workspaceLinkMatches(plan WorkspacePackageLink) (bool, error) {
	facts, err := os.Lstat(plan.Destination)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	if !workspaceDirectoryLink(plan.Destination, facts) {
		return false, fmt.Errorf("workspace package destination is occupied: %s", plan.Destination)
	}
	actual, err := os.Stat(plan.Destination)
	if err != nil {
		return false, fmt.Errorf("workspace package destination is dangling: %s", plan.Destination)
	}
	expected, err := os.Stat(plan.Source)
	if err != nil {
		return false, err
	}
	if !os.SameFile(actual, expected) {
		return false, fmt.Errorf("workspace package destination names another source: %s", plan.Destination)
	}
	return true, nil
}

func publishWorkspacePackageLink(ctx context.Context, root string, plan WorkspacePackageLink) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	matched, err := workspaceLinkMatches(plan)
	if err != nil || matched {
		return err
	}
	parent := filepath.Dir(plan.Destination)
	if err := workspaceDirectoryWithin(root, parent); err != nil {
		return err
	}
	if err := os.MkdirAll(parent, 0o755); err != nil {
		return err
	}
	if err := workspaceDirectoryWithin(root, parent); err != nil {
		return err
	}
	parentBefore, err := os.Stat(parent)
	if err != nil {
		return err
	}
	var nonce [6]byte
	if _, err := rand.Read(nonce[:]); err != nil {
		return err
	}
	stage := filepath.Join(parent, ".repo-link-"+hex.EncodeToString(nonce[:]))
	if err := createWorkspaceDirectoryLink(plan.Source, stage); err != nil {
		return err
	}
	defer os.Remove(stage)
	if err := ctx.Err(); err != nil {
		return err
	}
	parentAfter, err := os.Stat(parent)
	if err != nil {
		return err
	}
	if !os.SameFile(parentBefore, parentAfter) {
		return fmt.Errorf("workspace package parent changed before publication: %s", parent)
	}
	if err := workspaceDirectoryWithin(root, parent); err != nil {
		return err
	}
	if err := workspaceDirectoryWithin(root, plan.Source); err != nil {
		return err
	}
	if err := publishWorkspaceDirectoryLink(stage, plan.Destination); err != nil {
		matched, collision := workspaceLinkMatches(plan)
		if matched {
			return nil
		}
		if collision != nil {
			return collision
		}
		return err
	}
	return nil
}

// 🌱️PrepareWorkspacePackageLinks validates the whole finite roster, then atomically publishes local resolver links.
func PrepareWorkspacePackageLinks(ctx context.Context, root string, policy WorkspaceLinksPolicy) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	namespace, err := ClassifyWorkspaceNamespace(root, policy)
	if err != nil {
		return err
	}
	if namespace.Kind == "managed" {
		return ctx.Err()
	}
	var manifest struct {
		Workspaces *[]string `json:"workspaces"`
	}
	if err := readWorkspaceManifest(filepath.Join(root, "package.json"), policy.MaximumManifestBytes, &manifest); err != nil {
		return err
	}
	if manifest.Workspaces == nil {
		return fmt.Errorf("declared root workspace array required")
	}
	workspaces := *manifest.Workspaces
	if len(workspaces) > policy.MaximumWorkspaces {
		return fmt.Errorf("workspace roster exceeds its entry budget")
	}
	plans := make([]WorkspacePackageLink, 0, len(workspaces))
	paths, names := map[string]bool{}, map[string]bool{}
	for _, workspace := range workspaces {
		if err := ctx.Err(); err != nil {
			return err
		}
		plan, err := PlanWorkspacePackageLink(root, workspace, "workspace", policy)
		if err != nil {
			return err
		}
		key := strings.ToLower(workspace)
		if paths[key] {
			return fmt.Errorf("duplicate workspace path: %s", workspace)
		}
		paths[key] = true
		if err := workspaceDirectoryWithin(root, plan.Source); err != nil {
			return err
		}
		var item struct {
			Name string `json:"name"`
		}
		if err := readWorkspaceManifest(filepath.Join(plan.Source, "package.json"), policy.MaximumManifestBytes, &item); err != nil {
			return err
		}
		plan, err = PlanWorkspacePackageLink(root, workspace, item.Name, policy)
		if err != nil {
			return err
		}
		key = strings.ToLower(item.Name)
		if names[key] {
			return fmt.Errorf("duplicate portable workspace package name: %s", item.Name)
		}
		names[key] = true
		if err := workspaceDirectoryWithin(root, filepath.Dir(plan.Destination)); err != nil {
			return err
		}
		if _, err := workspaceLinkMatches(plan); err != nil {
			return err
		}
		plans = append(plans, plan)
	}
	for _, plan := range plans {
		if err := publishWorkspacePackageLink(ctx, root, plan); err != nil {
			return err
		}
	}
	return nil
}

func prepareWorkspacePackageLinks(ctx context.Context, root string) error {
	policy, err := ReadWorkspaceLinksPolicy(root)
	if err != nil {
		return err
	}
	return PrepareWorkspacePackageLinks(ctx, root, policy)
}
