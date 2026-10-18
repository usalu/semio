package goals

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region ❄️Goals

// ⛳️ListGoals MUST return all available goals entries.
// 📋️ListGoals returns a list of goals entries.
func ListGoals() ([]*model.Goal, error) {
	dir := workspace.GetRepoGoalsDir()
	var goals []*model.Goal
	if !workspace.FileExists(dir) {
		return goals, nil
	}
	err := filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && d.Name() == "🎯️goal.json" {
			content, err := workspace.ReadTextFile(path)
			if err != nil {
				return err
			}
			var goal model.Goal
			if err := json.Unmarshal([]byte(content), &goal); err != nil {
				return err
			}
			relPath, _ := filepath.Rel(dir, path)
			idPath := filepath.Dir(relPath)
			goal.ID = filepath.ToSlash(idPath)

			if idx := strings.LastIndex(goal.ID, "/"); idx != -1 {
				goal.Parent = goal.ID[:idx]
			} else {
				goal.Parent = ""
			}
			goal.Path = path
			goals = append(goals, &goal)
		}
		return nil
	})
	return goals, err
}

// ❌️ReadGoal MUST return the goal content or an error if unavailable.
// ⬛️ReadGoal reads and returns goal from the source.
func ReadGoal(id string) (*model.Goal, error) {
	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(id), "🎯️goal.json")
	if !workspace.FileExists(path) {
		return nil, fmt.Errorf("goal file not found: %s", path)
	}
	content, err := workspace.ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = id
	goal.Path = path

	if idx := strings.LastIndex(id, "/"); idx != -1 {
		goal.Parent = id[:idx]
	} else {
		goal.Parent = ""
	}
	return &goal, nil
}

// 🎯️StreamGoals MUST invoke the callback for each matching goals entry.
// ⚡️StreamGoals streams goals entries through the callback.
func StreamGoals(ctx context.Context, out chan<- *model.Goal, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	goals, err := ListGoals()
	if err != nil {
		return err
	}

	for _, g := range goals {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !model.MatchesFilter(g.ID, options) && !model.MatchesFilter(g.Title, options) {
				continue
			}
			if !model.MatchesQuery(g.ID+" "+g.Title+" "+g.Description+" "+g.Status, options) {
				continue
			}
			out <- g
		}
	}
	return nil
}

// 📍️StreamStatutes MUST invoke the callback for each matching statutes entry.
// 📜️StreamStatutes streams statutes entries through the callback.
func StreamStatutes(ctx context.Context, out chan<- model.StatuteMeta, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	for _, meta := range model.StatuteInfoTable {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !model.MatchesFilter(string(meta.Kind), options) && !model.MatchesFilter(meta.Reason, options) {
				continue
			}
			if !model.MatchesQuery(string(meta.Kind)+" "+meta.Reason+" "+meta.Solution, options) {
				continue
			}
			out <- meta
		}
	}
	return nil
}

// 🏪️SaveGoal MUST persist the goal atomically to the data store.
// 💾️SaveGoal persists goal to the data store.
func SaveGoal(goal model.Goal) error {
	dir := workspace.GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(goal.ID), "🎯️goal.json")
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return err
	}
	data, err := json.MarshalIndent(goal, "", "  ")
	if err != nil {
		return err
	}
	return workspace.WriteTextFile(path, string(data))
}

// 🔶️goalDepth holds the data fields for a goalDepth record.
func GoalDepth(goalID string) int {
	return strings.Count(goalID, "/")
}

// 🛤️goalIDForFilesystem maps repo emoji goal IDs to `.🦑️repo/🎯️goals/...` paths for filesystem access.
func GoalIDForFilesystem(goalID string) string {
	goalID = strings.TrimSpace(goalID)
	if goalID == "" {
		return ""
	}
	if strings.Contains(goalID, "/") {
		return goalID
	}
	if strings.Contains(goalID, identity.EmojiText(identity.Entity("goal"))) {
		if path := ComposeIDToGoalPath(goalID); path != "" && path != goalID {
			return path
		}
	}
	return goalID
}

// 🔺️getRootGoalID holds the data fields for a getRootGoalID record.
func GetRootGoalID(goalID string) string {
	goalID = GoalIDForFilesystem(goalID)
	if idx := strings.Index(goalID, "/"); idx != -1 {
		return goalID[:idx]
	}
	return goalID
}

func GetParentGoalID(goalID string) string {
	goalID = GoalIDForFilesystem(goalID)
	if idx := strings.LastIndex(goalID, "/"); idx != -1 {
		return goalID[:idx]
	}
	return ""
}

// 🔻️getRootGoalMilestone holds the data fields for a getRootGoalMilestone record.
func GetRootGoalMilestone(goalID string) (*int, error) {
	rootID := GetRootGoalID(goalID)
	rootGoal, err := ReadGoal(rootID)
	if err != nil {
		return nil, err
	}
	if rootGoal.Management == nil || rootGoal.Management.Milestone == "" {
		return nil, nil
	}
	n, err := model.ParseMilestoneNumber(rootGoal.Management.Milestone)
	if err != nil {
		return nil, err
	}
	return &n, nil
}

// 🌿️getParentGoalIssueNodeID holds the data fields for a getParentGoalIssueNodeID record.
func getParentGoalIssueNodeID(goalID string) (string, error) {
	parentID := GetParentGoalID(goalID)
	if parentID == "" {
		return "", nil
	}
	parentGoal, err := ReadGoal(parentID)
	if err != nil {
		return "", err
	}
	if parentGoal.Management == nil || parentGoal.Management.Issue == "" {
		return "", nil
	}
	return providers.GhGetIssueNodeID(parentGoal.Management.Issue)
}

// 🔢️parseIssueNumber holds the data fields for a parseIssueNumber record.
func parseIssueNumber(issueURL string) (int, error) {
	parts := strings.Split(issueURL, "/")
	if len(parts) > 0 {
		if n, err := strconv.Atoi(parts[len(parts)-1]); err == nil {
			return n, nil
		}
	}
	return 0, fmt.Errorf("could not parse issue number from %s", issueURL)
}

// #endregion ❄️Goals
