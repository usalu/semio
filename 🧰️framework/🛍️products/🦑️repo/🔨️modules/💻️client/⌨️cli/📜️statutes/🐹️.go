// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Statute codes and info table.

// #endregion 🧲️Header

package client



// 🟢️Statute represents a statute value.
type Statute string

const (
	BreachCodeFileMissingHeaderRegion                  Statute = "code/file/missing-header-region"
	BreachCodeFileWrongHeaderRegionFormat              Statute = "code/file/wrong-header-region-format"
	BreachCodeFileMissingContributors                  Statute = "code/file/missing-contributors"
	BreachCodeFileMissingSummary                       Statute = "code/file/missing-summary"
	BreachCodeFileMissingLicense                       Statute = "code/file/missing-license"
	BreachCodeFileWrongLicense                         Statute = "code/file/wrong-license"
	BreachCodeFileMissingRequirements                  Statute = "code/file/missing-requirements"
	BreachCodeFileMissingDocs                          Statute = "code/file/missing-docs"
	BreachCodeSectionEmpty                             Statute = "code/section/empty"
	BreachCodeSectionOrphanDefinition                  Statute = "code/section/orphan-definition"
	BreachCodeSectionMissingStartName                  Statute = "code/section/missing-start-name"
	BreachCodeSectionMissingEndName                    Statute = "code/section/missing-end-name"
	BreachCodeSectionNameMismatch                      Statute = "code/section/name-mismatch"
	BreachCodeSectionWrongFormat                       Statute = "code/section/wrong-format"
	BreachCodeSectionWrongFormatSummaryTooLong         Statute = "code/section/wrong-format/summary/too-long-summary"
	BreachCodeSectionWrongFormatNewlineAfterRegion     Statute = "code/section/wrong-format/newline-after-region"
	BreachCodeSectionWrongFormatRequirementsSplitBlock Statute = "code/section/wrong-format/requirements/split-block"
	BreachCodeSectionWrongFormatDocs                   Statute = "code/section/wrong-format/docs"
	BreachCodeSectionMissingSummary                    Statute = "code/section/missing-summary"
	BreachCodeSectionMissingRequirements               Statute = "code/section/missing-requirements"
	BreachCodeSectionMissingDocs                       Statute = "code/section/missing-docs"
	BreachCodeDefWrongFormat                           Statute = "code/definition/wrong-format"
	BreachCodeDefNotNativeDocstring                    Statute = "code/definition/wrong-format/not-native-docstring"
	BreachCodeDefMissingSummary                        Statute = "code/definition/missing-summary"
	BreachCodeDefMissingRequirements                   Statute = "code/definition/missing-requirements"
	BreachCodeDefMissingDocs                           Statute = "code/definition/missing-docs"
	BreachCodeCommentInline                            Statute = "code/comment/inline"
	BreachCodeCommentBlock                             Statute = "code/comment/block"
	BreachCodeCommentJSDoc                             Statute = "code/comment/jsdoc"
	BreachCodeRequirementsSyntax                       Statute = "code/requirements/implementation-syntax"
	BreachCodeDocsMissingReadme                        Statute = "code/doc/missing-readme"
	BreachDevDocsMissingFile                           Statute = "dev-docs/missing-file"
	BreachDevDocsMissingFolder                         Statute = "dev-docs/missing-folder"
	BreachDevDocsWrongFilePath                         Statute = "dev-docs/wrong-file-path"
	BreachDevDocsWrongFolderPath                       Statute = "dev-docs/wrong-folder-path"
	BreachDevDocsWrongFileName                         Statute = "dev-docs/wrong-file-name"
	BreachDevDocsWrongFolderName                       Statute = "dev-docs/wrong-folder-name"
	BreachDevDocsWrongFileOrder                        Statute = "dev-docs/wrong-file-order"
	BreachDevDocsWrongFolderOrder                      Statute = "dev-docs/wrong-folder-order"
	BreachDevDocsMissingComponent                      Statute = "dev-docs/missing-component"
	BreachDevDocsWrongComponentName                    Statute = "dev-docs/wrong-component-name"
	BreachDevDocsWrongComponentOrder                   Statute = "dev-docs/wrong-component-order"
	BreachSketchpadImportThirdParty                    Statute = "sketchpad/import/third-party-outside-elements"
	BreachSketchpadStateMultipleMachines               Statute = "sketchpad/state/multiple-machines"
	BreachSketchpadStateCreateActor                    Statute = "sketchpad/state/create-actor-usage"
	BreachSketchpadStateYjsAppState                    Statute = "sketchpad/state/yjs-app-state"
	BreachSketchpadStateForbiddenStore                 Statute = "sketchpad/state/forbidden-store"
	BreachSketchpadHooksNonTriadic                     Statute = "sketchpad/hook/non-triadic"
	BreachCodeUnicodeEmojiVariation                    Statute = "code/unicode/emoji-variation"
	BreachRepoMissingCommand                           Statute = "repo/missing-command"
	BreachRepoMissingTicketTracking                    Statute = "repo/missing-ticket-tracking"
	BreachSystemIdeConfiguration                       Statute = "system/ide-configuration"
	BreachFolderIllegalEmpty                           Statute = "folder/illegal/empty"
	BreachFolderNameMissingEmoji                       Statute = "folder/name/missing-emoji"
	BreachFolderNameGenericEmoji                       Statute = "folder/name/generic-emoji"
	BreachFolderNameNoncanonicalEmojiPresentation      Statute = "folder/name/noncanonical-emoji-presentation"
	BreachFolderNameWhitespaceAfterEmoji               Statute = "folder/name/whitespace-after-emoji"
	BreachFolderNameEmojiNotUnique                     Statute = "folder/name/emoji-not-unique"
	BreachFolderNameMultipleEmojis                     Statute = "folder/name/multiple-emojis"
	BreachFileIllegalUseGodfile                        Statute = "file/illegal/use-godfile"
	BreachFileNameMissingEmoji                         Statute = "file/name/missing-emoji"
	BreachFileNameGenericEmoji                         Statute = "file/name/generic-emoji"
	BreachFileNameNoncanonicalEmojiPresentation        Statute = "file/name/noncanonical-emoji-presentation"
	BreachFileNameWhitespaceAfterEmoji                 Statute = "file/name/whitespace-after-emoji"
	BreachFileNameEmojiNotUnique                       Statute = "file/name/emoji-not-unique"
	BreachFileNameMultipleEmojis                       Statute = "file/name/multiple-emojis"
	BreachFileNameReservedEmoji                        Statute = "file/name/reserved-name-has-emoji"
	BreachComposeNoUiDependency                        Statute = "compose/import/no-ui-dependency"
	BreachDependencyBoundaryDirectImport               Statute = "dependency-boundary/import/direct-third-party"
	BreachComposeDescriptionMissingEmoji               Statute = "compose/description/missing-emoji"
	BreachComposeDescriptionEmojiNotUnique             Statute = "compose/description/emoji-not-unique"
	BreachCodeRustRegionComment                        Statute = "code/rust/region-comment-instead-of-mod"
)

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

// 📊️statuteInfoTable holds the data fields for a statuteInfoTable record.
var statuteInfoTable = map[Statute]StatuteMeta{
	BreachRepoMissingCommand: {
		Kind:        BreachRepoMissingCommand,
		Priority:    BreachPriorityHigh,
		Reason:      "Command is missing from parity implementation (CLI, MCP, VS Code)",
		Solution:    "Implement the command in the missing platform",
		Autofixable: false,
	},
	BreachRepoMissingTicketTracking: {
		Kind:        BreachRepoMissingTicketTracking,
		Priority:    BreachPriorityHigh,
		Reason:      "Ticket tracking code is missing or incomplete",
		Solution:    "Implement strict ticket tracking (open/close/log)",
		Autofixable: false,
	},
	BreachCodeFileMissingHeaderRegion: {
		Kind:        BreachCodeFileMissingHeaderRegion,
		Priority:    BreachPriorityLow,
		Reason:      "Header region with license, filename, and contributors is required",
		Solution:    "Add header region with SPDX license, filename, and contributors",
		Autofixable: true,
	},
	BreachCodeFileWrongHeaderRegionFormat: {
		Kind:        BreachCodeFileWrongHeaderRegionFormat,
		Priority:    BreachPriorityLow,
		Reason:      "Header region format is incorrect (missing License or Requirements subregion)",
		Solution:    "Add License and Requirements subregions inside Header",
		Autofixable: false,
	},
	BreachCodeFileMissingContributors: {
		Kind:        BreachCodeFileMissingContributors,
		Priority:    BreachPriorityLow,
		Reason:      "Contributors must be documented in header",
		Solution:    "Add contributor line in header region",
		Autofixable: false,
	},
	BreachCodeFileMissingSummary: {
		Kind:        BreachCodeFileMissingSummary,
		Priority:    BreachPriorityLow,
		Reason:      "Summary must be documented in header",
		Solution:    "Add summary line in header region after the file ID",
		Autofixable: false,
	},
	BreachCodeFileMissingLicense: {
		Kind:        BreachCodeFileMissingLicense,
		Priority:    BreachPriorityLow,
		Reason:      "License text is required in header License subregion",
		Solution:    "Add AGPL license text in License subregion",
		Autofixable: true,
	},
	BreachCodeFileWrongLicense: {
		Kind:        BreachCodeFileWrongLicense,
		Priority:    BreachPriorityLow,
		Reason:      "License must be AGPL-3.0-or-later",
		Solution:    "Replace license with AGPL-3.0-or-later",
		Autofixable: true,
	},
	BreachCodeFileMissingRequirements: {
		Kind:        BreachCodeFileMissingRequirements,
		Priority:    BreachPriorityLow,
		Reason:      "Requirements subregion is required inside Header",
		Solution:    "Add Requirements subregion inside Header",
		Autofixable: false,
	},
	BreachCodeFileMissingDocs: {
		Kind:        BreachCodeFileMissingDocs,
		Priority:    BreachPriorityLow,
		Reason:      "File must be documented in bundle README.md Docs section",
		Solution:    "Add file documentation to bundle README.md",
		Autofixable: false,
	},
	BreachCodeSectionEmpty: {
		Kind:        BreachCodeSectionEmpty,
		Priority:    BreachPriorityLow,
		Reason:      "Empty sections should be removed",
		Solution:    "Remove empty section or add content",
		Autofixable: true,
	},
	BreachCodeSectionOrphanDefinition: {
		Kind:        BreachCodeSectionOrphanDefinition,
		Priority:    BreachPriorityLow,
		Reason:      "All code must be inside named sections",
		Solution:    "Move code into an existing section or add a new section",
		Autofixable: false,
	},
	BreachCodeSectionMissingStartName: {
		Kind:        BreachCodeSectionMissingStartName,
		Priority:    BreachPriorityLow,
		Reason:      "Section start marker must have a name",
		Solution:    "Add name to section start marker",
		Autofixable: false,
	},
	BreachCodeSectionMissingEndName: {
		Kind:        BreachCodeSectionMissingEndName,
		Priority:    BreachPriorityLow,
		Reason:      "Section end marker should have matching name",
		Solution:    "Add matching name to section end marker",
		Autofixable: true,
	},
	BreachCodeSectionNameMismatch: {
		Kind:        BreachCodeSectionNameMismatch,
		Priority:    BreachPriorityLow,
		Reason:      "Section start and end names must match",
		Solution:    "Fix section end name to match start name",
		Autofixable: true,
	},
	BreachCodeSectionWrongFormat: {
		Kind:        BreachCodeSectionWrongFormat,
		Priority:    BreachPriorityLow,
		Reason:      "Section region marker format is incorrect",
		Solution:    "Use correct region marker format with emoji bookmark",
		Autofixable: false,
	},
	BreachCodeSectionWrongFormatSummaryTooLong: {
		Kind:        BreachCodeSectionWrongFormatSummaryTooLong,
		Priority:    BreachPriorityLow,
		Reason:      "Section summary must not exceed 256 characters",
		Solution:    "Shorten the section summary to 256 characters or less",
		Autofixable: false,
	},
	BreachCodeSectionWrongFormatNewlineAfterRegion: {
		Kind:        BreachCodeSectionWrongFormatNewlineAfterRegion,
		Reason:      "No blank lines allowed between region start marker and region information comments",
		Solution:    "Remove blank lines between region start marker and the first comment line",
		Autofixable: true,
	},
	BreachCodeSectionWrongFormatRequirementsSplitBlock: {
		Kind:        BreachCodeSectionWrongFormatRequirementsSplitBlock,
		Priority:    BreachPriorityLow,
		Reason:      "Spec comment block must be contiguous without blank lines",
		Solution:    "Remove blank lines between spec comment lines",
		Autofixable: true,
	},
	BreachCodeSectionWrongFormatDocs: {
		Kind:        BreachCodeSectionWrongFormatDocs,
		Priority:    BreachPriorityLow,
		Reason:      "Section docs format is incorrect",
		Solution:    "Fix the section docs format",
		Autofixable: false,
	},
	BreachCodeSectionMissingSummary: {
		Kind:        BreachCodeSectionMissingSummary,
		Priority:    BreachPriorityLow,
		Reason:      "Section must have a summary comment after the region start",
		Solution:    "Add summary comment after section region start marker",
		Autofixable: true,
	},
	BreachCodeSectionMissingRequirements: {
		Kind:        BreachCodeSectionMissingRequirements,
		Priority:    BreachPriorityLow,
		Reason:      "Section must have requirements comments after the summary",
		Solution:    "Add requirements comments after section summary",
		Autofixable: false,
	},
	BreachCodeSectionMissingDocs: {
		Kind:        BreachCodeSectionMissingDocs,
		Priority:    BreachPriorityLow,
		Reason:      "Section must be documented in bundle README.md Docs section",
		Solution:    "Add section documentation to bundle README.md",
		Autofixable: false,
	},
	BreachCodeDefWrongFormat: {
		Kind:        BreachCodeDefWrongFormat,
		Priority:    BreachPriorityLow,
		Reason:      "Definition does not have a proper docstring",
		Solution:    "Add language-native docstring to definition",
		Autofixable: false,
	},
	BreachCodeDefNotNativeDocstring: {
		Kind:        BreachCodeDefNotNativeDocstring,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must use language-native docstring format (JSDoc for TS/JS, /// for C#/Rust)",
		Solution:    "Convert line comments to language-native docstring format",
		Autofixable: true,
	},
	BreachCodeDefMissingSummary: {
		Kind:        BreachCodeDefMissingSummary,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must have a summary in its docstring",
		Solution:    "Add summary line to definition docstring",
		Autofixable: true,
	},
	BreachCodeDefMissingRequirements: {
		Kind:        BreachCodeDefMissingRequirements,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must have requirements in its docstring",
		Solution:    "Add requirements to definition docstring",
		Autofixable: true,
	},
	BreachCodeDefMissingDocs: {
		Kind:        BreachCodeDefMissingDocs,
		Priority:    BreachPriorityLow,
		Reason:      "Definition must be documented in bundle README.md Docs section",
		Solution:    "Add definition documentation to bundle README.md",
		Autofixable: false,
	},
	BreachCodeCommentInline: {
		Kind:        BreachCodeCommentInline,
		Priority:    BreachPriorityLow,
		Reason:      "Inline comments are forbidden",
		Solution:    "Remove inline comment",
		Autofixable: true,
	},
	BreachCodeCommentBlock: {
		Kind:        BreachCodeCommentBlock,
		Priority:    BreachPriorityLow,
		Reason:      "Block comments are forbidden",
		Solution:    "Remove block comment",
		Autofixable: true,
	},
	BreachCodeCommentJSDoc: {
		Kind:        BreachCodeCommentJSDoc,
		Priority:    BreachPriorityLow,
		Reason:      "JSDoc comments are forbidden",
		Solution:    "Remove JSDoc comment",
		Autofixable: true,
	},
	BreachCodeRequirementsSyntax: {
		Kind:        BreachCodeRequirementsSyntax,
		Priority:    BreachPriorityLow,
		Reason:      "Requirements must be implementation-agnostic and must not contain code syntax",
		Solution:    "Remove backticks, function calls, and other code syntax from spec text",
		Autofixable: false,
	},
	BreachCodeDocsMissingReadme: {
		Kind:        BreachCodeDocsMissingReadme,
		Priority:    BreachPriorityLow,
		Reason:      "Bundle or folder is missing a README.md with summary and requirements",
		Solution:    "Add a README.md with # Summary and # 💯️Requirements sections",
		Autofixable: false,
	},
	BreachCodeUnicodeEmojiVariation: {
		Kind:        BreachCodeUnicodeEmojiVariation,
		Priority:    BreachPriorityLow,
		Reason:      "Emoji variation selectors (VS15/VS16) are forbidden",
		Solution:    "Strip variation selectors from emoji",
		Autofixable: true,
	},
	BreachDevDocsMissingFile: {
		Kind:        BreachDevDocsMissingFile,
		Priority:    BreachPriorityLow,
		Reason:      "File exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📄️PATH section in AGENTS.md",
		Autofixable: true,
	},
	BreachDevDocsMissingFolder: {
		Kind:        BreachDevDocsMissingFolder,
		Priority:    BreachPriorityLow,
		Reason:      "Folder exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📁️PATH section in AGENTS.md",
		Autofixable: true,
	},
	BreachDevDocsWrongFilePath: {
		Kind:        BreachDevDocsWrongFilePath,
		Priority:    BreachPriorityLow,
		Reason:      "File section path does not match actual file path",
		Solution:    "Update file section path to match actual path",
		Autofixable: true,
	},
	BreachDevDocsWrongFolderPath: {
		Kind:        BreachDevDocsWrongFolderPath,
		Priority:    BreachPriorityLow,
		Reason:      "Folder section path does not match actual folder path",
		Solution:    "Update folder section path to match actual path",
		Autofixable: true,
	},
	BreachDevDocsWrongFileName: {
		Kind:        BreachDevDocsWrongFileName,
		Priority:    BreachPriorityLow,
		Reason:      "File section name format is incorrect (should be ## 📄️PATH)",
		Solution:    "Rename section to ## 📄️PATH",
		Autofixable: true,
	},
	BreachDevDocsWrongFolderName: {
		Kind:        BreachDevDocsWrongFolderName,
		Priority:    BreachPriorityLow,
		Reason:      "Folder section name format is incorrect (should be ## 📁️PATH/)",
		Solution:    "Rename section to ## 📁️PATH/",
		Autofixable: true,
	},
	BreachDevDocsWrongFileOrder: {
		Kind:        BreachDevDocsWrongFileOrder,
		Priority:    BreachPriorityLow,
		Reason:      "File sections are not in alphabetical order",
		Solution:    "Reorder file sections alphabetically",
		Autofixable: true,
	},
	BreachDevDocsWrongFolderOrder: {
		Kind:        BreachDevDocsWrongFolderOrder,
		Priority:    BreachPriorityLow,
		Reason:      "Folder sections are not in alphabetical order",
		Solution:    "Reorder folder sections alphabetically",
		Autofixable: true,
	},
	BreachDevDocsMissingComponent: {
		Kind:        BreachDevDocsMissingComponent,
		Priority:    BreachPriorityLow,
		Reason:      "Package.json workspace has no corresponding component in README.md",
		Solution:    "Add component section in README.md Components",
		Autofixable: true,
	},
	BreachDevDocsWrongComponentName: {
		Kind:        BreachDevDocsWrongComponentName,
		Priority:    BreachPriorityLow,
		Reason:      "Component section name does not match workspace name",
		Solution:    "Rename component section to match workspace",
		Autofixable: true,
	},
	BreachDevDocsWrongComponentOrder: {
		Kind:        BreachDevDocsWrongComponentOrder,
		Priority:    BreachPriorityLow,
		Reason:      "Component sections are not in package.json workspaces order",
		Solution:    "Reorder components to match package.json workspaces",
		Autofixable: true,
	},
	BreachSketchpadImportThirdParty: {
		Kind:        BreachSketchpadImportThirdParty,
		Priority:    BreachPriorityHigh,
		Reason:      "Third party imports must only be in elements.tsx",
		Solution:    "Move third party import to elements.tsx and re-export from there",
		Autofixable: false,
	},
	BreachSketchpadStateMultipleMachines: {
		Kind:        BreachSketchpadStateMultipleMachines,
		Priority:    BreachPriorityHigh,
		Reason:      "Only one state machine is allowed (createMachine can only be used once)",
		Solution:    "Consolidate state management into a single state machine",
		Autofixable: false,
	},
	BreachSketchpadStateCreateActor: {
		Kind:        BreachSketchpadStateCreateActor,
		Priority:    BreachPriorityHigh,
		Reason:      "createActor is forbidden in sketchpad",
		Solution:    "Remove createActor usage and use the single state machine instead",
		Autofixable: false,
	},
	BreachSketchpadStateYjsAppState: {
		Kind:        BreachSketchpadStateYjsAppState,
		Priority:    BreachPriorityHigh,
		Reason:      "Yjs app state is forbidden in sketchpad state management",
		Solution:    "Keep app state in the single state machine and use Yjs only for synchronized document data",
		Autofixable: false,
	},
	BreachSketchpadStateForbiddenStore: {
		Kind:        BreachSketchpadStateForbiddenStore,
		Priority:    BreachPriorityHigh,
		Reason:      "Stores outside of State Management sections are forbidden",
		Solution:    "Move store to a State Management section or remove it",
		Autofixable: false,
	},
	BreachSketchpadHooksNonTriadic: {
		Kind:        BreachSketchpadHooksNonTriadic,
		Priority:    BreachPriorityHigh,
		Reason:      "Client elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()",
		Solution:    "Refactor to use triadic hook pattern with useSELECTOR",
		Autofixable: false,
	},
	BreachSystemIdeConfiguration: {
		Kind:        BreachSystemIdeConfiguration,
		Priority:    BreachPriorityHigh,
		Reason:      "Developer commands and configuration belong to the repository dashboard",
		Solution:    "Remove editor-owned configuration and register the command in the repository dashboard",
		Autofixable: false,
	},
	BreachFolderIllegalEmpty: {
		Kind:        BreachFolderIllegalEmpty,
		Priority:    BreachPriorityLow,
		Reason:      "Empty folders are not allowed",
		Solution:    "Remove the empty folder",
		Autofixable: true,
	},
	BreachFolderNameMissingEmoji:                  pathEmojiStatuteMeta(BreachFolderNameMissingEmoji),
	BreachFolderNameGenericEmoji:                  pathEmojiStatuteMeta(BreachFolderNameGenericEmoji),
	BreachFolderNameNoncanonicalEmojiPresentation: pathEmojiStatuteMeta(BreachFolderNameNoncanonicalEmojiPresentation),
	BreachFolderNameWhitespaceAfterEmoji:          pathEmojiStatuteMeta(BreachFolderNameWhitespaceAfterEmoji),
	BreachFolderNameEmojiNotUnique:                pathEmojiStatuteMeta(BreachFolderNameEmojiNotUnique),
	BreachFolderNameMultipleEmojis:                pathEmojiStatuteMeta(BreachFolderNameMultipleEmojis),
	BreachFileIllegalUseGodfile: {
		Kind:        BreachFileIllegalUseGodfile,
		Priority:    BreachPriorityHigh,
		Reason:      "File is not listed in .🧬semio/🦑️repo/📁️files.json godfile",
		Solution:    "Add the file to .🧬semio/🦑️repo/📁️files.json or remove it",
		Autofixable: false,
	},
	BreachFileNameMissingEmoji:                  pathEmojiStatuteMeta(BreachFileNameMissingEmoji),
	BreachFileNameGenericEmoji:                  pathEmojiStatuteMeta(BreachFileNameGenericEmoji),
	BreachFileNameNoncanonicalEmojiPresentation: pathEmojiStatuteMeta(BreachFileNameNoncanonicalEmojiPresentation),
	BreachFileNameWhitespaceAfterEmoji:          pathEmojiStatuteMeta(BreachFileNameWhitespaceAfterEmoji),
	BreachFileNameEmojiNotUnique:                pathEmojiStatuteMeta(BreachFileNameEmojiNotUnique),
	BreachFileNameMultipleEmojis:                pathEmojiStatuteMeta(BreachFileNameMultipleEmojis),
	BreachFileNameReservedEmoji:                 pathEmojiStatuteMeta(BreachFileNameReservedEmoji),
	BreachComposeNoUiDependency: {
		Kind:        BreachComposeNoUiDependency,
		Priority:    BreachPriorityHigh,
		Reason:      "compose.* files must be self-contained and not import from UI dependencies",
		Solution:    "Remove the UI dependency import and use only non-UI alternatives",
		Autofixable: false,
	},
	BreachDependencyBoundaryDirectImport: {
		Kind:        BreachDependencyBoundaryDirectImport,
		Priority:    BreachPriorityHigh,
		Reason:      "Third-party packages must only be imported in adapter regions or adapter paths",
		Solution:    "Move the import into a //#region 🔌️Adapter (or /adapters/) module and depend on a first-party port interface elsewhere",
		Autofixable: false,
	},
	BreachComposeDescriptionMissingEmoji: {
		Kind:        BreachComposeDescriptionMissingEmoji,
		Priority:    BreachPriorityMedium,
		Reason:      "Every entity description must start with an emoji as the first symbol",
		Solution:    "Add a leading emoji to the description",
		Autofixable: false,
	},
	BreachComposeDescriptionEmojiNotUnique: {
		Kind:        BreachComposeDescriptionEmojiNotUnique,
		Priority:    BreachPriorityMedium,
		Reason:      "Every entity must have a unique emoji among its siblings",
		Solution:    "Change the leading emoji to one that is not already used by a sibling entity",
		Autofixable: false,
	},
	BreachCodeRustRegionComment: {
		Kind:        BreachCodeRustRegionComment,
		Priority:    BreachPriorityHigh,
		Reason:      "Rust files must use mod blocks for sections instead of region comments",
		Solution:    "Replace // #region with mod section_name { // 🔖️Name and // #endregion with } // 🔖️Name",
		Autofixable: false,
	},
}

// 📍️Info MUST return the metadata entry for the statute.
// ℹInfo returns the metadata for the statute.
func (k Statute) Info() StatuteMeta {
	if info, ok := statuteInfoTable[k]; ok {
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
