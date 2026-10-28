/** 🧹️ Contract: the TypeScript planner reproduces, from real files in a throwaway git repository, the plan that the independent Python model froze for every shared purge case (the same fixture Go and Rust reproduce); a plan changes nothing and a removal of exactly the planned paths leaves exactly the predicted tree, enumerated with the third-party `glob`. */
import { afterAll, expect, setDefaultTimeout, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { closeSync, ftruncateSync, mkdirSync, mkdtempSync, openSync, readFileSync, realpathSync, rmSync, symlinkSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { globSync } from "glob";
import { testPythonV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {
  CLEAN_GENERATED_LIVE_WRITER_MS,
  CLEAN_GENERATED_OVERSIZED_FILE_BYTES,
  CLEAN_GENERATED_OVERSIZED_FOLDER_BYTES,
  CLEAN_TICKET_GENERATED_DIRECTORY,
  cleanPlanTicketGeneratedPurge,
  type CleanGeneratedPlan,
} from "../../🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts";

setDefaultTimeout(120000);
type Entry = { path: string; kind: "file" | "directory" | "symlink"; bytes?: number; ageSeconds?: number; tracked?: boolean; target?: string };
type Case = { id: string; git: boolean; request: { scope: "closed" | "ticket"; ticket?: string; selection: "oversized" | "generated" }; tickets: { id: string; manifest: string; entries: Entry[] }[]; expect: CleanGeneratedPlan };
const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../🎫️tickets/🧪️tests/🧫️fixtures/🧹️purge-generated/🧹️cases.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { policy: { generatedDirectory: string; oversizedFileBytes: number; oversizedFolderBytes: number; liveWriterSeconds: number }; cases: Case[] };
const manifests: Record<string, string> = { open: '{"status":"open"}', closed: '{"status":"closed"}', closing: '{"status":"closing"}', invalid: "{" };
const scratch = realpathSync(mkdtempSync(join(tmpdir(), "purge-generated-ts-")));
afterAll(() => rmSync(scratch, { recursive: true, force: true }));

const git = (cwd: string, input: string | undefined, ...args: string[]): void => {
  const env = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.toUpperCase().startsWith("GIT_")));
  const result = spawnSync("git", ["-C", cwd, ...args], { env, input, encoding: "utf8" });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")}: ${result.stderr}`);
};
const sparse = (path: string, size: number, modified: number): void => {
  mkdirSync(dirname(path), { recursive: true });
  const descriptor = openSync(path, "w");
  try { ftruncateSync(descriptor, size); } finally { closeSync(descriptor); }
  utimesSync(path, modified / 1000, modified / 1000);
};
const tree = (root: string): string[] => globSync("**/*", { cwd: root, dot: true, ignore: ".git/**", posix: true }).sort();

function materialize(item: Case, now: number): { root: string; ticketsDir: string; external: string } | undefined {
  const root = mkdtempSync(join(scratch, "repo-")), external = mkdtempSync(join(scratch, "external-"));
  sparse(join(external, "blob.bin"), 11 << 20, now - 86_400_000);
  if (item.git) git(root, undefined, "init", "-q");
  const ticketsDir = join(root, ".🧬semio", "🦑️repo", "🎫️tickets"), tracked: string[] = [];
  for (const ticket of item.tickets) {
    const directory = join(ticketsDir, ...ticket.id.split("/"));
    mkdirSync(directory, { recursive: true });
    if (manifests[ticket.manifest] !== undefined) writeFileSync(join(directory, "🎫️ticket.json"), manifests[ticket.manifest]!);
    for (const entry of ticket.entries) {
      const path = join(directory, ...entry.path.split("/"));
      mkdirSync(dirname(path), { recursive: true });
      if (entry.kind === "directory") mkdirSync(path, { recursive: true });
      else if (entry.kind === "symlink") {
        try { symlinkSync(join(external, ...entry.target!.split("/").slice(1)), path); } catch { return undefined; }
      } else {
        sparse(path, entry.bytes ?? 0, now - (entry.ageSeconds ?? 0) * 1000);
        if (entry.tracked) tracked.push(path.slice(root.length + 1).split("\\").join("/"));
      }
    }
  }
  if (tracked.length > 0) git(root, tracked.join("\0"), "add", "-f", "--pathspec-from-file=-", "--pathspec-file-nul");
  return { root, ticketsDir, external };
}

test("the policy matches the schema fixture", () => {
  expect(fixture.policy).toEqual({ generatedDirectory: CLEAN_TICKET_GENERATED_DIRECTORY, oversizedFileBytes: CLEAN_GENERATED_OVERSIZED_FILE_BYTES, oversizedFolderBytes: CLEAN_GENERATED_OVERSIZED_FOLDER_BYTES, liveWriterSeconds: CLEAN_GENERATED_LIVE_WRITER_MS / 1000 });
});

for (const item of fixture.cases) {
  test("plan of " + item.id, () => {
    const now = Math.floor(Date.now() / 1000) * 1000;
    const world = materialize(item, now);
    if (world === undefined) return;
    const before = tree(world.root), externalBefore = tree(world.external);
    const request = { root: world.root, ticketsDir: world.ticketsDir, selection: item.request.selection, now, ...(item.request.scope === "ticket" ? { ticketDirs: [join(world.ticketsDir, ...item.request.ticket!.split("/"))] } : {}) };
    expect(cleanPlanTicketGeneratedPurge(request)).toEqual(item.expect);
    expect(tree(world.root)).toEqual(before);
    for (const removal of item.expect.removals) rmSync(join(world.ticketsDir, ...removal.path.split("/")), { recursive: true, force: true });
    const survivors = before.filter((path) => !item.expect.removals.some((removal) => { const prefix = ".🧬semio/🦑️repo/🎫️tickets/" + removal.path; return path === prefix || path.startsWith(prefix + "/"); }));
    expect(tree(world.root)).toEqual(survivors);
    expect(tree(world.external)).toEqual(externalBefore);
  });
}

test("the independent Python model reaches the same plans and git agrees with the declared tracked files", () => {
  const python = testPythonV1();
  if (!python) throw Error("Independent Python oracle required");
  const result = Bun.spawnSync([python, "-I", join(dirname(fixturePath), "../../🧹️purge-generated/🐍️.py"), "check"]);
  expect(result.stderr.toString()).toBe("");
  expect(result.stdout.toString()).toContain(" 0 failures");
  expect(result.exitCode).toBe(0);
});
