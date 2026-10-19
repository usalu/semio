/** 🎵️ Path helpers
 *
 * Resolution of scope paths and ticket paths and opening of files at a line.
 */

// #region 🔌️Adapters
import * as path from "path";
import * as vscode from "vscode";
import { bundleCache, type BundleInfo } from "../🎩️globals/🟦️.ts";
import { getWorkspaceRoot } from "../🎼️utilities/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎵️Helpers
// Helpers MUST provide file path extraction, ticket path resolution, and editor navigation.

/**
 * 📄️ extractFilePathFromScope holds the data fields for a extractFilePathFromScope record.
 **/
export function extractFilePathFromScope(scope: string): string | undefined {
  let cleanScope = scope;
  if (cleanScope.startsWith("@semio-tech/breachs/")) {
    cleanScope = cleanScope.replace("@semio-tech/breachs/", "");
  }

  let bestBundle: BundleInfo | undefined;
  for (const b of bundleCache.current) {
    if (cleanScope.startsWith(b.id + "/")) {
      if (!bestBundle || b.id.length > bestBundle.id.length) {
        bestBundle = b;
      }
    } else if (cleanScope === b.id) {
      if (!bestBundle || b.id.length > bestBundle.id.length) {
        bestBundle = b;
      }
    }
  }

  if (bestBundle) {
    const relPath = cleanScope === bestBundle.id ? "" : cleanScope.substring(bestBundle.id.length + 1);
    const parts = relPath.split(/[#§:]/);
    const fileName = parts[0];
    const root = bestBundle.root === "." ? "" : bestBundle.root.endsWith("/") ? bestBundle.root : bestBundle.root + "/";
    const filePath = root + fileName;
    return filePath.endsWith("/") ? filePath.slice(0, -1) : filePath;
  }

  if (cleanScope.startsWith("@repo/") || cleanScope.startsWith("@semio-tech/")) {
    const parts = cleanScope.split("/");
    if (parts.length > 2) {
      cleanScope = parts.slice(2).join("/");
    }
  }

  const parts = cleanScope.split(/[#§:]/);
  const filePath = parts[0];
  if (filePath.endsWith(".ts") || filePath.endsWith(".tsx") || filePath.endsWith(".js") || filePath.endsWith(".json") || filePath.endsWith(".py") || filePath.endsWith(".cs") || filePath.endsWith(".go") || filePath.endsWith(".sh")) {
    return filePath;
  }
  return undefined;
}

/**
 * 🗃️ resolveTicketPath holds the data fields for a resolveTicketPath record.
 **/
export function resolveTicketPath(ticket: { year: number; month: number; day: number; slug: string; folderPath?: string }): string | undefined {
  if (ticket.folderPath) return ticket.folderPath;
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const relPath = path.join(`🎆️${String(ticket.year).padStart(2, "0")}`, `🌙️${String(ticket.month).padStart(2, "0")}`, `☀️${String(ticket.day).padStart(2, "0")}`, ticket.slug);
  return path.join(root, ".🧬semio", "🦑️repo", "🎫️tickets", relPath);
}

/**
 * 📥️ openFileAtLine holds the data fields for a openFileAtLine record.
 **/
export async function openFileAtLine(filePath: string, startLine: number, endLine?: number): Promise<void> {
  const root = getWorkspaceRoot();
  if (!root) return;
  const abs = path.isAbsolute(filePath) ? filePath : path.join(root, filePath);
  const uri = vscode.Uri.file(abs);
  const doc = await vscode.workspace.openTextDocument(uri);
  const editor = await vscode.window.showTextDocument(doc);
  const startPos = new vscode.Position(Math.max(0, startLine - 1), 0);
  const endPos = typeof endLine === "number" ? new vscode.Position(Math.max(0, endLine - 1), 0) : startPos;
  const range = new vscode.Range(startPos, endPos);
  editor.selection = new vscode.Selection(startPos, startPos);
}
// #endregion 🎵️Helpers
