// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎫️tickets is the tickets domain of the semio repository tooling, split out of the pre-split godfile.

// #endregion 🧲️Header

package tickets

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
	"github.com/usalu/semio/repo/identity"
	"github.com/usalu/semio/repo/providers"

)

// #region 🪪️Contracts

// 🐙️ManagementIssue is the issue shape of 🧩️providers, reexported so a client needs one import.
type ManagementIssue = providers.ManagementIssue

// 🎯️ManagementMilestone is the milestone shape of 🧩️providers.
type ManagementMilestone = providers.ManagementMilestone

// 🏷️ManagementLabel is the label shape of 🧩️providers.
type ManagementLabel = providers.ManagementLabel

// 🪪️McpClientKind is the MCP surface a plan or spec id is resolved against.
type McpClientKind = providers.McpClientKind

// 🐙️IssueTracker is everything the ticket domain asks of an issue tracker.
type IssueTracker interface {
	// 🆕️CreateIssue opens an issue and returns its URL.
	CreateIssue(title, body string, milestone *int) (string, error)
	// 📪️CloseIssue closes an issue.
	CloseIssue(issueURL string) error
	// 🔓️ReopenIssue reopens an issue.
	ReopenIssue(issueURL string) error
	// 🔎️GetIssueDetails reads an issue, or nothing when it cannot be read.
	GetIssueDetails(issueURL string) (*ManagementIssue, error)
	// 💬️AddComment comments on an issue.
	AddComment(issueURL, comment string) error
	// 🏷️AddLabels labels an issue.
	AddLabels(issueURL string, labels []string) error
	// 🏷️RemoveLabels unlabels an issue.
	RemoveLabels(issueURL string, labels []string) error
	// 🎯️FindMilestoneByTitle looks a milestone up by title.
	FindMilestoneByTitle(title string) (*ManagementMilestone, error)
	// 🏷️ListRepoLabels lists the labels of the repository.
	ListRepoLabels() ([]ManagementLabel, error)
}

// ⚠️TicketError is every ticket failure, carrying a class for machine consumers and a message.
type TicketError struct {
	Class   string `json:"class"`
	Message string `json:"message"`
}

// 🔤️Error renders the failure message.
func (err TicketError) Error() string { return err.Message }

// 🏷️TicketErrorClass returns the class of any failure this domain produced.
func TicketErrorClass(err error) string {
	if err == nil {
		return ""
	}
	var ticketError TicketError
	if errors.As(err, &ticketError) {
		return ticketError.Class
	}
	return "store"
}

// 🚫️errInvalid refuses an input the caller may correct.
func errInvalid(message string) error { return TicketError{Class: "invalid", Message: message} }

// 🔍️errTicketNotFound refuses a ticket, document or node that is not there.
func errTicketNotFound(message string) error {
	return TicketError{Class: "not-found", Message: message}
}

// 💥️errStore refuses a store operation that failed.
func errStore(message string) error { return TicketError{Class: "store", Message: message} }

// 🔀️errConflict refuses a state that forbids the requested transition.
func errConflict(message string) error { return TicketError{Class: "conflict", Message: message} }

// 🎆️EmojiYear is the year directory prefix.
const EmojiYear = "🎆️"

// 🌙️EmojiMonth is the month directory prefix.
const EmojiMonth = "🌙️"

// ☀️EmojiDay is the day directory prefix.
const EmojiDay = "☀️"

// 🎫️TicketsDirName is the tickets collection directory, under the repository meta directory.
const TicketsDirName = "🎫️tickets"

// 🎫️TicketDocumentName is the persisted ticket document.
const TicketDocumentName = "🎫️ticket.json"

// 📌️ImportantDirName is the directory holding the important document of a ticket.
const ImportantDirName = "📌️important"

// 📝️ImportantDocumentName is the important document itself.
const ImportantDocumentName = "📝️.md"

// 🚷️SkippedWalkDirs are the directory names a ticket walk never descends into.
var SkippedWalkDirs = []string{"node_modules", "dist", "build", "target", "__pycache__"}

// 🔢️PadNumber left-pads a number with zeroes to a width.
func PadNumber(value int, width int) string {
	negative := value < 0
	magnitude := value
	if negative {
		magnitude = -value
	}
	digits := strconv.Itoa(magnitude)
	padding := width - len(digits)
	if negative {
		padding--
	}
	var builder strings.Builder
	if negative {
		builder.WriteByte('-')
	}
	for index := 0; index < padding; index++ {
		builder.WriteByte('0')
	}
	builder.WriteString(digits)
	return builder.String()
}

// 🎆️FormatYearDir returns the emoji-prefixed year directory segment.
func FormatYearDir(year int) string { return EmojiYear + PadNumber(year, 2) }

// 🌙️FormatMonthDir returns the emoji-prefixed month directory segment.
func FormatMonthDir(month int) string { return EmojiMonth + PadNumber(month, 2) }

// ☀️FormatDayDir returns the emoji-prefixed day directory segment.
func FormatDayDir(day int) string { return EmojiDay + PadNumber(day, 2) }

// 🧭️ParseDatedDirSegment parses one canonical emoji-prefixed date directory segment.
func ParseDatedDirSegment(segment, prefix string) (int, error) {
	if !strings.HasPrefix(segment, prefix) {
		return 0, errInvalid(fmt.Sprintf("date directory %q is missing prefix %q", segment, prefix))
	}
	value, err := strconv.Atoi(segment[len(prefix):])
	if err != nil {
		return 0, errInvalid(fmt.Sprintf("invalid date directory %q", segment))
	}
	return value, nil
}

// 🎫️TicketID is the dated identity of a ticket, in both of its spellings.
type TicketID struct {
	Year  int    `json:"year"`
	Month int    `json:"month"`
	Day   int    `json:"day"`
	Slug  string `json:"slug"`
}

// 🆕️NewTicketID builds an identity from its parts.
func NewTicketID(year, month, day int, slug string) TicketID {
	return TicketID{Year: year, Month: month, Day: day, Slug: slug}
}

// 🪪️ID returns the logical identity a dev types and a URI carries.
func (id TicketID) ID() string {
	return fmt.Sprintf("%s/%s/%s/%s", PadNumber(id.Year, 2), PadNumber(id.Month, 2), PadNumber(id.Day, 2), id.Slug)
}

// 🗂️RelPath returns the relative folder path under the tickets directory.
func (id TicketID) RelPath() string {
	return fmt.Sprintf("%s/%s/%s/%s", FormatYearDir(id.Year), FormatMonthDir(id.Month), FormatDayDir(id.Day), id.Slug)
}

// 🌐️URI returns the artifact URI of the ticket.
func (id TicketID) URI() string { return "repo://ticket/" + id.RelPath() }

// 👪️ParentSlug returns the slug of the parent ticket when this one is nested.
func (id TicketID) ParentSlug() (string, bool) {
	index := strings.LastIndex(id.Slug, "/")
	if index < 0 {
		return "", false
	}
	return id.Slug[:index], true
}

// 🧭️ParseTicketID reads either spelling back. A slug may itself carry a separator.
func ParseTicketID(value string) (TicketID, error) {
	trimmed := strings.Trim(strings.TrimSpace(value), "/")
	segments := strings.Split(trimmed, "/")
	if len(segments) < 4 {
		return TicketID{}, errInvalid(fmt.Sprintf("ticket id %q must be YY/MM/DD/SLUG", value))
	}
	read := func(segment, prefix string) (int, error) {
		if strings.HasPrefix(segment, prefix) {
			return ParseDatedDirSegment(segment, prefix)
		}
		parsed, err := strconv.Atoi(segment)
		if err != nil {
			return 0, errInvalid(fmt.Sprintf("invalid date segment %q", segment))
		}
		return parsed, nil
	}
	year, err := read(segments[0], EmojiYear)
	if err != nil {
		return TicketID{}, err
	}
	month, err := read(segments[1], EmojiMonth)
	if err != nil {
		return TicketID{}, err
	}
	day, err := read(segments[2], EmojiDay)
	if err != nil {
		return TicketID{}, err
	}
	slug := strings.Join(segments[3:], "/")
	if slug == "" {
		return TicketID{}, errInvalid(fmt.Sprintf("ticket id %q carries no slug", value))
	}
	return TicketID{Year: year, Month: month, Day: day, Slug: slug}, nil
}

// 🗺️TicketLayout derives every path the ticket domain owns from one repository meta directory.
type TicketLayout struct {
	repoMetaDir string
}

// 🆕️NewTicketLayout anchors the layout at a repository meta directory.
func NewTicketLayout(repoMetaDir string) TicketLayout {
	return TicketLayout{repoMetaDir: NormalizeSeparators(repoMetaDir)}
}

// 🏠️RepoMetaDir returns the meta directory the layout is anchored at.
func (layout TicketLayout) RepoMetaDir() string { return layout.repoMetaDir }

// 🎫️TicketsDir returns the tickets collection directory.
func (layout TicketLayout) TicketsDir() string { return JoinPath(layout.repoMetaDir, TicketsDirName) }

// 🎆️YearDir returns the directory of one year.
func (layout TicketLayout) YearDir(year int) string {
	return JoinPath(layout.TicketsDir(), FormatYearDir(year))
}

// 🌙️MonthDir returns the directory of one month.
func (layout TicketLayout) MonthDir(year, month int) string {
	return JoinPath(layout.YearDir(year), FormatMonthDir(month))
}

// ☀️DayDir returns the directory of one day.
func (layout TicketLayout) DayDir(year, month, day int) string {
	return JoinPath(layout.MonthDir(year, month), FormatDayDir(day))
}

// 🗂️TicketDir returns the folder of one ticket.
func (layout TicketLayout) TicketDir(id TicketID) string {
	return JoinPath(layout.TicketsDir(), id.RelPath())
}

// 🎫️DocumentPath returns the persisted document of one ticket.
func (layout TicketLayout) DocumentPath(id TicketID) string {
	return JoinPath(layout.TicketDir(id), TicketDocumentName)
}

// 📌️ImportantDir returns the important directory of one ticket.
func (layout TicketLayout) ImportantDir(id TicketID) string {
	return JoinPath(layout.TicketDir(id), ImportantDirName)
}

// 📝️ImportantPath returns the important document of one ticket.
func (layout TicketLayout) ImportantPath(id TicketID) string {
	return JoinPath(layout.ImportantDir(id), ImportantDocumentName)
}

// ➗️NormalizeSeparators replaces every backslash with a forward slash and collapses repeats.
func NormalizeSeparators(path string) string {
	replaced := strings.ReplaceAll(path, "\\", "/")
	var builder strings.Builder
	previousSeparator := false
	for _, character := range replaced {
		if character == '/' {
			if previousSeparator {
				continue
			}
			previousSeparator = true
		} else {
			previousSeparator = false
		}
		builder.WriteRune(character)
	}
	normalized := builder.String()
	if len(normalized) > 1 && strings.HasSuffix(normalized, "/") {
		normalized = normalized[:len(normalized)-1]
	}
	return normalized
}

// 🔗️JoinPath joins two path fragments with a single forward slash.
func JoinPath(base, child string) string {
	base = NormalizeSeparators(base)
	child = NormalizeSeparators(child)
	if base == "" {
		return child
	}
	if child == "" {
		return base
	}
	return strings.TrimRight(base, "/") + "/" + strings.TrimLeft(child, "/")
}

// 📛️BaseName returns the last segment of a path.
func BaseName(path string) string {
	normalized := NormalizeSeparators(path)
	if index := strings.LastIndex(normalized, "/"); index >= 0 {
		return normalized[index+1:]
	}
	return normalized
}

// 🗂️DirName returns everything before the last segment of a path.
func DirName(path string) string {
	normalized := NormalizeSeparators(path)
	index := strings.LastIndex(normalized, "/")
	if index < 0 {
		return "."
	}
	if index == 0 {
		return "/"
	}
	return normalized[:index]
}

// 🔤️TicketSlugFromTitle returns the ticket slug of a title.
func TicketSlugFromTitle(title string) (string, error) {
	trimmed := strings.TrimSpace(title)
	if trimmed == "" {
		return "", errInvalid("ticket title is required")
	}
	slug := identity.Slugify(trimmed)
	if slug == "" {
		return "", errInvalid("ticket title must contain at least one alphanumeric character")
	}
	return slug, nil
}

// 🎫️ValidateTicketEmojiTitle validates the emoji and title pair and returns the derived slug.
func ValidateTicketEmojiTitle(emoji, title string) (string, error) {
	emoji = strings.TrimSpace(emoji)
	if emoji == "" {
		return "", errInvalid("ticket emoji is required")
	}
	extracted, remaining := identity.ExtractEntityEmoji(emoji)
	if extracted == "" || strings.TrimSpace(remaining) != "" {
		return "", errInvalid("ticket emoji must be a single emoji character")
	}
	return TicketSlugFromTitle(title)
}

// 🔤️GoJSONString encodes one string the way Go's encoding/json does, HTML escaping included.
func GoJSONString(value string) string {
	var builder strings.Builder
	builder.WriteByte('"')
	for _, character := range value {
		switch {
		case character == '"':
			builder.WriteString("\\\"")
		case character == '\\':
			builder.WriteString("\\\\")
		case character == '\n':
			builder.WriteString("\\n")
		case character == '\r':
			builder.WriteString("\\r")
		case character == '\t':
			builder.WriteString("\\t")
		case character == '<':
			builder.WriteString("\\u003c")
		case character == '>':
			builder.WriteString("\\u003e")
		case character == '&':
			builder.WriteString("\\u0026")
		case character == ' ':
			builder.WriteString("\\u2028")
		case character == ' ':
			builder.WriteString("\\u2029")
		case character < 0x20:
			builder.WriteString(fmt.Sprintf("\\u%04x", character))
		default:
			builder.WriteRune(character)
		}
	}
	builder.WriteByte('"')
	return builder.String()
}

// #endregion 🪪️Contracts
