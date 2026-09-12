import { lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { getRepoMetaDir } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { canonicalJson, TICKET_GENERATED_OUTPUT_DIRECTORY } from "../🟦️.ts";

/** 🎫️ Resolves a ticket id through actual Unicode directory entries instead of constructing emoji mounts. */
export function taxonomyTicketDirectory(repoRoot: string, ticketId: string): string {
  const parts = ticketId.split("/").filter(Boolean);
  if (parts.length !== 4) throw new Error(`[taxonomy] ticket id must be YYYY/MM/DD/TICKETSLUG, got ${JSON.stringify(ticketId)}.`);
  let current = realpathSync(join(getRepoMetaDir(repoRoot), "🎫️tickets"));
  for (const part of parts) {
    const entry = readdirSync(current, { withFileTypes: true })
      .filter((candidate) => candidate.isDirectory())
      .sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0)
      .find((candidate) => candidate.name === part || candidate.name.replace(/^\p{Extended_Pictographic}\uFE0F?/u, "") === part);
    if (!entry) throw new Error(`[taxonomy] ticket segment ${JSON.stringify(part)} does not exist below ${current}.`);
    current = realpathSync(join(current, entry.name));
  }
  return current;
}


export type TaxonomyCliFormat = "human" | "json";

export type TaxonomyCliOperation = "inventory" | "plan" | "apply" | "verify";

export type TaxonomyCliKind = "mutation";


export const TAXONOMY_CLI_ARTIFACT_DIRECTORIES: Readonly<Record<TaxonomyCliOperation, Readonly<{ json: string; markdown: string }>>> = {
  inventory: { json: "📊️taxonomy-inventory", markdown: "📓️taxonomy-inventory" },
  plan: { json: "📊️taxonomy-plan", markdown: "📓️taxonomy-plan" },
  apply: { json: "📊️taxonomy-apply", markdown: "📓️taxonomy-apply" },
  verify: { json: "📊️taxonomy-verification", markdown: "📓️taxonomy-verification" },
};


export type TaxonomyCliOptions = {
  baseline?: string;
  cancelFile?: string;
  digest?: string;
  failOnWarning: boolean;
  format: TaxonomyCliFormat;
  kind?: TaxonomyCliKind;
  plan?: string;
  resume?: string;
  scope?: string;
  ticket?: string;
  workers?: number;
};


export function taxonomyCliOptions(args: readonly string[]): TaxonomyCliOptions {
  const options: TaxonomyCliOptions = { failOnWarning: false, format: "human" };
  const values: Readonly<Record<string, keyof Omit<TaxonomyCliOptions, "failOnWarning" | "format" | "workers">>> = {
    "--baseline": "baseline",
    "--cancel-file": "cancelFile",
    "--digest": "digest",
    "--kind": "kind",
    "--plan": "plan",
    "--resume": "resume",
    "--scope": "scope",
    "--ticket": "ticket",
  };
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index]!;
    if (arg === "--fail-on-warning") {
      options.failOnWarning = true;
      continue;
    }
    if (arg === "--format") {
      const format = args[++index];
      if (format !== "human" && format !== "json") throw new Error(`[clean taxonomy] --format must be human or json, got ${JSON.stringify(format)}.`);
      options.format = format;
      continue;
    }
    if (arg === "--workers") {
      const workers = Number(args[++index]);
      if (!Number.isSafeInteger(workers) || workers < 1) throw new Error("[clean taxonomy] --workers must be a positive integer.");
      options.workers = workers;
      continue;
    }
    const key = values[arg];
    if (!key) throw new Error(`[clean taxonomy] unknown option ${JSON.stringify(arg)}.`);
    const value = args[++index];
    if (!value || value.startsWith("--")) throw new Error(`[clean taxonomy] ${arg} requires a value.`);
    if (key === "kind") {
      if (value !== "mutation") throw new Error(`[clean taxonomy] --kind must be mutation, got ${JSON.stringify(value)}.`);
      options.kind = value;
    } else options[key] = value;
  }
  return options;
}


/** 🔒️ Rejects authority-bearing options outside their exact taxonomy operation. */
export function taxonomyCliValidateOperationOptions(operation: TaxonomyCliOperation, options: TaxonomyCliOptions): void {
  const restricted: readonly [keyof TaxonomyCliOptions, string, readonly TaxonomyCliOperation[]][] = [
    ["baseline", "--baseline", ["plan", "apply"]],
    ["digest", "--digest", ["apply"]],
    ["plan", "--plan", ["plan", "apply"]],
    ["resume", "--resume", ["apply"]],
    ["scope", "--scope", ["inventory", "plan", "verify"]],
    ["workers", "--workers", ["inventory", "plan", "verify"]],
  ];
  for (const [key, option, operations] of restricted) {
    if (options[key] !== undefined && !operations.includes(operation)) throw new Error(`[clean taxonomy ${operation}] ${option} is not valid for this operation.`);
  }
  if (options.failOnWarning && operation !== "verify") throw new Error(`[clean taxonomy ${operation}] --fail-on-warning is not valid for this operation.`);
}


/** 🚫️ Resolves one CLI path lexically and rejects opaque prefixes before filesystem access. */
export function taxonomyCliGuardedPath(root: string, path: string | undefined, option: "--plan" | "--cancel-file" | "--resume"): string | undefined {
  if (!path) return undefined;
  const workspaceRoot = resolve(root);
  const absolute = resolve(workspaceRoot, path);
  const workspaceRelative = relative(workspaceRoot, absolute).replaceAll("\\", "/").normalize("NFC");
  if (workspaceRelative === ".." || workspaceRelative.startsWith("../") || isAbsolute(workspaceRelative)) throw new Error(`[clean taxonomy] ${option} must remain inside the repository.`);
  if (workspaceRelative === "compose" || workspaceRelative.startsWith("compose/") || workspaceRelative === "temp/compose" || workspaceRelative.startsWith("temp/compose/")) throw new Error(`[clean taxonomy] ${option} cannot access opaque path ${JSON.stringify(workspaceRelative)}.`);
  const segments = workspaceRelative.split("/").filter(Boolean);
  let ancestor = workspaceRoot;
  for (let index = 0; index < segments.length; index += 1) {
    ancestor = join(ancestor, segments[index]!);
    let state: ReturnType<typeof lstatSync> | null = null;
    try {
      state = lstatSync(ancestor);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
    if (!state) break;
    if (state.isSymbolicLink()) throw new Error(`[clean taxonomy] ${option} cannot traverse symlink ${JSON.stringify(segments.slice(0, index + 1).join("/"))}.`);
    if (index < segments.length - 1 && !state.isDirectory()) throw new Error(`[clean taxonomy] ${option} has a non-directory ancestor ${JSON.stringify(segments.slice(0, index + 1).join("/"))}.`);
    if (index === segments.length - 1 && state.isDirectory()) throw new Error(`[clean taxonomy] ${option} must name a file path.`);
  }
  return absolute;
}


export function taxonomyCliTicket(root: string, ticket: string | undefined): string | undefined {
  return ticket ? taxonomyTicketDirectory(root, ticket) : undefined;
}


export function taxonomyCliArtifactPath(ticketDir: string, operation: TaxonomyCliOperation, format: "json" | "markdown"): string {
  const directory = TAXONOMY_CLI_ARTIFACT_DIRECTORIES[operation][format];
  return join(ticketDir, TICKET_GENERATED_OUTPUT_DIRECTORY, directory, format === "json" ? "🔣️.json" : "📝️.md");
}

//#endregion 📊️TaxonomyInventoryShards

export function taxonomyCliWriteJson(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${canonicalJson(value)}\n`);
}


export function taxonomyCliWriteSummary(ticketDir: string | undefined, operation: TaxonomyCliOperation, title: string, rows: readonly string[]): void {
  if (!ticketDir) return;
  const path = taxonomyCliArtifactPath(ticketDir, operation, "markdown");
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, [`# ${title}`, "", ...rows, ""].join("\n"));
}


export function taxonomyCliProgress(event: { readonly operation: string; readonly phase: string; readonly current: number; readonly total: number; readonly path?: string }): void {
  if (event.total === 0 && event.phase.startsWith("transaction-")) return;
  if (event.current === 0 || event.current === event.total || event.current % 100 === 0) console.error(`[clean taxonomy progress] ${event.operation} ${event.phase} ${event.current}/${event.total}${event.path ? ` ${event.path}` : ""}`);
}


export function taxonomyCliInventoryOptions(root: string, options: TaxonomyCliOptions, cancelFile: string | undefined) {
  return {
    repoRoot: root,
    ...(options.scope ? { scope: options.scope } : {}),
    ...(cancelFile ? { cancelFile } : {}),
    ...(options.workers ? { workers: options.workers } : {}),
    progress: taxonomyCliProgress,
  };
}


export interface TaxonomyCliPlanOperationCounts {
  readonly moves: number;
  readonly embeddedTicketRoots: number;
  readonly embeddedTicketRootRelocations: number;
  readonly symlinkTargetEdits: number;
  readonly evidenceRemovals: number;
  readonly edits: number;
  readonly regenerations: number;
}


/** 📊️ Captures every root mutation group shown by plan reports. */
export function taxonomyCliPlanOperationCounts(plan: Readonly<Record<keyof TaxonomyCliPlanOperationCounts, readonly unknown[]>>): TaxonomyCliPlanOperationCounts {
  return {
    moves: plan.moves.length,
    embeddedTicketRoots: plan.embeddedTicketRoots.length,
    embeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length,
    symlinkTargetEdits: plan.symlinkTargetEdits.length,
    evidenceRemovals: plan.evidenceRemovals.length,
    edits: plan.edits.length,
    regenerations: plan.regenerations.length,
  };
}


/** 📝️ Renders every plan operation group for the Markdown evidence. */
export function taxonomyCliPlanOperationSummaryRows(counts: TaxonomyCliPlanOperationCounts): readonly string[] {
  return [
    `- Moves: ${counts.moves}`,
    `- Embedded ticket roots: ${counts.embeddedTicketRoots}`,
    `- Embedded ticket-root relocations: ${counts.embeddedTicketRootRelocations}`,
    `- Symlink target edits: ${counts.symlinkTargetEdits}`,
    `- Evidence removals: ${counts.evidenceRemovals}`,
    `- Edits: ${counts.edits}`,
    `- Regenerations: ${counts.regenerations}`,
  ];
}


/** ⌨️ Renders every plan operation group for the human console. */
export function taxonomyCliPlanOperationConsoleFields(counts: TaxonomyCliPlanOperationCounts): string {
  return `moves=${counts.moves} roots=${counts.embeddedTicketRoots} relocations=${counts.embeddedTicketRootRelocations} symlinks=${counts.symlinkTargetEdits} removals=${counts.evidenceRemovals} edits=${counts.edits} regenerations=${counts.regenerations}`;
}


/** ✅️ Makes every non-committed apply terminal state fail after evidence publication. */
export function taxonomyCliRequireCommittedApply(state: string): void {
  if (state !== "committed") throw new Error(`[clean taxonomy apply] terminal state ${JSON.stringify(state)} is not committed.`);
}


export function taxonomyCliPrintJson(value: unknown): void {
  process.stdout.write(`${canonicalJson(value)}\n`);
}
