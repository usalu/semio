package goals

import (
	strconv "strconv"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🪪️Id Scheme

// 🌱️IsRootGoal reports whether an identifier has no ancestor, which is the goal that owns a
// management milestone.
func IsRootGoal(goalID string) bool { return GoalDepth(goalID) == 0 }

// 🔹️IsFirstGenGoal reports whether an identifier is a direct child of a root goal.
func IsFirstGenGoal(goalID string) bool { return GoalDepth(goalID) == 1 }

// 🔸️IsDeeperGoal reports whether an identifier sits at depth two or deeper.
func IsDeeperGoal(goalID string) bool { return GoalDepth(goalID) >= 2 }

// 🎯️GoalPathToComposeID returns the compose identifier of a goal path, one tagged segment per
// level. The flattening keeps every character above the ASCII range, so an identifier that already
// carries goal segments composes into a nested one rather than losing them.
func GoalPathToComposeID(goalPath string) string {
	if goalPath == "" {
		return ""
	}
	prefix := identity.EmojiText(identity.Entity("goal"))
	composed := ""
	for _, part := range strings.Split(goalPath, "/") {
		composed += prefix + workspace.Flat(part)
	}
	return composed
}

// 🛤️ComposeIDToGoalPath returns the goal path of a compose identifier, empty when it carries none.
func ComposeIDToGoalPath(composeID string) string {
	return strings.Join(identity.ComposeIDToGoalSegments(composeID), "/")
}

// 🔺️RootGoalID returns the outermost ancestor of an identifier, in filesystem path form.
func RootGoalID(goalID string) string {
	goalID = GoalIDForFilesystem(goalID)
	if index := strings.Index(goalID, "/"); index != -1 {
		return goalID[:index]
	}
	return goalID
}

// 🔻️ParentGoalID returns the immediate ancestor of an identifier, empty for a root goal.
func ParentGoalID(goalID string) string {
	goalID = GoalIDForFilesystem(goalID)
	if index := strings.LastIndex(goalID, "/"); index != -1 {
		return goalID[:index]
	}
	return ""
}

// 🧬️ComposeGoalID returns the identifier a title takes under a parent.
func ComposeGoalID(parent, title string) string {
	slug := identity.Slugify(title)
	if parent == "" {
		return slug
	}
	return parent + "/" + slug
}

// 🔢️ParseMilestoneNumber returns the trailing number of a milestone reference.
func ParseMilestoneNumber(milestone string) (int64, error) { return parseTrailingNumber(milestone) }

// 🔢️ParseIssueNumber returns the trailing number of an issue URL.
func ParseIssueNumber(issueURL string) (int64, error) { return parseTrailingNumber(issueURL) }

// 🔢️parseTrailingNumber accepts a bare number or the last segment of a slash separated reference.
func parseTrailingNumber(value string) (int64, error) {
	if number, err := strconv.ParseInt(value, 10, 64); err == nil {
		return number, nil
	}
	parts := strings.Split(value, "/")
	if len(parts) > 0 {
		if number, err := strconv.ParseInt(parts[len(parts)-1], 10, 64); err == nil {
			return number, nil
		}
	}
	return 0, errUnparsable(value)
}

// #endregion 🪪️Id Scheme
