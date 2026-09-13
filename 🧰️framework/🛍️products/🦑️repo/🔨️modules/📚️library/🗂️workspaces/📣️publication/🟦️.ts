import { lstatSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { type WorkspaceDiscoveryOptions } from "../🟦️.ts";
import { parseWorkspaceRootDocument, renderWorkspaceRootDocument } from "../📄️manifest-projection/🟦️.ts";
import { compareWorkspaceMembership } from "../⚖️membership-comparison/🟦️.ts";

export interface WorkspacePublicationOperations {
  readonly readText: (path: string) => string;
  readonly report: (stream: "stderr" | "stdout", message: string) => void;
  readonly state: (path: string) => "file" | "other" | "symlink";
  readonly writeText: (path: string, source: string) => void;
}

export interface WorkspacePublicationResult {
  readonly expectedCount: number;
  readonly fresh: boolean;
  readonly missing: readonly string[];
  readonly orderChanged: boolean;
  readonly stale: readonly string[];
  readonly written: boolean;
}

export interface WorkspacePublicationOptions {
  readonly discovery?: WorkspaceDiscoveryOptions;
  readonly operations?: Partial<WorkspacePublicationOperations>;
  readonly report?: WorkspacePublicationOperations["report"];
}

const NATIVE_OPERATIONS: WorkspacePublicationOperations = {
  readText: (path) => readFileSync(path, "utf8"),
  report: (stream, message) => (stream === "stderr" ? console.error(message) : console.log(message)),
  state: (path) => {
    const state = lstatSync(path);
    return state.isSymbolicLink() ? "symlink" : state.isFile() ? "file" : "other";
  },
  writeText: (path, source) => writeFileSync(path, source),
};

/** 📣️ Checks or publishes root workspace membership through one admitted physical package document. */
export function publishWorkspaceMembership(repoRoot: string, mode: "check" | "write", options: WorkspacePublicationOptions = {}): WorkspacePublicationResult {
  const operations = { ...NATIVE_OPERATIONS, ...options.operations, report: options.report ?? options.operations?.report ?? NATIVE_OPERATIONS.report };
  const path = join(repoRoot, "package.json");
  const state = operations.state(path);
  if (state !== "file") throw new Error(`Root package.json must be a physical regular file (${state})`);
  const source = operations.readText(path);
  const document = parseWorkspaceRootDocument(source);
  const current = document.workspaces ?? [];
  const comparison = compareWorkspaceMembership(repoRoot, current, options.discovery);
  if (operations.state(path) !== "file" || operations.readText(path) !== source) throw new Error("Root package.json changed during workspace discovery");
  if (mode === "check") {
    if (!comparison.fresh) {
      operations.report("stderr", `root package.json workspaces is stale (${comparison.expected.length} expected, ${current.length} current).`);
      if (comparison.missing.length > 0) operations.report("stderr", `  missing: ${comparison.missing.join(", ")}`);
      if (comparison.stale.length > 0) operations.report("stderr", `  stale:   ${comparison.stale.join(", ")}`);
      if (comparison.orderChanged) operations.report("stderr", "  (same set, different order)");
      operations.report("stderr", "run `bun ./📜️script.ts workspaces --write` to refresh.");
      throw new Error("Root workspace membership is stale");
    }
    operations.report("stdout", `root package.json workspaces is fresh (${comparison.expected.length} packages).`);
    return { ...comparison, expectedCount: comparison.expected.length, written: false };
  }
  if (comparison.fresh) {
    operations.report("stdout", `root package.json workspaces already fresh (${comparison.expected.length} packages) — no write needed.`);
    return { ...comparison, expectedCount: comparison.expected.length, written: false };
  }
  operations.writeText(path, renderWorkspaceRootDocument(document, comparison.expected));
  operations.report("stdout", `root package.json workspaces regenerated -> ${comparison.expected.length} packages.`);
  return { ...comparison, expectedCount: comparison.expected.length, written: true };
}
