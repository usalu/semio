// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Tree node, filter, and flexible time parsing models.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"strings"
	"time"
)

// ⏰️parseFlexibleTime holds the data fields for a parseFlexibleTime record.
func parseFlexibleTime(t string) (time.Time, error) {
	if t == "" {
		return time.Time{}, fmt.Errorf("empty time string")
	}
	layouts := []string{
		time.RFC3339,
		"2006-01-02",
		"2006-01-02 15:04:05",
	}
	for _, layout := range layouts {
		if parsed, err := time.Parse(layout, t); err == nil {
			return parsed, nil
		}
	}
	return time.Time{}, fmt.Errorf("could not parse time: %s", t)
}

// 🎫️TicketNode holds the data fields for a ticket node record.
type TicketNode struct {
	ID, Slug, Status string
	Title, URI       string
	GoalID, ParentID string
	Children         []*TicketNode
	Created          string
	Finished         string
	Description      string
	Summary          string
}

// 💿️GoalNode holds the data fields for a goal node record.
type GoalNode struct {
	ID, Title, Status  string
	DueDate, CreatedAt string
	Description        string
	Children           []*GoalNode
	Tickets            []*TicketNode
}

// 🏷️EntityKinds holds the data fields for a EntityKinds record.
var EntityKinds = []string{
	"root", "year", "month", "day", "hour", "minute", "second",
	"technology", "bundle", "folder", "file", "line", "range",
	"section", "definition", "goal", "ticket", "draft", "todo",
	"policy", "breach", "contributor", "checkpoint", "interaction", "session",
}

// 🎁️ArtifactKinds holds the data fields for a ArtifactKinds record.
var ArtifactKinds = []string{
	"repo", "technology", "bundle", "folder", "file", "section", "definition",
}

// 💿️DiffableKinds holds the data fields for a DiffableKinds record.
var DiffableKinds = []string{
	"root", "year", "month", "day", "hour",
	"technology", "bundle", "folder", "file", "section", "definition",
	"goal", "ticket", "contributor", "checkpoint", "interaction", "session",
}

// 📄️RelatedToFileKinds holds the data fields for a RelatedToFileKinds record.
var RelatedToFileKinds = []string{
	"root", "year", "month", "day", "hour", "minute", "second",
	"technology", "bundle", "folder", "goal", "ticket", "draft", "todo",
	"policy", "breach", "contributor", "checkpoint", "interaction", "session",
}

// 🌳️TreeNodeKind represents a tree node kind value.
type TreeNodeKind string

const (
	TreeNodeTechnology  TreeNodeKind = "technology"
	TreeNodeBundle      TreeNodeKind = "bundle"
	TreeNodeFolder      TreeNodeKind = "folder"
	TreeNodeFile        TreeNodeKind = "file"
	TreeNodeSection     TreeNodeKind = "section"
	TreeNodeDefinition  TreeNodeKind = "definition"
	TreeNodeGoal        TreeNodeKind = "goal"
	TreeNodeTicket      TreeNodeKind = "ticket"
	TreeNodeDraft       TreeNodeKind = "draft"
	TreeNodeTodo        TreeNodeKind = "todo"
	TreeNodePolicy      TreeNodeKind = "policy"
	TreeNodeBreach      TreeNodeKind = "breach"
	TreeNodeContributor TreeNodeKind = "contributor"
	TreeNodeCheckpoint  TreeNodeKind = "checkpoint"
	TreeNodeSession     TreeNodeKind = "session"
	TreeNodeStatute     TreeNodeKind = "statute"
	TreeNodeCategory    TreeNodeKind = "category"
)

// 🌿️TreeNode holds the data fields for a tree node record.
type TreeNode struct {
	Kind        TreeNodeKind
	ID          string
	Label       string
	URI         string
	SubKind     string
	Description string
	Summary     string
	Year        int
	Month       int
	Day         int
	Status      string
	Contributor string
	Data        map[string]interface{}
	Children    []*TreeNode
	matched     bool
}

// 🧹️TreeFilter holds the data fields for a tree filter record.
type TreeFilter struct {
	Query               string
	OnlyKinds           map[TreeNodeKind]bool
	ExcludeKinds        map[TreeNodeKind]bool
	OnlySubKinds        map[TreeNodeKind][]string
	ExcludeSubKinds     map[TreeNodeKind][]string
	OnlyYears           []int
	ExcludeYears        []int
	OnlyMonths          []int
	ExcludeMonths       []int
	OnlyDays            []int
	ExcludeDays         []int
	OnlyStatus          string
	OnlyContributors    []string
	ExcludeContributors []string
	OnlyPolicies        []string
	ExcludePolicies     []string
}

// 🏷️HasOnlyKinds MUST return true only when the property is present.
// 🔍️HasOnlyKinds reports whether the TreeFilter has only kinds.
func (f *TreeFilter) HasOnlyKinds() bool {
	return len(f.OnlyKinds) > 0
}

// 🔷️IsKindVisible MUST return true only when the condition is met.
// ❓️IsKindVisible reports whether the TreeFilter is kind visible.
func (f *TreeFilter) IsKindVisible(kind TreeNodeKind) bool {
	if kind == TreeNodeCategory {
		return true
	}
	if f.HasOnlyKinds() {
		return f.OnlyKinds[kind]
	}
	return !f.ExcludeKinds[kind]
}

// 📥️MatchesSubKind MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesSubKind(kind TreeNodeKind, subKind string) bool {
	if subKind == "" {
		return true
	}
	if only, ok := f.OnlySubKinds[kind]; ok && len(only) > 0 {
		for _, s := range only {
			if strings.EqualFold(s, subKind) {
				return true
			}
		}
		return false
	}
	if exclude, ok := f.ExcludeSubKinds[kind]; ok {
		for _, s := range exclude {
			if strings.EqualFold(s, subKind) {
				return false
			}
		}
	}
	return true
}

// 🎯️MatchesDate MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesDate(year, month, day int) bool {
	if len(f.OnlyYears) > 0 {
		found := false
		for _, y := range f.OnlyYears {
			if y == year {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	for _, y := range f.ExcludeYears {
		if y == year {
			return false
		}
	}
	if len(f.OnlyMonths) > 0 && month > 0 {
		found := false
		for _, m := range f.OnlyMonths {
			if m == month {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	for _, m := range f.ExcludeMonths {
		if m == month {
			return false
		}
	}
	if len(f.OnlyDays) > 0 && day > 0 {
		found := false
		for _, d := range f.OnlyDays {
			if d == day {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	for _, d := range f.ExcludeDays {
		if d == day {
			return false
		}
	}
	return true
}

// 🔶️MatchesStatus MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesStatus(status string) bool {
	if f.OnlyStatus == "" {
		return true
	}
	return strings.EqualFold(f.OnlyStatus, status)
}

// 🤝️MatchesContributor MUST operate on the TreeFilter receiver and return consistent results.
func (f *TreeFilter) MatchesContributor(contributor string) bool {
	if len(f.OnlyContributors) > 0 {
		for _, c := range f.OnlyContributors {
			if strings.EqualFold(c, contributor) {
				return true
			}
		}
		return false
	}
	for _, c := range f.ExcludeContributors {
		if strings.EqualFold(c, contributor) {
			return false
		}
	}
	return true
}
