import { readFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { runCliGraphql } from "../🖥️cli/🟦️.ts";
import type { BreachRecord } from "../🔖️breach/🟦️.ts";

//#region 🔖️linter
const NODE_QUERY = `
query NodeQ($id: ID!) {
  node(id: $id) {
    __typename
    ... on File {
      id path name extension fileKind: kind
      sections { id name path range { start end } }
      definitions { id name kind range { start end } }
    }
    ... on Folder {
      id path name
    }
    ... on Bundle {
      id name root bundleKind: kind
    }
    ... on Section {
      id name path
      range { start end }
      sectionFile: file { path }
      definitions { id name kind range { start end } }
    }
    ... on Definition {
      id name defKind: kind
      range { start end }
      defFile: file { path }
    }
  }
}
`;

export type GraphNode = Record<string, unknown> & { __typename?: string };

/** 🧷️BaseLinter holds shared repo-root + graphql helpers for lint scripts. */
export abstract class BaseLinter {
  readonly entityId: string;
  protected readonly repoRoot: string;

  constructor(entityId: string, repoRoot: string = getWorkspaceRoot()) {
    this.entityId = entityId;
    this.repoRoot = repoRoot;
  }

  protected gql<T = unknown>(query: string, variables: Record<string, unknown> = {}): T {
    return runCliGraphql(query, variables, { repoRoot: this.repoRoot }) as T;
  }

  protected loadNode(): GraphNode {
    const data = this.gql<{ node: GraphNode | null }>(NODE_QUERY, { id: this.entityId });
    const n = data.node;
    if (!n || !n.__typename) {
      throw new Error(`[linter] node not found for id ${this.entityId}`);
    }
    return n;
  }

  /** 🚫️Builds a breach with default scope = entity id. */
  breach(p: Omit<BreachRecord, "scope"> & { scope?: string }): BreachRecord {
    const { scope, ...rest } = p;
    return {
      ...rest,
      scope: scope ?? this.entityId,
    };
  }
}

/** 🏗️TechnologyLinter queries a technology node by id. */
export class TechnologyLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) {
      const data = this.gql<{ technologies: GraphNode[] }>(`query T { technologies { id name kind root } }`);
      const found = (data.technologies ?? []).find((t) => String(t.id) === this.entityId);
      if (!found) {
        throw new Error(`[TechnologyLinter] technology not found for id ${this.entityId}`);
      }
      this.node = { ...found, __typename: "Technology" };
    }
    return this.node;
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
  }

  root(): string {
    return String(this.load().root ?? "");
  }

  /** 📦️Lists bundle rows for this technology. */
  bundles(): GraphNode[] {
    const data = this.gql<{ bundles: GraphNode[] }>(`query B { bundles { id name root kind } }`);
    const tech = this.name();
    return (data.bundles ?? []).filter((b) => String(b.name ?? "").split("/")[0] === tech);
  }
}

/** 🧰️BundleLinter queries a bundle node by id. */
export class BundleLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Bundle") {
      throw new Error(`[BundleLinter] expected Bundle, got ${this.node.__typename}`);
    }
    return this.node;
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  root(): string {
    return String(this.load().root ?? "");
  }

  kind(): string {
    return String(this.load().bundleKind ?? "");
  }

  technologyName(): string {
    return this.name().split("/")[0] ?? "";
  }
}

/** 📁️FolderLinter queries a folder node by id. */
export class FolderLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Folder") {
      throw new Error(`[FolderLinter] expected Folder, got ${this.node.__typename}`);
    }
    return this.node;
  }

  path(): string {
    return String(this.load().path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }
}

/** 📄️FileLinter queries a file node by id and reads bytes from disk. */
export class FileLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "File") {
      throw new Error(`[FileLinter] expected File, got ${this.node.__typename}`);
    }
    return this.node;
  }

  path(): string {
    return String(this.load().path ?? "").replaceAll("\\", "/");
  }

  ext(): string {
    return String(this.load().extension ?? "");
  }

  kind(): string {
    return String(this.load().fileKind ?? "");
  }

  content(): string {
    const p = this.path();
    return readFileSync(join(this.repoRoot, p), "utf8");
  }

  lines(): string[] {
    return this.content().split(/\r?\n/);
  }

  sections(): GraphNode[] {
    return (this.load().sections as GraphNode[]) ?? [];
  }

  definitions(): GraphNode[] {
    return (this.load().definitions as GraphNode[]) ?? [];
  }
}

/** 🔖️SectionLinter queries a section node by id. */
export class SectionLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Section") {
      throw new Error(`[SectionLinter] expected Section, got ${this.node.__typename}`);
    }
    return this.node;
  }

  filePath(): string {
    const f = this.load().sectionFile as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  sectionPath(): string {
    return String(this.load().path ?? "");
  }

  startLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.start ?? 0);
  }

  endLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.end ?? 0);
  }

  content(): string {
    const full = readFileSync(join(this.repoRoot, this.filePath()), "utf8");
    const lines = full.split(/\r?\n/);
    const s = this.startLine();
    const e = this.endLine();
    if (s <= 0 || e < s) return "";
    return lines.slice(s - 1, e).join("\n");
  }

  definitions(): GraphNode[] {
    return (this.load().definitions as GraphNode[]) ?? [];
  }
}

/** 🏷️DefinitionLinter queries a definition node by id. */
export class DefinitionLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Definition") {
      throw new Error(`[DefinitionLinter] expected Definition, got ${this.node.__typename}`);
    }
    return this.node;
  }

  filePath(): string {
    const f = this.load().defFile as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().defKind ?? "");
  }

  startLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.start ?? 0);
  }

  endLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.end ?? 0);
  }

  content(): string {
    const full = readFileSync(join(this.repoRoot, this.filePath()), "utf8");
    const lines = full.split(/\r?\n/);
    const s = this.startLine();
    const e = this.endLine();
    if (s <= 0 || e < s) return "";
    return lines.slice(s - 1, e).join("\n");
  }
}

/** 🔎️Resolves folder path to folder graphql row (for script.ts policy placement). */
export function resolveFolderByPath(repoRoot: string, folderPath: string): GraphNode {
  const rel = folderPath.replaceAll("\\", "/").replace(/^\/+/, "");
  const data = runCliGraphql(`query F($p: String!) { folder(path: $p) { __typename id path name } }`, { p: rel }, { repoRoot }) as { folder: GraphNode };
  if (!data.folder?.id) throw new Error(`[linter] folder not found for path ${rel}`);
  return data.folder;
}

/** 🎁️Resolves bundle name like `repo/client` to bundle id. */
export function resolveBundleByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(`query B($n: String!) { bundle(name: $n) { __typename id name root kind } }`, { n: name }, { repoRoot }) as { bundle: GraphNode };
  if (!data.bundle?.id) throw new Error(`[linter] bundle not found for name ${name}`);
  return data.bundle;
}

/** 🗂️Resolves technology folder name (e.g. `repo`) to technology id. */
export function resolveTechnologyByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(`query T { technologies { id name root kind } }`, {}, { repoRoot }) as { technologies: GraphNode[] };
  const hit = (data.technologies ?? []).find((t) => String(t.name ?? "") === name);
  if (!hit?.id) throw new Error(`[linter] technology not found for name ${name}`);
  return hit;
}

//#endregion 🔖️linter

export type LintFn<T extends BaseLinter> = (linter: T) => BreachRecord[] | Promise<BreachRecord[]>;

/** 📜️Tags a lint callback for tooling (runner unwraps default export). */
export function defineLint<T extends BaseLinter>(_tag: string, fn: LintFn<T>): LintFn<T> {
  return fn;
}
