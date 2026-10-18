// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Internal codebase analysis types.

// #endregion 🧲️Header

package client



// 💿️BundleMetricsInternal holds the data fields for a bundle metrics internal record.
type BundleMetricsInternal struct {
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Breachs     int `json:"breachs"`
}

// 📁️FolderMetricsInternal holds the data fields for a folder metrics internal record.
type FolderMetricsInternal struct {
	Files   int `json:"files"`
	Lines   int `json:"lines"`
	Breachs int `json:"breachs"`
}

// 📄️FileMetricsInternal holds the data fields for a file metrics internal record.
type FileMetricsInternal struct {
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
}

// 📑️SectionMetricsInternal holds the data fields for a section metrics internal record.
type SectionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Breachs     int `json:"breachs"`
}

// 📖️DefinitionMetricsInternal holds the data fields for a definition metrics internal record.
type DefinitionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Breachs     int `json:"breachs"`
}

// 🔷️RangePosition holds the data fields for a range position record.
type RangePosition struct {
	Line   int `json:"line"`
	Column int `json:"column"`
}

// 🔶️FileRange holds the data fields for a file range record.
type FileRange struct {
	Start RangePosition `json:"start"`
	End   RangePosition `json:"end"`
}

// 🔹️BreachFile holds the data fields for a breach file record.
type BreachFile struct {
	ID    string     `json:"id"`
	Path  string     `json:"path"`
	URI   string     `json:"uri"`
	Range *FileRange `json:"range,omitempty"`
}

// 🔸️BreachFolder holds the data fields for a breach folder record.
type BreachFolder struct {
	ID   string `json:"id"`
	Path string `json:"path"`
	URI  string `json:"uri"`
}

// 🔺️CodebaseBreach holds the data fields for a codebase breach record.
type CodebaseBreach struct {
	ID          string         `json:"id"`
	Folders     []BreachFolder `json:"folders,omitempty"`
	Files       []BreachFile   `json:"files,omitempty"`
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Reason      string         `json:"reason"`
	Solution    string         `json:"solution"`
}

// 📦️CodebaseBundle holds the data fields for a codebase bundle record.
type CodebaseBundle struct {
	ID           string                 `json:"id"`
	Folder       string                 `json:"folder"`
	URI          string                 `json:"uri"`
	Contributors []string               `json:"contributors,omitempty"`
	Tickets      []string               `json:"tickets,omitempty"`
	Metrics      *BundleMetricsInternal `json:"metrics,omitempty"`
}

// 🔻️CodebaseFolder holds the data fields for a codebase folder record.
type CodebaseFolder struct {
	ID       string                 `json:"id"`
	Path     string                 `json:"path"`
	URI      string                 `json:"uri"`
	Name     string                 `json:"name"`
	ParentID *string                `json:"parentId,omitempty"`
	Metrics  *FolderMetricsInternal `json:"metrics,omitempty"`
}

// 🔗️FileBreachRef holds the data fields for a file breach ref record.
type FileBreachRef struct {
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Solution    string         `json:"solution"`
}

// ⬛️CodebaseFile holds the data fields for a codebase file record.
type CodebaseFile struct {
	ID      string               `json:"id"`
	Path    string               `json:"path"`
	URI     string               `json:"uri"`
	Metrics *FileMetricsInternal `json:"metrics,omitempty"`
	Breachs []FileBreachRef      `json:"breachs,omitempty"`
}

// ⬜️CodebaseSection holds the data fields for a codebase section record.
type CodebaseSection struct {
	ID      string                  `json:"id"`
	Path    string                  `json:"path"`
	URI     string                  `json:"uri"`
	Metrics *SectionMetricsInternal `json:"metrics,omitempty"`
}

// 🟥️CodebaseDefinition holds the data fields for a codebase definition record.
type CodebaseDefinition struct {
	ID      string                     `json:"id"`
	Path    string                     `json:"path"`
	URI     string                     `json:"uri"`
	Metrics *DefinitionMetricsInternal `json:"metrics,omitempty"`
}

// 🤝️ContributorBundleContrib holds the data fields for a contributor bundle contrib record.
type ContributorBundleContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🟧️ContributorFolderContrib holds the data fields for a contributor folder contrib record.
type ContributorFolderContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🟨️ContributorFileContrib holds the data fields for a contributor file contrib record.
type ContributorFileContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟩️ContributorSectionContrib holds the data fields for a contributor section contrib record.
type ContributorSectionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟦️ContributorDefinitionContrib holds the data fields for a contributor definition contrib record.
type ContributorDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🟪️ContributorContributionsInternal holds the data fields for a contributor contributions internal record.
type ContributorContributionsInternal struct {
	Bundles     []ContributorBundleContrib     `json:"bundles,omitempty"`
	Folders     []ContributorFolderContrib     `json:"folders,omitempty"`
	Files       []ContributorFileContrib       `json:"files,omitempty"`
	Sections    []ContributorSectionContrib    `json:"sections,omitempty"`
	Definitions []ContributorDefinitionContrib `json:"definitions,omitempty"`
}

// 🟫️ContributorMetricsInternal holds the data fields for a contributor metrics internal record.
type ContributorMetricsInternal struct {
	Checkpoints int `json:"checkpoints"`
	Tickets     int `json:"tickets"`
	Bundles     int `json:"bundles"`
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Lines       int `json:"lines"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
}

// 💠️CodebaseContributor holds the data fields for a codebase contributor record.
type CodebaseContributor struct {
	ID            string                            `json:"id"`
	URI           string                            `json:"uri"`
	Path          string                            `json:"path"`
	Name          string                            `json:"name,omitempty"`
	Icons         *ContributorIcons                 `json:"icons,omitempty"`
	Emails        []string                          `json:"emails,omitempty"`
	Links         map[string]string                 `json:"links,omitempty"`
	Contributions *ContributorContributionsInternal `json:"contributions,omitempty"`
	Metrics       *ContributorMetricsInternal       `json:"metrics,omitempty"`
}

// 🎫️TicketDateInfo holds the data fields for a ticket date info record.
type TicketDateInfo struct {
	Created  string `json:"created,omitempty"`
	Finished string `json:"finished,omitempty"`
}

// 🔳️TicketBundleContribInfo holds the data fields for a ticket bundle contrib info record.
type TicketBundleContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// 🔲️TicketFolderContribInfo holds the data fields for a ticket folder contrib info record.
type TicketFolderContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ▪️TicketFileContribInfo holds the data fields for a ticket file contrib info record.
type TicketFileContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ▫️TicketSectionContribInfo holds the data fields for a ticket section contrib info record.
type TicketSectionContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

// ◾TicketDefinitionContrib holds the data fields for a ticket definition contrib record.
type TicketDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

// 🗃️CodebaseTicket holds the data fields for a codebase ticket record.
type CodebaseTicket struct {
	ID          string                     `json:"id"`
	Path        string                     `json:"path"`
	URI         string                     `json:"uri"`
	Date        *TicketDateInfo            `json:"date,omitempty"`
	Checkpoint  string                     `json:"checkpoint,omitempty"`
	Year        string                     `json:"year"`
	Month       string                     `json:"month"`
	Day         string                     `json:"day"`
	Slug        string                     `json:"slug"`
	Prompt      string                     `json:"prompt,omitempty"`
	LLM         string                     `json:"llm,omitempty"`
	Author      string                     `json:"author,omitempty"`
	Status      TicketStatus               `json:"status"`
	Bundles     []TicketBundleContribInfo  `json:"bundles,omitempty"`
	Folders     []TicketFolderContribInfo  `json:"folders,omitempty"`
	Files       []TicketFileContribInfo    `json:"files,omitempty"`
	Sections    []TicketSectionContribInfo `json:"sections,omitempty"`
	Definitions []TicketDefinitionContrib  `json:"definitions,omitempty"`
}

// 📜️PolicyBreachRef holds the data fields for a policy breach ref record.
type PolicyBreachRef struct {
	Kind        Statute        `json:"kind"`
	Priority    BreachPriority `json:"priority"`
	Autofixable bool           `json:"autofixable"`
	Solution    string         `json:"solution"`
}

// ◽CodebasePolicy holds the data fields for a codebase policy record.
type CodebasePolicy struct {
	ID      string            `json:"id"`
	Name    string            `json:"name"`
	Scopes  []string          `json:"scopes,omitempty"`
	Breachs []PolicyBreachRef `json:"breachs,omitempty"`
}

// 🌳️CbTreeNodeKind represents a cb tree node kind value.
type CbTreeNodeKind string

const (
	CbTreeNodeRepo       CbTreeNodeKind = "repo"
	CbTreeNodeBundle     CbTreeNodeKind = "bundle"
	CbTreeNodeFolder     CbTreeNodeKind = "folder"
	CbTreeNodeFile       CbTreeNodeKind = "file"
	CbTreeNodeSection    CbTreeNodeKind = "section"
	CbTreeNodeDefinition CbTreeNodeKind = "definition"
)

// 🌿️CbTreeNode holds the data fields for a cb tree node record.
type CbTreeNode struct {
	Kind     CbTreeNodeKind         `json:"kind"`
	Children map[string]*CbTreeNode `json:"children,omitempty"`
}

// ◻Codebase holds the data fields for a codebase record.
type Codebase struct {
	Bundles      []CodebaseBundle       `json:"bundles"`
	Folders      []CodebaseFolder       `json:"folders"`
	Files        []CodebaseFile         `json:"files"`
	Sections     []CodebaseSection      `json:"sections"`
	Definitions  []CodebaseDefinition   `json:"definitions"`
	Contributors []CodebaseContributor  `json:"contributors"`
	Tickets      []CodebaseTicket       `json:"tickets"`
	Policies     []CodebasePolicy       `json:"policies"`
	Breachs      []CodebaseBreach       `json:"breachs"`
	Tree         map[string]*CbTreeNode `json:"tree"`
}
