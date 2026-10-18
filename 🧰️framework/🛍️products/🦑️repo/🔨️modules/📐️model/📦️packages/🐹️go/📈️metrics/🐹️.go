// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📈️metrics carries the line, count and codebase metric shapes.

// #endregion 🧲️Header

package model

// #region 📈️Metrics

// 💿️Range holds the data fields for a range record.
type Range struct {
	Start int `json:"start"`
	End   int `json:"end"`
}

// 🟨️LineMetrics holds the data fields for a line metrics record.
type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
}

// 🟩️DiffLines holds the data fields for a diff lines record.
type DiffLines struct {
	Added   []int
	Removed []int
}

// 🟦️CountMetrics holds the data fields for a count metrics record.
type CountMetrics struct {
	Added   int `json:"added"`
	Updated int `json:"updated"`
	Removed int `json:"removed"`
}

// 📑️TicketSectionMetrics holds the data fields for a ticket section metrics record.
type TicketSectionMetrics struct {
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

// 📍️TicketFileMetricsEntry holds the data fields for a ticket file metrics entry record.
type TicketFileMetricsEntry struct {
	Path     string                          `json:"path"`
	Lines    *LineMetrics                    `json:"lines,omitempty"`
	Sections map[string]TicketSectionMetrics `json:"sections,omitempty"`
}

// 🔬️AnalyzeMetrics holds the data fields for a analyze metrics record.
type AnalyzeMetrics struct {
	Total       int            `json:"total"`
	ByPriority  *PriorityCount `json:"byPriority"`
	Autofixable int            `json:"autofixable"`
}

// 🟫️PriorityCount holds the data fields for a priority count record.
type PriorityCount struct {
	High   int `json:"high"`
	Medium int `json:"medium"`
	Low    int `json:"low"`
}

// 🔖️mergeLineMetrics holds the data fields for a mergeLineMetrics record.
func mergeLineMetrics(target *LineMetrics, add LineMetrics) {
	if target == nil {
		return
	}
	target.Added += add.Added
	target.Removed += add.Removed
}

// 🎫️TicketFileMetrics holds the data fields for a ticket file metrics record.
type TicketFileMetrics struct {
	Sections map[string]TicketSectionMetrics `yaml:"sections" json:"sections"`
}

// 📦️TicketBundleMetrics holds the data fields for a ticket bundle metrics record.
type TicketBundleMetrics struct {
	Files map[string]TicketFileMetrics `yaml:"files" json:"files"`
}

// ⬛️TicketBundles represents a ticket bundles value.
type TicketBundles map[string]TicketBundleMetrics

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

// #endregion 📈️Metrics
