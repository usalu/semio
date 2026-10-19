// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏁️checkpoints carries the checkpoint loading, streaming and the checkpoint log sources.

// #endregion 🧲️Header

package contributors

import (
	context "context"
	fmt "fmt"
	exec "os/exec"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🏁️Checkpoints

// ◼LoadCheckpoints MUST return all matching checkpoints from the data source.
// 🔢️LoadCheckpoints loads the checkpoints of the current repository root through the ported reader,
// so the author identifier the log carries reaches the checkpoint rather than being dropped.
func LoadCheckpoints(limit *int) []model.Checkpoint {
	return ListCheckpoints(NewFsCheckpointSource(workspace.GetRootDir()), limit)
}

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

// #endregion 🏁️Checkpoints
