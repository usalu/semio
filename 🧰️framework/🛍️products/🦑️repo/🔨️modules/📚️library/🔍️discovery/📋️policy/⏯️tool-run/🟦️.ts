import {policyMaskLiterals,policyTestModSpans,policyLineInTestMod,interactivityCfgTestItemSpans} from "../🔎️source/🟦️.ts";

/**
 * ⏯️ One tool the phase-4 inventory (`26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📓️audit-p4-tool-inventory.md`) classifies
 * `algorithmic-mutating`. `toolId` is the Tool/UtilityDefinition id expected under `root` (artifact subset or engine
 * dir); `scope` lists the tool's own files or dirs (relative to `root` unless repo-rooted); `verbs` are per-plugin run verbs and lifecycle states that must vanish from `root` once `run` is declared, `measures` the
 * plugin-local progress/cancel measures that must vanish from `scope`; `lane` is the converting contract lane.
 */
export type InteractivityToolRunRequirement = {
  readonly toolId: string;
  readonly root: string;
  readonly scope: readonly string[];
  readonly verbs: readonly string[];
  readonly measures: readonly string[];
  readonly lane: string;
  readonly inventory: string;
};

/** ⏯️ A repo-relative source handed to the tool-run policy predicates. */
export type InteractivityToolRunSource = { readonly path: string; readonly text: string };

/** ⏯️ A tool-run policy violation anchored to a file and a 1-based line (0 when a whole file or table row is at fault). */
export type InteractivityToolRunFinding = { readonly file: string; readonly line: number; readonly text: string };

type InteractivityToolRunDeclaration = { readonly id: string; readonly kind: "Tool" | "Utility"; readonly file: string; readonly line: number; readonly run: boolean };

type InteractivityToolRunTool = { readonly label: string; readonly root: string; readonly scope: readonly string[]; readonly flat: boolean; readonly verbs: readonly string[]; readonly measures: readonly string[]; readonly declared: boolean };

type InteractivityToolRunIndex = { readonly code: ReadonlyMap<string, readonly string[]>; readonly declarations: readonly InteractivityToolRunDeclaration[] };

export const INTERACTIVITY_TOOL_RUN_TRIGGER = /run\s*:\s*Some\s*\(|\.run\s*=\s*Some\s*\(|toolRun[A-Z]|TOOL_RUN_\w+_ACTION_ID|TRIED_|FillTried|WorldFillTried|push_tried|fillBuildPreview|FillBuildPreview|fill_build_preview|FILL_PREVIEW_JSON|FillPreviewJson|fill_preview_json/;

const INTERACTIVITY_TOOL_RUN_LEGACY_TRACE = /\b(?:\w*_TRIED_RING|\w*_TRIED_MAX|FillTried\w*|WorldFillTried\w*|push_tried|fillBuildPreview|FillBuildPreview|fill_build_preview|\w*FILL_PREVIEW_JSON\w*|FillPreviewJson\w*|fill_preview_json\w*)\b/;

const INTERACTIVITY_TOOL_RUN_LOCAL_LIFECYCLE: readonly RegExp[] = [
  /\bfn\s+\w*(?:progress|cancel)_measure\b/,
  /["'`](?:cancel|abort|stop|retry|discard|adopt|pause|resume|finalize)[A-Z]\w*["'`]/,
  /["'`][a-z]\w*(?:Tick|Cancel|Abort|Adopt|Discard|Retry|Finalize)["'`]/,
  /["'`](?:start|cancel|abort|stop|retry|discard|adopt|pause|resume|finalize)-[a-z0-9-]+["'`]/,
];

const INTERACTIVITY_TOOL_RUN_RESERVED: readonly RegExp[] = [
  /["'`]toolRun[A-Z]\w*["'`]/,
  /\bActionDefinition::new(?:_catalog)?\s*\(\s*(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\b/,
  /\b(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\s*(?:\|\s*(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\s*)*=>/,
];

const INTERACTIVITY_TOOL_RUN_INDEXES = new WeakMap<readonly InteractivityToolRunSource[], InteractivityToolRunIndex>();

/** 📍️ True when `path` is `prefix` or lies below it. */
export function interactivityToolRunWithin(path: string, prefix: string): boolean {
  return path === prefix || path.startsWith(`${prefix}/`);
}

/** 📍️ A requirement scope entry as a repo-relative path. */
export function interactivityToolRunScopePath(root: string, entry: string): string {
  return entry.startsWith("✏️s/") || entry.startsWith("🧰️framework/") ? entry : `${root}/${entry}`;
}

/** ✂️ Line-aligned code of one source: comments blanked, string literals kept, Rust `#[cfg(test)]` items emptied. */
export function interactivityToolRunCode(source: InteractivityToolRunSource): string[] {
  const lines = source.text.split(/\r?\n/);
  const tests = source.path.endsWith(".rs") ? [...policyTestModSpans(lines), ...interactivityCfgTestItemSpans(lines)] : [];
  let depth = 0;
  return lines.map((raw, index) => {
    const masked = policyMaskLiterals(raw);
    let code = "";
    for (let at = 0; at < raw.length; ) {
      const pair = masked.slice(at, at + 2);
      if (depth > 0) {
        depth += pair === "/*" ? 1 : pair === "*/" ? -1 : 0;
        at += pair === "/*" || pair === "*/" ? 2 : 1;
      } else if (pair === "//") {
        break;
      } else if (pair === "/*") {
        depth = 1;
        at += 2;
      } else {
        code += raw[at];
        at += 1;
      }
    }
    return policyLineInTestMod(tests, index + 1) ? "" : code;
  });
}

/** 🧱️ The innermost `opener … }` span of `text` (literal-safe) that encloses offset `at`. */
export function interactivityToolRunEnclosing(text: string, at: number, opener: RegExp): string | undefined {
  if (at < 0) return undefined;
  const masked = policyMaskLiterals(text);
  let enclosing: string | undefined;
  for (const match of masked.matchAll(opener)) {
    const open = match.index + match[0].length - 1;
    if (open > at) break;
    let depth = 0;
    let close = masked.length;
    for (let index = open; index < masked.length; index += 1) {
      depth += masked[index] === "{" ? 1 : masked[index] === "}" ? -1 : 0;
      if (depth === 0) {
        close = index;
        break;
      }
    }
    if (close >= at) enclosing = text.slice(match.index, close + 1);
  }
  return enclosing;
}

/** 🗂️ Code lines per source plus every Tool/UtilityDefinition declaration under `✏️s/`, its resolved id and whether it declares `run`. */
function interactivityToolRunIndex(sources: readonly InteractivityToolRunSource[]): InteractivityToolRunIndex {
  const cached = INTERACTIVITY_TOOL_RUN_INDEXES.get(sources);
  if (cached) return cached;
  const code = new Map(sources.map((source) => [source.path, interactivityToolRunCode(source)] as const));
  const constants = new Map<string, Map<string, Set<string>>>();
  const plugin = (path: string) => path.split("/").slice(0, 3).join("/");
  for (const [path, lines] of code) {
    if (!path.endsWith(".rs")) continue;
    for (const match of lines.join("\n").matchAll(/\bconst\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]*)"/g)) {
      for (const key of [path, plugin(path)]) {
        const byName = constants.get(key) ?? new Map<string, Set<string>>();
        byName.set(match[1]!, (byName.get(match[1]!) ?? new Set<string>()).add(match[2]!));
        constants.set(key, byName);
      }
    }
  }
  const resolve = (path: string, expression: string): string | undefined => {
    const literal = expression.match(/^"([^"]*)"$/);
    if (literal) return literal[1];
    const name = expression.match(/^(?:[\w]+::)*([A-Z][A-Z0-9_]*)$/)?.[1];
    if (!name) return undefined;
    for (const key of [path, plugin(path)]) {
      const values = constants.get(key)?.get(name);
      if (values?.size === 1) return [...values][0];
    }
    return undefined;
  };
  const declarations: InteractivityToolRunDeclaration[] = [];
  for (const [path, lines] of code) {
    if (!path.startsWith("✏️s/") || !path.endsWith(".rs")) continue;
    const joined = lines.join("\n");
    for (const match of joined.matchAll(/\b(Tool|Utility)Definition::new\(\s*([^,()]+?)\s*,/g)) {
      const id = resolve(path, match[2]!.trim());
      if (id === undefined) continue;
      const kind = match[1] as "Tool" | "Utility";
      const literal = interactivityToolRunEnclosing(joined, match.index, new RegExp(`\\b${kind}Definition\\s*\\{`, "g"));
      const body = interactivityToolRunEnclosing(joined, match.index, /\bfn\s+\w+[^{;]*\{/g);
      const run = (literal !== undefined && /\brun\s*:\s*Some\s*\(/.test(literal)) || (body !== undefined && /\.run\s*=\s*Some\s*\(/.test(body));
      declarations.push({ id, kind, file: path, line: joined.slice(0, match.index).split("\n").length, run });
    }
  }
  const index = { code, declarations };
  INTERACTIVITY_TOOL_RUN_INDEXES.set(sources, index);
  return index;
}

/** 🧰️ Tools the lifecycle predicate governs: every requirement row, plus every run-declaring tool no row names (scoped to its declaring directory). */
function interactivityToolRunTools(sources: readonly InteractivityToolRunSource[], requirements: readonly InteractivityToolRunRequirement[]): InteractivityToolRunTool[] {
  const { declarations } = interactivityToolRunIndex(sources);
  const rows = requirements.map((row) => ({
    label: `${row.toolId} (${row.root}, lane ${row.lane}, inventory ${row.inventory})`,
    root: row.root,
    scope: row.scope.map((entry) => interactivityToolRunScopePath(row.root, entry)),
    flat: false,
    verbs: row.verbs,
    measures: row.measures,
    declared: declarations.some((declaration) => declaration.run && declaration.id === row.toolId && interactivityToolRunWithin(declaration.file, row.root)),
  }));
  const discovered = declarations
    .filter((declaration) => declaration.run && !requirements.some((row) => row.toolId === declaration.id && interactivityToolRunWithin(declaration.file, row.root)))
    .map((declaration) => {
      const directory = declaration.file.slice(0, declaration.file.lastIndexOf("/"));
      return { label: `${declaration.id} (${declaration.file}:${declaration.line})`, root: directory, scope: [directory], flat: true, verbs: [], measures: [], declared: true };
    });
  return [...rows, ...discovered];
}

/** 📍️ True when `path` belongs to the tool's own files. */
function interactivityToolRunInScope(tool: InteractivityToolRunTool, path: string): boolean {
  return tool.scope.some((entry) => (tool.flat ? path.slice(0, path.lastIndexOf("/")) === entry : interactivityToolRunWithin(path, entry)));
}

/** 🔎️ A word-bounded token matcher that treats `-` as part of a token. */
function interactivityToolRunToken(token: string): RegExp {
  return new RegExp(`(?<![\\w-])${token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}(?![\\w-])`);
}

/** ⏯️ (1) Once a tool declares `run`, no per-plugin run verb, lifecycle state or plugin-local progress/cancel measure survives (`📋️tool-run-contract.md` §2.5, §3.6). */
export function interactivityToolRunLocalLifecycleFailures(sources: readonly InteractivityToolRunSource[], requirements: readonly InteractivityToolRunRequirement[]): InteractivityToolRunFinding[] {
  const { code } = interactivityToolRunIndex(sources);
  const findings: InteractivityToolRunFinding[] = [];
  for (const tool of interactivityToolRunTools(sources, requirements).filter((candidate) => candidate.declared)) {
    const verbs = tool.verbs.map((verb) => [verb, interactivityToolRunToken(verb)] as const);
    const measures = tool.measures.map((measure) => [measure, interactivityToolRunToken(measure)] as const);
    for (const [path, lines] of code) {
      const rooted = interactivityToolRunWithin(path, tool.root);
      const scoped = interactivityToolRunInScope(tool, path);
      if (!rooted && !scoped) continue;
      lines.forEach((line, index) => {
        const hits = [...(rooted ? verbs : []), ...(scoped ? measures : [])].filter(([, pattern]) => pattern.test(line)).map(([token]) => token);
        if (scoped) hits.push(...INTERACTIVITY_TOOL_RUN_LOCAL_LIFECYCLE.flatMap((pattern) => line.match(pattern)?.[0] ?? []));
        for (const hit of new Set(hits)) findings.push({ file: path, line: index + 1, text: `[⏯️ local-lifecycle] ${tool.label}: ${hit} survives next to a declared ToolRunDefinition; use the framework toolRun* actions, ToolRunState and the ToolRun panel (contract §2.5, §3.6)` });
      });
    }
  }
  return findings;
}

/** ⏯️ (2) No tried-candidate ring, `fillBuildPreview` tail or fill preview JSON cap survives anywhere; tested candidates travel only as trace pages (`📋️tool-run-contract.md` §3.2, §3.6). */
export function interactivityToolRunLegacyTraceFailures(sources: readonly InteractivityToolRunSource[]): InteractivityToolRunFinding[] {
  const { code } = interactivityToolRunIndex(sources);
  const findings: InteractivityToolRunFinding[] = [];
  for (const [path, lines] of code) {
    lines.forEach((line, index) => {
      const hit = line.match(INTERACTIVITY_TOOL_RUN_LEGACY_TRACE)?.[0];
      if (hit) findings.push({ file: path, line: index + 1, text: `[⏯️ legacy-trace] ${hit} survives; tested candidates travel only as ToolRunTracePage deltas rendered by the ToolRunTraceLayer (contract §3.2, §3.6)` });
    });
  }
  return findings;
}

/** ⏯️ (3) Every `algorithmic-mutating` tool of [[INTERACTIVITY_TOOL_RUN_REQUIREMENTS]] declares `run: Some(ToolRunDefinition)` on its Tool/UtilityDefinition, and every table row still points at real sources (`📋️tool-run-contract.md` §2.4, §3.7). */
export function interactivityToolRunDeclarationFailures(sources: readonly InteractivityToolRunSource[], requirements: readonly InteractivityToolRunRequirement[]): InteractivityToolRunFinding[] {
  const { code, declarations } = interactivityToolRunIndex(sources);
  const paths = [...code.keys()];
  const findings: InteractivityToolRunFinding[] = [];
  const seen = new Set<string>();
  for (const row of requirements) {
    const label = `[⏯️ declaration] ${row.toolId} (${row.root}, lane ${row.lane}, inventory ${row.inventory})`;
    const key = `${row.root}#${row.toolId}`;
    if (seen.has(key)) findings.push({ file: row.root, line: 0, text: `${label}: duplicate requirement row` });
    seen.add(key);
    if (!paths.some((path) => interactivityToolRunWithin(path, row.root))) {
      findings.push({ file: row.root, line: 0, text: `${label}: requirement root has no sources (stale row)` });
      continue;
    }
    if (row.scope.length > 0 && !row.scope.some((entry) => paths.some((path) => interactivityToolRunWithin(path, interactivityToolRunScopePath(row.root, entry))))) findings.push({ file: row.root, line: 0, text: `${label}: no scope entry exists any more (stale row)` });
    const matching = declarations.filter((declaration) => declaration.id === row.toolId && interactivityToolRunWithin(declaration.file, row.root));
    if (matching.some((declaration) => declaration.run)) continue;
    const found = matching[0];
    findings.push(found ? { file: found.file, line: found.line, text: `${label}: ${found.kind}Definition "${row.toolId}" declares no run: Some(ToolRunDefinition) (contract §2.4)` } : { file: row.root, line: 0, text: `${label}: no Tool/UtilityDefinition "${row.toolId}" exists; the algorithm is not a tool run yet (contract §2.4, §3.7)` });
  }
  return findings;
}

/** ⏯️ (4) `toolRun*` action ids are framework-reserved: plugins never declare, literal-copy or route them (`📋️tool-run-contract.md` §2.5). */
export function interactivityToolRunReservedActionFailures(sources: readonly InteractivityToolRunSource[]): InteractivityToolRunFinding[] {
  const { code } = interactivityToolRunIndex(sources);
  const findings: InteractivityToolRunFinding[] = [];
  for (const [path, lines] of code) {
    if (!path.startsWith("✏️s/")) continue;
    lines.forEach((line, index) => {
      const hit = INTERACTIVITY_TOOL_RUN_RESERVED.map((pattern) => line.match(pattern)?.[0]).find((match) => match !== undefined);
      if (hit) findings.push({ file: path, line: index + 1, text: `[⏯️ reserved-action] ${hit.trim()}: toolRun* actions are framework-reserved and injected from ToolRunDefinition; reference the framework constants only to dispatch (contract §2.5)` });
    });
  }
  return findings;
}
