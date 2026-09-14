// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎯️goals is the goals domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package goals

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

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

// #endregion 🚚️Split

// #region 🚪️Ports

// #region 🔁️Reexports

// 🎯️Goal is the goal document shape of 📐️model, reexported so a client needs one import.
type Goal = model.Goal

// 🟠️GoalDates is the dated members of a goal.
type GoalDates = model.GoalDates

// 🟡️GoalManagementData is the milestone and issue a goal owns.
type GoalManagementData = model.GoalManagementData

// 🆕️GoalCreateInput is the input an open takes.
type GoalCreateInput = model.GoalCreateInput

// ♻️GoalChangeInput is the input a change takes.
type GoalChangeInput = model.GoalChangeInput

// 📪️GoalCloseInput is the input a close takes.
type GoalCloseInput = model.GoalCloseInput

// 🔓️GoalReopenInput is the input a reopen takes.
type GoalReopenInput = model.GoalReopenInput

// 🗑️GoalDeleteInput is the input a delete takes.
type GoalDeleteInput = model.GoalDeleteInput

// 💿️GoalNode is a goal node of the goal and ticket tree.
type GoalNode = model.GoalNode

// 💿️TicketNode is a ticket node of the goal and ticket tree.
type TicketNode = model.TicketNode

// #endregion 🔁️Reexports

// #region ❗️Errors

// ❗️GoalError is everything the goals domain can refuse, as a class rather than a rendered string.
type GoalError struct {
	Kind    string
	Message string
}

// 🔤️Error renders the refusal.
func (err GoalError) Error() string { return err.Message }

// 🏷️Class returns the stable class of the refusal.
func (err GoalError) Class() string { return err.Kind }

// 🏷️ErrorClass returns the stable class of any refusal this domain produced.
func ErrorClass(err error) string {
	if err == nil {
		return ""
	}
	var goalError GoalError
	if errors.As(err, &goalError) {
		return goalError.Kind
	}
	return "port"
}

// 🕳️errMissing refuses a blank required create input.
func errMissing(field string) error {
	return GoalError{Kind: "missing", Message: "missing " + field}
}

// 🚫️errNotAllowed refuses a slug outside the allowed vocabulary.
func errNotAllowed(message string) error { return GoalError{Kind: "not-allowed", Message: message} }

// 📛️errAlreadyExists refuses a duplicate identifier.
func errAlreadyExists(id string) error {
	return GoalError{Kind: "already-exists", Message: "goal with id " + id + " already exists"}
}

// 🔍️errNotFound refuses an identifier that carries no document.
func errNotFound(id string) error {
	return GoalError{Kind: "not-found", Message: "goal file not found: " + id}
}

// 🔁️errAlreadyInState refuses a lifecycle transition that is a no-operation.
func errAlreadyInState(state string) error {
	return GoalError{Kind: "already-in-state", Message: "goal is already " + state}
}

// 🔤️errTitle refuses a blank title or a slug used as a title.
func errTitle(message string) error { return GoalError{Kind: "title", Message: message} }

// 🔢️errUnparsable refuses a reference that carries no number.
func errUnparsable(value string) error {
	return GoalError{Kind: "unparsable", Message: "could not parse number from " + value}
}

// 📄️errDocument refuses a document this codec does not accept.
func errDocument(message string) error { return GoalError{Kind: "document", Message: message} }

// 🗄️errPort refuses when the store or the management provider refused.
func errPort(message string) error { return GoalError{Kind: "port", Message: message} }

// #endregion ❗️Errors

// #region 🚦️GoalStatus

// 🚦️GoalStatus is the closed set of states a goal can be in.
type GoalStatus string

// 🟢️GoalStatusOpen is a goal that is still being worked on.
const GoalStatusOpen GoalStatus = "open"

// 🔴️GoalStatusClosed is a goal that has been finished.
const GoalStatusClosed GoalStatus = "closed"

// 📋️AllGoalStatuses is the whole vocabulary, in declaration order.
func AllGoalStatuses() []GoalStatus { return []GoalStatus{GoalStatusOpen, GoalStatusClosed} }

// 📥️ParseGoalStatus accepts only a member of the closed vocabulary.
func ParseGoalStatus(value string) (GoalStatus, error) {
	for _, status := range AllGoalStatuses() {
		if string(status) == value {
			return status, nil
		}
	}
	return "", errDocument("unknown variant `" + value + "`, expected `open` or `closed`")
}

// #endregion 🚦️GoalStatus

// #region 🪪️IdScheme

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

// #endregion 🪪️IdScheme

// #region 📄️DocumentCodec

// 📄️goalRequiredMembers are the members a stored goal document must carry.
var goalRequiredMembers = []string{"title", "description", "prompt", "status", "client", "llm"}

// 📥️DecodeGoal decodes a goal document, deriving the identifier and parent from the path the
// document was found at, exactly as the reader does.
func DecodeGoal(id string, document string) (*model.Goal, error) {
	var members map[string]json.RawMessage
	if err := json.Unmarshal([]byte(document), &members); err != nil {
		return nil, errDocument(err.Error())
	}
	for _, required := range goalRequiredMembers {
		if _, found := members[required]; !found {
			return nil, errDocument("missing field `" + required + "`")
		}
	}
	var status string
	if err := json.Unmarshal(members["status"], &status); err != nil {
		return nil, errDocument(err.Error())
	}
	if _, err := ParseGoalStatus(status); err != nil {
		return nil, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(document), &goal); err != nil {
		return nil, errDocument(err.Error())
	}
	goal.ID = id
	goal.Parent = ParentGoalID(id)
	return &goal, nil
}

// 📤️EncodeGoal encodes a goal back into its document: two-space indentation, members in
// declaration order, a trailing newline, and the parent member in its compose form.
func EncodeGoal(goal *model.Goal) (string, error) {
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(goal); err != nil {
		return "", errDocument(err.Error())
	}
	return buffer.String(), nil
}

// #endregion 📄️DocumentCodec

// #region 🗄️Store

// 📄️StoredGoal is one identifier and document pair of a goal store.
type StoredGoal struct {
	ID       string `json:"id"`
	Document string `json:"document"`
}

// 🗄️GoalStore is where goal documents live, one entry per identifier.
type GoalStore interface {
	// 📖️Read returns the document at an identifier.
	Read(id string) (string, error)
	// 💾️Write replaces or creates the document at an identifier.
	Write(id string, document string) error
	// 🔍️Exists reports whether a document exists at an identifier.
	Exists(id string) bool
	// 🚚️Rename moves a goal and everything below it to a new identifier.
	Rename(from, to string) error
	// 🗑️Remove removes a goal and everything below it.
	Remove(id string) error
	// 📋️IDs returns every identifier that carries a document, in ascending order.
	IDs() []string
}

// 🧠️MemoryGoalStore is the in-memory store the language-agnostic tests run the lifecycle against.
type MemoryGoalStore struct {
	documents map[string]string
}

// 🆕️NewMemoryGoalStore builds an empty store.
func NewMemoryGoalStore() *MemoryGoalStore {
	return &MemoryGoalStore{documents: map[string]string{}}
}

// 🌱️NewSeededMemoryGoalStore builds a store seeded with identifier and document pairs.
func NewSeededMemoryGoalStore(entries []StoredGoal) *MemoryGoalStore {
	store := NewMemoryGoalStore()
	for _, entry := range entries {
		store.documents[entry.ID] = entry.Document
	}
	return store
}

// 📸️Snapshot returns every stored pair, in identifier order.
func (store *MemoryGoalStore) Snapshot() []StoredGoal {
	snapshot := make([]StoredGoal, 0, len(store.documents))
	for _, id := range store.IDs() {
		snapshot = append(snapshot, StoredGoal{ID: id, Document: store.documents[id]})
	}
	return snapshot
}

// 📖️Read returns the document at an identifier.
func (store *MemoryGoalStore) Read(id string) (string, error) {
	document, found := store.documents[id]
	if !found {
		return "", errNotFound(id)
	}
	return document, nil
}

// 💾️Write replaces or creates the document at an identifier.
func (store *MemoryGoalStore) Write(id string, document string) error {
	store.documents[id] = document
	return nil
}

// 🔍️Exists reports whether a document exists at an identifier.
func (store *MemoryGoalStore) Exists(id string) bool {
	_, found := store.documents[id]
	return found
}

// 🚚️Rename moves a goal and everything below it to a new identifier.
func (store *MemoryGoalStore) Rename(from, to string) error {
	moved := make([]string, 0)
	for _, id := range store.IDs() {
		if id == from || strings.HasPrefix(id, from+"/") {
			moved = append(moved, id)
		}
	}
	if len(moved) == 0 {
		return errNotFound(from)
	}
	for _, id := range moved {
		document := store.documents[id]
		delete(store.documents, id)
		store.documents[to+id[len(from):]] = document
	}
	return nil
}

// 🗑️Remove removes a goal and everything below it.
func (store *MemoryGoalStore) Remove(id string) error {
	for _, candidate := range store.IDs() {
		if candidate == id || strings.HasPrefix(candidate, id+"/") {
			delete(store.documents, candidate)
		}
	}
	return nil
}

// 📋️IDs returns every stored identifier in ascending order.
func (store *MemoryGoalStore) IDs() []string {
	ids := make([]string, 0, len(store.documents))
	for id := range store.documents {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	return ids
}

// 💽️FsGoalStore is the store over a real goals tree of a repository root.
type FsGoalStore struct {
	root string
}

// 🆕️NewFsGoalStore builds the store of the goals directory of a repository root.
func NewFsGoalStore(repoRoot string) *FsGoalStore {
	return &FsGoalStore{root: workspace.GoalsDirForRoot(repoRoot)}
}

// 📁️Directory returns the directory of one goal.
func (store *FsGoalStore) Directory(id string) string {
	return filepath.Join(append([]string{store.root}, strings.Split(id, "/")...)...)
}

// 📄️documentPath returns the document path of one goal.
func (store *FsGoalStore) documentPath(id string) string {
	return filepath.Join(store.Directory(id), "🎯️goal.json")
}

// 📖️Read returns the document at an identifier.
func (store *FsGoalStore) Read(id string) (string, error) {
	data, err := os.ReadFile(store.documentPath(id))
	if err != nil {
		return "", errNotFound(id)
	}
	return string(data), nil
}

// 💾️Write replaces or creates the document at an identifier.
func (store *FsGoalStore) Write(id string, document string) error {
	path := store.documentPath(id)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return errPort(err.Error())
	}
	if err := os.WriteFile(path, []byte(document), 0o644); err != nil {
		return errPort(err.Error())
	}
	return nil
}

// 🔍️Exists reports whether a document exists at an identifier.
func (store *FsGoalStore) Exists(id string) bool {
	_, err := os.Stat(store.documentPath(id))
	return err == nil
}

// 🚚️Rename moves a goal directory and everything below it.
func (store *FsGoalStore) Rename(from, to string) error {
	target := store.Directory(to)
	if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
		return errPort(err.Error())
	}
	if err := os.Rename(store.Directory(from), target); err != nil {
		return errPort(err.Error())
	}
	return nil
}

// 🗑️Remove removes a goal directory and everything below it.
func (store *FsGoalStore) Remove(id string) error {
	if err := os.RemoveAll(store.Directory(id)); err != nil {
		return errPort(err.Error())
	}
	return nil
}

// 📋️IDs returns every identifier that carries a document, in ascending order.
func (store *FsGoalStore) IDs() []string {
	found := make([]string, 0)
	collectGoalIDs(store.root, "", &found)
	sort.Strings(found)
	return found
}

// 🚶️collectGoalIDs descends a goals directory collecting every identifier that carries a document.
func collectGoalIDs(directory, prefix string, found *[]string) {
	entries, err := os.ReadDir(directory)
	if err != nil {
		return
	}
	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		id := entry.Name()
		if prefix != "" {
			id = prefix + "/" + entry.Name()
		}
		child := filepath.Join(directory, entry.Name())
		if _, err := os.Stat(filepath.Join(child, "🎯️goal.json")); err == nil {
			*found = append(*found, id)
		}
		collectGoalIDs(child, id, found)
	}
}

// #endregion 🗄️Store

// #region 🧩️Management

// 🧩️ManagementPort is the provider that owns milestones and issues for the goals of a repository.
type ManagementPort interface {
	// 🏁️CreateMilestone creates a milestone and returns its number.
	CreateMilestone(title, description string) (int64, error)
	// 🔁️UpdateMilestone updates a milestone, leaving blank members untouched.
	UpdateMilestone(number int64, title, description, state, dueOn string) error
	// 🗑️DeleteMilestone deletes a milestone.
	DeleteMilestone(number int64) error
	// 🆕️CreateGoalIssue creates the issue of a goal and returns its URL.
	CreateGoalIssue(title, description string, milestone *int64) (string, error)
	// 🔷️UpdateGoalIssue updates the title and body of a goal issue.
	UpdateGoalIssue(issueURL, title, description string) error
	// 📪️CloseIssue closes an issue.
	CloseIssue(issueURL string) error
	// 🔓️ReopenIssue reopens an issue.
	ReopenIssue(issueURL string) error
	// 🗑️DeleteIssue deletes an issue by its number.
	DeleteIssue(number string) error
	// ➕️AddSubIssue nests a child issue under a parent issue.
	AddSubIssue(parentIssueURL, childIssueURL string) error
	// 🌐️RepoURL returns the repository URL milestone references are built from.
	RepoURL() string
}

// 🚫️NullManagement is the provider that does nothing, for a run with management switched off.
type NullManagement struct{}

// 🏁️CreateMilestone does nothing and hands out the zero number.
func (NullManagement) CreateMilestone(string, string) (int64, error) { return 0, nil }

// 🔁️UpdateMilestone does nothing.
func (NullManagement) UpdateMilestone(int64, string, string, string, string) error { return nil }

// 🗑️DeleteMilestone does nothing.
func (NullManagement) DeleteMilestone(int64) error { return nil }

// 🆕️CreateGoalIssue does nothing and hands out no URL.
func (NullManagement) CreateGoalIssue(string, string, *int64) (string, error) { return "", nil }

// 🔷️UpdateGoalIssue does nothing.
func (NullManagement) UpdateGoalIssue(string, string, string) error { return nil }

// 📪️CloseIssue does nothing.
func (NullManagement) CloseIssue(string) error { return nil }

// 🔓️ReopenIssue does nothing.
func (NullManagement) ReopenIssue(string) error { return nil }

// 🗑️DeleteIssue does nothing.
func (NullManagement) DeleteIssue(string) error { return nil }

// ➕️AddSubIssue does nothing.
func (NullManagement) AddSubIssue(string, string) error { return nil }

// 🌐️RepoURL returns no repository URL.
func (NullManagement) RepoURL() string { return "" }

// 📝️RecordingManagement hands out deterministic numbers and keeps an ordered log of every call.
type RecordingManagement struct {
	repoURL string
	next    int64
	calls   []string
}

// 🆕️NewRecordingManagement builds a provider that numbers milestones and issues from first.
func NewRecordingManagement(repoURL string, first int64) *RecordingManagement {
	return &RecordingManagement{repoURL: repoURL, next: first}
}

// 📜️Calls returns every call made so far, in order.
func (management *RecordingManagement) Calls() []string {
	return append([]string{}, management.calls...)
}

// 🔢️takeNumber hands out the next deterministic number.
func (management *RecordingManagement) takeNumber() int64 {
	number := management.next
	management.next++
	return number
}

// ✍️record appends one call to the log.
func (management *RecordingManagement) record(call string) {
	management.calls = append(management.calls, call)
}

// 🏁️CreateMilestone records the call and hands out the next number.
func (management *RecordingManagement) CreateMilestone(title, description string) (int64, error) {
	number := management.takeNumber()
	management.record(fmt.Sprintf("create-milestone %d %s %s", number, title, description))
	return number, nil
}

// 🔁️UpdateMilestone records the call.
func (management *RecordingManagement) UpdateMilestone(number int64, title, description, state, dueOn string) error {
	management.record(fmt.Sprintf("update-milestone %d %s %s %s %s", number, title, description, state, dueOn))
	return nil
}

// 🗑️DeleteMilestone records the call.
func (management *RecordingManagement) DeleteMilestone(number int64) error {
	management.record(fmt.Sprintf("delete-milestone %d", number))
	return nil
}

// 🆕️CreateGoalIssue records the call and hands out a deterministic issue URL.
func (management *RecordingManagement) CreateGoalIssue(title, description string, milestone *int64) (string, error) {
	number := management.takeNumber()
	reference := "-"
	if milestone != nil {
		reference = fmt.Sprintf("%d", *milestone)
	}
	management.record(fmt.Sprintf("create-goal-issue %d %s %s %s", number, title, description, reference))
	return fmt.Sprintf("%s/issues/%d", management.repoURL, number), nil
}

// 🔷️UpdateGoalIssue records the call.
func (management *RecordingManagement) UpdateGoalIssue(issueURL, title, description string) error {
	management.record(fmt.Sprintf("update-goal-issue %s %s %s", issueURL, title, description))
	return nil
}

// 📪️CloseIssue records the call.
func (management *RecordingManagement) CloseIssue(issueURL string) error {
	management.record(fmt.Sprintf("close-issue %s", issueURL))
	return nil
}

// 🔓️ReopenIssue records the call.
func (management *RecordingManagement) ReopenIssue(issueURL string) error {
	management.record(fmt.Sprintf("reopen-issue %s", issueURL))
	return nil
}

// 🗑️DeleteIssue records the call.
func (management *RecordingManagement) DeleteIssue(number string) error {
	management.record(fmt.Sprintf("delete-issue %s", number))
	return nil
}

// ➕️AddSubIssue records the call.
func (management *RecordingManagement) AddSubIssue(parentIssueURL, childIssueURL string) error {
	management.record(fmt.Sprintf("add-sub-issue %s %s", parentIssueURL, childIssueURL))
	return nil
}

// 🌐️RepoURL returns the repository URL the provider was built with.
func (management *RecordingManagement) RepoURL() string { return management.repoURL }

// #endregion 🧩️Management

// #region 📡️Emission

// 🧠️MemoryEmitter keeps kind, source and payload lines instead of reaching a coordinator.
type MemoryEmitter struct {
	envelopes []string
}

// 🆕️NewMemoryEmitter builds an emitter with an empty log.
func NewMemoryEmitter() *MemoryEmitter { return &MemoryEmitter{} }

// 📤️Emit records one envelope with its payload rendered as canonical JSON.
func (emitter *MemoryEmitter) Emit(kind events.EventKind, source string, payload interface{}) {
	emitter.envelopes = append(emitter.envelopes, fmt.Sprintf("%s\t%s\t%s", kind, source, canonicalPayload(payload)))
}

// 📜️Envelopes returns every envelope emitted so far, in order.
func (emitter *MemoryEmitter) Envelopes() []string {
	return append([]string{}, emitter.envelopes...)
}

// 🧮️canonicalPayload renders a payload as compact JSON with object members in ascending key order
// and no HTML escaping, which is the one rendering both implementations agree on.
func canonicalPayload(payload interface{}) string {
	data, err := json.Marshal(payload)
	if err != nil {
		return ""
	}
	var generic interface{}
	if err := json.Unmarshal(data, &generic); err != nil {
		return ""
	}
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(generic); err != nil {
		return ""
	}
	return strings.TrimRight(buffer.String(), "\n")
}

// #endregion 📡️Emission

// #region 🔓️Lifecycle

// 🎯️Goals is the goal aggregate bound to its ports.
type Goals struct {
	store      GoalStore
	management ManagementPort
	emitter    events.Emitter
	author     string
}

// 🆕️NewGoals binds the aggregate to a store, a management provider, an emitter and an author.
func NewGoals(store GoalStore, management ManagementPort, emitter events.Emitter, author string) *Goals {
	return &Goals{store: store, management: management, emitter: emitter, author: author}
}

// 📋️List returns every goal, identifier ascending.
func (aggregate *Goals) List() ([]*model.Goal, error) {
	ids := aggregate.store.IDs()
	found := make([]*model.Goal, 0, len(ids))
	for _, id := range ids {
		goal, err := aggregate.Read(id)
		if err != nil {
			return nil, err
		}
		found = append(found, goal)
	}
	return found, nil
}

// 📖️Read returns one goal by identifier, accepting the compose form.
func (aggregate *Goals) Read(id string) (*model.Goal, error) {
	id = GoalIDForFilesystem(id)
	document, err := aggregate.store.Read(id)
	if err != nil {
		return nil, err
	}
	return DecodeGoal(id, document)
}

// 🔎️Search returns every goal whose identifier, title, description or status contains the term.
func (aggregate *Goals) Search(term string) ([]*model.Goal, error) {
	all, err := aggregate.List()
	if err != nil {
		return nil, err
	}
	needle := strings.ToLower(term)
	found := make([]*model.Goal, 0, len(all))
	for _, goal := range all {
		haystack := strings.ToLower(goal.ID + " " + goal.Title + " " + goal.Description + " " + goal.Status)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, goal)
		}
	}
	return found, nil
}

// 🆕️Create opens a goal, asks the management provider for the milestone or issue the depth calls
// for, persists the document and emits the open event.
func (aggregate *Goals) Create(input model.GoalCreateInput) (*model.Goal, error) {
	if input.Title == "" {
		return nil, errMissing("title")
	}
	if input.Description == "" {
		return nil, errMissing("description")
	}
	if input.Prompt == "" {
		return nil, errMissing("prompt")
	}
	if input.DueDate == "" {
		return nil, errMissing("due date")
	}
	if input.LLM == "" {
		return nil, errMissing("llm")
	}
	if input.Client == "" {
		return nil, errMissing("client")
	}
	llm, err := model.ResolveAllowedLLM(input.LLM)
	if err != nil {
		return nil, errNotAllowed(err.Error())
	}
	effort := ""
	if input.Effort != "" {
		effort, err = model.ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
	}
	client, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, errNotAllowed(err.Error())
	}
	id := ComposeGoalID(input.Parent, input.Title)
	if aggregate.store.Exists(id) {
		return nil, errAlreadyExists(id)
	}
	goal := &model.Goal{
		Title:       input.Title,
		Description: input.Description,
		Prompt:      input.Prompt,
		Status:      string(GoalStatusOpen),
		Dates:       model.GoalDates{Due: input.DueDate},
		Client:      client,
		LLM:         llm,
		Effort:      effort,
		Parent:      input.Parent,
		ID:          id,
	}
	if !input.NoManagement {
		management, err := aggregate.openManagement(id, input)
		if err != nil {
			return nil, err
		}
		goal.Management = management
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(id, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalOpenEnded, events.GoalOpenPayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Title:       goal.Title,
		Description: goal.Description,
		LLM:         llm,
		Effort:      effort,
		Parent:      goal.Parent,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 🧩️openManagement asks the provider for the milestone or issue an identifier's depth calls for.
func (aggregate *Goals) openManagement(id string, input model.GoalCreateInput) (*model.GoalManagementData, error) {
	if IsRootGoal(id) {
		if input.Milestone != "" {
			return &model.GoalManagementData{Milestone: input.Milestone}, nil
		}
		number, err := aggregate.management.CreateMilestone(input.Title, input.Description)
		if err != nil {
			return nil, err
		}
		if input.DueDate != "" {
			_ = aggregate.management.UpdateMilestone(number, "", "", "", input.DueDate)
		}
		return &model.GoalManagementData{Milestone: fmt.Sprintf("%s/milestone/%d", aggregate.management.RepoURL(), number)}, nil
	}
	if IsFirstGenGoal(id) {
		var milestone *int64
		if root, err := aggregate.Read(RootGoalID(id)); err == nil && root.Management != nil && root.Management.Milestone != "" {
			if number, err := ParseMilestoneNumber(root.Management.Milestone); err == nil {
				milestone = &number
			}
		}
		issue, err := aggregate.management.CreateGoalIssue(input.Title, input.Description, milestone)
		if err != nil {
			return nil, err
		}
		return &model.GoalManagementData{Issue: issue}, nil
	}
	issue, err := aggregate.management.CreateGoalIssue(input.Title, input.Description, nil)
	if err != nil {
		return nil, err
	}
	if parent, err := aggregate.Read(ParentGoalID(id)); err == nil && parent.Management != nil && parent.Management.Issue != "" {
		_ = aggregate.management.AddSubIssue(parent.Management.Issue, issue)
	}
	return &model.GoalManagementData{Issue: issue}, nil
}

// ♻️Change changes a goal: retitling moves the document, re-parenting moves it too, and the
// management provider is told about the new title, body, state and due date.
func (aggregate *Goals) Change(input model.GoalChangeInput) (*model.Goal, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return nil, err
	}
	if input.Title != nil {
		if err := aggregate.retitle(goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.LLM != nil {
		llm, err := model.ResolveAllowedLLM(*input.LLM)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
		goal.LLM = llm
	}
	if input.Effort != nil {
		effort, err := model.ResolveAllowedEffort(*input.Effort)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
		goal.Effort = effort
	}
	if input.Parent != nil {
		slug := goal.ID
		if index := strings.LastIndex(goal.ID, "/"); index != -1 {
			slug = goal.ID[index+1:]
		}
		target := slug
		if *input.Parent != "" {
			target = *input.Parent + "/" + slug
		}
		if err := aggregate.relocate(goal, target); err != nil {
			return nil, err
		}
		goal.Parent = *input.Parent
	}
	if !input.NoManagement {
		if err := aggregate.syncManagement(goal, goal.Status, goal.Dates.Due); err != nil {
			return nil, err
		}
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(goal.ID, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalChangeEnded, events.GoalChangePayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Title:       input.Title,
		Description: input.Description,
		LLM:         input.LLM,
		Effort:      input.Effort,
		Parent:      input.Parent,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 📪️Close closes a goal, refusing a goal that is already closed.
func (aggregate *Goals) Close(input model.GoalCloseInput) (*model.Goal, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return nil, err
	}
	if goal.Status == string(GoalStatusClosed) {
		return nil, errAlreadyInState("closed")
	}
	goal.Status = string(GoalStatusClosed)
	goal.Summary = input.Summary
	if !input.NoManagement && goal.Management != nil {
		if IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			if number, err := ParseMilestoneNumber(goal.Management.Milestone); err == nil {
				if err := aggregate.management.UpdateMilestone(number, goal.Title, goal.Description, "closed", ""); err != nil {
					return nil, err
				}
			}
		} else if !IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			if err := aggregate.management.CloseIssue(goal.Management.Issue); err != nil {
				return nil, err
			}
		}
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(goal.ID, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalCloseEnded, events.GoalClosePayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Summary:     goal.Summary,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 🔓️Reopen reopens a goal, refusing a goal that is already open, and replaces the interaction
// members with the ones the reopen carries.
func (aggregate *Goals) Reopen(input model.GoalReopenInput) (*model.Goal, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return nil, err
	}
	if goal.Status == string(GoalStatusOpen) {
		return nil, errAlreadyInState("open")
	}
	goal.Status = string(GoalStatusOpen)
	if input.Title != nil {
		if err := aggregate.retitle(goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}
	goal.Prompt = input.Prompt
	goal.LLM = input.LLM
	goal.Client = input.Client
	if input.Effort != "" {
		effort, err := model.ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
		goal.Effort = effort
	}
	if !input.NoManagement {
		if err := aggregate.syncManagement(goal, string(GoalStatusOpen), goal.Dates.Due); err != nil {
			return nil, err
		}
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(goal.ID, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalReopenEnded, events.GoalReopenPayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Prompt:      input.Prompt,
		Client:      input.Client,
		LLM:         input.LLM,
		Effort:      goal.Effort,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 🗑️Delete deletes a goal and everything below it, after retiring its milestone or issue.
func (aggregate *Goals) Delete(input model.GoalDeleteInput) (bool, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return false, err
	}
	if !input.NoManagement && goal.Management != nil {
		if IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			if number, err := ParseMilestoneNumber(goal.Management.Milestone); err == nil {
				if err := aggregate.management.DeleteMilestone(number); err != nil {
					return false, err
				}
			}
		} else if !IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			parts := strings.Split(goal.Management.Issue, "/")
			if err := aggregate.management.DeleteIssue(parts[len(parts)-1]); err != nil {
				return false, err
			}
		}
	}
	if err := aggregate.store.Remove(goal.ID); err != nil {
		return false, err
	}
	return true, nil
}

// 🔁️syncManagement tells the provider about a goal's current title, body, state and due date.
func (aggregate *Goals) syncManagement(goal *model.Goal, state, due string) error {
	if goal.Management == nil {
		return nil
	}
	if IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
		if number, err := ParseMilestoneNumber(goal.Management.Milestone); err == nil {
			return aggregate.management.UpdateMilestone(number, goal.Title, goal.Description, state, due)
		}
		return nil
	}
	if !IsRootGoal(goal.ID) && goal.Management.Issue != "" {
		return aggregate.management.UpdateGoalIssue(goal.Management.Issue, goal.Title, goal.Description)
	}
	return nil
}

// 🔤️retitle refuses a blank title or a slug used as a title and moves the document.
func (aggregate *Goals) retitle(goal *model.Goal, title string) error {
	title = strings.TrimSpace(title)
	if title == "" {
		return errTitle("goal title is required")
	}
	slug := identity.Slugify(title)
	if title == slug {
		return errTitle("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(slug) {
		return errTitle("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}
	target := ComposeGoalID(goal.Parent, title)
	if err := aggregate.relocate(goal, target); err != nil {
		return err
	}
	goal.Title = title
	return nil
}

// 🚚️relocate moves a goal to a target identifier, refusing a target that is taken.
func (aggregate *Goals) relocate(goal *model.Goal, target string) error {
	if target == goal.ID {
		return nil
	}
	if aggregate.store.Exists(target) {
		return errAlreadyExists(target)
	}
	if err := aggregate.store.Rename(goal.ID, target); err != nil {
		return err
	}
	goal.ID = target
	return nil
}

// 📤️emit sends one envelope to the bound emitter.
func (aggregate *Goals) emit(kind events.EventKind, payload interface{}) {
	if aggregate.emitter == nil {
		return
	}
	aggregate.emitter.Emit(kind, "repo-cli", payload)
}

// #endregion 🔓️Lifecycle

// #region 🌳️Tree

// 🌱️GoalSeed is the flat goal record the tree is assembled from.
type GoalSeed struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Status      string `json:"status"`
	DueDate     string `json:"dueDate"`
	CreatedAt   string `json:"createdAt"`
	Description string `json:"description"`
}

// 🎫️TicketSeed is the flat ticket record the tree is assembled from.
type TicketSeed struct {
	ID       string `json:"id"`
	Slug     string `json:"slug"`
	Status   string `json:"status"`
	Title    string `json:"title"`
	Goal     string `json:"goal"`
	Parent   string `json:"parent"`
	Prompt   string `json:"prompt"`
	Summary  string `json:"summary"`
	Created  string `json:"created"`
	Finished string `json:"finished"`
}

// 🖨️TreeFormat states how a rendered tree lays its lines out.
type TreeFormat int

// 🌲️TreeFormatText draws box connectors, one indentation step per level.
const TreeFormatText TreeFormat = 0

// 📝️TreeFormatMarkdown draws markdown list items, two spaces per level.
const TreeFormatMarkdown TreeFormat = 1

// 🖋️TreeLines renders the content of one tree line.
type TreeLines interface {
	// 🎯️Goal renders the line of a goal node.
	Goal(node *model.GoalNode) string
	// 🎫️Ticket renders the line of a ticket node.
	Ticket(node *model.TicketNode) string
}

// 🔤️PlainTreeLines states the identifying members and nothing else.
type PlainTreeLines struct{}

// 🎯️Goal renders a goal node as its identifying members.
func (PlainTreeLines) Goal(node *model.GoalNode) string {
	return fmt.Sprintf("goal %s %s [%s] %s", node.ID, node.Title, node.Status, node.DueDate)
}

// 🎫️Ticket renders a ticket node as its identifying members.
func (PlainTreeLines) Ticket(node *model.TicketNode) string {
	return fmt.Sprintf("ticket %s %s [%s] %s", node.Slug, node.Title, node.Status, node.URI)
}

// 🌳️BuildGoalTree assembles the goal forest out of goal and ticket seeds.
func BuildGoalTree(goalSeeds []GoalSeed, ticketSeeds []TicketSeed) []*model.GoalNode {
	nodes := map[string]*model.GoalNode{}
	for _, seed := range goalSeeds {
		nodes[seed.ID] = &model.GoalNode{
			ID:          seed.ID,
			Title:       seed.Title,
			Status:      seed.Status,
			DueDate:     seed.DueDate,
			CreatedAt:   seed.CreatedAt,
			Description: seed.Description,
		}
	}
	known := make([]string, 0, len(nodes))
	for id := range nodes {
		known = append(known, id)
	}
	sort.Strings(known)

	ticketNodes := make([]*model.TicketNode, 0, len(ticketSeeds))
	for _, seed := range ticketSeeds {
		ticketNodes = append(ticketNodes, &model.TicketNode{
			ID:          seed.ID,
			Slug:        seed.Slug,
			Status:      seed.Status,
			Title:       seed.Title,
			URI:         "repo://ticket/" + identity.EmojiText(identity.Entity("ticket")) + workspace.Flat(seed.Slug),
			GoalID:      seed.Goal,
			ParentID:    seed.Parent,
			Created:     seed.Created,
			Finished:    seed.Finished,
			Description: seed.Prompt,
			Summary:     seed.Summary,
		})
	}

	perGoal := map[string][]*model.TicketNode{}
	orphans := make([]*model.TicketNode, 0)
	for _, node := range ticketNodes {
		if node.GoalID != "" && containsString(known, node.GoalID) {
			perGoal[node.GoalID] = append(perGoal[node.GoalID], node)
		} else {
			orphans = append(orphans, node)
		}
	}
	for id, owned := range perGoal {
		if node, found := nodes[id]; found {
			node.Tickets = nestTickets(owned)
		}
	}

	childIDs := map[string][]string{}
	rootIDs := make([]string, 0)
	for _, id := range known {
		parent := ""
		if index := strings.LastIndex(id, "/"); index != -1 {
			parent = id[:index]
		}
		if parent != "" && parent != id && containsString(known, parent) {
			childIDs[parent] = append(childIDs[parent], id)
		} else {
			rootIDs = append(rootIDs, id)
		}
	}
	roots := make([]*model.GoalNode, 0, len(rootIDs))
	for _, id := range rootIDs {
		if node := assembleGoalNode(id, nodes, childIDs); node != nil {
			roots = append(roots, node)
		}
	}
	sortGoalNodes(roots)

	if len(orphans) > 0 {
		roots = append(roots, &model.GoalNode{Title: "No Goal", Tickets: nestTickets(orphans)})
	}
	return roots
}

// 🔎️containsString reports whether a slice carries a value.
func containsString(values []string, value string) bool {
	for _, candidate := range values {
		if candidate == value {
			return true
		}
	}
	return false
}

// 🧩️assembleGoalNode copies a goal node together with everything below it.
func assembleGoalNode(id string, nodes map[string]*model.GoalNode, childIDs map[string][]string) *model.GoalNode {
	source, found := nodes[id]
	if !found {
		return nil
	}
	node := *source
	children := make([]*model.GoalNode, 0)
	for _, child := range childIDs[id] {
		if assembled := assembleGoalNode(child, nodes, childIDs); assembled != nil {
			children = append(children, assembled)
		}
	}
	if len(children) == 0 {
		node.Children = nil
	} else {
		node.Children = children
	}
	return &node
}

// 🧬️nestTickets hangs every ticket under the ticket it names.
func nestTickets(nodes []*model.TicketNode) []*model.TicketNode {
	ids := make([]string, 0, len(nodes))
	byID := map[string]*model.TicketNode{}
	for _, node := range nodes {
		ids = append(ids, node.ID)
		byID[node.ID] = node
	}
	childIDs := map[string][]string{}
	rootIDs := make([]string, 0)
	for _, node := range nodes {
		if node.ParentID != "" && containsString(ids, node.ParentID) && node.ParentID != node.ID {
			childIDs[node.ParentID] = append(childIDs[node.ParentID], node.ID)
		} else {
			rootIDs = append(rootIDs, node.ID)
		}
	}
	roots := make([]*model.TicketNode, 0, len(rootIDs))
	for _, id := range rootIDs {
		if assembled := assembleTicketNode(id, byID, childIDs); assembled != nil {
			roots = append(roots, assembled)
		}
	}
	return roots
}

// 🧩️assembleTicketNode copies a ticket node together with everything below it.
func assembleTicketNode(id string, nodes map[string]*model.TicketNode, childIDs map[string][]string) *model.TicketNode {
	source, found := nodes[id]
	if !found {
		return nil
	}
	node := *source
	children := make([]*model.TicketNode, 0)
	for _, child := range childIDs[id] {
		if assembled := assembleTicketNode(child, nodes, childIDs); assembled != nil {
			children = append(children, assembled)
		}
	}
	if len(children) == 0 {
		node.Children = nil
	} else {
		node.Children = children
	}
	return &node
}

// 🔢️sortGoalNodes orders siblings by due date and then by identifier, a blank due date last.
func sortGoalNodes(nodes []*model.GoalNode) {
	sort.SliceStable(nodes, func(left, right int) bool {
		first, second := nodes[left], nodes[right]
		if first.DueDate != second.DueDate {
			if first.DueDate == "" {
				return false
			}
			if second.DueDate == "" {
				return true
			}
			return first.DueDate < second.DueDate
		}
		return first.ID < second.ID
	})
	for _, node := range nodes {
		sortGoalNodes(node.Children)
	}
}

// 🔢️CountOpenSubgoals returns how many open goals hang below a node, at any depth.
func CountOpenSubgoals(node *model.GoalNode) int {
	total := 0
	for _, child := range node.Children {
		if child.Status == "open" {
			total++
		}
		total += CountOpenSubgoals(child)
	}
	return total
}

// 🔢️CountOpenTickets returns how many open tickets hang below a node, at any depth.
func CountOpenTickets(node *model.GoalNode) int {
	total := countOpenTicketNodes(node.Tickets)
	for _, child := range node.Children {
		total += CountOpenTickets(child)
	}
	return total
}

// 🔢️countOpenTicketNodes counts the open tickets of a ticket forest.
func countOpenTicketNodes(nodes []*model.TicketNode) int {
	total := 0
	for _, node := range nodes {
		if node.Status == "open" {
			total++
		}
		total += countOpenTicketNodes(node.Children)
	}
	return total
}

// 🎨️RenderGoalTree renders a goal forest, goals first and their tickets after them at every level.
func RenderGoalTree(roots []*model.GoalNode, format TreeFormat, lines TreeLines) string {
	var out strings.Builder
	last := len(roots) - 1
	for index, root := range roots {
		renderGoalNode(root, "", index == last, true, format, lines, &out)
	}
	return out.String()
}

// 🎯️renderGoalNode renders one goal node and everything below it.
func renderGoalNode(node *model.GoalNode, prefix string, isLast, isRoot bool, format TreeFormat, lines TreeLines, out *strings.Builder) {
	content := lines.Goal(node)
	total := len(node.Children) + len(node.Tickets)
	next := writeTreeLine(out, prefix, content, isLast, isRoot, format)
	for index, child := range node.Children {
		renderGoalNode(child, next, index == total-1, false, format, lines, out)
	}
	for index, ticket := range node.Tickets {
		renderTicketNode(ticket, next, len(node.Children)+index == total-1, format, lines, out)
	}
}

// 🎫️renderTicketNode renders one ticket node and everything below it.
func renderTicketNode(node *model.TicketNode, prefix string, isLast bool, format TreeFormat, lines TreeLines, out *strings.Builder) {
	content := lines.Ticket(node)
	next := writeTreeLine(out, prefix, content, isLast, false, format)
	last := len(node.Children) - 1
	for index, child := range node.Children {
		renderTicketNode(child, next, index == last, format, lines, out)
	}
}

// 🖊️writeTreeLine writes one rendered line and returns the prefix of everything below it.
func writeTreeLine(out *strings.Builder, prefix, content string, isLast, isRoot bool, format TreeFormat) string {
	if format == TreeFormatMarkdown {
		out.WriteString(prefix)
		out.WriteString("- ")
		out.WriteString(content)
		out.WriteString("\n")
		return prefix + "  "
	}
	connector := "├️─️─️ "
	switch {
	case isRoot:
		connector = ""
	case isLast:
		connector = "└️─️─️ "
	}
	out.WriteString(prefix)
	out.WriteString(connector)
	out.WriteString(content)
	out.WriteString("\n")
	switch {
	case isRoot:
		return prefix
	case isLast:
		return prefix + "    "
	}
	return prefix + "│️   "
}

// #endregion 🌳️Tree

// #endregion 🚪️Ports
