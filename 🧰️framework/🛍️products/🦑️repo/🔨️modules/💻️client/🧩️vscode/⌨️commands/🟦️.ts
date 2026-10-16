/** ⌨️ Extension commands
 *
 * Registration of every command the extension contributes.
 */

// #region 🔌️Adapters
import * as path from "path";
import * as vscode from "vscode";
import { getRepoBinaryPath, getWorkspaceRoot } from "../🎼️utilities/🟦️.ts";
import { openFileAtLine, resolveTicketPath } from "../🎵️path-navigation/🟦️.ts";
import { getTreeNodeCache, navigateToUri, slugify, type TreeNodeData } from "../🏬️uri-resolution/🟦️.ts";
import { filterProvider, type FilterTreeDataProvider, loadAvailableFilterValues, monorepoProvider, type MonorepoTreeItem } from "../🌳️tree-providers/🟦️.ts";
// #endregion 🔌️Adapters

// #region 📜️Activation
/**
 * registerCommands holds the data fields for a registerCommands record.
 **/
export function registerCommands(context: vscode.ExtensionContext): void {
  const registered = new Set<string>();
  const register = (command: string, handler: (...args: any[]) => any): void => {
    if (registered.has(command)) return;
    registered.add(command);
    context.subscriptions.push(vscode.commands.registerCommand(command, handler));
  };

  register("compose.copyId", (item: MonorepoTreeItem) => {
    const id = item?.nodeId || (typeof item?.label === "string" ? item.label : "");
    if (id) {
      vscode.env.clipboard.writeText(id);
      vscode.window.showInformationMessage(`Copied: ${id}`);
    }
  });

  register("compose.mailto", (email: string) => {
    if (email) vscode.env.openExternal(vscode.Uri.parse(`mailto:${email}`));
  });

  register("compose.openLink", (url: string) => {
    if (url) vscode.env.openExternal(vscode.Uri.parse(url));
  });

  register("compose.refreshMonorepo", () => {
    monorepoProvider.current?.refresh();
  });

  register("compose.refreshCodebase", () => {
    filterProvider.current?.refresh();
    monorepoProvider.current?.refresh();
  });

  register("compose.refreshItem", (item: MonorepoTreeItem) => {
    monorepoProvider.current?.refreshItem(item);
  });

  register("compose.filter.search", async () => {
    const q = await vscode.window.showInputBox({ prompt: "Search..." });
    if (q !== undefined && filterProvider.current) {
      filterProvider.current.searchQuery = q;
      filterProvider.current.refresh();
      monorepoProvider.current?.refresh();
    }
  });

  register("compose.filter.toggle", (kind: string, key: string) => {
    filterProvider.current?.toggle(kind, key);
  });

  const filterToggleEntries: Record<string, string[]> = {
    bundle: ["library", "binary", "ui", "site", "assets", "schema", "default", "none", "all"],
    technology: ["user", "infrastructure", "research", "none", "all"],
    folder: ["organization", "required", "none", "all"],
    file: ["code", "script", "config", "lab", "docs", "resource", "license", "none", "all"],
    section: ["none", "all"],
    definition: ["implementation", "interface", "constant", "none", "all"],
    goal: ["open", "closed", "none", "all"],
    ticket: ["open", "closed", "none", "all"],
    policy: ["none", "all"],
    contributor: ["none", "all"],
    checkpoint: ["none", "all"],
    time: ["none", "all"],
  };
  for (const [kind, keys] of Object.entries(filterToggleEntries)) {
    for (const key of keys) {
      register(`compose.filter.toggle.${kind}.${key}`, () => filterProvider.current?.toggle(kind, key));
    }
  }

  const timeModes: Array<["year" | "month" | "day", "none" | "all"]> = [
    ["year", "none"],
    ["year", "all"],
    ["month", "none"],
    ["month", "all"],
    ["day", "none"],
    ["day", "all"],
  ];
  for (const [unit, mode] of timeModes) {
    register(`compose.filter.time.${unit}.${mode}`, () => filterProvider.current?.setTimeMode(unit, mode));
  }

  register("compose.filter.toggleYear", (year: number) => filterProvider.current?.toggleYear(year));
  register("compose.filter.toggleMonth", (month: number) => filterProvider.current?.toggleMonth(month));
  register("compose.filter.toggleDay", (day: number) => filterProvider.current?.toggleDay(day));

  const searchToggles: Array<[string, keyof FilterTreeDataProvider]> = [
    ["compose.filter.search.matchCase", "matchCase"],
    ["compose.filter.search.wholeWord", "matchWholeWord"],
    ["compose.filter.search.regex", "useRegex"],
  ];
  for (const [cmd, prop] of searchToggles) {
    register(cmd, () => {
      if (filterProvider.current) {
        (filterProvider.current as any)[prop] = !(filterProvider.current as any)[prop];
        filterProvider.current.refresh();
        monorepoProvider.current?.refresh();
      }
    });
  }

  const revealInExplorer = (targetPath: string) => {
    const wsRoot = getWorkspaceRoot();
    if (!wsRoot) return;
    const abs = path.isAbsolute(targetPath) ? targetPath : path.join(wsRoot, targetPath);
    return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs));
  };
  register("compose.navigateToBundle", revealInExplorer);
  register("compose.navigateToFolder", revealInExplorer);

  register("compose.navigateToFile", async (filePath: string) => {
    const root = getWorkspaceRoot();
    if (root) {
      const abs = path.isAbsolute(filePath) ? filePath : path.join(root, filePath);
      const uri = vscode.Uri.file(abs);
      try {
        const doc = await vscode.workspace.openTextDocument(uri);
        await vscode.window.showTextDocument(doc);
      } catch (e) {
        vscode.window.showErrorMessage(`Failed to open file: ${filePath}`);
      }
    }
  });

  const navigateToRangedItem = (payload: any, rangeKey: string) => {
    const filePath = payload?.filePath;
    const item = payload?.[rangeKey];
    if (!filePath || typeof item?.range?.start !== "number") return;
    return openFileAtLine(filePath, item.range.start, item.range.end ?? undefined);
  };
  register("compose.navigateToSection", (s: any) => navigateToRangedItem(s, "section"));
  register("compose.navigateToDefinition", (d: any) => navigateToRangedItem(d, "definition"));

  register("compose.navigate", async (target: string) => {
    if (!target) return;
    if (target.startsWith("repo://")) {
      return navigateToUri(target);
    }
    const cache = await getTreeNodeCache();
    for (const [uri, node] of cache) {
      if (node.ID === target || node.Label === target || slugify(node.Label) === slugify(target)) {
        return navigateToUri(uri);
      }
    }
  });

  register("compose.navigateTo", async () => {
    const cache = await getTreeNodeCache();
    const items: vscode.QuickPickItem[] = [];
    for (const [uri, node] of cache) {
      if (node.Kind === "category") continue;
      items.push({ label: node.Label, description: node.Kind, detail: uri });
    }
    const picked = await vscode.window.showQuickPick(items, { placeHolder: "Navigate to..." });
    if (picked?.detail) {
      return navigateToUri(picked.detail);
    }
  });

  register("compose.ticketOpen", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    if (!node) return;
    const year = node.Year ?? node.Data?.year;
    const month = node.Month ?? node.Data?.month;
    const day = node.Day ?? node.Data?.day;
    const slug = node.Data?.slug ?? node.Label;
    if (!year || !month || !day || !slug) return;
    const t = { year, month, day, slug, folderPath: undefined as string | undefined };
    const p = resolveTicketPath(t);
    if (!p) return;
    return vscode.commands.executeCommand("compose.navigateToFile", p);
  });

  register("compose.ticketClose", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    if (!node) return;
    const year = node.Year ?? node.Data?.year;
    const month = node.Month ?? node.Data?.month;
    const day = node.Day ?? node.Data?.day;
    const slug = node.Data?.slug ?? node.Label;
    if (!year || !month || !day || !slug) return;
    const ticketId = `🎆️${year}/🌙️${String(month).padStart(2, "0")}/☀️${String(day).padStart(2, "0")}/${slug}`;
    return vscode.window.showInformationMessage(`Close ticket: ${ticketId}?`, "Yes", "No").then((answer) => {
      if (answer === "Yes") {
        const binaryPath = getRepoBinaryPath();
        if (!binaryPath) return;
        const cp = require("child_process");
        cp.execSync(`${binaryPath} ticket close ${ticketId} "Closed via VS Code" .`, { cwd: getWorkspaceRoot() });
        monorepoProvider.current?.refresh();
      }
    });
  });

  register("compose.ticketReopen", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    if (!node) return;
    const year = node.Year ?? node.Data?.year;
    const month = node.Month ?? node.Data?.month;
    const day = node.Day ?? node.Data?.day;
    const slug = node.Data?.slug ?? node.Label;
    if (!year || !month || !day || !slug) return;
    const ticketId = `🎆️${year}/🌙️${String(month).padStart(2, "0")}/☀️${String(day).padStart(2, "0")}/${slug}`;
    return vscode.window.showInputBox({ prompt: "Reopen prompt" }).then((prompt) => {
      if (!prompt) return;
      const binaryPath = getRepoBinaryPath();
      if (!binaryPath) return;
      const cp = require("child_process");
      cp.execSync(`${binaryPath} ticket reopen ${ticketId} "${prompt}" copilot-chat`, { cwd: getWorkspaceRoot() });
      monorepoProvider.current?.refresh();
    });
  });

  register("compose.draftCreate", async () => {
    const title = await vscode.window.showInputBox({ prompt: "Draft title" });
    if (!title) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    cp.execSync(`${binaryPath} draft create "${title}"`, { cwd: getWorkspaceRoot() });
    monorepoProvider.current?.refresh();
  });

  register("compose.draftDelete", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const slug = node?.Data?.slug ?? node?.Label;
    if (!slug) return;
    return vscode.window.showInformationMessage(`Delete draft: ${slug}?`, "Yes", "No").then((answer) => {
      if (answer === "Yes") {
        const binaryPath = getRepoBinaryPath();
        if (!binaryPath) return;
        const cp = require("child_process");
        cp.execSync(`${binaryPath} draft delete ${slug}`, { cwd: getWorkspaceRoot() });
        monorepoProvider.current?.refresh();
      }
    });
  });

  register("compose.copyCheckpointSha", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const sha = node?.Data?.sha;
    if (sha) {
      vscode.env.clipboard.writeText(sha);
      vscode.window.showInformationMessage(`Copied SHA: ${sha.substring(0, 7)}`);
    }
  });

  register("compose.openCheckpointInGitHub", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const sha = node?.Data?.sha;
    if (sha) vscode.env.openExternal(vscode.Uri.parse(`https://github.com/usalu/semio/commit/${sha}`));
  });

  register("compose.policyCheck", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const policyId = node?.Data?.id || node?.Label;
    if (!policyId) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    cp.execSync(`${binaryPath} policy check ${policyId}`, { cwd: getWorkspaceRoot() });
  });

  register("compose.open", (target: string) => {
    if (target) {
      vscode.commands.executeCommand("compose.navigate", target);
    }
  });

  register("compose.analyze", async (id: string) => {
    if (!id) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    try {
      const output = await new Promise<string>((resolve, reject) => {
        cp.exec(`"${binaryPath}" analyze "${id}"`, { cwd: getWorkspaceRoot(), encoding: "utf-8" }, (error: any, stdout: string, stderr: string) => {
          if (error && !stdout && !stderr) reject(error);
          else resolve((stdout || "") + (stderr ? "\n" + stderr : ""));
        });
      });
      const doc = await vscode.workspace.openTextDocument({ content: output, language: "markdown" });
      await vscode.window.showTextDocument(doc, { preview: true, preserveFocus: true, viewColumn: vscode.ViewColumn.Beside });
    } catch (e: any) {
      vscode.window.showErrorMessage(`Failed to analyze ${id}: ${e.message}`);
    }
  });

  const contributedCommands: string[] = [
    "compose.analyze",
    "compose.analyzeFile",
    "compose.autofix",
    "compose.autofixFile",
    "compose.policyList",
    "compose.policyTree",
    "compose.ticketList",
    "compose.ticketRead",
    "compose.ticketTree",
    "compose.technologyList",
    "compose.technologyTree",
    "compose.contributorAdd",
    "compose.contributorList",
    "compose.contributorRemove",
    "compose.sectionTree",
    "compose.sectionList",
    "compose.sectionCreate",
    "compose.sectionMove",
    "compose.sectionDelete",
    "compose.sectionOpen",
    "compose.sectionRename",
    "compose.sectionCreateChild",
    "compose.sectionRemove",
    "compose.sectionIntegrate",
    "compose.definitionList",
    "compose.definitionTree",
    "compose.folderTree",
    "compose.folderCreate",
    "compose.folderMove",
    "compose.folderDelete",
    "compose.folderList",
    "compose.fileCreate",
    "compose.fileMove",
    "compose.fileDelete",
    "compose.fileList",
    "compose.fileTree",
    "compose.refreshDiagnostics",
    "compose.autofixBreach",
    "compose.navigateToRepo",
    "compose.navigateTo",
    "compose.goalOpen",
    "compose.goalList",
  ];

  for (const command of contributedCommands) {
    if (registered.has(command)) continue;
    register(command, (..._args: unknown[]) => undefined);
  }

  loadAvailableFilterValues();
}
// #endregion 📜️Activation
