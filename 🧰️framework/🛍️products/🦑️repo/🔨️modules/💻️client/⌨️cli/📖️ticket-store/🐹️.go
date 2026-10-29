// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Ticket persistence and streaming.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// 🔲️SaveTicket MUST persist the ticket atomically to the data store.
// 💾️SaveTicket persists ticket to the data store.
func SaveTicket(ticket *Ticket) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if !ticket.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \"open\" or \"closed\"")
	}
	jsonBytes, err := json.MarshalIndent(ticket, "", "  ")
	if err != nil {
		return err
	}
	if err := WriteTextFile(ticket.JsonPath, string(jsonBytes)); err != nil {
		return err
	}
	// Sync ticket state to server (fire-and-forget, don't block on failure)
	action := "open"
	if ticket.Status == TicketStatusClosed {
		action = "close"
	}
	go syncTicketToServer(ticket, action)
	return nil
}

// 📖️ReadTicket MUST return the ticket content or an error if unavailable.
// 🔸️ReadTicket reads and returns ticket from the source.
func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	folderPath := GetTicketPath(year, month, day, slug)
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	if !FileExists(jsonPath) {
		return nil, fmt.Errorf("ticket not found: %s", jsonPath)
	}
	raw, err := ReadTextFile(jsonPath)
	if err != nil {
		return nil, err
	}
	var ticket Ticket
	if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
		return nil, err
	}

	ticket.Year = year
	ticket.Month = month
	ticket.Day = day
	ticket.Slug = slug
	ticket.FolderPath = folderPath
	ticket.JsonPath = jsonPath
	ticket.ImportantPath = GetImportantFilePath(year, month, day, slug)

	return &ticket, nil
}

// ▪️ListTickets MUST return all available tickets entries.
// 📋️ListTickets returns a list of tickets entries.
func ListTickets(year, month, day *int) ([]Ticket, error) {
	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil, nil
	}
	var tickets []Ticket
	var years []string
	if year != nil {
		years = []string{FormatYearDir(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return nil, err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}
	for _, y := range years {
		yearPath := filepath.Join(ticketsDir, y)
		if !FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{FormatMonthDir(*month)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{FormatDayDir(*day)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !FileExists(dayPath) {
					continue
				}
				yearInt, yearErr := parseDatedDir(y, EmojiYear)
				monthInt, monthErr := parseDatedDir(m, EmojiMonth)
				dayInt, dayErr := parseDatedDir(d, EmojiDay)
				if yearErr != nil || monthErr != nil || dayErr != nil {
					continue
				}
				filepath.WalkDir(dayPath, func(path string, dEntry fs.DirEntry, err error) error {
					if err != nil {
						return nil
					}
					if !dEntry.IsDir() {
						return nil
					}
					name := dEntry.Name()
					if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
						return filepath.SkipDir
					}
					if path == dayPath {
						return nil
					}
					ticketFilePath := filepath.Join(path, "🎫️ticket.json")
					if !FileExists(ticketFilePath) {
						return nil
					}
					rel, relErr := filepath.Rel(dayPath, path)
					if relErr != nil {
						return filepath.SkipDir
					}
					slug := filepath.ToSlash(rel)
					ticket, readErr := ReadTicket(yearInt, monthInt, dayInt, slug)
					if readErr == nil {
						tickets = append(tickets, *ticket)
					}
					return filepath.SkipDir
				})
			}
		}
	}
	return tickets, nil
}

// 📍️StreamTickets MUST invoke the callback for each matching tickets entry.
// ⚡️StreamTickets streams tickets entries through the callback.
func StreamTickets(ctx context.Context, year, month, day *int, out chan<- Ticket, opts ...StreamOptions) error {
	defer close(out)

	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil
	}
	var years []string
	if year != nil {
		years = []string{FormatYearDir(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}

	for _, y := range years {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
		}

		yearPath := filepath.Join(ticketsDir, y)
		if !FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{FormatMonthDir(*month)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{FormatDayDir(*day)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !FileExists(dayPath) {
					continue
				}
				yearInt, yearErr := parseDatedDir(y, EmojiYear)
				monthInt, monthErr := parseDatedDir(m, EmojiMonth)
				dayInt, dayErr := parseDatedDir(d, EmojiDay)
				if yearErr != nil || monthErr != nil || dayErr != nil {
					continue
				}
				walkErr := filepath.WalkDir(dayPath, func(path string, dEntry fs.DirEntry, err error) error {
					if err != nil {
						return nil
					}
					select {
					case <-ctx.Done():
						return ctx.Err()
					default:
					}
					if !dEntry.IsDir() {
						return nil
					}
					name := dEntry.Name()
					if strings.HasPrefix(name, ".") || name == "node_modules" || name == "dist" || name == "build" || name == "target" || name == "__pycache__" {
						return filepath.SkipDir
					}
					if path == dayPath {
						return nil
					}
					ticketFilePath := filepath.Join(path, "🎫️ticket.json")
					if !FileExists(ticketFilePath) {
						return nil
					}
					rel, relErr := filepath.Rel(dayPath, path)
					if relErr != nil {
						return filepath.SkipDir
					}
					slug := filepath.ToSlash(rel)
					ticket, readErr := ReadTicket(yearInt, monthInt, dayInt, slug)
					if readErr != nil {
						return filepath.SkipDir
					}
					if !matchesFilter(ticket.GetID(), options) && !matchesFilter(ticket.Slug, options) && !matchesFilter(ticket.Title, options) {
						return filepath.SkipDir
					}
					queryMatches, queryErr := matchesQuery(ctx, ticket.GetID()+" "+ticket.Slug+" "+ticket.Title+" "+ticket.Description+" "+string(ticket.Status), options)
					if queryErr != nil {
						return queryErr
					}
					if !queryMatches {
						return filepath.SkipDir
					}
					if !ticketMatchesKinds(ticket, options) {
						return filepath.SkipDir
					}
					out <- *ticket
					return filepath.SkipDir
				})
				if walkErr != nil {
					return walkErr
				}
			}
		}
	}
	return nil
}

// ▫️ticketMatchesKinds holds the data fields for a ticketMatchesKinds record.
func ticketMatchesKinds(t *Ticket, opts StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}
	allFiles := map[string]bool{}
	for _, f := range t.GetInteractionFiles() {
		allFiles[f.Path] = true
	}

	if len(allFiles) == 0 {

		if len(opts.IncludeKinds) > 0 {
			return false
		}

		return true
	}

	hasIncluded := false
	hasExcluded := false

	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	for f := range allFiles {
		kind := DeriveFileKind(filepath.Base(f))

		if len(opts.IncludeKinds) > 0 {
			for _, k := range opts.IncludeKinds {
				if k == kind {
					hasIncluded = true
				}
			}
		}

		for _, k := range opts.ExcludeKinds {
			if k == kind {
				hasExcluded = true
			}
		}
	}

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}
