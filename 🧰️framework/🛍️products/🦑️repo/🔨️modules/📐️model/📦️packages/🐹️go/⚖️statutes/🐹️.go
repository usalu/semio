// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⚖️statutes carries the statute kinds, the statute info lookup and the entity kind inference.

// #endregion 🧲️Header

package model

import (
	strings "strings"
)

// #region ⚖️Statutes

// 🔷️inferEntityKindFromStatute holds the data fields for a inferEntityKindFromStatute record.
func InferEntityKindFromStatute(statute Statute) string {
	path := string(statute)
	switch {
	case strings.Contains(path, "/definition/"):
		return "definition"
	case strings.Contains(path, "/section/"):
		return "section"
	case strings.Contains(path, "/file/"):
		return "file"
	case strings.Contains(path, "/folder/"):
		return "folder"
	case strings.Contains(path, "/bundle/"):
		return "bundle"
	case strings.Contains(path, "/technology/"):
		return "technology"
	default:
		return "repo"
	}
}

// 🟢️Statute represents a statute value.
type Statute string

const BreachCodeFileMissingHeaderRegion Statute = "code/file/missing-header-region"

const BreachCodeFileWrongHeaderRegionFormat Statute = "code/file/wrong-header-region-format"

const BreachCodeFileMissingContributors Statute = "code/file/missing-contributors"

const BreachCodeFileMissingSummary Statute = "code/file/missing-summary"

const BreachCodeFileMissingLicense Statute = "code/file/missing-license"

const BreachCodeFileWrongLicense Statute = "code/file/wrong-license"

const BreachCodeFileMissingRequirements Statute = "code/file/missing-requirements"

const BreachCodeFileMissingDocs Statute = "code/file/missing-docs"

const BreachCodeSectionEmpty Statute = "code/section/empty"

const BreachCodeSectionOrphanDefinition Statute = "code/section/orphan-definition"

const BreachCodeSectionMissingStartName Statute = "code/section/missing-start-name"

const BreachCodeSectionMissingEndName Statute = "code/section/missing-end-name"

const BreachCodeSectionNameMismatch Statute = "code/section/name-mismatch"

const BreachCodeSectionWrongFormat Statute = "code/section/wrong-format"

const BreachCodeSectionWrongFormatSummaryTooLong Statute = "code/section/wrong-format/summary/too-long-summary"

const BreachCodeSectionWrongFormatNewlineAfterRegion Statute = "code/section/wrong-format/newline-after-region"

const BreachCodeSectionWrongFormatRequirementsSplitBlock Statute = "code/section/wrong-format/requirements/split-block"

const BreachCodeSectionWrongFormatDocs Statute = "code/section/wrong-format/docs"

const BreachCodeSectionMissingSummary Statute = "code/section/missing-summary"

const BreachCodeSectionMissingRequirements Statute = "code/section/missing-requirements"

const BreachCodeSectionMissingDocs Statute = "code/section/missing-docs"

const BreachCodeDefWrongFormat Statute = "code/definition/wrong-format"

const BreachCodeDefNotNativeDocstring Statute = "code/definition/wrong-format/not-native-docstring"

const BreachCodeDefMissingSummary Statute = "code/definition/missing-summary"

const BreachCodeDefMissingRequirements Statute = "code/definition/missing-requirements"

const BreachCodeDefMissingDocs Statute = "code/definition/missing-docs"

const BreachCodeCommentInline Statute = "code/comment/inline"

const BreachCodeCommentBlock Statute = "code/comment/block"

const BreachCodeCommentJSDoc Statute = "code/comment/jsdoc"

const BreachCodeRequirementsSyntax Statute = "code/requirements/implementation-syntax"

const BreachCodeDocsMissingReadme Statute = "code/doc/missing-readme"

const BreachDevDocsMissingFile Statute = "dev-docs/missing-file"

const BreachDevDocsMissingFolder Statute = "dev-docs/missing-folder"

const BreachDevDocsWrongFilePath Statute = "dev-docs/wrong-file-path"

const BreachDevDocsWrongFolderPath Statute = "dev-docs/wrong-folder-path"

const BreachDevDocsWrongFileName Statute = "dev-docs/wrong-file-name"

const BreachDevDocsWrongFolderName Statute = "dev-docs/wrong-folder-name"

const BreachDevDocsWrongFileOrder Statute = "dev-docs/wrong-file-order"

const BreachDevDocsWrongFolderOrder Statute = "dev-docs/wrong-folder-order"

const BreachDevDocsMissingComponent Statute = "dev-docs/missing-component"

const BreachDevDocsWrongComponentName Statute = "dev-docs/wrong-component-name"

const BreachDevDocsWrongComponentOrder Statute = "dev-docs/wrong-component-order"

const BreachSketchpadImportThirdParty Statute = "sketchpad/import/third-party-outside-elements"

const BreachSketchpadStateMultipleMachines Statute = "sketchpad/state/multiple-machines"

const BreachSketchpadStateCreateActor Statute = "sketchpad/state/create-actor-usage"

const BreachSketchpadStateYjsAppState Statute = "sketchpad/state/yjs-app-state"

const BreachSketchpadStateForbiddenStore Statute = "sketchpad/state/forbidden-store"

const BreachSketchpadHooksNonTriadic Statute = "sketchpad/hook/non-triadic"

const BreachCodeUnicodeEmojiVariation Statute = "code/unicode/emoji-variation"

const BreachRepoMissingCommand Statute = "repo/missing-command"

const BreachRepoMissingTicketTracking Statute = "repo/missing-ticket-tracking"

const BreachSystemDevcontainerVscodeSettingsDrift Statute = "system/devcontainer/vscode/settings-drift"

const BreachSystemDevcontainerVscodeExtensionsDrift Statute = "system/devcontainer/vscode/extensions-drift"

const BreachFolderIllegalEmpty Statute = "folder/illegal/empty"

const BreachFolderNameMissingEmoji Statute = "folder/name/missing-emoji"

const BreachFolderNameGenericEmoji Statute = "folder/name/generic-emoji"

const BreachFolderNameNoncanonicalEmojiPresentation Statute = "folder/name/noncanonical-emoji-presentation"

const BreachFolderNameWhitespaceAfterEmoji Statute = "folder/name/whitespace-after-emoji"

const BreachFolderNameEmojiNotUnique Statute = "folder/name/emoji-not-unique"

const BreachFolderNameMultipleEmojis Statute = "folder/name/multiple-emojis"

const BreachFileIllegalUseGodfile Statute = "file/illegal/use-godfile"

const BreachFileNameMissingEmoji Statute = "file/name/missing-emoji"

const BreachFileNameGenericEmoji Statute = "file/name/generic-emoji"

const BreachFileNameNoncanonicalEmojiPresentation Statute = "file/name/noncanonical-emoji-presentation"

const BreachFileNameWhitespaceAfterEmoji Statute = "file/name/whitespace-after-emoji"

const BreachFileNameEmojiNotUnique Statute = "file/name/emoji-not-unique"

const BreachFileNameMultipleEmojis Statute = "file/name/multiple-emojis"

const BreachFileNameReservedEmoji Statute = "file/name/reserved-name-has-emoji"

const BreachComposeNoUiDependency Statute = "compose/import/no-ui-dependency"

const BreachDependencyBoundaryDirectImport Statute = "dependency-boundary/import/direct-third-party"

const BreachComposeDescriptionMissingEmoji Statute = "compose/description/missing-emoji"

const BreachComposeDescriptionEmojiNotUnique Statute = "compose/description/emoji-not-unique"

const BreachCodeRustRegionComment Statute = "code/rust/region-comment-instead-of-mod"

// 🔏️pathEmojiStatuteMeta returns the shared high-priority contract for path identity statutes.
func pathEmojiStatuteMeta(kind Statute) StatuteMeta {
	return StatuteMeta{
		Kind:        kind,
		Priority:    BreachPriorityHigh,
		Reason:      "Every non-reserved file and folder requires exactly one handpicked, meaningful emoji unique among all siblings; reserved filenames remain literal",
		Solution:    "Handpick one meaningful sibling-unique emoji, or restore the unprefixed reserved name, and update only resolved path references",
		Autofixable: false,
	}
}

// 📍️Info MUST return the metadata entry for the statute.
// ℹInfo returns the metadata for the statute.
func (k Statute) Info() StatuteMeta {
	if info, ok := StatuteInfoTable[k]; ok {
		return info
	}
	return StatuteMeta{
		Kind:        k,
		Priority:    BreachPriorityLow,
		Reason:      "Unknown breach",
		Solution:    "Fix the breach",
		Autofixable: false,
	}
}

// #endregion ⚖️Statutes
