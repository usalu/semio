/** ⚙️ Extension data models
 *
 * Structured CLI output and the data models the extension reads from the repo CLI.
 */

// #region 🔌️Adapters
/**
 * Structured event emitted by the repo CLI binary.
 **/
export type RepoEvent = {
  kind: string;
  data?: unknown;
  result?: unknown;
  error?: { message?: string; fatal?: boolean };
  done?: { exit_code?: number };
};
// #endregion 🔌️Adapters

// #region ⚙️Types
// Types MUST define interfaces for repo events, tool results, and data models.

/**
 * Structured output from a repo CLI tool invocation.
 **/
export interface ToolResult<T = unknown> {
  output: { lines: { type: string; text: string }[]; exitCode: number };
  data?: T;
  error?: string;
}

/**
 * 🧩️ Repository project metadata for a workspace package.
 **/
export interface TechnologyData {
  name: string;
  kind?: string;
  root: string;
  sourceRoot?: string;
  projectType?: string;
  tags?: string[];
}

/**
 * Code policy configuration with id, name, and description.
 **/
export interface PolicyData {
  id: string;
  name: string;
  description: string;
}

/**
 * YAML frontmatter fields parsed from a ticket markdown file.
 **/
export interface TicketFrontmatter {
  status: string;
  prompt: string;
  summary?: string;
  author?: string;
  checkpoint?: string;
  started?: string;
  finished?: string;
  ignore?: boolean;
}

/**
 * Single interaction record within a ticket lifecycle.
 **/
export interface TicketInteraction {
  prompt: string;
  llm: string;
  effort?: string;
  client: string;
  author: string;
  date: string;
  checkpoint: string;
}

/**
 * Full ticket data including date, slug, frontmatter, and interactions.
 **/
export interface TicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: TicketFrontmatter;
  folderPath: string;
  interactions?: TicketInteraction[];
}

/**
 * Line-level contribution metrics for added and removed lines.
 **/
export interface ContributorLineMetrics {
  added: number;
  removed: number;
}

/**
 * Contributor metrics scoped to a single definition.
 **/
export interface ContributorDefinitionData {
  name: string;
  lines: ContributorLineMetrics;
}

/**
 * Contributor metrics scoped to a file section and its definitions.
 **/
export interface ContributorSectionData {
  name: string;
  lines: ContributorLineMetrics;
  definitions: ContributorDefinitionData[];
}

/**
 * Contributor metrics scoped to a single file and its sections.
 **/
export interface ContributorFileData {
  name: string;
  lines: ContributorLineMetrics;
  sections: ContributorSectionData[];
}

/**
 * Contributor metrics scoped to a folder and its files.
 **/
export interface ContributorFolderData {
  name: string;
  lines: ContributorLineMetrics;
  files: ContributorFileData[];
}

/**
 * Contributor metrics scoped to a bundle and its folders.
 **/
export interface ContributorBundleData {
  name: string;
  lines: ContributorLineMetrics;
  folders: ContributorFolderData[];
}

/**
 * Ticket metadata associated with a contributor.
 **/
export interface ContributorTicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  status: string;
  title: string;
  emoji: string;
  summary: string;
  folderPath?: string;
}

/**
 * Checkpoint metadata associated with a contributor.
 **/
export interface ContributorCheckpointData {
  title: string;
  sha: string;
}

/**
 * Full contributor profile with contributions across bundles, tickets, and checkpoints.
 **/
export interface ContributorData {
  github: string;
  name?: string;
  emails?: string[];
  links?: Record<string, string>;
  contributions?: {
    checkpoints: ContributorCheckpointData[];
    tickets: ContributorTicketData[];
    bundles: ContributorBundleData[];
  };
}

/**
 * TextEdit holds the data fields for a TextEdit record.
 **/
interface TextEdit {
  start: number;
  end: number;
  newText: string;
}

/**
 * AutoFix holds the data fields for a AutoFix record.
 **/
interface AutoFix {
  description: string;
  edits: Record<string, TextEdit[]>;
}

/**
 * Breach holds the data fields for a Breach record.
 **/
export interface Breach {
  id: string;
  summary: string;
  kind: { id: string };
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  autofix?: AutoFix;
}

/**
 * SectionInfo holds the data fields for a SectionInfo record.
 **/
export interface SectionInfo {
  name: string;
  kind: string;
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
  children: SectionInfo[];
}

/**
 * DefinitionInfo holds the data fields for a DefinitionInfo record.
 **/
interface DefinitionInfo {
  name: string;
  startLine: number;
  endLine: number;
  endIndex: number;
}

/**
 * GraphqlSectionRange holds the data fields for a GraphqlSectionRange record.
 **/
interface GraphqlSectionRange {
  start?: number;
  end?: number;
}

/**
 * GraphqlSection holds the data fields for a GraphqlSection record.
 **/
interface GraphqlSection {
  name: string;
  __typename?: string;
  children?: GraphqlSection[] | null;
}
// #endregion ⚙️Types
