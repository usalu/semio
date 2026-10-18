// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔌️ports carries the tree ports: render constants, artifact identification, entity rendering and the tree source and statute catalog interfaces.

// #endregion 🧲️Header

package tree

import (
	model "github.com/usalu/semio/repo/model"
)

// #region 🔌️Ports

// 🌿️TextBranchConnector is the connector drawn in front of a non-last child in the text tree.
const TextBranchConnector = "├️─️─️ "

// 🌿️TextLastConnector is the connector drawn in front of the last child in the text tree.
const TextLastConnector = "└️─️─️ "

// 🌿️TextPipeIndent is the indent carried down a subtree that still has following siblings.
const TextPipeIndent = "│️   "

// 🌿️TextBlankIndent is the indent carried down the last subtree of a level.
const TextBlankIndent = "    "

// 🟢️PriorityIconLow is the icon of a low-priority statute.
const PriorityIconLow = "🟢️"

// 🟡️PriorityIconMedium is the icon of a medium-priority statute.
const PriorityIconMedium = "🟡️"

// 🔴️PriorityIconHigh is the icon of a high-priority statute.
const PriorityIconHigh = "🔴️"

// 🎫️NoGoalLabel is the label of the synthetic goal that collects every ticket without a goal.
const NoGoalLabel = "No Goal"

// 📌️TreeCacheSchemaVersion is the schema version of the tree cache envelope.
const TreeCacheSchemaVersion = 3

// 🪪️ArtifactIdentifier mints the artifact id of one entity from its tree-node data.
type ArtifactIdentifier interface {
	ArtifactID(entityKind string, data map[string]interface{}) string
}

// 🎨️EntityRenderer renders one entity as a single line.
type EntityRenderer interface {
	Human(entityKind string, data map[string]interface{}) string
	Markdown(entityKind string, data map[string]interface{}) string
	MarkdownLink(entityKind string, data map[string]interface{}) string
}

// 📜️StatuteCatalog resolves statute metadata, ids, uris and labels.
type StatuteCatalog interface {
	Info(statute model.Statute) (model.StatuteMeta, bool)
	StatuteID(statute model.Statute) string
	StatuteURI(statute model.Statute) string
	StatuteLabel(statute model.Statute) string
	EntityKind(statute model.Statute) string
	TerritoryID(territory model.Territory) string
}

// 🗂️TreeSource supplies the already-loaded records the monorepo tree projects.
type TreeSource interface {
	Technologies() []TechnologyRecord
	Folders() []FolderRecord
	Files() []FileRecord
	GoalRecords() []GoalRecord
	TicketRecords() []TicketRecord
	Drafts() []DraftRecord
	Policies() []PolicyRecord
	Contributors() []ContributorRecord
	Checkpoints() []CheckpointRecord
	Sessions() []SessionRecord
	Sections(filePath string) []SectionRecord
}

// 🪪️DefaultArtifactIdentifier mints artifact ids through the repository's own id builder.
type DefaultArtifactIdentifier struct{}

// 🆔️ArtifactID MUST return the artifact id the repository's id builder mints.
func (DefaultArtifactIdentifier) ArtifactID(entityKind string, data map[string]interface{}) string {
	return model.GetArtifactID(entityKind, data)
}

// 🎨️DefaultEntityRenderer renders entities through the repository's own renderers.
type DefaultEntityRenderer struct{}

// 🖥️Human MUST render the human, single-line form.
func (DefaultEntityRenderer) Human(entityKind string, data map[string]interface{}) string {
	return model.RenderEntityHuman(entityKind, data, false)
}

// 📰️Markdown MUST render the markdown list-item form.
func (DefaultEntityRenderer) Markdown(entityKind string, data map[string]interface{}) string {
	return model.RenderEntityMarkdown(entityKind, data)
}

// 🔗️MarkdownLink MUST render the markdown link form used inside a goal tree line.
func (DefaultEntityRenderer) MarkdownLink(entityKind string, data map[string]interface{}) string {
	return model.RenderEntityMarkdownLink(entityKind, data)
}

// #endregion 🔌️Ports
