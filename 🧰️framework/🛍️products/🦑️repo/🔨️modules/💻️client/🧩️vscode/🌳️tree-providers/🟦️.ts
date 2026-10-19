import { updateBundleCache } from "../🗺️file-analysis/🟦️.ts";
/** 🌳️ Tree view providers
 *
 * The filter, monorepo and sections tree views with their providers and filter values.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import * as path from "path";
import * as vscode from "vscode";
import { type SectionInfo } from "../⚙️data-models/🟦️.ts";
import { execFileAsync, getRepoBinaryPath, getWorkspaceRoot } from "../🎼️utilities/🟦️.ts";
import { fetchTreeWithArgs, getTreeRoot, treeNodeCommand, treeNodeContextValue, type TreeNodeData, treeNodeDisplayLabel } from "../🏬️uri-resolution/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎩️Globals
/**
 * 🔎️ filterProvider.current holds the data fields for a filterProvider.current record.
 **/
export const filterProvider = ephemeralBox<FilterTreeDataProvider | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.filterProvider", undefined);
/**
 * 🗂️ monorepoProvider.current holds the data fields for a monorepoProvider.current record.
 **/
export const monorepoProvider = ephemeralBox<MonorepoTreeDataProvider | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.monorepoProvider", undefined);
// #endregion 🎩️Globals

// #region 🏬️URI Resolution
/**
 * 🏗️ Builds CLI tree command arguments from the current filter provider state.
 * Implementations MUST translate each filter toggle into the corresponding CLI flag.
 **/
export function buildCliTreeArgs(fp?: FilterTreeDataProvider): string[] {
  const args: string[] = [];
  if (!fp) return args;
  const query = fp.searchQuery?.trim();
  if (query) args.push(query);
  const ff = fp.filters.file;
  if (!ff.code) args.push("--no-code");
  if (!ff.script) args.push("--no-script");
  if (!ff.config) args.push("--no-config");
  if (!ff.lab) args.push("--no-lab");
  if (!ff.docs) args.push("--no-docs");
  if (!ff.resource) args.push("--no-resource");
  if (!ff.template) args.push("--no-template");
  if (!ff.license) args.push("--no-license");
  if (!fp.filters.section.all) {
    args.push("--no-section", "--no-definition");
  } else {
    const df = fp.filters.definition;
    if (!df.implementation) args.push("--no-implementation");
    if (!df.interface) args.push("--no-interface");
    if (!df.constant) args.push("--no-constant");
  }
  const fo = fp.filters.folder;
  if (!fo.organization && !fo.required) args.push("--no-folder");
  else if (!fo.organization && fo.required) args.push("--only-required");
  else if (fo.organization && !fo.required) args.push("--only-organization");
  const bf = fp.filters.bundle;
  if (!bf.library) args.push("--no-library");
  if (!bf.schema) args.push("--no-schema");
  if (!bf.binary) args.push("--no-binary");
  if (!bf.ui) args.push("--no-client");
  if (!bf.site) args.push("--no-site");
  if (!bf.assets) args.push("--no-assets");
  const gf = fp.filters.goal;
  const tf = fp.filters.ticket;
  if (!gf.open && !gf.closed) args.push("--no-goal");
  if (!tf.open && !tf.closed) args.push("--no-ticket");
  if (gf.open && !gf.closed && tf.open && !tf.closed) args.push("--only-open");
  else if (!gf.open && gf.closed && !tf.open && tf.closed) args.push("--only-closed");
  for (const year of fp.excludedYears) args.push("--no-year", String(year));
  for (const month of fp.excludedMonths) args.push("--no-month", String(month));
  for (const day of fp.excludedDays) args.push("--no-day", String(day));
  if (Object.values(ff).every((v) => !v)) args.push("--no-file");
  if (!fp.filters.policy.all) args.push("--no-policy");
  if (!fp.filters.contributor.all) args.push("--no-contributor");
  if (!fp.filters.checkpoint.all) args.push("--no-checkpoint");
  const pf = fp.filters.technology;
  if (!pf.user && !pf.infrastructure && !pf.research) args.push("--no-technology");
  return args;
}
// #endregion 🏬️URI Resolution

// #region 🪵️Providers
// Providers MUST implement VS Code tree data providers for filter, monorepo, and sections views.

/**
 * 🧪️ Tree item representing a filter option in the filter sidebar view.
 *Implementations MUST extend vscode.TreeItem and expose filter metadata.
 **/
export class FilterTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly type: "root" | "search" | "filter" | "time" | "filterOption" | "timeValue",
    public readonly collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None,
    public readonly contextValue?: string,
    public readonly filterKey?: string,
    public readonly filterValue?: any,
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
  }
}

/**
 * 🧲️ Provides the tree data for the filter sidebar view with search and toggle state.
 *Implementations MUST implement vscode.TreeDataProvider and emit change events on toggle.
 **/
export class FilterTreeDataProvider implements vscode.TreeDataProvider<FilterTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<FilterTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  public searchQuery: string = "";
  public matchCase: boolean = false;
  public matchWholeWord: boolean = false;
  public useRegex: boolean = false;

  public filters: Record<string, Record<string, boolean>> = {
    technology: { user: true, infrastructure: true, research: true },
    bundle: { library: true, binary: true, ui: true, site: true, assets: true, schema: true, default: true },
    folder: { organization: true, required: true },
    file: { code: true, script: true, config: true, lab: true, docs: true, resource: true, template: true, license: true },
    section: { none: false, all: true },
    definition: { implementation: true, interface: true, constant: true },
    goal: { open: true, closed: true },
    ticket: { open: true, closed: true },
    policy: { all: true },
    contributor: { all: true },
    checkpoint: { all: true },
  };

  public timeFilter: Record<string, boolean> = { none: false, all: true };
  public excludedYears: number[] = [];
  public excludedMonths: number[] = [];
  public excludedDays: number[] = [];

  constructor() {
    this.updateContextKeys();
  }

  refresh(): void {
    this.updateContextKeys();
    this._onDidChangeTreeData.fire();
  }

  updateContextKeys(): void {
    for (const [kind, values] of Object.entries(this.filters)) {
      for (const [key, enabled] of Object.entries(values)) {
        vscode.commands.executeCommand("setContext", `compose.filter.${kind}.${key}`, enabled);
      }
    }
  }

  public availableYears: number[] = [];
  public availableMonths: number[] = [];
  public availableDays: number[] = [];
  public availableContributors: string[] = [];
  public availablePolicies: string[] = [];

  getTreeItem(element: FilterTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: FilterTreeItem): Promise<FilterTreeItem[]> {
    if (!element) {
      return [
        this.createSearchItem(),
        this.createFilterItem("🏗️Technologies", "filter_technology", "Technologies filter"),
        this.createFilterItem("📦️Bundles", "filter_bundle", "Bundles filter"),
        this.createFilterItem("📁️Folders", "filter_folder", "Folders filter"),
        this.createFilterItem("📄️Files", "filter_file", "Files filter"),
        this.createFilterItem("🔖️Sections", "filter_section", "Sections filter"),
        this.createFilterItem("🏷️Definitions", "filter_definition", "Definitions filter"),
        this.createFilterItem("🎯️Goals", "filter_goal", "Goals filter"),
        this.createFilterItem("🎫️Tickets", "filter_ticket", "Tickets filter"),
        this.createFilterItem("🎫️Dates", "filter_time", "Dates filter", vscode.TreeItemCollapsibleState.Collapsed),
        this.createFilterItem("👮️Policies", "filter_policy", "Policies filter"),
        this.createFilterItem("🧑️‍💻️Contributors", "filter_contributor", "Contributors filter"),
        this.createFilterItem("🔄️Checkpoints", "filter_checkpoint", "Checkpoints filter"),
      ];
    }

    if (element.contextValue === "filter_time") {
      return this.availableYears.map((y) => {
        const excluded = this.excludedYears.includes(y);
        const item = new FilterTreeItem(String(y), "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_year", "year", y);
        item.tooltip = excluded ? `Excluded year ${y}` : `Included year ${y}`;
        item.command = { command: "compose.filter.toggleYear", title: "Toggle Year", arguments: [y] };
        return item;
      });
    }

    if (element.contextValue === "filter_time_year") {
      const year = element.filterValue;
      return this.availableMonths.map((m) => {
        const excluded = this.excludedMonths.includes(m);
        const label = new Date(2000, m - 1, 1).toLocaleString("default", { month: "long" });
        const item = new FilterTreeItem(label, "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_month", "month", m);
        item.tooltip = excluded ? `Excluded month ${label}` : `Included month ${label}`;
        item.command = { command: "compose.filter.toggleMonth", title: "Toggle Month", arguments: [m] };
        return item;
      });
    }

    if (element.contextValue === "filter_time_month") {
      return this.availableDays.map((d) => {
        const excluded = this.excludedDays.includes(d);
        const item = new FilterTreeItem(String(d).padStart(2, "0"), "timeValue", vscode.TreeItemCollapsibleState.None, "filter_time_day", "day", d);
        item.tooltip = excluded ? `Excluded day ${d}` : `Included day ${d}`;
        item.command = { command: "compose.filter.toggleDay", title: "Toggle Day", arguments: [d] };
        return item;
      });
    }

    return [];
  }

  private createSearchItem(): FilterTreeItem {
    const item = new FilterTreeItem("🔍️Search", "search", vscode.TreeItemCollapsibleState.None, "filter_search");
    const details = [this.searchQuery ? `Query: ${this.searchQuery}` : "No query set", this.matchCase ? "Match case on" : "Match case off", this.matchWholeWord ? "Whole word on" : "Whole word off", this.useRegex ? "Regex on" : "Regex off"];
    item.tooltip = `Search filter\n${details.join("\n")}`;
    item.command = { command: "compose.filter.search", title: "Search" };
    return item;
  }

  private createFilterItem(label: string, contextValue: string, tooltip: string, collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None): FilterTreeItem {
    const item = new FilterTreeItem(label, "filter", collapsibleState, contextValue);
    item.tooltip = tooltip;
    return item;
  }

  toggle(kind: string, key: string) {
    const filterKeys = this.filters[kind] ? Object.keys(this.filters[kind]) : [];
    const hasRealKeys = filterKeys.some((k) => k !== "none" && k !== "all");
    if ((key === "none" || key === "all") && this.filters[kind] && hasRealKeys) {
      for (const k of Object.keys(this.filters[kind])) this.filters[kind][k] = key === "all";
      this.refresh();
      monorepoProvider.current?.refresh();
      return;
    }
    if (kind === "time") {
      if (key === "all") {
        this.excludedYears = [];
        this.excludedMonths = [];
        this.excludedDays = [];
        this.timeFilter.all = true;
        this.timeFilter.none = false;
      } else if (key === "none") {
        this.excludedYears = [...this.availableYears];
        this.excludedMonths = [...this.availableMonths];
        this.excludedDays = [...this.availableDays];
        this.timeFilter.all = false;
        this.timeFilter.none = true;
      }
    } else if (this.filters[kind]) {
      this.filters[kind][key] = !this.filters[kind][key];
    }
    this.refresh();
    monorepoProvider.current?.refresh();
  }

  setTimeMode(kind: "year" | "month" | "day", mode: "all" | "none") {
    if (kind === "year") this.excludedYears = mode === "all" ? [] : [...this.availableYears];
    if (kind === "month") this.excludedMonths = mode === "all" ? [] : [...this.availableMonths];
    if (kind === "day") this.excludedDays = mode === "all" ? [] : [...this.availableDays];
    this.refresh();
    monorepoProvider.current?.refresh();
  }

  toggleYear(year: number) {
    if (this.excludedYears.includes(year)) this.excludedYears = this.excludedYears.filter((y) => y !== year);
    else this.excludedYears.push(year);
    this.refresh();
    monorepoProvider.current?.refresh();
  }

  toggleMonth(month: number) {
    if (this.excludedMonths.includes(month)) this.excludedMonths = this.excludedMonths.filter((m) => m !== month);
    else this.excludedMonths.push(month);
    this.refresh();
    monorepoProvider.current?.refresh();
  }

  toggleDay(day: number) {
    if (this.excludedDays.includes(day)) this.excludedDays = this.excludedDays.filter((d) => d !== day);
    else this.excludedDays.push(day);
    this.refresh();
    monorepoProvider.current?.refresh();
  }
}

/**
 * 📁️ Tree item representing a monorepo artifact in the sidebar tree.
 *Implementations MUST extend vscode.TreeItem and carry the original node data.
 **/
export class MonorepoTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly contextValue?: string,
    public readonly data?: any,
    public readonly nodeId?: string,
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
    if (nodeId) this.tooltip = nodeId;
  }
}

/**
 * 🌲️ Converts a TreeNodeData to a VS Code MonorepoTreeItem for the sidebar.
 *Implementations MUST set label, description, tooltip, and command from node data.
 **/
export function treeNodeToItem(node: TreeNodeData): MonorepoTreeItem {
  const label = treeNodeDisplayLabel(node);
  const hasChildren = node.Children && node.Children.length > 0;
  const collapsible = hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
  const ctx = treeNodeContextValue(node);
  const item = new MonorepoTreeItem(label, collapsible, ctx, node, node.ID || undefined);
  item.command = treeNodeCommand(node);
  if (node.Description) item.tooltip = node.Description;
  if (node.Kind === "checkpoint" && node.Data?.sha) item.description = node.Data.sha.substring(0, 7);
  if (node.Kind === "statute") {
    item.description = node.Data?.autofixable ? "🔧️" : "";
    if (node.Description) item.tooltip = node.Description;
  }
  return item;
}

/**
 * 🧰️ Provides the tree data for the monorepo sidebar view using CLI tree output.
 *Implementations MUST implement vscode.TreeDataProvider and fetch data via CLI.
 **/
export class MonorepoTreeDataProvider implements vscode.TreeDataProvider<MonorepoTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<MonorepoTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(public filterProvider?: FilterTreeDataProvider) {}

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  refreshItem(item?: MonorepoTreeItem): void {
    this._onDidChangeTreeData.fire(item);
  }

  getTreeItem(element: MonorepoTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: MonorepoTreeItem): Promise<MonorepoTreeItem[]> {
    if (!element) {
      const args = buildCliTreeArgs(this.filterProvider);
      const tree = await fetchTreeWithArgs(args);
      if (!tree?.Children) return [];
      return tree.Children.map(treeNodeToItem);
    }
    const node = element.data as TreeNodeData;
    if (!node?.Children) return [];
    return node.Children.map(treeNodeToItem);
  }
}

/**
 * ✂️ SectionTreeItem holds the data fields for a SectionTreeItem record.
 **/
class SectionTreeItem extends vscode.TreeItem {
  constructor(
    public section: SectionInfo,
    public filePath: string,
  ) {
    super(section.name, section.children && section.children.length > 0 ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
    this.contextValue = "section";
    this.iconPath = new vscode.ThemeIcon("bookmark");
    this.tooltip = `Section: ${section.name}`;
    const start = section.startLine - 1;
    this.command = {
      command: "vscode.open",
      title: "Open Section",
      arguments: [vscode.Uri.file(path.join(getWorkspaceRoot() || "", filePath)), { selection: new vscode.Range(start, 0, start, 0) }],
    };
  }
}

/**
 * 🍰️ Provides the tree data for the sections sidebar view of the active file.
 *Implementations MUST refresh when the active editor changes or the document is edited.
 **/
export class SectionsTreeDataProvider implements vscode.TreeDataProvider<SectionTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<SectionTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private activeEditor: vscode.TextEditor | undefined;

  constructor(private context: vscode.ExtensionContext) {
    this.activeEditor = vscode.window.activeTextEditor;
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      this.refresh();
    });
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (this.activeEditor && e.document.uri.toString() === this.activeEditor.document.uri.toString()) {
        this.refresh();
      }
    });
  }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: SectionTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: SectionTreeItem): Promise<SectionTreeItem[]> {
    if (!this.activeEditor) return [];

    const root = getWorkspaceRoot();
    if (!root) return [];

    const uri = this.activeEditor.document.uri;
    const filePath = path.relative(root, uri.fsPath);
    if (!filePath || filePath.startsWith("..")) return [];

    if (element) {
      return this.createSectionItems(element.section.children || [], filePath);
    } else {
      const binaryPath = getRepoBinaryPath();
      if (!binaryPath) return [];

      try {
        const { stdout: output } = await execFileAsync(binaryPath, ["section", "list", "--file", filePath, "--json"], { cwd: root, timeout: 60000 });

        const sections: SectionInfo[] = [];
        const lines = output.split("\n");
        for (const line of lines) {
          if (!line.trim()) continue;
          try {
            const parsed = JSON.parse(line);
            if (parsed.section) {
              sections.push(parsed.section);
            }
          } catch (e) {}
        }
        return this.createSectionItems(sections, filePath);
      } catch (e) {
        console.error("Failed to fetch sections:", e);
        return [];
      }
    }
  }

  private createSectionItems(sections: SectionInfo[], filePath: string): SectionTreeItem[] {
    return sections.map((s) => {
      const item = new SectionTreeItem(s, filePath);

      return item;
    });
  }
}
// #endregion 🪵️Providers

// #region 📜️Activation
/**
 * 🏋️ loadAvailableFilterValues holds the data fields for a loadAvailableFilterValues record.
 **/
export async function loadAvailableFilterValues(): Promise<void> {
  await updateBundleCache();
  const years = new Set<number>();
  const months = new Set<number>();
  const days = new Set<number>();
  const contributors = new Set<string>();
  const policies = new Set<string>();
  const breachs = new Set<string>();

  const tree = await getTreeRoot();
  if (tree?.Children) {
    const walk = (nodes: TreeNodeData[]) => {
      for (const n of nodes) {
        if (n.Kind === "ticket") {
          if (n.Year) years.add(n.Year);
          if (n.Month) months.add(n.Month);
          if (n.Day) days.add(n.Day);
        }
        if (n.Kind === "contributor") contributors.add(n.Label || "");
        if (n.Kind === "policy") policies.add(n.Data?.id || n.Label || "");
        if (n.Kind === "statute") breachs.add(n.Data?.id || n.ID || "");
        if (n.Children) walk(n.Children);
      }
    };
    walk(tree.Children);
  }

  if (filterProvider.current) {
    filterProvider.current.availableYears = Array.from(years).sort((a, b) => b - a);
    filterProvider.current.availableMonths = Array.from(months).sort((a, b) => a - b);
    filterProvider.current.availableDays = Array.from(days).sort((a, b) => a - b);
    filterProvider.current.availableContributors = Array.from(contributors).sort();
    filterProvider.current.availablePolicies = Array.from(policies).sort();
    filterProvider.current.refresh();
  }
}
// #endregion 📜️Activation
