// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧑️contributors is the contributors domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package contributors

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"
	time "time"
	unicode "unicode"

	events "github.com/usalu/semio/repo/events"

	codebase "github.com/usalu/semio/repo/codebase"
	identity "github.com/usalu/semio/repo/identity"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🚚️Split

// #region 🎽️Languages

// 🤝️FindAndUpdateContributor MUST return nil when no match is found.
// 🔎️FindAndUpdateContributor searches for and returns the matching and update contributor.
func FindAndUpdateContributor(authorStr string) string {
	parsed := languages.ParseGitAuthor(authorStr)
	if parsed.Name == "" && parsed.Email == "" {
		return "unknown"
	}

	contributors := hostContributors()

	var found *model.Contributor

	if parsed.Email != "" {
		for i := range contributors {
			matches := false
			if strings.EqualFold(contributors[i].Email, parsed.Email) {
				matches = true
			} else {
				for _, e := range contributors[i].Emails {
					if strings.EqualFold(e, parsed.Email) {
						matches = true
						break
					}
				}
			}
			if matches {
				found = &contributors[i]

				nameExists := strings.EqualFold(found.Name, parsed.Name)
				if !nameExists && parsed.Name != "" {
					for _, n := range found.Names {
						if strings.EqualFold(n, parsed.Name) {
							nameExists = true
							break
						}
					}
				}
				if !nameExists && parsed.Name != "" {
					found.Names = append(found.Names, parsed.Name)
					SaveContributor(*found)
				}
				break
			}
		}
	}

	if found == nil && parsed.Name != "" {
		for i := range contributors {
			matches := false
			if strings.EqualFold(contributors[i].Name, parsed.Name) {
				matches = true
			} else {
				for _, n := range contributors[i].Names {
					if strings.EqualFold(n, parsed.Name) {
						matches = true
						break
					}
				}
			}
			if matches {
				found = &contributors[i]

				emailExists := strings.EqualFold(found.Email, parsed.Email)
				if !emailExists && parsed.Email != "" {
					for _, e := range found.Emails {
						if strings.EqualFold(e, parsed.Email) {
							emailExists = true
							break
						}
					}
				}
				if !emailExists && parsed.Email != "" {
					found.Emails = append(found.Emails, parsed.Email)
					SaveContributor(*found)
				}
				break
			}
		}
	}

	if found != nil {
		return found.Alias
	}

	return authorStr
}

// 🔐️resolveAuthorToAlias holds the data fields for a resolveAuthorToAlias record.
func resolveAuthorToAlias(name, email string) string {
	for _, c := range hostContributors() {
		if email != "" {
			for _, e := range c.Emails {
				if strings.EqualFold(e, email) {
					return c.Alias
				}
			}
		}
		if name != "" {
			if strings.EqualFold(c.Name, name) {
				return c.Alias
			}
			for _, n := range c.Names {
				if strings.EqualFold(n, name) {
					return c.Alias
				}
			}
		}
	}
	if name != "" && email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	if name != "" {
		return name
	}
	if email != "" {
		return email
	}
	return ""
}

// #endregion 🎽️Languages

// #region 📦️Utils

// 🔐️GetGitAuthorAlias MUST return the stored value without modification.
// 🟫️GetGitAuthorAlias returns the git author alias of the value.
func GetGitAuthorAlias() string {
	name, _, _ := workspace.ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	name = strings.TrimSpace(name)
	email, _, _ := workspace.ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	email = strings.TrimSpace(email)

	fallback := name
	if email != "" {
		fallback = fmt.Sprintf("%s <%s>", name, email)
	}

	return FindAndUpdateContributor(fallback)
}

// #endregion 📦️Utils

// #region 🏩️Codebase

// 🤝️BuildCodebaseContributors MUST assemble the codebase contributors from the available context data.
// 🔻️BuildCodebaseContributors constructs and returns the codebase contributors structure.
func BuildCodebaseContributors(ctx *codebase.CodebaseContext) []model.CodebaseContributor {
	contributors := hostContributors()

	var result []model.CodebaseContributor
	for _, c := range contributors {
		links := c.Links
		if links == nil {
			links = map[string]string{}
		}
		if c.Github != "" {
			links["github"] = c.Github
		}
		emails := append([]string{}, c.Emails...)
		if c.Email != "" {
			emails = append(emails, c.Email)
		}
		avatarPath := workspace.GetContributorAvatarPath(c.Alias)
		avatarRoundPath := workspace.GetContributorAvatarRoundPath(c.Alias)

		var icons *model.ContributorIcons
		if workspace.FileExists(avatarPath) || workspace.FileExists(avatarRoundPath) {
			icons = &model.ContributorIcons{}
			if workspace.FileExists(avatarPath) {
				avatar := ctx.FileURI(workspace.GetRelativePath(avatarPath))
				icons.Avatar = &avatar
			}
			if workspace.FileExists(avatarRoundPath) {
				avatarRound := ctx.FileURI(workspace.GetRelativePath(avatarRoundPath))
				icons.AvatarRound = &avatarRound
			}
			if githubLink, ok := links["github"]; ok {
				github := githubLink + ".png"
				icons.Github = &github
			}
		}

		var contributions *model.ContributorContributionsInternal
		if len(c.Contributions.Bundles) > 0 || len(c.Contributions.Files) > 0 {
			contributions = &model.ContributorContributionsInternal{}
			for _, b := range c.Contributions.Bundles {
				contributions.Bundles = append(contributions.Bundles, model.ContributorBundleContrib{ID: b})
			}
			for _, f := range c.Contributions.Folders {
				contributions.Folders = append(contributions.Folders, model.ContributorFolderContrib{ID: f})
			}
			for _, f := range c.Contributions.Files {
				contributions.Files = append(contributions.Files, model.ContributorFileContrib{ID: f})
			}
			for _, r := range c.Contributions.Regions {
				contributions.Sections = append(contributions.Sections, model.ContributorSectionContrib{ID: r})
			}
			for _, d := range c.Contributions.Definitions {
				contributions.Definitions = append(contributions.Definitions, model.ContributorDefinitionContrib{ID: d})
			}
		}

		linesTotal := 0
		if c.Contributions.Lines != nil {
			linesTotal = c.Contributions.Lines.Added + c.Contributions.Lines.Removed
		}

		result = append(result, model.CodebaseContributor{
			ID:            c.Alias,
			URI:           ctx.FileURI(".🧬semio/🦑️repo/🧑️‍💻️devs/" + c.Alias),
			Path:          ".🧬semio/🦑️repo/🧑️‍💻️devs/" + c.Alias + "/contributor.json",
			Name:          c.Name,
			Icons:         icons,
			Emails:        emails,
			Links:         links,
			Contributions: contributions,
			Metrics: &model.ContributorMetricsInternal{
				Checkpoints: len(c.Contributions.Checkpoints),
				Tickets:     len(c.Contributions.Tickets),
				Bundles:     len(c.Contributions.Bundles),
				Folders:     len(c.Contributions.Folders),
				Files:       len(c.Contributions.Files),
				Lines:       linesTotal,
				Sections:    len(c.Contributions.Regions),
				Definitions: len(c.Contributions.Definitions),
			},
		})
	}
	return result
}

// #endregion 🏩️Codebase

// #region 📋️Tickets

// ◼LoadCheckpoints MUST return all matching checkpoints from the data source.
// 🔢️LoadCheckpoints loads the checkpoints of the current repository root through the ported reader,
// so the author identifier the log carries reaches the checkpoint rather than being dropped.
func LoadCheckpoints(limit *int) []model.Checkpoint {
	return ListCheckpoints(NewFsCheckpointSource(workspace.RootDir), limit)
}

// #endregion 📋️Tickets

// #region 🔊️Cli

// 🎯️StreamContributors MUST invoke the callback for each matching contributors entry.
// 🔖️StreamContributors streams contributors entries through the callback.
func StreamContributors(ctx context.Context, out chan<- model.Contributor, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	contributors := hostContributors()

	for _, c := range contributors {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !model.MatchesFilter(c.Alias, options) && !model.MatchesFilter(c.Name, options) {
				continue
			}
			queryTextParts := []string{c.Alias, c.Name, c.Github, c.Email}
			queryTextParts = append(queryTextParts, c.Aliases...)
			queryTextParts = append(queryTextParts, c.Githubs...)
			queryTextParts = append(queryTextParts, c.Names...)
			queryTextParts = append(queryTextParts, c.Emails...)
			if !model.MatchesQuery(strings.Join(queryTextParts, " "), options) {
				continue
			}
			if len(options.IncludeContributors) > 0 {
				found := false
				for _, id := range options.IncludeContributors {
					if c.Alias == id || c.Name == id {
						found = true
						break
					}
				}
				if !found {
					continue
				}
			}
			if len(options.ExcludeContributors) > 0 {
				excluded := false
				for _, id := range options.ExcludeContributors {
					if c.Alias == id || c.Name == id {
						excluded = true
						break
					}
				}
				if excluded {
					continue
				}
			}
			out <- c
		}
	}
	return nil
}

// 🆕️CreateContributor MUST create a new entry and return an error on conflict.
// 🔖️CreateContributor creates a new contributor entry.
func CreateContributor(alias string) (*model.Contributor, error) {
	dir := workspace.GetContributorPath(alias)
	if workspace.FileExists(dir) {
		return nil, fmt.Errorf("contributor already exists: %s", alias)
	}
	workspace.EnsureDir(dir)
	c := model.Contributor{Alias: alias}
	data, _ := json.MarshalIndent(c, "", "  ")
	workspace.WriteTextFile(filepath.Join(dir, "🧑️‍💻️contributor.json"), string(data))
	return &c, nil
}

// 🔷️LoadContributor MUST return all matching contributor from the data source.
// 🔖️LoadContributor loads and returns contributor from the data source.
func LoadContributor(alias string) (*model.Contributor, error) {
	path := filepath.Join(workspace.GetContributorPath(alias), "🧑️‍💻️contributor.json")
	if !workspace.FileExists(path) {
		return nil, fmt.Errorf("contributor not found: %s", alias)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var c model.Contributor
	if err := json.Unmarshal(data, &c); err != nil {
		return nil, err
	}
	if c.Alias == "" {
		c.Alias = alias
	}
	return &c, nil
}

// 🏪️SaveContributor MUST persist the contributor atomically to the data store.
// 🔖️SaveContributor persists contributor to the data store.
func SaveContributor(c model.Contributor) error {
	if c.Alias == "" {
		c.Alias = c.Github
	}
	dir := workspace.GetContributorPath(c.Alias)
	if !workspace.FileExists(dir) {
		workspace.EnsureDir(dir)
	}
	data, err := json.MarshalIndent(c, "", "  ")
	if err != nil {
		return err
	}
	return workspace.WriteTextFile(filepath.Join(dir, "🧑️‍💻️contributor.json"), string(data))
}

// 🚚️RemoveContributor MUST remove the target and return an error on failure.
// 🔖️RemoveContributor removes the specified contributor.
func RemoveContributor(alias string) error {
	dir := filepath.Join(workspace.GetRepoMetaDir(), "🧑️‍💻️devs", alias)
	if !workspace.FileExists(dir) {
		return fmt.Errorf("contributor not found: %s", alias)
	}
	return os.RemoveAll(dir)
}

// #endregion 🔊️Cli

// #region ❄️Goals

// 💾️StreamCheckpoints MUST invoke the callback for each matching checkpoints entry.
// ✔️StreamCheckpoints streams checkpoints entries through the callback.
func StreamCheckpoints(ctx context.Context, limit *int, out chan<- model.Checkpoint, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	checkpoints := LoadCheckpoints(limit)
	for _, c := range checkpoints {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !model.MatchesFilter(c.SHA, options) && !model.MatchesFilter(c.Title, options) {
				continue
			}
			if !model.MatchesQuery(c.SHA+" "+c.Title, options) {
				continue
			}
			out <- c
		}
	}
	return nil
}

// #endregion ❄️Goals

// #region 🪅️Sessions

// 🪪️SessionKind represents the status of a session.
type SessionKind string

const SessionKindRunning SessionKind = "running"

const SessionKindCompleted SessionKind = "completed"

const SessionKindInterrupted SessionKind = "interrupted"

// 💿️Session holds the data fields for a session record.
type Session struct {
	UUID       string      `json:"uuid"`
	Year       int         `json:"year"`
	Month      int         `json:"month"`
	Day        int         `json:"day"`
	Checkpoint string      `json:"checkpoint,omitempty"`
	Kind       SessionKind `json:"kind"`
	Client     string      `json:"client,omitempty"`
	LLM        string      `json:"llm,omitempty"`
	StartedAt  string      `json:"startedAt,omitempty"`
	EndedAt    string      `json:"endedAt,omitempty"`
}

// 🔷️GetID returns the repo ID for a session.
// ✔️GetID MUST use the checkpoint ID as parent for the session ID. Falls back to date document if checkpoint is unknown.
func (s *Session) GetID() string {
	var parentId string
	if s.Checkpoint != "" {
		parentId = model.GetArtifactID("checkpoint", map[string]interface{}{"sha": s.Checkpoint})
	} else {
		parentId = model.GetArtifactID("day", map[string]interface{}{
			"parentId": model.GetArtifactID("month", map[string]interface{}{
				"parentId": model.GetArtifactID("year", map[string]interface{}{
					"parentId": "",
					"yy":       fmt.Sprintf("%02d", s.Year),
				}),
				"mm": fmt.Sprintf("%02d", s.Month),
			}),
			"dd": fmt.Sprintf("%02d", s.Day),
		})
	}
	return model.GetArtifactID("session", map[string]interface{}{
		"parentId": parentId,
		"uuid":     s.UUID,
	})
}

// 🔗️GetURI returns the repo URI for a session.
func (s *Session) GetURI() string {
	return model.GetArtifactURI("session", map[string]interface{}{
		"uuid": s.UUID,
	})
}

// 😀️SessionKindEmoji returns the emoji for the given session kind.
func SessionKindEmoji(kind SessionKind) string {
	switch kind {
	case SessionKindRunning:
		return identity.Entity("session-running")
	case SessionKindCompleted:
		return identity.Entity("session-completed")
	case SessionKindInterrupted:
		return identity.Entity("session-interrupted")
	}
	return identity.Entity("session")
}

// 📡️DeriveSessionKind determines the session kind by examining its event files.
func DeriveSessionKind(sessionDir string) SessionKind {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta model.SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil {
			for _, entry := range meta.Events {
				var evt map[string]interface{}
				if err := json.Unmarshal(entry.Event, &evt); err != nil {
					continue
				}
				if kind, ok := evt["kind"].(string); ok && kind == string(model.HookAgentEnded) {
					return SessionKindCompleted
				}
			}
			if info, statErr := os.Stat(metaPath); statErr == nil && time.Since(info.ModTime()) < 30*time.Minute {
				return SessionKindRunning
			}
			if len(meta.Events) > 0 {
				return SessionKindInterrupted
			}
		}
	}
	entries, err := os.ReadDir(sessionDir)
	if err != nil {
		return SessionKindInterrupted
	}
	for _, e := range entries {
		if strings.Contains(e.Name(), "agent-ended") {
			return SessionKindCompleted
		}
	}
	if len(entries) == 0 {
		return SessionKindInterrupted
	}

	var latestTime time.Time
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		info, err := e.Info()
		if err != nil {
			continue
		}
		if info.ModTime().After(latestTime) {
			latestTime = info.ModTime()
		}
	}
	if !latestTime.IsZero() && time.Since(latestTime) < 30*time.Minute {
		return SessionKindRunning
	}
	return SessionKindInterrupted
}

// 🧲️ExtractSessionClient reads the client from session event files.
func ExtractSessionClient(sessionDir string) string {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta model.SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil && meta.Client != "" {
			return meta.Client
		}
		if err := json.Unmarshal(data, &meta); err == nil {
			for _, item := range meta.Events {
				var evt map[string]interface{}
				if err := json.Unmarshal(item.Event, &evt); err == nil {
					if client, ok := evt["client"].(string); ok && client != "" {
						return client
					}
				}
			}
		}
	}
	entries, err := os.ReadDir(sessionDir)
	if err != nil || len(entries) == 0 {
		return ""
	}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") || e.Name() == "session.json" {
			continue
		}
		data, err := os.ReadFile(filepath.Join(sessionDir, e.Name()))
		if err != nil {
			continue
		}
		var entry struct {
			Event struct {
				Client string `json:"client"`
			} `json:"event"`
		}
		if err := json.Unmarshal(data, &entry); err == nil && entry.Event.Client != "" {
			return entry.Event.Client
		}
	}
	return ""
}

// 📄️ExtractSessionSecond reads the earliest second from session event files.
func ExtractSessionSecond(sessionDir string) string {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta model.SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil && meta.Second != "" {
			return meta.Second
		}
		if err := json.Unmarshal(data, &meta); err == nil {
			var earliest string
			for _, item := range meta.Events {
				var evt map[string]interface{}
				if err := json.Unmarshal(item.Event, &evt); err == nil {
					if second, ok := evt["second"].(string); ok && second != "" {
						if earliest == "" || second < earliest {
							earliest = second
						}
					}
				}
			}
			if earliest != "" {
				return earliest
			}
		}
	}
	entries, err := os.ReadDir(sessionDir)
	if err != nil || len(entries) == 0 {
		return ""
	}
	var earliest string
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") || e.Name() == "session.json" {
			continue
		}
		data, err := os.ReadFile(filepath.Join(sessionDir, e.Name()))
		if err != nil {
			continue
		}
		var entry struct {
			Event struct {
				Second string `json:"second"`
			} `json:"event"`
		}
		if err := json.Unmarshal(data, &entry); err == nil && entry.Event.Second != "" {
			if earliest == "" || entry.Event.Second < earliest {
				earliest = entry.Event.Second
			}
		}
	}
	return earliest
}

// 💾️ExtractSessionCheckpoint reads the checkpoint SHA from the session.json file.
// 💾️ExtractSessionCheckpoint MUST return the checkpoint SHA stored in session.json, or empty string if not present.
func ExtractSessionCheckpoint(sessionDir string) string {
	metaPath := filepath.Join(sessionDir, "session.json")
	if data, err := os.ReadFile(metaPath); err == nil {
		var meta model.SessionMeta
		if err := json.Unmarshal(data, &meta); err == nil {
			return meta.Checkpoint
		}
	}
	return ""
}

// 📂️StreamSessions streams all sessions found in the events directory.
func StreamSessions(ctx context.Context, out chan<- Session, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}
	agentEventsDir := filepath.Join(workspace.GetRepoMetaDir(), "⚡️cache", "🤖️generated")
	yearDirs, err := os.ReadDir(agentEventsDir)
	if err != nil {
		return nil
	}
	for _, yd := range yearDirs {
		if !yd.IsDir() {
			continue
		}
		yy, err := strconv.Atoi(yd.Name())
		if err != nil {
			continue
		}
		monthDirs, err := os.ReadDir(filepath.Join(agentEventsDir, yd.Name()))
		if err != nil {
			continue
		}
		for _, md := range monthDirs {
			if !md.IsDir() {
				continue
			}
			mm, err := strconv.Atoi(md.Name())
			if err != nil {
				continue
			}
			dayDirs, err := os.ReadDir(filepath.Join(agentEventsDir, yd.Name(), md.Name()))
			if err != nil {
				continue
			}
			for _, dd := range dayDirs {
				if !dd.IsDir() {
					continue
				}
				dayNum, err := strconv.Atoi(dd.Name())
				if err != nil {
					continue
				}
				sessionDirs, err := os.ReadDir(filepath.Join(agentEventsDir, yd.Name(), md.Name(), dd.Name()))
				if err != nil {
					continue
				}
				for _, sd := range sessionDirs {
					if !sd.IsDir() {
						continue
					}
					sessionDir := filepath.Join(agentEventsDir, yd.Name(), md.Name(), dd.Name(), sd.Name())
					kind := DeriveSessionKind(sessionDir)
					client := ExtractSessionClient(sessionDir)
					startedAt := ExtractSessionSecond(sessionDir)
					checkpoint := ExtractSessionCheckpoint(sessionDir)
					s := Session{
						UUID:       sd.Name(),
						Year:       yy,
						Month:      mm,
						Day:        dayNum,
						Checkpoint: checkpoint,
						Kind:       kind,
						Client:     client,
						StartedAt:  startedAt,
					}
					if !model.MatchesFilter(s.UUID, options) && !model.MatchesFilter(string(s.Kind), options) && !model.MatchesFilter(s.Client, options) {
						continue
					}
					if !model.MatchesQuery(s.UUID+" "+string(s.Kind)+" "+s.Client, options) {
						continue
					}
					select {
					case <-ctx.Done():
						return ctx.Err()
					case out <- s:
					}
				}
			}
		}
	}
	return nil
}

// #endregion 🪅️Sessions

// #region 🔭️Missing Hook Functions

// 🎫️currentTicketSessionID returns the current ticket session ID.
func CurrentTicketSessionID() string {
	if TestSessionIDOverride != "" {
		return TestSessionIDOverride
	}
	return GenerateHookSessionID()
}

// #endregion 🔭️Missing Hook Functions

// #region 💾️Missing Utility Functions

// 🪪️testSessionIDOverride is a test override for session ID generation.
var TestSessionIDOverride string

// #endregion 💾️Missing Utility Functions

// #region 📰️Todos

// #endregion 🪨️Missing Hook Functions
func GenerateHookSessionID() string {
	return identity.New().String()
}

// #endregion 📰️Todos

// #region 🔌️Contributor Ports

// 🔌️init wires the contributor reads 📐️model and 🗂️codebase declare but cannot implement.
func init() {
	model.LookupContributors = func() ([]model.Contributor, error) { return hostContributors(), nil }
	codebase.LookupCodebaseContributors = BuildCodebaseContributors
}

// #endregion 🔌️Contributor Ports

// #endregion 🚚️Split

// #region 🚪️Ports

// #region ✍️AuthorShapes

// ✍️GitAuthor is a git author line split into its parts.
type GitAuthor struct {
	Name   string `json:"name,omitempty"`
	Email  string `json:"email,omitempty"`
	Github string `json:"github,omitempty"`
}

// 🔤️String renders the author back as `Name <email>`, or as the bare name without an email.
func (author GitAuthor) String() string {
	if author.Email == "" {
		return author.Name
	}
	return author.Name + " <" + author.Email + ">"
}

// ✂️ParseGitAuthor splits a `Name <email>` author line. A line without ` <` is all name.
func ParseGitAuthor(line string) GitAuthor {
	index := strings.Index(line, " <")
	if index < 0 {
		return GitAuthor{Name: line}
	}
	rest := line[index+2:]
	return GitAuthor{Name: strings.TrimSpace(line[:index]), Email: strings.TrimSuffix(rest, ">")}
}

// 🤝️ParseContributorIdentity splits a git shortlog or git log identity line into its name and
// email. The line must carry a four digit run before the name and an angle bracketed email.
func ParseContributorIdentity(line string) (string, string, bool) {
	characters := []rune(line)
	for start := range characters {
		if !runOfFourDigits(characters, start) {
			continue
		}
		afterDigits := start + 4
		cursor := afterDigits
		for cursor < len(characters) && unicode.IsSpace(characters[cursor]) {
			cursor++
		}
		if cursor == afterDigits {
			continue
		}
		open := -1
		for index := cursor; index < len(characters); index++ {
			if characters[index] == '<' {
				open = index
				break
			}
		}
		if open < 0 {
			continue
		}
		closing := -1
		for index := open + 1; index < len(characters); index++ {
			if characters[index] == '>' {
				closing = index
				break
			}
		}
		if closing < 0 {
			continue
		}
		name := string(characters[cursor:open])
		email := string(characters[open+1 : closing])
		if strings.TrimSpace(name) == "" || email == "" {
			continue
		}
		return strings.TrimSpace(name), strings.TrimSpace(email), true
	}
	return "", "", false
}

// 🔢️runOfFourDigits reports whether exactly four digits start at an index.
func runOfFourDigits(characters []rune, start int) bool {
	if start+4 > len(characters) {
		return false
	}
	if start > 0 && isASCIIDigit(characters[start-1]) {
		return false
	}
	for index := start; index < start+4; index++ {
		if !isASCIIDigit(characters[index]) {
			return false
		}
	}
	return start+4 >= len(characters) || !isASCIIDigit(characters[start+4])
}

// 🔢️isASCIIDigit reports whether a rune is a decimal digit.
func isASCIIDigit(value rune) bool { return value >= '0' && value <= '9' }

// #endregion ✍️AuthorShapes

// #region 🧑️ContributorStore

// 📄️StoredContributor is one directory name and document pair of a contributor store.
type StoredContributor struct {
	Directory string `json:"directory"`
	Document  string `json:"document"`
}

// 🗄️ContributorStore is where contributor documents live, one per directory.
type ContributorStore interface {
	// 📋️Documents returns every directory name and document pair, in directory name order.
	Documents() []StoredContributor
}

// 🧠️MemoryContributorStore is the in-memory store the language-agnostic tests run against.
type MemoryContributorStore struct {
	documents []StoredContributor
}

// 🌱️NewMemoryContributorStore seeds a store with directory name and document pairs.
func NewMemoryContributorStore(entries []StoredContributor) *MemoryContributorStore {
	documents := append([]StoredContributor{}, entries...)
	sort.SliceStable(documents, func(left, right int) bool { return documents[left].Directory < documents[right].Directory })
	return &MemoryContributorStore{documents: documents}
}

// 📋️Documents returns the seeded pairs in directory name order.
func (store *MemoryContributorStore) Documents() []StoredContributor {
	return append([]StoredContributor{}, store.documents...)
}

// 💽️FsContributorStore is the store over a real devs tree of a repository root.
type FsContributorStore struct {
	root string
}

// 🆕️NewFsContributorStore builds the store of the devs directory of a repository root.
func NewFsContributorStore(repoRoot string) *FsContributorStore {
	return &FsContributorStore{root: workspace.DevsDirForRoot(repoRoot)}
}

// 📋️Documents reads every contributor document of the devs directory, in directory name order.
func (store *FsContributorStore) Documents() []StoredContributor {
	entries, err := os.ReadDir(store.root)
	if err != nil {
		return nil
	}
	var found []StoredContributor
	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		data, err := os.ReadFile(filepath.Join(store.root, entry.Name(), "🧑️‍💻️contributor.json"))
		if err != nil {
			continue
		}
		found = append(found, StoredContributor{Directory: entry.Name(), Document: string(data)})
	}
	sort.SliceStable(found, func(left, right int) bool { return found[left].Directory < found[right].Directory })
	return found
}

// 🏠️hostContributors returns every contributor of the repository the process runs in.
func hostContributors() []model.Contributor {
	return ListContributors(NewFsContributorStore(workspace.GetRootDir()))
}

// 📋️ListContributors returns every contributor of a store, with the directory name standing in
// for a blank alias or handle. An empty store yields the single unknown contributor.
func ListContributors(store ContributorStore) []model.Contributor {
	documents := store.Documents()
	if len(documents) == 0 {
		return []model.Contributor{{Alias: "unknown", Github: "unknown", Name: "Unknown"}}
	}
	result := make([]model.Contributor, 0, len(documents))
	for _, entry := range documents {
		var contributor model.Contributor
		if err := json.Unmarshal([]byte(entry.Document), &contributor); err != nil {
			continue
		}
		if contributor.Alias == "" {
			contributor.Alias = entry.Directory
		}
		if contributor.Github == "" {
			contributor.Github = entry.Directory
		}
		result = append(result, contributor)
	}
	return result
}

// 🔎️SearchContributorsFrom returns every contributor whose alias, name or handle contains the term.
func SearchContributorsFrom(store ContributorStore, term string) []model.Contributor {
	needle := strings.ToLower(term)
	found := make([]model.Contributor, 0)
	for _, contributor := range ListContributors(store) {
		haystack := strings.ToLower(contributor.Alias + " " + contributor.Name + " " + contributor.Github)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, contributor)
		}
	}
	return found
}

// 🤝️ResolveAuthorToAlias returns the alias of the contributor an author line belongs to: an email
// match wins over a name match, both case-insensitive over the primary value and every recorded
// alternative, and an author nobody claims falls back to the author line itself.
func ResolveAuthorToAlias(store ContributorStore, name, email string) string {
	for _, contributor := range ListContributors(store) {
		if email != "" && containsFold(append([]string{contributor.Email}, contributor.Emails...), email) {
			return contributor.Alias
		}
		if name != "" && containsFold(append([]string{contributor.Name}, contributor.Names...), name) {
			return contributor.Alias
		}
	}
	switch {
	case name != "" && email != "":
		return name + " <" + email + ">"
	case name != "":
		return name
	case email != "":
		return email
	}
	return ""
}

// 🔎️FindContributor returns the alias an author line resolves to, unknown when it names nobody.
func FindContributor(store ContributorStore, authorLine string) string {
	parsed := ParseGitAuthor(authorLine)
	if parsed.Name == "" && parsed.Email == "" {
		return "unknown"
	}
	return ResolveAuthorToAlias(store, parsed.Name, parsed.Email)
}

// 🔠️containsFold reports whether a candidate equals a value, case-insensitively.
func containsFold(candidates []string, value string) bool {
	for _, candidate := range candidates {
		if strings.EqualFold(candidate, value) {
			return true
		}
	}
	return false
}

// #endregion 🧑️ContributorStore

// #region 🪅️SessionSource

// 🔑️SessionKey states where one session sits.
type SessionKey struct {
	Year  int64  `json:"year"`
	Month int64  `json:"month"`
	Day   int64  `json:"day"`
	UUID  string `json:"uuid"`
}

// 📦️MemorySession holds the files and age of one in-memory session.
type MemorySession struct {
	Files      map[string]string `json:"files"`
	AgeSeconds *int64            `json:"ageSeconds"`
}

// 📥️MemorySessionEntry pairs a key with its in-memory session.
type MemorySessionEntry struct {
	Key     SessionKey
	Session MemorySession
}

// 📂️SessionSource is a directory of session event files addressed by date and identifier.
type SessionSource interface {
	// 📋️Sessions returns every key the source carries, in ascending order.
	Sessions() []SessionKey
	// 📄️Meta returns the session.json document of one session.
	Meta(key SessionKey) (string, bool)
	// 📁️Entries returns the event file names of one session, in directory order.
	Entries(key SessionKey) []string
	// 📄️Entry returns one event file of one session.
	Entry(key SessionKey, name string) (string, bool)
	// 🕰️AgeSeconds returns how many seconds ago the newest file of one session was written.
	AgeSeconds(key SessionKey) (int64, bool)
}

// 🧠️MemorySessionSource is the in-memory source: a session is a name, a set of files and an age.
type MemorySessionSource struct {
	keys     []SessionKey
	sessions map[SessionKey]MemorySession
}

// 🌱️NewMemorySessionSource seeds a source with sessions, ordered by key.
func NewMemorySessionSource(entries []MemorySessionEntry) *MemorySessionSource {
	source := &MemorySessionSource{sessions: map[SessionKey]MemorySession{}}
	for _, entry := range entries {
		if _, seen := source.sessions[entry.Key]; !seen {
			source.keys = append(source.keys, entry.Key)
		}
		source.sessions[entry.Key] = entry.Session
	}
	sort.SliceStable(source.keys, func(left, right int) bool { return lessSessionKey(source.keys[left], source.keys[right]) })
	return source
}

// 🔢️lessSessionKey orders keys by year, month, day and then identifier.
func lessSessionKey(left, right SessionKey) bool {
	if left.Year != right.Year {
		return left.Year < right.Year
	}
	if left.Month != right.Month {
		return left.Month < right.Month
	}
	if left.Day != right.Day {
		return left.Day < right.Day
	}
	return left.UUID < right.UUID
}

// 📋️Sessions returns every seeded key in ascending order.
func (source *MemorySessionSource) Sessions() []SessionKey {
	return append([]SessionKey{}, source.keys...)
}

// 📄️Meta returns the session.json document of one session.
func (source *MemorySessionSource) Meta(key SessionKey) (string, bool) {
	return source.Entry(key, "session.json")
}

// 📁️Entries returns the event file names of one session, in name order.
func (source *MemorySessionSource) Entries(key SessionKey) []string {
	session, found := source.sessions[key]
	if !found {
		return nil
	}
	names := make([]string, 0, len(session.Files))
	for name := range session.Files {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// 📄️Entry returns one event file of one session.
func (source *MemorySessionSource) Entry(key SessionKey, name string) (string, bool) {
	session, found := source.sessions[key]
	if !found {
		return "", false
	}
	content, found := session.Files[name]
	return content, found
}

// 🕰️AgeSeconds returns how many seconds ago the newest file of one session was written.
func (source *MemorySessionSource) AgeSeconds(key SessionKey) (int64, bool) {
	session, found := source.sessions[key]
	if !found || session.AgeSeconds == nil {
		return 0, false
	}
	return *session.AgeSeconds, true
}

// 💽️FsSessionSource is the source over a real agent event directory.
type FsSessionSource struct {
	root  string
	clock Clock
}

// 🆕️NewFsSessionSource builds the source of an agent event directory.
func NewFsSessionSource(root string, clock Clock) *FsSessionSource {
	if clock == nil {
		clock = SystemClock{}
	}
	return &FsSessionSource{root: root, clock: clock}
}

// 📁️directory returns the absolute directory of one session.
func (source *FsSessionSource) directory(key SessionKey) string {
	return filepath.Join(source.root, fmt.Sprintf("%d", key.Year), fmt.Sprintf("%d", key.Month), fmt.Sprintf("%d", key.Day), key.UUID)
}

// 📋️Sessions walks the year, month, day and session directories of the source.
func (source *FsSessionSource) Sessions() []SessionKey {
	var keys []SessionKey
	years, err := os.ReadDir(source.root)
	if err != nil {
		return nil
	}
	for _, year := range years {
		yearNumber, err := strconv.ParseInt(year.Name(), 10, 64)
		if !year.IsDir() || err != nil {
			continue
		}
		months, err := os.ReadDir(filepath.Join(source.root, year.Name()))
		if err != nil {
			continue
		}
		for _, month := range months {
			monthNumber, err := strconv.ParseInt(month.Name(), 10, 64)
			if !month.IsDir() || err != nil {
				continue
			}
			days, err := os.ReadDir(filepath.Join(source.root, year.Name(), month.Name()))
			if err != nil {
				continue
			}
			for _, day := range days {
				dayNumber, err := strconv.ParseInt(day.Name(), 10, 64)
				if !day.IsDir() || err != nil {
					continue
				}
				sessions, err := os.ReadDir(filepath.Join(source.root, year.Name(), month.Name(), day.Name()))
				if err != nil {
					continue
				}
				for _, session := range sessions {
					if !session.IsDir() {
						continue
					}
					keys = append(keys, SessionKey{Year: yearNumber, Month: monthNumber, Day: dayNumber, UUID: session.Name()})
				}
			}
		}
	}
	sort.SliceStable(keys, func(left, right int) bool { return lessSessionKey(keys[left], keys[right]) })
	return keys
}

// 📄️Meta reads the session.json document of one session.
func (source *FsSessionSource) Meta(key SessionKey) (string, bool) {
	return source.Entry(key, "session.json")
}

// 📁️Entries lists the event file names of one session.
func (source *FsSessionSource) Entries(key SessionKey) []string {
	entries, err := os.ReadDir(source.directory(key))
	if err != nil {
		return nil
	}
	names := make([]string, 0, len(entries))
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	sort.Strings(names)
	return names
}

// 📄️Entry reads one event file of one session.
func (source *FsSessionSource) Entry(key SessionKey, name string) (string, bool) {
	data, err := os.ReadFile(filepath.Join(source.directory(key), name))
	if err != nil {
		return "", false
	}
	return string(data), true
}

// 🕰️AgeSeconds returns how many seconds ago the newest file of one session was written.
func (source *FsSessionSource) AgeSeconds(key SessionKey) (int64, bool) {
	entries, err := os.ReadDir(source.directory(key))
	if err != nil || len(entries) == 0 {
		return 0, false
	}
	var latest time.Time
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		if info.ModTime().After(latest) {
			latest = info.ModTime()
		}
	}
	if latest.IsZero() {
		return 0, false
	}
	return source.clock.NowSeconds() - latest.Unix(), true
}

// ⏱️RunningWindowSeconds is how long a session may stay silent before it stops counting as running.
const RunningWindowSeconds int64 = 30 * 60

// 🕵️AgentEndedKind is the event kind whose presence ends a session.
const AgentEndedKind = "agent.ended"

// 📡️DeriveSessionKindFrom decides what became of a session from its own files.
func DeriveSessionKindFrom(source SessionSource, key SessionKey) SessionKind {
	if document, found := source.Meta(key); found {
		var meta model.SessionMeta
		if err := json.Unmarshal([]byte(document), &meta); err == nil {
			for _, entry := range meta.Events {
				var event map[string]interface{}
				if err := json.Unmarshal(entry.Event, &event); err != nil {
					continue
				}
				if kind, ok := event["kind"].(string); ok && kind == AgentEndedKind {
					return SessionKindCompleted
				}
			}
			if age, aged := source.AgeSeconds(key); aged && age < RunningWindowSeconds {
				return SessionKindRunning
			}
			if len(meta.Events) > 0 {
				return SessionKindInterrupted
			}
		}
	}
	entries := source.Entries(key)
	for _, name := range entries {
		if strings.Contains(name, "agent-ended") {
			return SessionKindCompleted
		}
	}
	if len(entries) == 0 {
		return SessionKindInterrupted
	}
	if age, aged := source.AgeSeconds(key); aged && age < RunningWindowSeconds {
		return SessionKindRunning
	}
	return SessionKindInterrupted
}

// 💻️ExtractSessionClientFrom returns the client of a session by the three-step fallback.
func ExtractSessionClientFrom(source SessionSource, key SessionKey) string {
	if document, found := source.Meta(key); found {
		var meta model.SessionMeta
		if err := json.Unmarshal([]byte(document), &meta); err == nil {
			if meta.Client != "" {
				return meta.Client
			}
			for _, entry := range meta.Events {
				var event map[string]interface{}
				if err := json.Unmarshal(entry.Event, &event); err != nil {
					continue
				}
				if client, ok := event["client"].(string); ok && client != "" {
					return client
				}
			}
		}
	}
	for _, name := range source.Entries(key) {
		if name == "session.json" || !strings.HasSuffix(name, ".json") {
			continue
		}
		document, found := source.Entry(key, name)
		if !found {
			continue
		}
		var entry struct {
			Event struct {
				Client string `json:"client"`
			} `json:"event"`
		}
		if err := json.Unmarshal([]byte(document), &entry); err == nil && entry.Event.Client != "" {
			return entry.Event.Client
		}
	}
	return ""
}

// 📄️ExtractSessionSecondFrom returns the earliest second a session recorded.
func ExtractSessionSecondFrom(source SessionSource, key SessionKey) string {
	if document, found := source.Meta(key); found {
		var meta model.SessionMeta
		if err := json.Unmarshal([]byte(document), &meta); err == nil {
			if meta.Second != "" {
				return meta.Second
			}
			earliest := ""
			for _, entry := range meta.Events {
				var event map[string]interface{}
				if err := json.Unmarshal(entry.Event, &event); err != nil {
					continue
				}
				if second, ok := event["second"].(string); ok && second != "" && (earliest == "" || second < earliest) {
					earliest = second
				}
			}
			if earliest != "" {
				return earliest
			}
		}
	}
	earliest := ""
	for _, name := range source.Entries(key) {
		if name == "session.json" || !strings.HasSuffix(name, ".json") {
			continue
		}
		document, found := source.Entry(key, name)
		if !found {
			continue
		}
		var entry struct {
			Event struct {
				Second string `json:"second"`
			} `json:"event"`
		}
		if err := json.Unmarshal([]byte(document), &entry); err == nil && entry.Event.Second != "" && (earliest == "" || entry.Event.Second < earliest) {
			earliest = entry.Event.Second
		}
	}
	return earliest
}

// 💾️ExtractSessionCheckpointFrom returns the checkpoint a session recorded in its session.json.
func ExtractSessionCheckpointFrom(source SessionSource, key SessionKey) string {
	document, found := source.Meta(key)
	if !found {
		return ""
	}
	var meta model.SessionMeta
	if err := json.Unmarshal([]byte(document), &meta); err != nil {
		return ""
	}
	return meta.Checkpoint
}

// 📂️ListSessions assembles every session the source carries from its own files.
func ListSessions(source SessionSource) []Session {
	keys := source.Sessions()
	sessions := make([]Session, 0, len(keys))
	for _, key := range keys {
		sessions = append(sessions, Session{
			UUID:       key.UUID,
			Year:       int(key.Year),
			Month:      int(key.Month),
			Day:        int(key.Day),
			Checkpoint: ExtractSessionCheckpointFrom(source, key),
			Kind:       DeriveSessionKindFrom(source, key),
			Client:     ExtractSessionClientFrom(source, key),
			StartedAt:  ExtractSessionSecondFrom(source, key),
		})
	}
	return sessions
}

// 🔎️SearchSessions returns every session whose identifier, kind or client contains the term.
func SearchSessions(source SessionSource, term string) []Session {
	needle := strings.ToLower(term)
	found := make([]Session, 0)
	for _, session := range ListSessions(source) {
		haystack := strings.ToLower(session.UUID + " " + string(session.Kind) + " " + session.Client)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, session)
		}
	}
	return found
}

// 🪪️ID returns the artifact identifier of a session: the checkpoint when it recorded one,
// otherwise the year, month and day it sits under, followed by the session own segment.
func (s Session) ID() string {
	parent := ""
	if s.Checkpoint == "" {
		parent = segment("year", fmt.Sprintf("%02d", s.Year), "")
		parent = segment("month", fmt.Sprintf("%02d", s.Month), parent)
		parent = segment("day", fmt.Sprintf("%02d", s.Day), parent)
	} else {
		parent = segment("checkpoint", s.Checkpoint, "")
	}
	return segment("session", workspace.Flat(s.UUID), parent)
}

// 🔗️URI returns the artifact URI of a session, which carries the session segment alone.
func (s Session) URI() string {
	return "repo://session/" + segment("session", workspace.Flat(s.UUID), "")
}

// 🧱️segment appends one emoji-tagged identifier segment to a parent identifier.
func segment(entity, value, parent string) string {
	return parent + identity.SemanticId{Emoji: identity.Entity(entity), Value: value}.String()
}

// #endregion 🪅️SessionSource

// #region 🏁️CheckpointSource

// 📜️CheckpointLogFormat is the pretty format the checkpoint log is read in.
const CheckpointLogFormat = "%H|%aN|%ad|%s"

// 🏁️Checkpoint is one recorded checkpoint of the repository history, the shape 📐️model declares
// and the Rust twin reexports; the date is the `--date=iso-strict` text the log carries.
type Checkpoint = model.Checkpoint

// 📖️CheckpointSource is where the checkpoint log comes from.
type CheckpointSource interface {
	// 📜️Log returns the checkpoint log, at most limit entries when a limit is given.
	Log(limit *int) string
}

// 🧠️MemoryCheckpointSource serves a frozen log.
type MemoryCheckpointSource struct {
	log string
}

// 🌱️NewMemoryCheckpointSource builds a source that serves this log.
func NewMemoryCheckpointSource(log string) *MemoryCheckpointSource {
	return &MemoryCheckpointSource{log: log}
}

// 📜️Log returns the frozen log, truncated to a limit when one is given.
func (source *MemoryCheckpointSource) Log(limit *int) string {
	if limit == nil {
		return source.log
	}
	lines := strings.Split(source.log, "\n")
	if *limit < len(lines) {
		lines = lines[:*limit]
	}
	return strings.Join(lines, "\n")
}

// 💽️FsCheckpointSource reads the log out of a real git repository.
type FsCheckpointSource struct {
	root string
}

// 🆕️NewFsCheckpointSource builds the source of a repository root.
func NewFsCheckpointSource(root string) *FsCheckpointSource {
	return &FsCheckpointSource{root: root}
}

// 📜️Log runs git log under the pretty format the checkpoint reader expects.
func (source *FsCheckpointSource) Log(limit *int) string {
	args := []string{"log", "--pretty=format:" + CheckpointLogFormat, "--date=iso-strict"}
	if limit != nil {
		args = append(args, fmt.Sprintf("-n%d", *limit))
	}
	command := exec.Command("git", args...)
	command.Dir = source.root
	out, err := command.Output()
	if err != nil {
		return ""
	}
	return string(out)
}

// 📥️ParseCheckpointLog parses a checkpoint log. A line needs four separated parts; the title
// keeps every separator it contains, and the author becomes the checkpoint author identifier.
func ParseCheckpointLog(log string) []Checkpoint {
	found := make([]Checkpoint, 0)
	for _, line := range strings.Split(log, "\n") {
		parts := strings.Split(line, "|")
		if len(parts) < 4 {
			continue
		}
		var author *string
		if parts[1] != "" {
			value := parts[1]
			author = &value
		}
		found = append(found, Checkpoint{ID: parts[0], SHA: parts[0], Title: strings.Join(parts[3:], "|"), AuthorID: author, Date: parts[2]})
	}
	return found
}

// 📋️ListCheckpoints returns every checkpoint the source carries, newest first.
func ListCheckpoints(source CheckpointSource, limit *int) []Checkpoint {
	return ParseCheckpointLog(source.Log(limit))
}

// 🔎️SearchCheckpoints returns every checkpoint whose sha or title contains the term.
func SearchCheckpoints(source CheckpointSource, limit *int, term string) []Checkpoint {
	needle := strings.ToLower(term)
	found := make([]Checkpoint, 0)
	for _, checkpoint := range ListCheckpoints(source, limit) {
		haystack := strings.ToLower(checkpoint.SHA + " " + checkpoint.Title)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, checkpoint)
		}
	}
	return found
}

// 🪪️CheckpointID returns the artifact identifier of a checkpoint: the contributor segment, when
// the checkpoint names an author, followed by the checkpoint segment carrying the full sha.
func CheckpointID(checkpoint Checkpoint) string {
	contributor := ""
	if checkpoint.AuthorID != nil && *checkpoint.AuthorID != "" {
		contributor = segment("contributor", workspace.Flat(*checkpoint.AuthorID), "")
	}
	return segment("checkpoint", checkpoint.SHA, contributor)
}

// #endregion 🏁️CheckpointSource

// #region 🎁️Interactions

// 🎁️InteractionOrigin states where an interaction was recorded.
type InteractionOrigin struct {
	SourceKind string `json:"sourceKind"`
	SourceID   string `json:"sourceId"`
	GoalID     string `json:"goalId,omitempty"`
	TicketID   string `json:"ticketId,omitempty"`
}

// ✍️InteractionsAuthor returns the first interaction author that is not blank.
func InteractionsAuthor(interactions []model.Interaction) string {
	for _, interaction := range interactions {
		if interaction.Author != "" {
			return interaction.Author
		}
	}
	return ""
}

// 💾️InteractionsCheckpoint returns the first interaction checkpoint that is not blank.
func InteractionsCheckpoint(interactions []model.Interaction) string {
	for _, interaction := range interactions {
		if interaction.Checkpoint != "" {
			return interaction.Checkpoint
		}
	}
	return ""
}

// 🧑️AttributedInteraction pairs an origin with the alias its interaction resolved to.
type AttributedInteraction struct {
	Origin InteractionOrigin `json:"origin"`
	Alias  string            `json:"alias"`
}

// 🧑️AttributeInteractions resolves the interactions of one origin to contributor aliases.
func AttributeInteractions(store ContributorStore, origin InteractionOrigin, interactions []model.Interaction) []AttributedInteraction {
	attributed := make([]AttributedInteraction, 0, len(interactions))
	for _, interaction := range interactions {
		parsed := ParseGitAuthor(interaction.Author)
		attributed = append(attributed, AttributedInteraction{Origin: origin, Alias: ResolveAuthorToAlias(store, parsed.Name, parsed.Email)})
	}
	return attributed
}

// #endregion 🎁️Interactions

// #region 🕰️Clock

// 🕰️Clock is the instant a decision about elapsed time is made against.
type Clock interface {
	// ⏱️NowSeconds returns seconds since the Unix epoch.
	NowSeconds() int64
}

// 📌️FixedClock is a clock frozen at one instant.
type FixedClock struct {
	Instant int64
}

// ⏱️NowSeconds returns the frozen instant.
func (clock FixedClock) NowSeconds() int64 { return clock.Instant }

// 🕰️SystemClock is the clock of the machine this runs on.
type SystemClock struct{}

// ⏱️NowSeconds returns the current second since the Unix epoch.
func (SystemClock) NowSeconds() int64 { return time.Now().Unix() }

// #endregion 🕰️Clock

// #region 🧾️Recording

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

// #endregion 🧾️Recording

// #endregion 🚪️Ports
