// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 💿️records carries the tree records, their in-memory sources and catalogs and the node and filter specs.

// #endregion 🧲️Header

package tree

import (
	json "encoding/json"

	model "github.com/usalu/semio/repo/model"
)

// #region 💿️Records

// 🧪️TechnologyRecord is a technology with its bundles.
type TechnologyRecord struct {
	ID      string         `json:"id"`
	Name    string         `json:"name"`
	URI     string         `json:"uri"`
	Kind    string         `json:"kind"`
	Emoji   string         `json:"emoji"`
	Bundles []BundleRecord `json:"bundles"`
}

// 📦️BundleRecord is one bundle of a technology.
type BundleRecord struct {
	ID         string `json:"id"`
	Name       string `json:"name"`
	URI        string `json:"uri"`
	Kind       string `json:"kind"`
	Emoji      string `json:"emoji"`
	Root       string `json:"root"`
	SourceRoot string `json:"sourceRoot"`
}

// 📁️FolderRecord is one folder of the repository.
type FolderRecord struct {
	ID       string `json:"id"`
	Path     string `json:"path"`
	Name     string `json:"name"`
	URI      string `json:"uri"`
	Kind     string `json:"kind"`
	ParentID string `json:"parentId,omitempty"`
}

// 📄️FileRecord is one file of the repository.
type FileRecord struct {
	ID       string `json:"id"`
	Path     string `json:"path"`
	Name     string `json:"name"`
	URI      string `json:"uri"`
	Kind     string `json:"kind"`
	ParentID string `json:"parentId,omitempty"`
}

// 📑️SectionRecord is one section of a file, with its definitions and nested sections.
type SectionRecord struct {
	ID          string             `json:"id"`
	Path        string             `json:"path"`
	Name        string             `json:"name"`
	URI         string             `json:"uri"`
	StartLine   int                `json:"startLine"`
	EndLine     int                `json:"endLine"`
	Definitions []DefinitionRecord `json:"definitions"`
	Children    []SectionRecord    `json:"children"`
}

// 🏷️DefinitionRecord is one definition inside a section.
type DefinitionRecord struct {
	ID        string `json:"id"`
	Name      string `json:"name"`
	URI       string `json:"uri"`
	Kind      string `json:"kind"`
	FilePath  string `json:"filePath"`
	StartLine int    `json:"startLine"`
	EndLine   int    `json:"endLine"`
}

// 🎯️GoalRecord is one goal.
type GoalRecord struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	URI         string `json:"uri"`
	ArtifactID  string `json:"artifactId"`
	Status      string `json:"status"`
	DueDate     string `json:"dueDate"`
	CreatedAt   string `json:"createdAt"`
	Description string `json:"description"`
}

// 🎫️TicketRecord is one ticket.
type TicketRecord struct {
	ID          string `json:"id"`
	Slug        string `json:"slug"`
	Title       string `json:"title"`
	URI         string `json:"uri"`
	Description string `json:"description"`
	Summary     string `json:"summary"`
	Goal        string `json:"goal"`
	Parent      string `json:"parent"`
	Year        int    `json:"year"`
	Month       int    `json:"month"`
	Day         int    `json:"day"`
	Status      string `json:"status"`
	Started     string `json:"started"`
	Finished    string `json:"finished"`
}

// ✍️DraftRecord is one draft.
type DraftRecord struct {
	ID         string `json:"id"`
	URI        string `json:"uri"`
	ArtifactID string `json:"artifactId"`
}

// 🛡️PolicyRecord is one policy with its territories.
type PolicyRecord struct {
	ID          string            `json:"id"`
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Groups      []model.Territory `json:"groups"`
}

// 🧑️ContributorRecord is one contributor.
type ContributorRecord struct {
	ID      string   `json:"id"`
	Alias   string   `json:"alias"`
	URI     string   `json:"uri"`
	Name    string   `json:"name"`
	Aliases []string `json:"aliases"`
	Github  string   `json:"github"`
	Githubs []string `json:"githubs"`
	Names   []string `json:"names"`
	Email   string   `json:"email"`
	Emails  []string `json:"emails"`
}

// 🔀️CheckpointRecord is one checkpoint.
type CheckpointRecord struct {
	ID       string `json:"id"`
	Sha      string `json:"sha"`
	Title    string `json:"title"`
	URI      string `json:"uri"`
	Year     int    `json:"year"`
	Month    int    `json:"month"`
	Day      int    `json:"day"`
	AuthorID string `json:"authorId"`
}

// ⚪️SessionRecord is one agent session.
type SessionRecord struct {
	ID         string `json:"id"`
	UUID       string `json:"uuid"`
	URI        string `json:"uri"`
	Kind       string `json:"kind"`
	KindEmoji  string `json:"kindEmoji"`
	Client     string `json:"client"`
	StartedAt  string `json:"startedAt"`
	Year       int    `json:"year"`
	Month      int    `json:"month"`
	Day        int    `json:"day"`
	Checkpoint string `json:"checkpoint"`
}

// 🗄️MemoryTreeSource is a TreeSource backed entirely by already-decoded records.
type MemoryTreeSource struct {
	TechnologyRecords  []TechnologyRecord         `json:"technologies"`
	FolderRecords      []FolderRecord             `json:"folders"`
	FileRecords        []FileRecord               `json:"files"`
	Goals              []GoalRecord               `json:"goals"`
	Tickets            []TicketRecord             `json:"tickets"`
	DraftRecords       []DraftRecord              `json:"drafts"`
	PolicyRecords      []PolicyRecord             `json:"policies"`
	ContributorRecords []ContributorRecord        `json:"contributors"`
	CheckpointRecords  []CheckpointRecord         `json:"checkpoints"`
	SessionRecords     []SessionRecord            `json:"sessions"`
	SectionRecords     map[string][]SectionRecord `json:"sections"`
}

// 📥️ParseTreeSource MUST return an error when the document is malformed.
func ParseTreeSource(document []byte) (*MemoryTreeSource, error) {
	source := &MemoryTreeSource{}
	if err := json.Unmarshal(document, source); err != nil {
		return nil, err
	}
	return source, nil
}

// 🧪️Technologies MUST return the declared technologies in load order.
func (s *MemoryTreeSource) Technologies() []TechnologyRecord { return s.TechnologyRecords }

// 📁️Folders MUST return every declared folder.
func (s *MemoryTreeSource) Folders() []FolderRecord { return s.FolderRecords }

// 📄️Files MUST return every declared file.
func (s *MemoryTreeSource) Files() []FileRecord { return s.FileRecords }

// 🎯️GoalRecords MUST return every declared goal.
func (s *MemoryTreeSource) GoalRecords() []GoalRecord { return s.Goals }

// 🎫️TicketRecords MUST return every declared ticket.
func (s *MemoryTreeSource) TicketRecords() []TicketRecord { return s.Tickets }

// ✍️Drafts MUST return every declared draft.
func (s *MemoryTreeSource) Drafts() []DraftRecord { return s.DraftRecords }

// 🛡️Policies MUST return every declared policy.
func (s *MemoryTreeSource) Policies() []PolicyRecord { return s.PolicyRecords }

// 🧑️Contributors MUST return every declared contributor.
func (s *MemoryTreeSource) Contributors() []ContributorRecord { return s.ContributorRecords }

// 🔀️Checkpoints MUST return every declared checkpoint.
func (s *MemoryTreeSource) Checkpoints() []CheckpointRecord { return s.CheckpointRecords }

// ⚪️Sessions MUST return every declared session.
func (s *MemoryTreeSource) Sessions() []SessionRecord { return s.SessionRecords }

// 📑️Sections MUST return the declared sections of one file.
func (s *MemoryTreeSource) Sections(filePath string) []SectionRecord {
	return s.SectionRecords[filePath]
}

// 🗄️MemoryStatuteCatalog is a StatuteCatalog backed by declared entries.
type MemoryStatuteCatalog struct {
	Statutes    []StatuteCatalogEntry `json:"statutes"`
	Territories map[string]string     `json:"territories"`
}

// 📜️StatuteCatalogEntry is one declared statute of a MemoryStatuteCatalog.
type StatuteCatalogEntry struct {
	ID         string            `json:"id"`
	ArtifactID string            `json:"artifactId"`
	URI        string            `json:"uri"`
	Label      string            `json:"label"`
	EntityKind string            `json:"entityKind"`
	Meta       StatuteMetaRecord `json:"meta"`
}

// 🔖️StatuteMetaRecord is the declared metadata of one catalog entry.
type StatuteMetaRecord struct {
	PolicyID    string `json:"policyId"`
	Priority    string `json:"priority"`
	Reason      string `json:"reason"`
	Solution    string `json:"solution"`
	Autofixable bool   `json:"autofixable"`
}

// 📥️ParseStatuteCatalog MUST return an error when the document is malformed.
func ParseStatuteCatalog(document []byte) (*MemoryStatuteCatalog, error) {
	catalog := &MemoryStatuteCatalog{}
	if err := json.Unmarshal(document, catalog); err != nil {
		return nil, err
	}
	return catalog, nil
}

// 🔎️entry returns the declared entry of a statute.
func (c *MemoryStatuteCatalog) entry(statute model.Statute) (StatuteCatalogEntry, bool) {
	for _, candidate := range c.Statutes {
		if candidate.ID == string(statute) {
			return candidate, true
		}
	}
	return StatuteCatalogEntry{}, false
}

// 🔖️Info MUST return the declared metadata of a statute.
func (c *MemoryStatuteCatalog) Info(statute model.Statute) (model.StatuteMeta, bool) {
	found, ok := c.entry(statute)
	if !ok {
		return model.StatuteMeta{}, false
	}
	return model.StatuteMeta{
		Kind:        statute,
		PolicyID:    found.Meta.PolicyID,
		Priority:    ParseBreachPriority(found.Meta.Priority),
		Reason:      found.Meta.Reason,
		Solution:    found.Meta.Solution,
		Autofixable: found.Meta.Autofixable,
	}, true
}

// 🆔️StatuteID MUST return the artifact id of a statute.
func (c *MemoryStatuteCatalog) StatuteID(statute model.Statute) string {
	found, _ := c.entry(statute)
	return found.ArtifactID
}

// 🔗️StatuteURI MUST return the uri of a statute.
func (c *MemoryStatuteCatalog) StatuteURI(statute model.Statute) string {
	found, _ := c.entry(statute)
	return found.URI
}

// 🏷️StatuteLabel MUST return the display label of a statute path.
func (c *MemoryStatuteCatalog) StatuteLabel(statute model.Statute) string {
	found, ok := c.entry(statute)
	if !ok {
		return string(statute)
	}
	return found.Label
}

// 🧱️EntityKind MUST return the entity kind a statute applies to.
func (c *MemoryStatuteCatalog) EntityKind(statute model.Statute) string {
	found, _ := c.entry(statute)
	return found.EntityKind
}

// 🆔️TerritoryID MUST return the artifact id of a territory.
func (c *MemoryStatuteCatalog) TerritoryID(territory model.Territory) string {
	if id, ok := c.Territories[territory.Name]; ok {
		return id
	}
	return territory.Name
}

// 🌿️TreeNodeSpec is a compact tree-node literal where every field defaults.
type TreeNodeSpec struct {
	Kind        string                 `json:"kind"`
	ID          string                 `json:"id"`
	Label       string                 `json:"label"`
	URI         string                 `json:"uri"`
	SubKind     string                 `json:"subKind"`
	Description string                 `json:"description"`
	Summary     string                 `json:"summary"`
	Year        int                    `json:"year"`
	Month       int                    `json:"month"`
	Day         int                    `json:"day"`
	Status      string                 `json:"status"`
	Contributor string                 `json:"contributor"`
	Data        map[string]interface{} `json:"data"`
	Children    []TreeNodeSpec         `json:"children"`
}

// 📥️ParseTreeNodeSpec MUST return an error when the document is malformed.
func ParseTreeNodeSpec(document []byte) (*TreeNodeSpec, error) {
	spec := &TreeNodeSpec{}
	if err := json.Unmarshal(document, spec); err != nil {
		return nil, err
	}
	return spec, nil
}

// 🌿️ToTreeNode MUST expand the literal into a real tree node.
func (s *TreeNodeSpec) ToTreeNode() *TreeNode {
	node := NewTreeNode(ParseTreeNodeKind(s.Kind), s.ID, s.Label, s.URI)
	node.SubKind = s.SubKind
	node.Description = s.Description
	node.Summary = s.Summary
	node.Year = s.Year
	node.Month = s.Month
	node.Day = s.Day
	node.Status = s.Status
	node.Contributor = s.Contributor
	node.Data = s.Data
	for index := range s.Children {
		node.Children = append(node.Children, s.Children[index].ToTreeNode())
	}
	return node
}

// 🧹️TreeFilterSpec is a compact tree-filter literal where every field defaults.
type TreeFilterSpec struct {
	Query               string              `json:"query"`
	OnlyKinds           []string            `json:"onlyKinds"`
	ExcludeKinds        []string            `json:"excludeKinds"`
	OnlySubKinds        map[string][]string `json:"onlySubKinds"`
	ExcludeSubKinds     map[string][]string `json:"excludeSubKinds"`
	OnlyYears           []int               `json:"onlyYears"`
	ExcludeYears        []int               `json:"excludeYears"`
	OnlyMonths          []int               `json:"onlyMonths"`
	ExcludeMonths       []int               `json:"excludeMonths"`
	OnlyDays            []int               `json:"onlyDays"`
	ExcludeDays         []int               `json:"excludeDays"`
	OnlyStatus          string              `json:"onlyStatus"`
	OnlyContributors    []string            `json:"onlyContributors"`
	ExcludeContributors []string            `json:"excludeContributors"`
	OnlyPolicies        []string            `json:"onlyPolicies"`
	ExcludePolicies     []string            `json:"excludePolicies"`
}

// 📥️ParseTreeFilterSpec MUST return an error when the document is malformed.
func ParseTreeFilterSpec(document []byte) (*TreeFilterSpec, error) {
	spec := &TreeFilterSpec{}
	if err := json.Unmarshal(document, spec); err != nil {
		return nil, err
	}
	return spec, nil
}

// 🧹️ToFilter MUST expand the literal into a real tree filter.
func (s *TreeFilterSpec) ToFilter() *TreeFilter {
	kindSet := func(kinds []string) map[TreeNodeKind]bool {
		out := map[TreeNodeKind]bool{}
		for _, kind := range kinds {
			out[ParseTreeNodeKind(kind)] = true
		}
		return out
	}
	subKindMap := func(entries map[string][]string) map[TreeNodeKind][]string {
		out := map[TreeNodeKind][]string{}
		for kind, values := range entries {
			out[ParseTreeNodeKind(kind)] = values
		}
		return out
	}
	return &TreeFilter{
		Query:               s.Query,
		OnlyKinds:           kindSet(s.OnlyKinds),
		ExcludeKinds:        kindSet(s.ExcludeKinds),
		OnlySubKinds:        subKindMap(s.OnlySubKinds),
		ExcludeSubKinds:     subKindMap(s.ExcludeSubKinds),
		OnlyYears:           s.OnlyYears,
		ExcludeYears:        s.ExcludeYears,
		OnlyMonths:          s.OnlyMonths,
		ExcludeMonths:       s.ExcludeMonths,
		OnlyDays:            s.OnlyDays,
		ExcludeDays:         s.ExcludeDays,
		OnlyStatus:          s.OnlyStatus,
		OnlyContributors:    s.OnlyContributors,
		ExcludeContributors: s.ExcludeContributors,
		OnlyPolicies:        s.OnlyPolicies,
		ExcludePolicies:     s.ExcludePolicies,
	}
}

// 📥️DecodeTerritories MUST return an error when the document is malformed.
func DecodeTerritories(document []byte) ([]model.Territory, error) {
	var territories []model.Territory
	if err := json.Unmarshal(document, &territories); err != nil {
		return nil, err
	}
	return territories, nil
}

// 📥️DecodeStatutes MUST return an error when the document is malformed.
func DecodeStatutes(document []byte) ([]model.Statute, error) {
	var statutes []model.Statute
	if err := json.Unmarshal(document, &statutes); err != nil {
		return nil, err
	}
	return statutes, nil
}

// 🔤️ParseTreeNodeKind MUST default to the structural category kind.
func ParseTreeNodeKind(value string) TreeNodeKind {
	switch value {
	case "technology", "bundle", "folder", "file", "section", "definition", "goal", "ticket", "draft", "todo", "policy", "breach", "contributor", "checkpoint", "session", "statute":
		return TreeNodeKind(value)
	}
	return TreeNodeCategory
}

// 🔤️ParseBreachPriority MUST default to the low priority the icon table also defaults to.
func ParseBreachPriority(value string) model.BreachPriority {
	switch value {
	case "high":
		return model.BreachPriorityHigh
	case "medium":
		return model.BreachPriorityMedium
	}
	return model.BreachPriorityLow
}

// #endregion 💿️Records
