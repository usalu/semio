package tickets

import (
	context "context"
	fmt "fmt"
	fs "io/fs"
	os "os"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	model "github.com/usalu/semio/repo/model"
	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔍️Query

// ⛳️ListInteractions aggregates interactions from all tickets and goals.
func ListInteractions() ([]model.InteractionResource, error) {
	var result []model.InteractionResource
	tickets, err := ListTickets(nil, nil, nil)
	if err == nil {
		for _, t := range tickets {
			tid := t.GetID()
			for _, ix := range t.Interactions {
				result = append(result, model.InteractionResource{
					Interaction: ix,
					SourceKind:  "ticket",
					SourceID:    tid,
					GoalID:      t.Goal,
					TicketID:    tid,
				})
			}
		}
	}
	return result, nil
}

// ▪️StreamInteractions streams interactions from all tickets and goals.
func StreamInteractions(ctx context.Context, out chan<- model.InteractionResource) {
	defer close(out)
	tickets, _ := ListTickets(nil, nil, nil)
	for _, t := range tickets {
		tid := t.GetID()
		for _, ix := range t.Interactions {
			select {
			case <-ctx.Done():
				return
			case out <- model.InteractionResource{
				Interaction: ix,
				SourceKind:  "ticket",
				SourceID:    tid,
				GoalID:      t.Goal,
				TicketID:    tid,
			}:
			}
		}
	}
}

// ▪️ListTickets MUST return all available tickets entries.
// 📋️ListTickets returns a list of tickets entries.
func ListTickets(year, month, day *int) ([]model.Ticket, error) {
	ticketsDir := GetTicketsDir()
	if !workspace.FileExists(ticketsDir) {
		return nil, nil
	}
	var tickets []model.Ticket
	var years []string
	if year != nil {
		years = []string{model.FormatYearDir(*year)}
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
		if !workspace.FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{model.FormatMonthDir(*month)}
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
			if !workspace.FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{model.FormatDayDir(*day)}
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
				if !workspace.FileExists(dayPath) {
					continue
				}
				yearInt, yearErr := workspace.ParseDatedDir(y, model.EmojiYear)
				monthInt, monthErr := workspace.ParseDatedDir(m, model.EmojiMonth)
				dayInt, dayErr := workspace.ParseDatedDir(d, model.EmojiDay)
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
					if !workspace.FileExists(ticketFilePath) {
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
func StreamTickets(ctx context.Context, year, month, day *int, out chan<- model.Ticket, opts ...model.StreamOptions) error {
	defer close(out)

	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	ticketsDir := GetTicketsDir()
	if !workspace.FileExists(ticketsDir) {
		return nil
	}
	var years []string
	if year != nil {
		years = []string{model.FormatYearDir(*year)}
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
		if !workspace.FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{model.FormatMonthDir(*month)}
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
			if !workspace.FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{model.FormatDayDir(*day)}
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
				if !workspace.FileExists(dayPath) {
					continue
				}
				yearInt, yearErr := workspace.ParseDatedDir(y, model.EmojiYear)
				monthInt, monthErr := workspace.ParseDatedDir(m, model.EmojiMonth)
				dayInt, dayErr := workspace.ParseDatedDir(d, model.EmojiDay)
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
					if !workspace.FileExists(ticketFilePath) {
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
					if !model.MatchesFilter(ticket.GetID(), options) && !model.MatchesFilter(ticket.Slug, options) && !model.MatchesFilter(ticket.Title, options) {
						return filepath.SkipDir
					}
					if !model.MatchesQuery(ticket.GetID()+" "+ticket.Slug+" "+ticket.Title+" "+ticket.Description+" "+string(ticket.Status), options) {
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
func ticketMatchesKinds(t *model.Ticket, opts model.StreamOptions) bool {
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
		kind := model.DeriveFileKind(filepath.Base(f))

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

// 🔵️StreamBundles MUST invoke the callback for each matching bundles entry.
// 🎯️StreamBundles streams bundles entries through the callback.
func StreamBundles(ctx context.Context, out chan<- model.Bundle, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	bundles := codebase.LoadBundles()
	for _, b := range bundles {
		if !model.MatchesFilter(b.Name, options) {
			continue
		}
		if !model.MatchesQuery(b.Name+" "+string(b.Kind), options) {
			continue
		}

		if !shouldIncludeBundleKind(b.Kind, options) {
			continue
		}

		if !bundleMatchesKinds(b, options) {
			continue
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- b
		}
	}
	return nil
}

// 🔴️bundleMatchesKinds holds the data fields for a bundleMatchesKinds record.
func bundleMatchesKinds(b model.Bundle, opts model.StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}

	bundleRoot := filepath.Join(workspace.GetRootDir(), b.Root)
	hasIncluded := false
	hasExcluded := false

	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() {
			return nil
		}
		if workspace.IsRepoExcludedPath(path) || workspace.IsGitIgnored(path) {
			return nil
		}

		kind := model.DeriveFileKind(filepath.Base(path))

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

		if hasIncluded && (len(opts.ExcludeKinds) == 0 || hasExcluded) {
			return filepath.SkipAll
		}
		return nil
	})

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}

// 🟡️StreamTechnologies MUST invoke the callback for each matching technologies entry.
// 🔻️StreamTechnologies streams technologies entries through the callback.
func StreamTechnologies(ctx context.Context, out chan<- model.Technology, opts ...model.StreamOptions) error {
	defer close(out)
	var options model.StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	technologies := codebase.LoadTechnologies()
	for _, p := range technologies {
		if !model.MatchesFilter(p.Name, options) {
			continue
		}
		if !model.MatchesQuery(p.Name+" "+string(p.Kind), options) {
			continue
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- p
		}
	}
	return nil
}

// 🔖️queryableItem holds the data fields for a queryableItem record.
type queryableItem struct {
	text string
	idx  int
}

// ⚪️searchFilterItems returns the matching item positions.
func searchFilterItems(items []queryableItem, query string) map[int]bool {
	result := make(map[int]bool)
	if query == "" {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	mapping := search.NewIndexMapping()
	index, err := search.NewMemOnly(mapping)
	if err != nil {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	defer index.Close()
	for _, item := range items {
		doc := map[string]interface{}{"text": item.text}
		index.Index(fmt.Sprintf("%d", item.idx), doc)
	}
	mq := search.NewMatchQuery(query)
	mq.SetFuzziness(2)
	searchRequest := search.NewSearchRequest(mq)
	searchRequest.Size = len(items)
	results, err := index.Search(searchRequest)
	if err != nil {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	for _, hit := range results.Hits {
		idx, err := strconv.Atoi(hit.ID)
		if err == nil {
			result[idx] = true
		}
	}
	return result
}

// ⚫️shouldIncludeKind holds the data fields for a shouldIncludeKind record.
func shouldIncludeKind(kind string, opts model.StreamOptions) bool {
	if len(opts.IncludeKinds) > 0 {
		found := false
		for _, k := range opts.IncludeKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩵️shouldIncludeBundleKind holds the data fields for a shouldIncludeBundleKind record.
func shouldIncludeBundleKind(kind model.BundleKind, opts model.StreamOptions) bool {
	if len(opts.IncludeBundleKinds) > 0 {
		found := false
		for _, k := range opts.IncludeBundleKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeBundleKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 📁️shouldIncludeFolderKind holds the data fields for a shouldIncludeFolderKind record.
func shouldIncludeFolderKind(kind model.FolderKind, opts model.StreamOptions) bool {
	if len(opts.IncludeFolderKinds) > 0 {
		found := false
		for _, k := range opts.IncludeFolderKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeFolderKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩶️shouldIncludeDefinitionKind holds the data fields for a shouldIncludeDefinitionKind record.
func shouldIncludeDefinitionKind(kind model.DefinitionKind, opts model.StreamOptions) bool {
	if len(opts.IncludeDefinitionKinds) > 0 {
		found := false
		for _, k := range opts.IncludeDefinitionKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeDefinitionKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// #endregion 🔍️Query
