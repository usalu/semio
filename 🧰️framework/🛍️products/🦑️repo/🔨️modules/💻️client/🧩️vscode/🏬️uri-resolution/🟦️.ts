/** 🏬️ Repo URI resolution
 *
 * Tree node cache and labels, CLI tree arguments and navigation to repo URIs.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { acquireCliSlot, releaseCliSlot } from "../🎩️globals/🟦️.ts";
import { execFileAsync, extractRepoResult, getGitHubRepoBaseUrl, getRepoCommand, getWorkspaceRoot, logError, parseRepoEvents } from "../🎼️utilities/🟦️.ts";
import { openFileAtLine } from "../🎵️path-navigation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🏬️URI Resolution
// URI Resolution MUST handle parsing, tree node caching, and repo URI navigation.

/**
 * Tree node data structure representing a monorepo artifact in the sidebar tree.
 **/
export interface TreeNodeData {
  Kind: string;
  ID: string;
  Label: string;
  URI: string;
  SubKind?: string;
  Description?: string;
  Year?: number;
  Month?: number;
  Day?: number;
  Status?: string;
  Contributor?: string;
  Data?: Record<string, any>;
  Children?: TreeNodeData[];
}

/**
 * treeNodeCache.current holds the data fields for a treeNodeCache.current record.
 **/
const treeNodeCache = ephemeralBox<Map<string, TreeNodeData> | null>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.treeNodeCache", null);
/**
 * treeRootCache.current holds the data fields for a treeRootCache.current record.
 **/
const treeRootCache = ephemeralBox<TreeNodeData | null>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.treeRootCache", null);
/**
 * treeNodeCacheTime.current holds the data fields for a treeNodeCacheTime.current record.
 **/
const treeNodeCacheTime = ephemeralBox("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.treeNodeCacheTime", 0);
/**
 * TREE_CACHE_TTL holds the data fields for a TREE_CACHE_TTL record.
 **/
const TREE_CACHE_TTL = 30000;

/**
 * Extracts the leading emoji characters from a text string.
 * Implementations MUST use Unicode emoji properties to detect the prefix.
 **/
export function extractLeadingEmoji(text: string): string {
  const match = text.match(/^[\p{Emoji_Presentation}\p{Extended_Pictographic}][\u{FE0E}\u{FE0F}\u{200D}\p{Emoji_Component}]*/u);
  return match ? match[0] : "";
}

/**
 * Computes the display label for a tree node including emoji prefix and status icon.
 * Implementations MUST prepend the node emoji and status indicator to the label.
 **/
export function treeNodeDisplayLabel(node: TreeNodeData): string {
  if (node.Kind === "category") return node.Label;
  const emoji = extractLeadingEmoji(node.ID);
  let statusIcon = "";
  if (node.Status === "open") statusIcon = "🔵️";
  else if (node.Status === "closed") statusIcon = "🟢️";
  const fallbackEmojis: Record<string, string> = {
    contributor: "🧑️‍💻️",
    checkpoint: "🔀️",
    policy: "👮️",
    statute: "⚠️",
  };
  const prefix = emoji || fallbackEmojis[node.Kind] || "";
  let label = node.Label;
  if (prefix && label.startsWith(prefix)) {
    label = label.substring(prefix.length);
  }
  return `${prefix}${statusIcon}${label}`;
}

/**
 * Returns the VS Code context value for a tree node based on its kind and status.
 * Implementations MUST distinguish open and closed tickets.
 **/
export function treeNodeContextValue(node: TreeNodeData): string {
  if (node.Kind === "ticket") return node.Status === "open" ? "ticketOpen" : "ticketClosed";
  return node.Kind;
}

/**
 * Returns the VS Code command to execute when a tree node is clicked.
 * Implementations MUST return undefined for category nodes and navigate for others.
 **/
export function treeNodeCommand(node: TreeNodeData): vscode.Command | undefined {
  if (node.Kind === "category") return undefined;
  if (node.URI) return { command: "compose.navigate", title: "Navigate", arguments: [node.URI] };
  return undefined;
}

/**
 * Converts text to an uppercase slug with non-alphanumeric characters replaced by hyphens.
 * Implementations MUST uppercase the input and strip leading and trailing hyphens.
 **/
export function slugify(text: string): string {
  return text
    .toUpperCase()
    .replace(/[^A-Z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}

/**
 * flattenTree holds the data fields for a flattenTree record.
 **/
function flattenTree(node: TreeNodeData, result: Map<string, TreeNodeData>): void {
  if (node.URI) {
    result.set(node.URI, node);
  }
  if (node.Children) {
    for (const child of node.Children) {
      flattenTree(child, result);
    }
  }
}

/**
 * getTreeNodeCache holds the data fields for a getTreeNodeCache record.
 **/
export async function getTreeNodeCache(): Promise<Map<string, TreeNodeData>> {
  const now = Date.now();
  if (treeNodeCache.current && now - treeNodeCacheTime.current < TREE_CACHE_TTL) {
    return treeNodeCache.current;
  }
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) return new Map();
  await acquireCliSlot();
  try {
    const { stdout } = await execFileAsync(command, ["--json", "search"], { cwd: root, timeout: 60000, maxBuffer: 50 * 1024 * 1024 });
    if (!stdout.trim()) return treeNodeCache.current ?? new Map();
    const events = parseRepoEvents(stdout);
    const result = extractRepoResult(events);
    const tree = result.data as TreeNodeData | undefined;
    if (tree) {
      const cache = new Map<string, TreeNodeData>();
      flattenTree(tree, cache);
      treeNodeCache.current = cache;
      treeRootCache.current = tree;
      treeNodeCacheTime.current = now;
      return cache;
    }
  } catch (error) {
    logError("[getTreeNodeCache] error:", error);
  } finally {
    releaseCliSlot();
  }
  return treeNodeCache.current ?? new Map();
}

/**
 * getTreeRoot holds the data fields for a getTreeRoot record.
 **/
export async function getTreeRoot(): Promise<TreeNodeData | null> {
  await getTreeNodeCache();
  return treeRootCache.current;
}

/**
 * fetchTreeWithArgs holds the data fields for a fetchTreeWithArgs record.
 **/
export async function fetchTreeWithArgs(args: string[]): Promise<TreeNodeData | null> {
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) return null;
  try {
    const fullArgs = ["--json", "search", ...args];
    const { stdout } = await execFileAsync(command, fullArgs, { cwd: root, timeout: 60000, maxBuffer: 50 * 1024 * 1024 });
    if (!stdout.trim()) return null;
    const events = parseRepoEvents(stdout);
    const result = extractRepoResult(events);
    return result.data as TreeNodeData | null;
  } catch (error) {
    logError("[fetchTreeWithArgs] error:", error);
    return null;
  }
}

/**
 * Clears the cached tree node data forcing a fresh fetch on next access.
 * Implementations MUST reset all cache fields and the timestamp.
 **/
export function invalidateTreeNodeCache(): void {
  treeNodeCache.current = null;
  treeRootCache.current = null;
  treeNodeCacheTime.current = 0;
}

/**
 * Parses a repo URI into its type and path components.
 * Implementations MUST return null for URIs that do not match the repo scheme.
 **/
export function parseUri(uri: string): { type: string; path: string } | null {
  const match = uri.match(/^repo:\/\/([a-zA-Z]+)(?:\/(.*)?)?$/);
  if (!match) return null;
  return { type: match[1], path: match[2] ? decodeURIComponent(match[2]) : "" };
}

/**
 * navigateToUri holds the data fields for a navigateToUri record.
 **/
export async function navigateToUri(uri: string): Promise<void> {
  const wsRoot = getWorkspaceRoot();
  if (!wsRoot) return;
  const parsed = parseUri(uri);
  if (!parsed) return;

  const cache = await getTreeNodeCache();
  const node = cache.get(uri);

  switch (parsed.type) {
    case "repo": {
      return vscode.commands.executeCommand("compose.monorepo.focus") as any;
    }
    case "cb":
    case "technologies":
    case "bundles":
    case "tickets":
    case "goals":
    case "drafts":
    case "todos":
    case "policies":
    case "statutes":
    case "contributors":
    case "checkpoints":
    case "folders":
    case "files":
    case "sections":
    case "definitions": {
      return vscode.commands.executeCommand("compose.monorepo.focus") as any;
    }
    case "ticket": {
      let ticketPath = "";
      if (node && node.Year && node.Month && node.Day && node.Data?.slug) {
        const year = `🎆️${String(node.Year).padStart(2, "0")}`;
        const month = `🌙️${String(node.Month).padStart(2, "0")}`;
        const day = `☀️${String(node.Day).padStart(2, "0")}`;
        ticketPath = path.join(wsRoot, ".🧬semio", "🦑️repo", "🎫️tickets", year, month, day, node.Data.slug);
      }

      if (ticketPath && fs.existsSync(ticketPath)) {
        return vscode.commands.executeCommand("compose.navigateToFile", ticketPath) as any;
      }
      break;
    }
    case "goal": {
      const goalId = node?.Data?.id || parsed.path;
      const goalJsonPath = path.join(wsRoot, ".🧬semio", "🦑️repo", "🎯️goals", goalId, "🎯️goal.json");
      if (fs.existsSync(goalJsonPath)) {
        return vscode.commands.executeCommand("compose.navigateToFile", goalJsonPath) as any;
      }
      break;
    }
    case "draft": {
      const slug = node?.Data?.slug || node?.Data?.id || parsed.path;
      const draftPath = path.join(wsRoot, ".🧬semio", "🦑️repo", "✍️notes", slug);
      if (fs.existsSync(draftPath)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(draftPath)) as any;
      }
      break;
    }
    case "todo": {
      const slug = node?.Data?.slug || node?.Data?.id || parsed.path;
      const todoPath = path.join(wsRoot, ".🧬semio", "🦑️repo", "todos", slug);
      if (fs.existsSync(todoPath)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(todoPath)) as any;
      }
      break;
    }
    case "contributor": {
      const github = node?.Data?.github || parsed.path;
      return vscode.env.openExternal(vscode.Uri.parse(`https://github.com/${encodeURIComponent(github)}`)) as any;
    }
    case "checkpoint": {
      const sha = node?.Data?.sha || parsed.path;
      const baseUrl = getGitHubRepoBaseUrl();
      if (baseUrl) {
        return vscode.env.openExternal(vscode.Uri.parse(`${baseUrl}/commit/${encodeURIComponent(sha)}`)) as any;
      }
      break;
    }
    case "technology": {
      if (node?.Data?.path) {
        const abs = path.join(wsRoot, node.Data.path);
        if (fs.existsSync(abs)) {
          return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
        }
      }
      break;
    }
    case "bundle": {
      if (node?.Data?.root) {
        const abs = path.join(wsRoot, node.Data.root);
        if (fs.existsSync(abs)) {
          return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
        }
      }

      const parts = parsed.path.split("/");
      if (parts.length >= 2) {
      }
      break;
    }
    case "folder": {
      if (node?.Data?.path) {
        const abs = path.join(wsRoot, node.Data.path);
        if (fs.existsSync(abs)) {
          return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
        }
      }
      break;
    }
    case "file": {
      const filePath = node?.Data?.path || parsed.path;
      const abs = path.join(wsRoot, filePath);
      if (fs.existsSync(abs)) {
        return vscode.commands.executeCommand("compose.navigateToFile", filePath) as any;
      }
      break;
    }
    case "section": {
      if (node?.Data?.startLine && node?.Data?.path) {
        return openFileAtLine(node.Data.path, node.Data.startLine, node.Data.endLine);
      }
      if (node?.Data?.path) {
        return vscode.commands.executeCommand("compose.navigateToFile", node.Data.path) as any;
      }
      break;
    }
    case "definition": {
      if (node?.Data?.startLine && node?.Data?.path) {
        return openFileAtLine(node.Data.path, node.Data.startLine, node.Data.endLine);
      }
      if (node?.Data?.path) {
        return vscode.commands.executeCommand("compose.navigateToFile", node.Data.path) as any;
      }
      break;
    }
    case "policy": {
      if (node) {
        vscode.window.showInformationMessage(`Policy: ${node.Label}${node.Description ? " - " + node.Description : ""}`);
      } else {
        vscode.window.showInformationMessage(`Policy: ${path.basename(parsed.path)}`);
      }
      break;
    }
    case "statute": {
      if (node) {
        vscode.window.showInformationMessage(`Breach Kind: ${node.Label}${node.Description ? " - " + node.Description : ""}`);
      } else {
        vscode.window.showInformationMessage(`Breach Kind: ${path.basename(parsed.path)}`);
      }
      break;
    }
  }
}
// #endregion 🏬️URI Resolution
