// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪝️webhooks carries the GitHub webhook handlers and the notification fan-out.

// #endregion 🧲️Header

package coordinator

import (
	"context"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	repopkg "github.com/usalu/semio/repo/events"
)

// #region 🪝️Webhooks

// GitHub webhook handlers for issue comment caching and issue event processing. MUST verify HMAC signatures when a secret is configured.

// 💬️GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.
type GitHubComment struct {
	Body   string
	Actor  string
	Repo   string
	Issue  int
	Second time.Time
}

// 🐙️handleGitHubWebhook processes incoming GitHub webhook events.
func (s *Server) handleGitHubWebhook(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	body, err := io.ReadAll(r.Body)
	if err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if s.config.GitHubSecret != "" {
		signature := r.Header.Get("X-Hub-Signature-256")
		if !verifyGitHubSignature(body, signature, s.config.GitHubSecret) {
			s.respondError(w, http.StatusUnauthorized, "invalid signature")
			return
		}
	}
	eventType := r.Header.Get("X-GitHub-Event")
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	if err := s.bus.Publish(ctx, "GitHubIssueEventReceived", "github", map[string]interface{}{"type": eventType}); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	if eventType == "issue_comment" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		s.cacheGitHubComment(payload)
	}
	if eventType == "issues" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		if err := s.handleGitHubIssueEvent(ctx, payload); err != nil {
			s.respondError(w, http.StatusInternalServerError, err.Error())
			return
		}
	}
	if eventType == "push" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		if err := s.handleGitHubPushEvent(ctx, payload); err != nil {
			s.respondError(w, http.StatusInternalServerError, err.Error())
			return
		}
	}
	w.WriteHeader(http.StatusOK)
}

// 📦️verifyGitHubSignature validates the HMAC-SHA256 signature of a webhook payload.
func verifyGitHubSignature(body []byte, signature string, secret string) bool {
	parts := strings.SplitN(signature, "=", 2)
	if len(parts) != 2 {
		return false
	}
	mac := hmac.New(sha256.New, []byte(secret))
	mac.Write(body)
	computed := hex.EncodeToString(mac.Sum(nil))
	return hmac.Equal([]byte(computed), []byte(parts[1]))
}

// 📡️cacheGitHubComment stores a GitHub comment for correlating subsequent events.
func (s *Server) cacheGitHubComment(payload map[string]interface{}) {
	issue, repo, actor, body := extractIssueComment(payload)
	if issue == 0 || repo == "" || actor == "" || body == "" {
		return
	}
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	s.githubCache[key] = GitHubComment{
		Body:   body,
		Actor:  actor,
		Repo:   repo,
		Issue:  issue,
		Second: time.Now().UTC(),
	}
	s.ghLock.Unlock()
}

// 🔓️handleGitHubIssueEvent processes GitHub issue close/reopen events.
func (s *Server) handleGitHubIssueEvent(ctx context.Context, payload map[string]interface{}) error {
	action, _ := payload["action"].(string)
	issueNumber := extractIssueNumber(payload)
	repo := extractRepoFullName(payload)
	actor := extractActorLogin(payload)
	if issueNumber == 0 || repo == "" || actor == "" {
		return nil
	}
	comment := s.findCachedComment(repo, issueNumber, actor)
	if action == "closed" && comment.Body != "" {
		if err := s.bus.Publish(ctx, "TicketClosed", "github", map[string]interface{}{"issue": issueNumber, "comment": comment.Body}); err != nil {
			return err
		}
	}
	if action == "reopened" && comment.Body != "" {
		if err := s.bus.Publish(ctx, "TicketReopened", "github", map[string]interface{}{"issue": issueNumber, "comment": comment.Body}); err != nil {
			return err
		}
	}
	return nil
}

// 💾️findCachedComment retrieves a recently cached GitHub comment for the given issue.
func (s *Server) findCachedComment(repo string, issue int, actor string) GitHubComment {
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	defer s.ghLock.Unlock()
	comment, ok := s.githubCache[key]
	if !ok {
		return GitHubComment{}
	}
	if time.Since(comment.Second) > 90*time.Second {
		delete(s.githubCache, key)
		return GitHubComment{}
	}
	return comment
}

// 🧲️extractIssueComment extracts issue number, repo, actor, and body from a webhook payload.
func extractIssueComment(payload map[string]interface{}) (int, string, string, string) {
	issueNumber := extractIssueNumber(payload)
	repo := extractRepoFullName(payload)
	actor := extractActorLogin(payload)
	body := ""
	if comment, ok := payload["comment"].(map[string]interface{}); ok {
		body, _ = comment["body"].(string)
	}
	return issueNumber, repo, actor, body
}

// 🔢️extractIssueNumber extracts the issue number from a GitHub webhook payload.
func extractIssueNumber(payload map[string]interface{}) int {
	if issue, ok := payload["issue"].(map[string]interface{}); ok {
		if number, ok := issue["number"].(float64); ok {
			return int(number)
		}
	}
	return 0
}

// 🔷️extractRepoFullName extracts the repository full name from a GitHub webhook payload.
func extractRepoFullName(payload map[string]interface{}) string {
	if repo, ok := payload["repository"].(map[string]interface{}); ok {
		if name, ok := repo["full_name"].(string); ok {
			return name
		}
	}
	return ""
}

// 💿️handleGitHubPushEvent holds the data fields for a handleGitHubPushEvent record.
func (s *Server) handleGitHubPushEvent(ctx context.Context, payload map[string]interface{}) error {
	actor := extractActorLogin(payload)
	if actor == "" {
		if pusher, ok := payload["pusher"].(map[string]interface{}); ok {
			if name, ok := pusher["name"].(string); ok {
				actor = name
			}
		}
	}
	var files []string
	if checkpoints, ok := payload["commits"].([]interface{}); ok {
		for _, c := range checkpoints {
			if cm, ok := c.(map[string]interface{}); ok {
				if added, ok := cm["added"].([]interface{}); ok {
					for _, a := range added {
						if p, ok := a.(string); ok {
							files = append(files, p)
						}
					}
				}
				if modified, ok := cm["modified"].([]interface{}); ok {
					for _, m := range modified {
						if p, ok := m.(string); ok {
							files = append(files, p)
						}
					}
				}
			}
		}
	}
	if actor != "" && len(files) > 0 {
		if err := s.db.recordCheckpoint(ctx, actor, files); err != nil {
			return err
		}
	}
	return nil
}

// 📤️extractActorLogin extracts the sender login from a GitHub webhook payload.
func extractActorLogin(payload map[string]interface{}) string {
	if sender, ok := payload["sender"].(map[string]interface{}); ok {
		if login, ok := sender["login"].(string); ok {
			return login
		}
	}
	return ""
}

// ⚙️Discord notification integration for ticket lifecycle events. MUST silently skip when no webhook URL is configured.
func (s *Server) notifyDiscord(title string, body string) {
	if s.config.DiscordWebhook == "" {
		return
	}
	payload := map[string]string{"content": fmt.Sprintf("%s\n%s", title, body)}
	data, _ := json.Marshal(payload)
	request, err := http.NewRequest(http.MethodPost, s.config.DiscordWebhook, strings.NewReader(string(data)))
	if err != nil {
		return
	}
	request.Header.Set("Content-Type", "application/json")
	client := &http.Client{Timeout: 5 * time.Second}
	_, _ = client.Do(request)
}

// 🔔️registerNotifications subscribes to ticket lifecycle events and sends Discord notifications.
func (s *Server) registerNotifications() {
	s.bus.Subscribe("TicketOpened", func(ctx context.Context, event Event) error {
		s.notifyDiscord("# Prompt", event.Payload)
		return nil
	})
	s.bus.Subscribe("TicketClosed", func(ctx context.Context, event Event) error {
		s.notifyDiscord("# Summary", event.Payload)
		return nil
	})
	s.bus.Subscribe("TicketReopened", func(ctx context.Context, event Event) error {
		s.notifyDiscord("# Prompt", event.Payload)
		return nil
	})
	for _, kind := range []repopkg.EventKind{
		repopkg.EventTicketOpenEnded, repopkg.EventTicketCloseEnded, repopkg.EventTicketReopenEnded, repopkg.EventTicketChangeEnded,
		repopkg.EventGoalOpenEnded, repopkg.EventGoalCloseEnded, repopkg.EventGoalReopenEnded, repopkg.EventGoalChangeEnded,
		repopkg.EventContributorAddEnded, repopkg.EventContributorRemoveEnded,
		repopkg.EventTodoCreateEnded, repopkg.EventTodoChangeEnded, repopkg.EventTodoDeleteEnded,
	} {
		k := kind
		s.bus.Subscribe(string(k), func(ctx context.Context, event Event) error {
			return s.onCLIEvent(ctx, k, event)
		})
	}
	s.bus.Subscribe(string(repopkg.EventCheckpointEnded), func(ctx context.Context, event Event) error {
		return s.onCheckpointEvent(ctx, event)
	})
}

// 💿️onCLIEvent holds the data fields for a onCLIEvent record.
func (s *Server) onCLIEvent(ctx context.Context, kind repopkg.EventKind, event Event) error {
	author, items := s.extractAuthorAndItems(kind, event.Payload)
	if author == "" {
		return nil
	}
	for _, item := range items {
		if item.Kind == "" || item.ID == "" {
			continue
		}
		others, err := s.db.projectContributorsOnItem(ctx, item.Kind, item.ID)
		if err != nil {
			return err
		}
		others = filterOut(others, author)
		if err := s.db.recordContributorWork(ctx, author, item.Kind, item.ID); err != nil {
			return err
		}
		s.notifyDiscord(string(kind), event.Payload)
		if len(others) > 0 {
			s.notifyDiscord("⚠️ Conflict", fmt.Sprintf("%s working on %s:%s (others: %v)", author, item.Kind, item.ID, others))
		}
	}
	return nil
}

// 💾️onCheckpointEvent holds the data fields for a onCheckpointEvent record.
func (s *Server) onCheckpointEvent(ctx context.Context, event Event) error {
	var p repopkg.CheckpointPayload
	if json.Unmarshal([]byte(event.Payload), &p) != nil {
		return errors.New("invalid checkpoint event payload")
	}
	files := p.FilesChanged
	if len(files) == 0 {
		files = p.Files
	}
	return s.db.recordCheckpoint(ctx, p.Author, files)
}

// 🧲️extractAuthorAndItems holds the data fields for a extractAuthorAndItems record.
func (s *Server) extractAuthorAndItems(kind repopkg.EventKind, payloadJSON string) (author string, items []repopkg.WorkItem) {
	switch kind {
	case repopkg.EventTicketOpenEnded, repopkg.EventTicketCloseEnded, repopkg.EventTicketReopenEnded, repopkg.EventTicketChangeEnded:
		var p repopkg.TicketPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		if author == "" {
			return "", nil
		}
		id := p.ID
		if id == "" && (p.Year|p.Month|p.Day) != 0 {
			id = fmt.Sprintf("%d/%02d/%02d/%s", p.Year, p.Month, p.Day, p.Slug)
		}
		return author, []repopkg.WorkItem{{Kind: "ticket", ID: id}}
	case repopkg.EventGoalOpenEnded, repopkg.EventGoalCloseEnded, repopkg.EventGoalReopenEnded, repopkg.EventGoalChangeEnded:
		var p repopkg.GoalPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		return author, []repopkg.WorkItem{{Kind: "goal", ID: p.ID}}
	case repopkg.EventContributorAddEnded, repopkg.EventContributorRemoveEnded:
		var p repopkg.ContributorPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "contributor", ID: p.Github}}
	case repopkg.EventTodoCreateEnded, repopkg.EventTodoChangeEnded, repopkg.EventTodoDeleteEnded:
		var p repopkg.TodoPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "todo", ID: p.ID}}
	default:
		return "", nil
	}
}

// 📦️getAuthorFromPayload holds the data fields for a getAuthorFromPayload record.
func getAuthorFromPayload(payloadJSON string) string {
	var m map[string]interface{}
	if json.Unmarshal([]byte(payloadJSON), &m) != nil {
		return ""
	}
	if a, ok := m["author"].(string); ok {
		return a
	}
	return ""
}

// 🧹️filterOut holds the data fields for a filterOut record.
func filterOut(list []string, exclude string) []string {
	var out []string
	for _, x := range list {
		if x != exclude {
			out = append(out, x)
		}
	}
	return out
}

// #endregion 🪝️Webhooks
