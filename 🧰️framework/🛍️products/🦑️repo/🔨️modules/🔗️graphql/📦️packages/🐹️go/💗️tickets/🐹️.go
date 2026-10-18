// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎫️tickets carries the ticket mutations of the repository context.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	os "os"
	strings "strings"

	contributorspkg "github.com/usalu/semio/repo/contributors"
	events "github.com/usalu/semio/repo/events"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🎫️Tickets

// 🔺️TicketChange MUST return a non-nil error when the operation fails.
// ♻️TicketChange performs the ticket change operation on the repo context.
func (c *RepoContext) TicketChange(input model.TicketChangeInput) (*model.Ticket, error) {
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}

	changed := false
	if input.Prompt != nil {
		ticket.Description = *input.Prompt
		if len(ticket.Interactions) > 0 {
			ticket.Interactions[len(ticket.Interactions)-1].Prompt = *input.Prompt
		}
		changed = true
	}

	if len(ticket.Interactions) > 0 {
		if input.LLM != nil {
			llmSlug, err := model.ResolveAllowedLLM(*input.LLM)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].LLM = llmSlug
			changed = true
		}
		if input.Effort != nil {
			effortSlug, err := model.ResolveAllowedEffort(*input.Effort)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].Effort = effortSlug
			changed = true
		}
		if input.Client != nil {
			uiSlug, err := model.ResolveAllowedClient(*input.Client)
			if err != nil {
				return nil, err
			}
			ticket.Interactions[len(ticket.Interactions)-1].Client = uiSlug
			changed = true
		}
	}
	if input.Goal != nil {
		ticket.Goal = *input.Goal
		changed = true
	}
	if input.Parent != nil {
		ticket.Parent = *input.Parent
		changed = true
	}

	if changed {
		if err := ticketspkg.SaveTicket(ticket); err != nil {
			return nil, err
		}
		ticketID := model.FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		events.Emit(events.EventTicketChangeEnded, "repo-cli", events.TicketChangePayload{
			TicketPayload: events.TicketPayload{ID: ticketID, Year: ticket.Year, Month: ticket.Month, Day: ticket.Day, Slug: ticket.Slug},
			Title:         input.Title, Prompt: input.Prompt, LLM: input.LLM, Effort: input.Effort, Goal: input.Goal, Parent: input.Parent, Author: contributorspkg.GetGitAuthorAlias(),
		})
	}

	return ticket, nil
}

// 🔻️TicketDelete MUST return a non-nil error when the operation fails.
// 🔴️TicketDelete performs the ticket delete operation on the repo context.
func (c *RepoContext) TicketDelete(input model.TicketDeleteInput) (bool, error) {
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return false, err
	}

	if !input.NoManagement && ticket.Management != nil {
		if ticket.Management.Issue != "" {
			parts := strings.Split(ticket.Management.Issue, "/")
			number := parts[len(parts)-1]
			if err := c.managementProvider.DeleteIssue(number); err != nil {
				return false, err
			}
		}
	}

	path := ticket.FolderPath
	if path == "" {
		path = ticketspkg.GetTicketPath(input.Year, input.Month, input.Day, input.Slug)
	}

	if err := os.RemoveAll(path); err != nil {
		return false, err
	}

	return true, nil
}

// 📬️TicketOpen MUST return a non-nil error when the operation fails.
// ⚫️TicketOpen performs the ticket open operation on the repo context.
func (c *RepoContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket, error) {
	resolvedClient, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}
	kind := providers.McpKindFromResolvedClient(resolvedClient)
	return ticketspkg.OpenTicket(input.Emoji, input.Title, input.Prompt, input.LLM, input.Effort, input.Client, input.Draft, input.NoIssue, input.Goal, input.Parent, input.NoManagement, input.Issue, kind, input.PlanID, input.SpecID)
}

// 🟦️TicketClose MUST return a non-nil error when the operation fails.
// 🩵️TicketClose performs the ticket close operation on the repo context.
func (c *RepoContext) TicketClose(input model.TicketCloseInput) (*model.Ticket, error) {
	if input.All {
		tickets, err := ticketspkg.ListTickets(nil, nil, nil)
		if err != nil {
			return nil, err
		}
		var lastTicket *model.Ticket
		for _, t := range tickets {
			if t.Status == model.TicketStatusOpen {
				ticket := t
				fmt.Printf("Closing ticket %s...\n", ticket.Slug)
				if err := ticketspkg.FinishTicket(&ticket, "Bulk close", []string{}, input.NoManagement, true); err != nil {
					workspace.WriteWarningf("Failed to close ticket %s: %v", ticket.Slug, err)
					continue
				}
				lastTicket = &ticket
			}
		}

		if !input.NoManagement {
			issueURLs, err := c.managementProvider.ListOpenIssuesWithLabel("ticket")
			if err != nil {
				workspace.WriteWarningf("Failed to list GitHub issues with 'ticket' label: %v", err)
			} else {
				for _, issueURL := range issueURLs {
					fmt.Printf("Closing GitHub issue %s...\n", issueURL)
					if err := c.managementProvider.CloseIssue(issueURL); err != nil {
						workspace.WriteWarningf("Failed to close GitHub issue %s: %v", issueURL, err)
					}
				}
			}
		}
		return lastTicket, nil
	}
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	if err := ticketspkg.FinishTicket(ticket, input.Summary, input.Files, input.NoManagement, false); err != nil {
		return nil, err
	}
	return ticket, nil
}

// 🟪️TicketReopen MUST return a non-nil error when the operation fails.
// 🩶️TicketReopen performs the ticket reopen operation on the repo context.
func (c *RepoContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket, error) {
	ticket, err := ticketspkg.ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	if input.Title != nil && *input.Title != "" {
		if err := ticketspkg.UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}

		if ticket.Management != nil && ticket.Management.Issue != "" && !input.NoManagement {
			if err := c.managementProvider.UpdateIssueTitle(ticket.Management.Issue, *input.Title); err != nil {
				workspace.WriteWarningf("Failed to update GitHub issue title: %v", err)
			}
		}
	}
	resolvedClient, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, err
	}
	kind := providers.McpKindFromResolvedClient(resolvedClient)
	if err := ticketspkg.ReopenTicket(ticket, input.Prompt, input.LLM, input.Effort, input.Client, input.Draft, input.Goal, input.Parent, input.NoManagement, kind, input.PlanID, input.SpecID); err != nil {
		return nil, err
	}
	return ticket, nil
}

// #endregion 🎫️Tickets
