import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join, resolve, sep, win32 } from "node:path";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

describe("active ticket clean protection", () => {
  test("projects synthetic candidates with strict manifest and third-party oracle parity", async () => {
    const { cleanRemovalProtection, cleanProjectRemovals } = await import("../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts");
    const fixturePath = join(contractRoot, "../🧫️fixtures/🧼️clean");
    const fixture = JSON.parse(readFileSync(join(fixturePath, "🔣️.json"), "utf8"));
    
    expect(fixture["version"]).toEqual(1);
    
    const root = resolve("/synthetic-workspace");
    const nodes = new Map<string, { kind: "directory" | "file" | "symlink" | "unreadable"; text?: string }>([[root, { kind: "directory" }]]);
    const add = (path: string, kind: "directory" | "file" | "symlink" | "unreadable", text?: string): void => {
      const absolute = resolve(root, path); let parent = dirname(absolute);
      while (parent !== root && parent.startsWith(root + sep)) { if (!nodes.has(parent)) nodes.set(parent, { kind: "directory" }); parent = dirname(parent); }
      nodes.set(absolute, { kind, text });
    };
    for (const ticket of fixture.tickets) { add(ticket.path, "directory"); add(`${ticket.path}/log.txt`, "file", "evidence"); add(`${ticket.path}/target`, "directory"); if (ticket.manifest !== null) add(`${ticket.path}/🎫️ticket.json`, "file", ticket.manifest); }
    for (const entry of fixture.special) add(entry.path, entry.kind);
    const view = {
      kind: (path: string) => nodes.get(path)?.kind ?? "missing" as const,
      children: (path: string) => nodes.get(path)?.kind === "directory" ? [...nodes.keys()].filter((key) => key !== path && dirname(key) === path).map((key) => basename(key)) : undefined,
      read: (path: string) => nodes.get(path)?.text,
    };
    const validClosed = new Ajv({ strict: true }).compile({ type: "object", required: ["status"], properties: { status: { const: "closed" } }, additionalProperties: true });
    const intersects = (left: string, right: string) => left === right || left.startsWith(right + "/") || right.startsWith(left + "/");
    const protectedPaths = fixture.tickets.filter((ticket: { manifest: string | null }) => { try { return !validClosed(JSON.parse(ticket.manifest ?? "null")); } catch { return true; } }).map((ticket: { path: string }) => ticket.path);
    protectedPaths.push(...fixture.special.map((entry: { path: string }) => entry.path));
    const oracle = (path: string) => path !== "." && !path.startsWith("..") && !protectedPaths.some((prefix: string) => intersects(path, prefix));
    for (const vector of fixture.cases) {
      expect(oracle(vector.candidate)).toBe(vector.allowed);
      expect(cleanRemovalProtection(root, vector.candidate, view).length === 0).toBe(vector.allowed);
    }
    const parent = fixture.tickets.find((ticket: { path: string }) => ticket.path.endsWith("/NESTED")).path;
    const candidates = [{ kind: "gitignore" as const, path: parent, bytes: 99 }, { kind: "ticket-dir" as const, path: `${parent}/target`, bytes: 11 }];
    expect(cleanProjectRemovals(root, candidates, [], view).map((row) => row.path)).toEqual([`${parent}/target`]);
    const allowed = `${fixture.reopened}/target`;
    expect(cleanProjectRemovals(root, [{ kind: "ticket-dir", path: allowed, bytes: 11 }], [], view)).toHaveLength(1);
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"open"}');
    expect(cleanRemovalProtection(root, allowed, view)).toContain(resolve(root, fixture.reopened));
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"closed"}');
    let reads = 0;
    const changesDuringWalk = { ...view, read: (path: string) => path === resolve(root, `${fixture.reopened}/🎫️ticket.json`) && ++reads > 1 ? '{"status":"open"}' : view.read(path) };
    expect(cleanRemovalProtection(root, allowed, changesDuringWalk)).toContain(resolve(root, fixture.reopened));
    for (const manifest of ['{"status":null}', '{"status":0}', 'null', '"closed"']) { add(`${fixture.reopened}/🎫️ticket.json`, "file", manifest); expect(cleanRemovalProtection(root, allowed, view)).toContain(resolve(root, fixture.reopened)); }
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"closed"}');
    const unreadableChildren = { ...view, children: (path: string) => path === resolve(root, allowed) ? undefined : view.children(path) };
    expect(cleanRemovalProtection(root, allowed, unreadableChildren)).toContain(resolve(root, allowed));
    const changesAncestor = { ...view, children: (path: string) => { if (path === resolve(root, allowed)) add(fixture.reopened, "symlink"); return view.children(path); } };
    expect(cleanRemovalProtection(root, allowed, changesAncestor)).toContain(resolve(root, fixture.reopened));
    add(fixture.reopened, "directory");
    add(`${fixture.reopened}/🎫️ticket.json`, "symlink");
    expect(cleanRemovalProtection(root, allowed, view)).toContain(resolve(root, fixture.reopened));
    add(`${fixture.reopened}/🎫️ticket.json`, "file", '{"status":"closed"}');
    const blocked: string[] = [];
    expect(cleanProjectRemovals(root, candidates, [], view, (path) => blocked.push(path)).map((row) => row.path)).toEqual([`${parent}/target`]);
    expect(blocked).toContain(resolve(root, `${parent}/child`));
    expect(cleanProjectRemovals(root, [{ kind: "misplaced", path: ".🧬semio", bytes: 99 }], [resolve(root, ".🧬semio/🗺️map")], view)).toEqual([]);
  });
});

describe("Windows checkout ticket paths", () => {
  test("rejects Windows-illegal components and keeps checkout paths below the legacy path limit", async () => {
    const { cleanIsWindowsIllegalName } = await import("../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts");
    const root = findWorkspaceRoot(contractRoot);
    const fixturePath = join(contractRoot, "../🧫️fixtures/🪟️windows-checkout-paths");
    const fixture = JSON.parse(readFileSync(join(fixturePath, "🔣️.json"), "utf8")) as {
      version: number;
      checkout: { root: string; maxPathUnits: number; ticketRoot: string };
      checkoutRoots?: { root: string; maxPathUnits: number; scope: "ticket" | "all" }[];
      components: { name: string; illegal: boolean }[];
    };
    const checkoutRoots = fixture.checkoutRoots ?? [{ ...fixture.checkout, scope: "ticket" as const }];
    
    
    expect(fixture["version"]).toEqual(2);
    
    for (const component of fixture.components) expect(cleanIsWindowsIllegalName(component.name), JSON.stringify(component.name)).toBe(component.illegal);
    const gitPaths = (args: string[], prefix?: string): string[] => {
      const result = spawnSync("git", args, { cwd: root, maxBuffer: 64 * 1024 * 1024 });
      expect(result.status).toBe(0);
      return result.stdout.toString("utf8").split("\0").filter((path) => !prefix || path.startsWith(`${prefix}/`));
    };
    const trackedAll = gitPaths(["ls-files", "-z"]);
    const untrackedAll = gitPaths(["ls-files", "-z", "--others", "--exclude-standard"]);
    const untrackedPaths = new Set(untrackedAll);
    const oversizeByRoot: Record<string, string[]> = {};
    for (const checkout of checkoutRoots) {
      const tracked = checkout.scope === "ticket" ? gitPaths(["ls-files", "-z", "--", fixture.checkout.ticketRoot]) : trackedAll;
      const untracked = checkout.scope === "ticket" ? gitPaths(["ls-files", "-z", "--others", "--exclude-standard", "--", fixture.checkout.ticketRoot]) : untrackedAll;
      const paths = [...new Set([...tracked, ...untracked])];
      if (checkout.scope === "ticket") expect(paths.length).toBeGreaterThan(0);
      const scopeUntracked = new Set(untracked);
      oversizeByRoot[checkout.root] = paths.filter((path) =>
        win32.join(checkout.root, path).length > checkout.maxPathUnits
        && (scopeUntracked.has(path) || untrackedPaths.has(path) || existsSync(join(root, path))),
      );
    }
    expect(oversizeByRoot).toEqual(Object.fromEntries(checkoutRoots.map((checkout) => [checkout.root, []])));
  }, { timeout: 15_000 });
});
