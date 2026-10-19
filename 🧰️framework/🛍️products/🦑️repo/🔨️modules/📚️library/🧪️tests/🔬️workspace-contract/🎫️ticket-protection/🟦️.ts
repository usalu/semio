import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, resolve, sep, win32 } from "node:path";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkoutPathInventory, contractRoot, normalizationGit, type CheckoutPathInventory } from "../🏗️shared-fixtures/🟦️.ts";

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

type CheckoutRoot = { root: string; maxPathUnits: number; scope?: "ticket" | "all" };

type CheckoutFixture = {
  version: number;
  checkout: { root: string; maxPathUnits: number; ticketRoot: string };
  checkoutRoots?: (CheckoutRoot & { scope: "ticket" | "all" })[];
  components: { name: string; illegal: boolean }[];
  inventory: {
    gitignore: string[];
    tracked: string[];
    missing: string[];
    untracked: string[];
    ignored: string[];
    nestedRepos: string[];
    checkout: { root: string; maxPathUnits: number; ticketRoot: string };
    oversize: { all: string[]; ticket: string[] };
  };
};

/** 🪟️ `win32.join` never lengthens `root + "\\" + path`, so the plain length bound screens candidates exactly before the normalized join decides. */
const oversizeCheckoutPaths = (inventory: CheckoutPathInventory, checkout: CheckoutRoot, ticketRoot: string, exists: (path: string) => boolean): string[] =>
  [...new Set([...inventory.tracked, ...inventory.untracked].filter((path) => checkout.root.length + 1 + path.length > checkout.maxPathUnits))].filter((path) =>
    (checkout.scope === "all" || path.startsWith(`${ticketRoot}/`))
    && win32.join(checkout.root, path).length > checkout.maxPathUnits
    && (inventory.untracked.has(path) || exists(path)),
  ).sort();

const oldCheckoutInventory = (root: string, scope?: string): CheckoutPathInventory => {
  const paths = (args: string[]): string[] => {
    const result = spawnSync("git", args, { cwd: root, maxBuffer: 64 * 1024 * 1024 });
    expect(result.status).toBe(0);
    return result.stdout.toString("utf8").split("\0").filter(Boolean);
  };
  const within = scope === undefined ? [] : ["--", scope];
  return { tracked: paths(["ls-files", "-z", ...within]), untracked: new Set(paths(["ls-files", "-z", "--others", "--exclude-standard", ...within])) };
};

describe("Windows checkout ticket paths", () => {
  test("rejects Windows-illegal components and keeps checkout paths below the legacy path limit", async () => {
    const { cleanIsWindowsIllegalName } = await import("../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts");
    const root = findWorkspaceRoot(contractRoot);
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🪟️windows-checkout-paths/🔣️.json"), "utf8")) as CheckoutFixture;
    const checkoutRoots = fixture.checkoutRoots ?? [{ ...fixture.checkout, scope: "ticket" as const }];

    expect(fixture["version"]).toEqual(3);

    for (const component of fixture.components) expect(cleanIsWindowsIllegalName(component.name), JSON.stringify(component.name)).toBe(component.illegal);
    const { ticketRoot } = fixture.checkout;
    const inventory = await checkoutPathInventory(root, checkoutRoots.some((checkout) => checkout.scope === "all") ? ["."] : [ticketRoot]);
    expect(inventory.tracked.filter((path) => path.startsWith(`${ticketRoot}/`)).length + [...inventory.untracked].filter((path) => path.startsWith(`${ticketRoot}/`)).length).toBeGreaterThan(0);
    const oversizeByRoot = Object.fromEntries(checkoutRoots.map((checkout) => [checkout.root, oversizeCheckoutPaths(inventory, checkout, ticketRoot, (path) => existsSync(join(root, path)))]));
    expect(oversizeByRoot).toEqual(Object.fromEntries(checkoutRoots.map((checkout) => [checkout.root, []])));
  }, { timeout: 15_000 });

  test("partitioned parallel inventory equals the single-process Git enumeration on a fixture repository", async () => {
    const { inventory } = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🪟️windows-checkout-paths/🔣️.json"), "utf8")) as CheckoutFixture;
    const repository = mkdtempSync(join(tmpdir(), "semio-checkout-inventory-"));
    try {
      normalizationGit(repository, ["init", "--quiet"]);
      const write = (path: string): void => { mkdirSync(dirname(join(repository, path)), { recursive: true }); writeFileSync(join(repository, path), path); };
      writeFileSync(join(repository, ".gitignore"), `${inventory.gitignore.join("\n")}\n`);
      for (const path of [...inventory.tracked, ...inventory.untracked, ...inventory.ignored]) write(path);
      for (const nested of inventory.nestedRepos) { write(`${nested}/file.txt`); normalizationGit(join(repository, nested), ["init", "--quiet"]); }
      normalizationGit(repository, ["add", "--", ".gitignore", ...inventory.tracked]);
      for (const path of inventory.missing) rmSync(join(repository, path));
      const exists = (path: string): boolean => existsSync(join(repository, path));
      const { ticketRoot } = inventory.checkout;
      const oracle = { all: oldCheckoutInventory(repository), ticket: oldCheckoutInventory(repository, ticketRoot) };
      expect([...oracle.all.untracked].sort()).toEqual([...inventory.untracked, ...inventory.nestedRepos.map((nested) => `${nested}/`)].sort());
      for (const [scope, scopes] of [["all", ["."]], ["ticket", [ticketRoot]]] as const) {
        for (const options of [{ parallelism: 1, budget: 1_000 }, { parallelism: 1, budget: 1 }, { parallelism: 3, budget: 2 }, { parallelism: 8, budget: 1 }, {}]) {
          const partitioned = await checkoutPathInventory(repository, scopes, options);
          expect([...partitioned.untracked].sort(), `${scope} ${JSON.stringify(options)}`).toEqual([...oracle[scope].untracked].sort());
          expect([...partitioned.tracked].sort()).toEqual([...oracle.all.tracked].sort());
          const checkout: CheckoutRoot = { ...inventory.checkout, scope: scope === "all" ? "all" : "ticket" };
          const expected = [...inventory.oversize[scope]].sort();
          expect(oversizeCheckoutPaths(partitioned, checkout, ticketRoot, exists), `${scope} ${JSON.stringify(options)}`).toEqual(expected);
          expect(oversizeCheckoutPaths(oracle[scope], checkout, ticketRoot, exists)).toEqual(expected);
        }
      }
    } finally {
      rmSync(repository, { recursive: true, force: true });
    }
  }, { timeout: 60_000 });
});
