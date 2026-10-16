/** 🗺️ File analysis and diagnostics
 *
 * Analysis of the active file, breach diagnostics and autofix.
 */

// #region 🔌️Adapters
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { type AnalyzeReport, type Breach, type ToolResult } from "../⚙️data-models/🟦️.ts";
import { DIAGNOSTIC_SOURCE } from "../🎞️constants/🟦️.ts";
import { bundleCache, type BundleInfo, fileBreachsMap, repoDiagnostics, runningProcesses } from "../🎩️globals/🟦️.ts";
import { execAsync, getRepoCommand, getWorkspaceRoot, hasRepoAccess, log, logError, runRepoCommandJson } from "../🎼️utilities/🟦️.ts";
import { extractFilePathFromScope } from "../🎵️path-helpers/🟦️.ts";
import { getTreeRoot, type TreeNodeData } from "../🏬️uri-resolution/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🗺️File Analysis & Diagnostics
// File Analysis & Diagnostics MUST handle analysis, breach diagnostics, bundle caching, and kit validation.

/** updateBundleCache holds the data fields for a updateBundleCache record.
 **/
async function updateBundleCache() {
  const root = await getTreeRoot();
  if (!root) return;
  const bundles: BundleInfo[] = [];
  function walk(node: TreeNodeData) {
    if (node.Kind === "bundle" && node.Data) {
      bundles.push({ id: node.Data.name || node.Label, root: node.Data.root || "" });
    }
    for (const child of node.Children || []) walk(child);
  }
  walk(root);
  if (bundles.length > 0) bundleCache.current = bundles;
}

/**
 * ignoredDirectories holds the data fields for a ignoredDirectories record.
 **/
const ignoredDirectories = new Set(["node_modules", "venv", "dist", "build", "out", "__pycache__", "coverage", "site-packages", "eggs", "wheels", "htmlcov", "target", "artifacts", "vendor"]);
/**
 * allowedDotDirectories holds the data fields for a allowedDotDirectories record.
 **/
const allowedDotDirectories = new Set([".github", ".devcontainer", ".🧬semio"]);

/**
 * isInIgnoredDirectory holds the data fields for a isInIgnoredDirectory record.
 **/
function isInIgnoredDirectory(relativePath: string): boolean {
  const segments = relativePath.split("/");
  return segments.some((segment) => {
    if (ignoredDirectories.has(segment)) return true;
    if (segment.startsWith(".") && !allowedDotDirectories.has(segment)) return true;
    return false;
  });
}

/** shouldAnalyzeFile holds the data fields for a shouldAnalyzeFile record.
 **/
export function shouldAnalyzeFile(document: vscode.TextDocument): boolean {
  const supportedLanguages = ["typescript", "javascript", "typescriptreact", "javascriptreact", "json", "python", "csharp", "go", "shellscript"];
  return supportedLanguages.includes(document.languageId);
}

/**
 * analyzeFile holds the data fields for a analyzeFile record.
 **/
export async function analyzeFile(document: vscode.TextDocument): Promise<void> {
  if (!shouldAnalyzeFile(document)) return;
  if (document.uri.scheme !== "file") return;
  const root = getWorkspaceRoot();
  if (!root) return;

  if (bundleCache.current.length === 0) {
    await updateBundleCache();
  }

  const relativePath = path.relative(root, document.uri.fsPath).replace(/\\/g, "/");
  if (relativePath.startsWith("..")) return;
  if (isInIgnoredDirectory(relativePath)) return;
  const fileUri = vscode.Uri.file(path.join(root, relativePath));
  const processKey = `analyze:${relativePath}`;

  if (runningProcesses.has(processKey)) {
    runningProcesses.get(processKey)?.abort();
    runningProcesses.delete(processKey);
  }

  const controller = new AbortController();
  runningProcesses.set(processKey, controller);

  try {
    const result = await runRepoCommandJson<ToolResult<{ analyze: AnalyzeReport }>>(`analyze "${relativePath}"`);

    const breachs = result?.data?.analyze?.breachs;
    if (breachs && breachs.length > 0) {
      fileBreachsMap.set(fileUri.toString(), breachs);
      updateFileDiagnostics(document, breachs);
    } else {
      fileBreachsMap.delete(fileUri.toString());
      repoDiagnostics().delete(fileUri);
    }
  } catch (error) {
    if (!controller.signal.aborted) {
      logError("Error analyzing file:", error);
    }
  } finally {
    runningProcesses.delete(processKey);
  }
}

/**
 * updateFileDiagnostics holds the data fields for a updateFileDiagnostics record.
 **/
function updateFileDiagnostics(document: vscode.TextDocument, breachs: Breach[]): void {
  const root = getWorkspaceRoot();
  if (!root) return;
  const diagnosticsByUri = new Map<string, { uri: vscode.Uri; diagnostics: vscode.Diagnostic[] }>();

  diagnosticsByUri.set(document.uri.toString(), { uri: document.uri, diagnostics: [] });

  for (const breach of breachs) {
    const filePath = extractFilePathFromScope(breach.scope);
    if (!filePath) continue;
    const absPath = path.join(root, filePath);
    const fileUri = vscode.Uri.file(absPath);
    const uriKey = fileUri.toString();
    if (!diagnosticsByUri.has(uriKey)) {
      diagnosticsByUri.set(uriKey, { uri: fileUri, diagnostics: [] });
    }
    const line = Math.max(0, (breach.line ?? 1) - 1);
    const column = Math.max(0, (breach.column ?? 1) - 1);
    const endColumn = breach.excerpt ? column + breach.excerpt.length : column + 1;
    const range = new vscode.Range(line, column, line, endColumn);
    const severity = vscode.DiagnosticSeverity.Warning;
    let kindId = breach.kind.id;
    if (kindId.startsWith("@semio-tech/policies//breachs/")) {
      kindId = kindId.replace("@semio-tech/policies//breachs/", "");
    }
    const diagnostic = new vscode.Diagnostic(range, breach.summary, severity);
    diagnostic.source = DIAGNOSTIC_SOURCE;
    diagnostic.code = { value: kindId, target: fileUri.with({ fragment: `L${line + 1}` }) };
    diagnosticsByUri.get(uriKey)!.diagnostics.push(diagnostic);
  }
  for (const { uri, diagnostics } of diagnosticsByUri.values()) {
    repoDiagnostics().set(uri, diagnostics);
  }
}

/** autofixBreach holds the data fields for a autofixBreach record.
 **/
async function autofixBreach(relativePath: string): Promise<void> {
  const root = getWorkspaceRoot();
  if (!root) return;
  if (!hasRepoAccess()) {
    vscode.window.showErrorMessage("repo binary not found in go/repo/");
    return;
  }
  const command = getRepoCommand();
  try {
    await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: "Autofixing breach..." }, async () => {
      const { stderr } = await execAsync(`"${command}" autofix "${relativePath}"`, { cwd: root, timeout: 30000 });
      if (stderr) log("Fix stderr:", stderr);
      const absPath = path.join(root, relativePath);
      const uri = vscode.Uri.file(absPath);
      const openDoc = vscode.workspace.textDocuments.find((d) => d.uri.fsPath === absPath);
      if (openDoc) {
        const newContent = fs.readFileSync(absPath, "utf-8");
        const edit = new vscode.WorkspaceEdit();
        const fullRange = new vscode.Range(openDoc.positionAt(0), openDoc.positionAt(openDoc.getText().length));
        edit.replace(uri, fullRange, newContent);
        await vscode.workspace.applyEdit(edit);
        await analyzeFile(openDoc);
      }
    });
    vscode.window.showInformationMessage(`Autofixed: ${relativePath}`);
  } catch (error) {
    logError("Failed to run autofix:", error);
    vscode.window.showErrorMessage(`Failed to autofix breach: ${error}`);
  }
}
// #endregion 🗺️File Analysis & Diagnostics
