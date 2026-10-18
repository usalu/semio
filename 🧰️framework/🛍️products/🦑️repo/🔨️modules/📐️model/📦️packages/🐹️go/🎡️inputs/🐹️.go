// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎡️inputs carries the GraphQL input shapes.

// #endregion 🧲️Header

package model

// #region 🎡️Inputs

// 💿️FileListInput holds the data fields for a file list input record.
type FileListInput struct {
	Updated []string `json:"updated,omitempty"`
	Created []string `json:"created,omitempty"`
	Removed []string `json:"removed,omitempty"`
}

// 🎫️TicketOpenInput holds the data fields for a ticket open input record.
type TicketOpenInput struct {
	Emoji        string `json:"emoji"`
	Title        string `json:"title"`
	Prompt       string `json:"prompt"`
	LLM          string `json:"llm,omitempty"`
	Effort       string `json:"effort,omitempty"`
	Client       string `json:"client"`
	NoIssue      bool   `json:"noIssue,omitempty"`
	Draft        string `json:"draft,omitempty"`
	Goal         string `json:"goal"`
	Parent       string `json:"parent,omitempty"`
	NoManagement bool   `json:"noManagement,omitempty"`
	Issue        string `json:"issue,omitempty"`
	PlanID       string `json:"planId,omitempty"`
	SpecID       string `json:"specId,omitempty"`
}

// 🆕️DraftCreateInput holds the data fields for a draft create input record.
type DraftCreateInput struct {
	Title string   `json:"title"`
	Files []string `json:"files,omitempty"`
}

// 📝️GoalCreateInput holds the data fields for a goal create input record.
type GoalCreateInput struct {
	Title        string `json:"title"`
	Description  string `json:"description"`
	Prompt       string `json:"prompt"`
	DueDate      string `json:"dueDate"`
	LLM          string `json:"llm"`
	Effort       string `json:"effort,omitempty"`
	Client       string `json:"client"`
	NoManagement bool   `json:"noManagement,omitempty"`
	Parent       string `json:"parent,omitempty"`
	Milestone    string `json:"milestone,omitempty"`
}

// ♻️GoalChangeInput holds the data fields for a goal change input record.
type GoalChangeInput struct {
	ID           string  `json:"id"`
	Title        *string `json:"title,omitempty"`
	Description  *string `json:"description,omitempty"`
	DueDate      *string `json:"dueDate,omitempty"`
	LLM          *string `json:"llm,omitempty"`
	Effort       *string `json:"effort,omitempty"`
	Parent       *string `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
}

// 📪️GoalCloseInput holds the data fields for a goal close input record.
type GoalCloseInput struct {
	ID           string `json:"id"`
	Summary      string `json:"summary"`
	NoManagement bool   `json:"noManagement,omitempty"`
}

// 🔓️GoalReopenInput holds the data fields for a goal reopen input record.
type GoalReopenInput struct {
	ID           string  `json:"id"`
	Prompt       string  `json:"prompt"`
	Client       string  `json:"client"`
	LLM          string  `json:"llm"`
	Effort       string  `json:"effort,omitempty"`
	Title        *string `json:"title,omitempty"`
	Description  *string `json:"description,omitempty"`
	DueDate      *string `json:"dueDate,omitempty"`
	Parent       *string `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
}

// 🗑️GoalDeleteInput holds the data fields for a goal delete input record.
type GoalDeleteInput struct {
	ID           string `json:"id"`
	NoManagement bool   `json:"noManagement,omitempty"`
}

// 🔷️TicketDeleteInput holds the data fields for a ticket delete input record.
type TicketDeleteInput struct {
	Year         int    `json:"year"`
	Month        int    `json:"month"`
	Day          int    `json:"day"`
	Slug         string `json:"slug"`
	NoManagement bool   `json:"noManagement,omitempty"`
}

// 🔶️TicketCloseInput holds the data fields for a ticket close input record.
type TicketCloseInput struct {
	Year         int      `json:"year"`
	Month        int      `json:"month"`
	Day          int      `json:"day"`
	Slug         string   `json:"slug"`
	Summary      string   `json:"summary"`
	Files        []string `json:"files"`
	Title        *string  `json:"title,omitempty"`
	NoManagement bool     `json:"noManagement,omitempty"`
	All          bool     `json:"all,omitempty"`
}

// 📬️TicketReopenInput holds the data fields for a ticket reopen input record.
type TicketReopenInput struct {
	Year         int     `json:"year"`
	Month        int     `json:"month"`
	Day          int     `json:"day"`
	Slug         string  `json:"slug"`
	Prompt       string  `json:"prompt"`
	LLM          string  `json:"llm,omitempty"`
	Effort       string  `json:"effort,omitempty"`
	Client       string  `json:"client"`
	Title        *string `json:"title,omitempty"`
	Draft        string  `json:"draft,omitempty"`
	Goal         string  `json:"goal,omitempty"`
	Parent       string  `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
	PlanID       string  `json:"planId,omitempty"`
	SpecID       string  `json:"specId,omitempty"`
}

// 🔹️TicketChangeInput holds the data fields for a ticket change input record.
type TicketChangeInput struct {
	Year         int     `json:"year"`
	Month        int     `json:"month"`
	Day          int     `json:"day"`
	Slug         string  `json:"slug"`
	Title        *string `json:"title,omitempty"`
	Prompt       *string `json:"prompt,omitempty"`
	LLM          *string `json:"llm,omitempty"`
	Effort       *string `json:"effort,omitempty"`
	Client       *string `json:"client,omitempty"`
	Goal         *string `json:"goal,omitempty"`
	Parent       *string `json:"parent,omitempty"`
	NoManagement bool    `json:"noManagement,omitempty"`
}

// 🤝️ContributorAddInput holds the data fields for a contributor add input record.
type ContributorAddInput struct {
	Github       string   `json:"github"`
	Name         *string  `json:"name,omitempty"`
	Names        []string `json:"names,omitempty"`
	Email        *string  `json:"email,omitempty"`
	Emails       []string `json:"emails,omitempty"`
	Fingerprint  *string  `json:"fingerprint,omitempty"`
	Fingerprints []string `json:"fingerprints,omitempty"`
}

// 🧹️FilterInput holds the data fields for a filter input record.
type FilterInput struct {
	Filter         *string  `json:"filter,omitempty"`
	Regex          *bool    `json:"regex,omitempty"`
	MatchCase      *bool    `json:"matchCase,omitempty"`
	MatchWholeWord *bool    `json:"matchWholeWord,omitempty"`
	ShowIgnored    *bool    `json:"showIgnored,omitempty"`
	ShowGenerated  *bool    `json:"showGenerated,omitempty"`
	ExcludeKinds   []string `json:"excludeKinds,omitempty"`
	IncludeKinds   []string `json:"includeKinds,omitempty"`
}

// 🗺️ToStreamOptions MUST map all filter input fields to stream options.
// 🔍️ToStreamOptions converts the filter input into stream options.
func (f *FilterInput) ToStreamOptions() StreamOptions {
	if f == nil {
		return StreamOptions{}
	}
	opts := StreamOptions{}
	if f.Filter != nil {
		opts.Filter = *f.Filter
	}
	if f.Regex != nil {
		opts.Regex = *f.Regex
	}
	if f.MatchCase != nil {
		opts.MatchCase = *f.MatchCase
	}
	if f.MatchWholeWord != nil {
		opts.MatchWholeWord = *f.MatchWholeWord
	}
	if f.ShowIgnored != nil {
		opts.ShowIgnored = *f.ShowIgnored
	}
	if f.ShowGenerated != nil {
		opts.ShowGenerated = *f.ShowGenerated
	}
	opts.ExcludeKinds = f.ExcludeKinds
	opts.IncludeKinds = f.IncludeKinds
	return opts
}

// #endregion 🎡️Inputs
