# Summary

VS Code extension source for repo tooling workflows.

# Docs

## .vscode-test.mjs

VS Code test-cli configuration entrypoint that defines the compiled test glob and Electron launch arguments for extension tests.

## 🟦extension.test.ts

VS Code extension integration tests covering command registration, diagnostics, sidebar view contributions, filter state behavior, and monorepo tree provider roots.

## 🟦extension.ts

Extension activation entrypoint that registers the two sidebar views (Monorepo and Filter) backed by tree data providers wired to a shared filter state source.
The Filter view exposes one item per filter kind with emoji + name labels, tooltip descriptions, and emoji-only menu actions for option toggles.
The Monorepo view applies the shared filter state across all branches and uses GraphQL-backed data retrieval via the repo CLI executor.
Section child rendering filters GraphQL section-interface children to section-typed nodes before building section rows so definitions are rendered only in definition rows.
URI resolution uses the `repo://` scheme. the `compose.navigate` command accepts either a `repo://` uri or a plain artifact id and resolves it to the appropriate resource. ticket and goal uris resolve directly to filesystem paths. file, folder, bundle, technology, section, and definition uris resolve via a tree node cache built from the cli `tree --json` output. the `compose.navigateto` command shows a quick pick of all cached tree nodes. a `vscode.urihandler` is registered for the `repo` scheme to handle external uri navigation. all tree items (including goals) have click-to-navigate commands.

## package.json

VS Code extension manifest with unscoped name for vsce packaging, command contributions, scripts, and engine compatibility for Cursor support.

## codegen/

Hand-maintained GraphQL typed document helpers for the VS Code extension (no codegen pipeline).

## codegen/graphql.ts

GraphQL operation result types used by the extension queries.

## Command Tree

The command browser groups actions by command families and subcommands, using the same command breakdown as the CLI so users learn one structure across tools.
Search matches either command labels or group names, and matching a group keeps its full subtree visible for quick discovery.

## Sections Explorer

The VS Code extension adds a Sections view to the built-in Explorer that lists nested regions for the active file via the repo section list.
Selecting a section navigates to its start, F2 triggers rename, drag-and-drop moves sections, and inline actions use repo commands to create child sections, rename sections, or delete them.
The view refreshes on editor focus and text changes so the tree stays aligned with the current file structure.
JSON files surface object keys as section entries so structured config files are navigable in the same tree.

## Breach Diagnostics

Shows policy breachs as diagnostics for all supported file types (TypeScript, JavaScript, JSON, Python, C#, Go):

- Loads cached breachs on file open for immediate feedback
- Re-runs `repo analyze` on file save and updates diagnostics
- Quick Fix actions to apply automated fixes via `repo autofix`
- Promotes preview editors opened from the Problems list into regular tabs for save-ready edits

## Kit Validation

Real-time validation for kit JSON files with Quick Fix code actions that apply `KitDiff`-based fixes.

## Sidebar

Tree views for tickets, policies, contributors, and commands with search and filter support.
Section tree expansion in the Monorepo view treats GraphQL section children as mixed interface nodes and only renders nodes identified as sections (`__typename: Section` or `section:` IDs), preventing definition entries from appearing twice.

## Tickets

Ticket tree items expose inline close or reopen actions that apply to the clicked ticket, list commit entries derived from ticket and interaction commits, and keep hover tooltips limited to the ticket description.

# 💯Requirements

## Sidebar

The repo sidebar MUST expose exactly two views: Monorepo and Filter.

The Filter view MUST represent each filter kind as a single item and expose filter options as view item menu actions.

Filter view items MUST render emoji plus name labels with tooltip descriptions; filter option menu actions MUST use emoji-only labels and MUST NOT use codeicons.

Filter state MUST apply globally to all Monorepo tree branches.

Monorepo root nodes MUST expand to show children for Technologies, Goals, Tickets, Policies, Contributors, and Commits.

## Tickets

Ticket tree items expose inline close and reopen actions that apply to the selected ticket based on status.

Ticket tree hovers show only the ticket description.

Ticket creation prompts for LLM and ticket UI selections.

Ticket tree items list commit entries as child nodes.

Ticket commands collect title/prompt/LLM for open, prompt/LLM for reopen, and operate on YYYY/MM/DD/SLUG ticket identifiers.

Ticket detail views consume git-derived per-file and total line stats stored on interactions and ticket close.

## Commands

Command trees mirror the CLI command and subcommand document; matching a command group keeps its subtree visible.

## Diagnostics

Problem list diagnostics open in pinned editor tabs for immediate saves.

Repo diagnostics and trees are driven by repo CLI ignore rules for gitignored files and repo directory content.

## Contributors

Contributor tree items list emails with mailto actions, links with external navigation, and contribution nodes with line summary descriptions.

Contributor contributions are grouped into commits, bundles, tickets (year/month/day), and files (folder/file) with navigation actions and inline ticket close/reopen actions.

## Sections View

The built-in Explorer hosts the Sections view; selecting a section navigates to it, F2 renames, drag-and-drop moves sections, JSON keys surface as sections, and inline actions create child sections, rename sections, and delete sections via repo commands.

The Sections view resolves the active file's section tree with line ranges so navigation and section actions match the current editor content.

Monorepo section tree rendering MUST include only section-typed section children and MUST exclude definition-typed children from section rows.

## General

Ticket tooling treats temporary artifacts as part of the active ticket workspace.

Devcontainer setup uninstalls any existing repo extension, clears stale VS Code and Cursor caches, then installs the workspace extension for VS Code, Cursor, Windsurf, and Antigravity on attach without manual installation actions, validating installs per detected editor IPC hook CLI and falling back to extensions directories with extensions.json registration on WSL-only CLI responses.

Extension engine compatibility targets the lowest supported editor version so Cursor accepts the packaged VSIX.

Sidebar view registration keeps a single filter view and monorepo view instance wired to the shared filter state.
