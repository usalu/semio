// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Goal storage and GitHub goal issue helpers.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// 📨️GetRepoGoalsDir MUST retrieve the requested value or return an error.
// 📖️GetRepoGoalsDir retrieves and returns the repo goals dir.
func GetRepoGoalsDir() string {
	return filepath.Join(GetRepoMetaDir(), "🎯️goals")
}

// ⛳️ListGoals MUST return all available goals entries.
// 📋️ListGoals returns a list of goals entries.
func ListGoals() ([]*Goal, error) {
	dir := GetRepoGoalsDir()
	var goals []*Goal
	if !FileExists(dir) {
		return goals, nil
	}
	err := filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && d.Name() == "🎯️goal.json" {
			content, err := ReadTextFile(path)
			if err != nil {
				return err
			}
			var goal Goal
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
func ReadGoal(id string) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(id), "🎯️goal.json")
	if !FileExists(path) {
		return nil, fmt.Errorf("goal file not found: %s", path)
	}
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
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
func StreamGoals(ctx context.Context, out chan<- *Goal, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
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
			if !matchesFilter(g.ID, options) && !matchesFilter(g.Title, options) {
				continue
			}
			queryMatches, queryErr := matchesQuery(ctx, g.ID+" "+g.Title+" "+g.Description+" "+g.Status, options)
			if queryErr != nil {
				return queryErr
			}
			if !queryMatches {
				continue
			}
			out <- g
		}
	}
	return nil
}

// 📍️StreamStatutes MUST invoke the callback for each matching statutes entry.
// 📜️StreamStatutes streams statutes entries through the callback.
func StreamStatutes(ctx context.Context, out chan<- StatuteMeta, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	for _, meta := range statuteInfoTable {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(string(meta.Kind), options) && !matchesFilter(meta.Reason, options) {
				continue
			}
			queryMatches, queryErr := matchesQuery(ctx, string(meta.Kind)+" "+meta.Reason+" "+meta.Solution, options)
			if queryErr != nil {
				return queryErr
			}
			if !queryMatches {
				continue
			}
			out <- meta
		}
	}
	return nil
}

// 💾️StreamCheckpoints MUST invoke the callback for each matching checkpoints entry.
// ✔️StreamCheckpoints streams checkpoints entries through the callback.
func StreamCheckpoints(ctx context.Context, limit *int, out chan<- Checkpoint, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	checkpoints := LoadCheckpoints(limit)
	for _, c := range checkpoints {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(c.SHA, options) && !matchesFilter(c.Title, options) {
				continue
			}
			queryMatches, queryErr := matchesQuery(ctx, c.SHA+" "+c.Title, options)
			if queryErr != nil {
				return queryErr
			}
			if !queryMatches {
				continue
			}
			out <- c
		}
	}
	return nil
}

// 🏪️SaveGoal MUST persist the goal atomically to the data store.
// 💾️SaveGoal persists goal to the data store.
func SaveGoal(goal Goal) error {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(goal.ID), "🎯️goal.json")
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return err
	}
	data, err := json.MarshalIndent(goal, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(path, string(data))
}

// 💿️ghCreateMilestone holds the data fields for a ghCreateMilestone record.
func ghCreateMilestone(title, description string) (int, error) {
	args := []string{"api", "repos/:owner/:repo/milestones", "-f", fmt.Sprintf("title=%s", title), "-f", fmt.Sprintf("description=%s", description), "--jq", ".number"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return 0, fmt.Errorf("gh api milestone create failed: %s", stderr)
	}
	num, _ := strconv.Atoi(strings.TrimSpace(stdout))
	return num, nil
}

// 🔁️ghUpdateMilestone holds the data fields for a ghUpdateMilestone record.
func ghUpdateMilestone(number int, title, description, state, dueOn string) error {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number), "-X", "PATCH"}
	if title != "" {
		args = append(args, "-f", fmt.Sprintf("title=%s", title))
	}
	if description != "" {
		args = append(args, "-f", fmt.Sprintf("description=%s", description))
	}
	if state != "" {
		args = append(args, "-f", fmt.Sprintf("state=%s", state))
	}
	if dueOn != "" {

		if !strings.Contains(dueOn, "T") {
			dueOn = dueOn + "T00:00:00Z"
		}
		args = append(args, "-f", fmt.Sprintf("due_on=%s", dueOn))
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh api milestone update failed: %s", stderr)
	}
	return nil
}

// 🗑️ghDeleteMilestone holds the data fields for a ghDeleteMilestone record.
func ghDeleteMilestone(number int) error {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number), "-X", "DELETE"}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh api milestone delete failed: %s", stderr)
	}
	return nil
}

// 🆕️ghCreateGoalIssue holds the data fields for a ghCreateGoalIssue record.
func ghCreateGoalIssue(title, description string, milestone *int) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", description, "--label", "goal"}
	if milestone != nil {
		milestoneTitle, err := ghGetMilestoneTitle(*milestone)
		if err != nil {
			return "", fmt.Errorf("could not resolve milestone %d: %w", *milestone, err)
		}
		args = append(args, "--milestone", milestoneTitle)
	}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", strings.TrimSpace(stderr))
	}
	issueURL := strings.TrimSpace(stdout)
	if issueURL != "" {
		ghAddIssueToProject(issueURL)
	}
	return issueURL, nil
}

// 🔷️ghUpdateGoalIssue holds the data fields for a ghUpdateGoalIssue record.
func ghUpdateGoalIssue(issueURL, title, description string) error {
	args := []string{"issue", "edit", issueURL}
	if title != "" {
		args = append(args, "--title", title)
	}
	if description != "" {
		args = append(args, "--body", description)
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔶️goalDepth holds the data fields for a goalDepth record.
func goalDepth(goalID string) int {
	return strings.Count(goalID, "/")
}

// 🌱️isRootGoal holds the data fields for a isRootGoal record.
func isRootGoal(goal *Goal) bool {
	return goalDepth(goal.ID) == 0
}

// 🔹️isFirstGenGoal holds the data fields for a isFirstGenGoal record.
func isFirstGenGoal(goal *Goal) bool {
	return goalDepth(goal.ID) == 1
}

// 🔸️isDeeperGoal holds the data fields for a isDeeperGoal record.
func isDeeperGoal(goal *Goal) bool {
	return goalDepth(goal.ID) >= 2
}

// 🛤️goalIDForFilesystem maps repo emoji goal IDs to `.🧬semio/🦑️repo/🎯️goals/...` paths for filesystem access.
func goalIDForFilesystem(goalID string) string {
	goalID = strings.TrimSpace(goalID)
	if goalID == "" {
		return ""
	}
	if strings.Contains(goalID, "/") {
		return goalID
	}
	if strings.Contains(goalID, emojiText(EmojiGoal)) {
		if path := composeIDToGoalPath(goalID); path != "" && path != goalID {
			return path
		}
	}
	return goalID
}

// 🔺️getRootGoalID holds the data fields for a getRootGoalID record.
func getRootGoalID(goalID string) string {
	goalID = goalIDForFilesystem(goalID)
	if idx := strings.Index(goalID, "/"); idx != -1 {
		return goalID[:idx]
	}
	return goalID
}

func getParentGoalID(goalID string) string {
	goalID = goalIDForFilesystem(goalID)
	if idx := strings.LastIndex(goalID, "/"); idx != -1 {
		return goalID[:idx]
	}
	return ""
}

// 🔻️getRootGoalMilestone holds the data fields for a getRootGoalMilestone record.
func getRootGoalMilestone(goalID string) (*int, error) {
	rootID := getRootGoalID(goalID)
	rootGoal, err := ReadGoal(rootID)
	if err != nil {
		return nil, err
	}
	if rootGoal.Management == nil || rootGoal.Management.Milestone == "" {
		return nil, nil
	}
	n, err := parseMilestoneNumber(rootGoal.Management.Milestone)
	if err != nil {
		return nil, err
	}
	return &n, nil
}

// 🌿️getParentGoalIssueNodeID holds the data fields for a getParentGoalIssueNodeID record.
func getParentGoalIssueNodeID(goalID string) (string, error) {
	parentID := getParentGoalID(goalID)
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
	return ghGetIssueNodeID(parentGoal.Management.Issue)
}

func ghGetIssueNodeID(issueURL string) (string, error) {
	args := []string{"issue", "view", issueURL, "--json", "id", "--jq", ".id"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue view failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 🌐️ghGetIssueParentURL holds the data fields for a ghGetIssueParentURL record.
func ghGetIssueParentURL(issueURL string) (string, error) {
	query := `query($url: URI!) {
		resource(url: $url) {
			... on Issue {
				parent {
					url
				}
			}
		}
	}`
	args := []string{"api", "graphql",
		"-f", fmt.Sprintf("url=%s", issueURL),
		"-f", fmt.Sprintf("query=%s", query),
	}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh api issue parent query failed: %s", strings.TrimSpace(stderr))
	}
	var response struct {
		Data struct {
			Resource *struct {
				Parent *struct {
					URL string `json:"url"`
				} `json:"parent"`
			} `json:"resource"`
		} `json:"data"`
	}
	if err := json.Unmarshal([]byte(stdout), &response); err != nil {
		return "", fmt.Errorf("failed to parse issue parent response: %w", err)
	}
	if response.Data.Resource == nil || response.Data.Resource.Parent == nil {
		return "", nil
	}
	return response.Data.Resource.Parent.URL, nil
}

// ➕️ghAddSubIssue holds the data fields for a ghAddSubIssue record.
func ghAddSubIssue(parentIssueURL, childIssueURL string) error {
	parentNodeID, err := ghGetIssueNodeID(parentIssueURL)
	if err != nil {
		return fmt.Errorf("failed to get parent node ID: %w", err)
	}
	childNodeID, err := ghGetIssueNodeID(childIssueURL)
	if err != nil {
		return fmt.Errorf("failed to get child node ID: %w", err)
	}
	query := `mutation($parentId: ID!, $childId: ID!) {
		addSubIssue(input: { issueId: $parentId, subIssueId: $childId }) {
			subIssue { url }
		}
	}`
	args := []string{"api", "graphql",
		"-H", "GraphQL-Features:sub_issues",
		"-f", fmt.Sprintf("parentId=%s", parentNodeID),
		"-f", fmt.Sprintf("childId=%s", childNodeID),
		"-f", fmt.Sprintf("query=%s", query),
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh api addSubIssue failed: %s", strings.TrimSpace(stderr))
	}
	return nil
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

// 📦️ghDeleteIssue holds the data fields for a ghDeleteIssue record.
func ghDeleteIssue(issueURLOrNumber string) error {
	args := []string{"issue", "delete", issueURLOrNumber, "--yes"}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue delete failed: %s", stderr)
	}
	return nil
}
