// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧪️entity_resolvers carries the entity resolvers.

// #endregion 🧲️Header

package graphql

import (
	sort "sort"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
)

// #region 🧪️EntityResolvers

/** 📅️ Tickets grouped by one calendar level. */
type ticketBucket struct {
	value    int
	children []*ticketBucket
	rows     []interface{}
}

/** 📅️ Finds or appends one bucket of a calendar level. */
func bucketOf(buckets *[]*ticketBucket, value int) *ticketBucket {
	for _, bucket := range *buckets {
		if bucket.value == value {
			return bucket
		}
	}
	bucket := &ticketBucket{value: value}
	*buckets = append(*buckets, bucket)
	return bucket
}

/** 🧑️‍💻️ The contribution tree of one contributor: their checkpoints and their tickets by date. */
func (run *execution) contributions(source interface{}) (interface{}, *ExecutionError) {
	fields, _ := source.(map[string]interface{})
	github, _ := fields["github"].(string)
	all, err := run.repo.GetTickets(nil, nil, nil, nil)
	if err != nil {
		return nil, lift(err)
	}
	tickets := make([]*model.Ticket, 0, len(all))
	for _, ticket := range all {
		if strings.EqualFold(ticketAuthor(ticket), github) {
			tickets = append(tickets, ticket)
		}
	}
	sort.SliceStable(tickets, func(left, right int) bool { return ticketStarted(tickets[left]) > ticketStarted(tickets[right]) })
	checkpoints := make([]interface{}, 0)
	seen := map[string]bool{}
	for _, ticket := range tickets {
		sha := ticketCheckpoint(ticket)
		if sha == "" || seen[sha] {
			continue
		}
		seen[sha] = true
		checkpoints = append(checkpoints, tagged(map[string]interface{}{
			"id": "repo/checkpoint/" + sha, "sha": sha, "title": ticket.Title, "date": ticketStarted(ticket),
		}, "Checkpoint"))
	}
	years := []*ticketBucket{}
	for _, ticket := range tickets {
		year := bucketOf(&years, ticket.Year)
		month := bucketOf(&year.children, ticket.Month)
		day := bucketOf(&month.children, ticket.Day)
		day.rows = append(day.rows, ticketSource(ticket, run.rootDir))
	}
	ticketYears := make([]interface{}, 0, len(years))
	for _, year := range years {
		months := make([]interface{}, 0, len(year.children))
		for _, month := range year.children {
			days := make([]interface{}, 0, len(month.children))
			for _, day := range month.children {
				days = append(days, tagged(map[string]interface{}{"day": day.value, "tickets": day.rows}, "TicketDay"))
			}
			months = append(months, tagged(map[string]interface{}{"month": month.value, "days": days}, "TicketMonth"))
		}
		ticketYears = append(ticketYears, tagged(map[string]interface{}{"year": year.value, "months": months}, "TicketYear"))
	}
	return tagged(map[string]interface{}{"checkpoints": checkpoints, "tickets": ticketYears, "bundles": []interface{}{}}, "ContributorContributions"), nil
}

/** 🎫️ The contributor a ticket's last interaction names. */
func (run *execution) ticketAuthorOf(source interface{}) (interface{}, *ExecutionError) {
	fields, ok := source.(map[string]interface{})
	if !ok {
		return nil, nil
	}
	interactions, ok := fields["interactions"].([]interface{})
	if !ok || len(interactions) == 0 {
		return nil, nil
	}
	last, ok := interactions[len(interactions)-1].(map[string]interface{})
	if !ok {
		return nil, nil
	}
	raw, _ := last["author"].(string)
	name, email := parseGitAuthor(raw)
	author := email
	if author == "" {
		author = name
	}
	contributors, err := run.repo.GetContributors()
	if err != nil {
		return nil, lift(err)
	}
	for _, contributor := range contributors {
		matched := contributor.Name == author
		for _, candidate := range contributor.Emails {
			if candidate == author || strings.Contains(author, candidate) {
				matched = true
				break
			}
		}
		if matched {
			return contributorSource(contributor), nil
		}
	}
	return contributorSource(&model.Contributor{Alias: author, Github: author, Name: author, Emails: []string{author}}), nil
}

/** 💗️ The children of a section: its nested sections and its definitions, in source order. */
func sectionChildren(source interface{}) interface{} {
	fields, ok := source.(map[string]interface{})
	items := make([]interface{}, 0)
	if !ok {
		return items
	}
	for _, key := range []string{"children", "definitions"} {
		if rows, ok := fields[key].([]interface{}); ok {
			items = append(items, rows...)
		}
	}
	position := func(item interface{}) (float64, float64) {
		row, ok := item.(map[string]interface{})
		if !ok {
			return 0, 0
		}
		start, _ := row["startLine"].(float64)
		index, _ := row["startIndex"].(float64)
		return start, index
	}
	sort.SliceStable(items, func(left, right int) bool {
		leftLine, leftIndex := position(items[left])
		rightLine, rightIndex := position(items[right])
		if leftLine != rightLine {
			return leftLine < rightLine
		}
		return leftIndex < rightIndex
	})
	return items
}

// #endregion 🧪️EntityResolvers
