import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative } from "node:path";
import { WORKSPACE_INDEX_RACY_MILLISECONDS_V1, deriveWorkspaceIndexedV1, forgetWorkspaceIndexV1, listWorkspaceIndexedV1, type WorkspaceChangeFeedV1, type WorkspaceFeedCursorV1, type WorkspaceIndexQueryV1 } from "../🟦️.ts";

const QUERY: WorkspaceIndexQueryV1 = { skipped: new Set(["skipped"]), suffixes: [".rs"], names: ["📋️project.json"] };
const ORACLE_QUERY: WorkspaceIndexQueryV1 = { ...QUERY, skipped: new Set(["skipped", "never-present"]) };
type RowV1 = readonly [string, 0 | 1, string];

class FakeFeedV1 implements WorkspaceChangeFeedV1 {
  rows: (readonly [number, RowV1])[] = [];
  seq = 0;
  refuse = false;
  calls = 0;
  emit(...rows: RowV1[]): void { this.seq++; for (const row of rows) this.rows.push([this.seq, row]); }
  excluder(): (path: string) => boolean { return () => false; }
  async cursor() { return { epoch: "e", seq: this.seq }; }
  async since(from: WorkspaceFeedCursorV1) {
    this.calls++;
    if (this.refuse) return { exact: false } as const;
    return { exact: true, cursor: { epoch: "e", seq: this.seq }, rows: this.rows.filter(([seq]) => seq > from.seq).map(([, row]) => row) } as const;
  }
}

type ModeV1 = "stamps" | "feed" | "feed-file-rows-only";
const mutations = (root: string, feed: FakeFeedV1 | undefined, mode: ModeV1) => {
  const rel = (path: string) => relative(root, join(root, path)).split("\\").join("/");
  const directoryRows = (path: string): RowV1[] => {
    if (mode === "feed-file-rows-only") return [];
    const rows: RowV1[] = [];
    for (let at = dirname(rel(path)); at !== "." && at !== ""; at = dirname(at)) rows.unshift(["a", 1, at]);
    return rows;
  };
  return {
    add(path: string, text = "") { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); feed?.emit(...directoryRows(path), ["a", 0, rel(path)]); },
    remove(path: string, directory = false) { rmSync(join(root, path), { recursive: true, force: true }); feed?.emit(["r", directory ? 1 : 0, rel(path)]); },
    rename(from: string, to: string, directory = false) { renameSync(join(root, from), join(root, to)); feed?.emit(["n", directory ? 1 : 0, rel(from)], ["n", directory ? 1 : 0, rel(to)]); },
  };
};

const sorted = (listing: Awaited<ReturnType<typeof listWorkspaceIndexedV1>>) => ({ suffixed: [...listing.suffixed].sort(), named: [...listing.named].sort() });
const fresh = async (root: string) => sorted(await listWorkspaceIndexedV1(root, ORACLE_QUERY));
const globbed = async (root: string) => {
  const found = async (pattern: string) => (await Array.fromAsync(new Bun.Glob(pattern).scan({ cwd: root, dot: true, onlyFiles: true }))).filter(path => !path.split("/").includes("skipped")).map(path => join(root, path)).sort();
  return { suffixed: await found("**/*.rs"), named: await found("**/📋️project.json") };
};
const workspace = () => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), "workspace-index-")));
  for (const path of ["a/x.rs", "a/b/y.rs", "a/b/c/📋️project.json", "d/z.rs", "d/e/f/g/deep.rs", "skipped/hidden.rs", "a/skipped/hidden.rs", "plain.txt"]) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), "1"); }
  return root;
};
const done = (root: string) => { rmSync(root, { recursive: true, force: true }); forgetWorkspaceIndexV1(root); };
const wait = (milliseconds: number) => new Promise(resolve => setTimeout(resolve, milliseconds));

type Mutations = ReturnType<typeof mutations>;
const steps: readonly ((m: Mutations) => void)[] = [
  m => m.add("a/b/added.rs"),
  m => m.add("new/tree/deeper/n.rs"),
  m => m.remove("a/b/y.rs"),
  m => m.rename("d/z.rs", "d/renamed.rs"),
  m => m.rename("d/e", "d/moved", true),
  m => m.remove("d/moved/f", true),
  m => m.add("d/moved/f/again.rs"),
  m => m.add("a/b/c/other/📋️project.json", "{}"),
  m => m.remove("a/b/c", true),
  m => { m.remove("a/b", true); m.add("a/b/replacement.rs"); },
  m => m.add("skipped/more/ignored.rs"),
  m => m.remove("new", true),
  m => m.rename("a", "a2", true),
  m => m.add("a2/b/c/d/e/f.rs"),
  m => m.remove("a2/b/replacement.rs"),
];

for (const mode of ["stamps", "feed", "feed-file-rows-only"] as const) {
  test(`the ${mode} listing equals a fresh full walk and a glob after adds, deletes, renames and vanished directories in the same clock tick`, async () => {
    const root = workspace(), feed = mode === "stamps" ? undefined : new FakeFeedV1(), change = mutations(root, feed, mode);
    const agrees = async () => {
      const incremental = sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed }));
      expect(incremental).toEqual(await fresh(root));
      expect(incremental).toEqual(await globbed(root));
    };
    try {
      await agrees();
      for (const step of steps) { step(change); await agrees(); }
    } finally { done(root); forgetWorkspaceIndexV1(root); }
  });
}

test("directories remembered as trusted still see every change after the racy window has passed", async () => {
  const root = workspace(), change = mutations(root, undefined, "stamps");
  const agrees = async () => { const incremental = sorted(await listWorkspaceIndexedV1(root, QUERY)); expect(incremental).toEqual(await fresh(root)); expect(incremental).toEqual(await globbed(root)); };
  try {
    await wait(WORKSPACE_INDEX_RACY_MILLISECONDS_V1 + 300);
    await agrees();
    for (const step of steps) { step(change); await agrees(); }
    await wait(WORKSPACE_INDEX_RACY_MILLISECONDS_V1 + 300);
    await agrees();
    change.add("a2/b/late.rs");
    change.remove("d", true);
    await agrees();
  } finally { done(root); }
}, 30000);

test("with an exact feed a repeated call reads only what the feed names, and falls back to stamping when the feed refuses", async () => {
  const root = workspace(), feed = new FakeFeedV1(), change = mutations(root, feed, "feed");
  try {
    const before = sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed }));
    writeFileSync(join(root, "a/unannounced.rs"), "");
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed }))).toEqual(before);
    feed.emit(["a", 0, "a/unannounced.rs"]);
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed })).suffixed).toContain(join(root, "a/unannounced.rs"));
    feed.refuse = true;
    change.add("a/while-refused.rs");
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed }))).toEqual(await fresh(root));
    feed.refuse = false;
    change.add("a/after-refusal.rs");
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed }))).toEqual(await fresh(root));
  } finally { done(root); }
});

test("a derived answer follows a same-size rewrite in the same clock tick and a vanished file answers undefined", async () => {
  const root = workspace();
  try {
    const files = [join(root, "a/x.rs"), join(root, "d/z.rs")], read = async (path: string) => (await Bun.file(path).text());
    const rewrite = (text: string) => { writeFileSync(files[0]!, text); };
    for (let round = 0; round < 6; round++) {
      rewrite(String(round));
      expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", files, read)).toEqual([String(round), "1"]);
    }
    rmSync(files[1]!);
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", files, read)).toEqual(["5", undefined]);
    await wait(WORKSPACE_INDEX_RACY_MILLISECONDS_V1 + 300);
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", files, read)).toEqual(["5", undefined]);
    let reads = 0;
    const counted = async (path: string) => { reads++; return read(path); };
    await deriveWorkspaceIndexedV1(root, QUERY, "counted", [files[0]!], counted);
    await deriveWorkspaceIndexedV1(root, QUERY, "counted", [files[0]!], counted);
    expect(reads).toBe(1);
    rewrite("changed");
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "counted", [files[0]!], counted)).toEqual(["changed"]);
    expect(reads).toBe(2);
  } finally { done(root); }
}, 30000);

test("with an exact feed a derived answer is dropped exactly when the feed names the file", async () => {
  const root = workspace(), feed = new FakeFeedV1();
  try {
    const file = join(root, "a/x.rs"), read = async (path: string) => (await Bun.file(path).text());
    await listWorkspaceIndexedV1(root, QUERY, { changes: feed });
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", [file], read, { changes: feed })).toEqual(["1"]);
    writeFileSync(file, "2");
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", [file], read, { changes: feed })).toEqual(["1"]);
    feed.emit(["m", 0, "a/x.rs"]);
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", [file], read, { changes: feed })).toEqual(["2"]);
  } finally { done(root); }
});

test("a fresh process resumes from the saved snapshot and still equals a full walk after changes made while it was away", async () => {
  const root = workspace(), store = mkdtempSync(join(tmpdir(), "workspace-index-store-")), feed = new FakeFeedV1(), change = mutations(root, feed, "feed");
  try {
    const read = async (path: string) => (await Bun.file(path).text());
    const first = sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed, store }));
    await deriveWorkspaceIndexedV1(root, QUERY, "text", first.suffixed, read, { changes: feed, store });
    forgetWorkspaceIndexV1(root);
    change.add("a/b/while-away.rs", "away");
    change.remove("d", true);
    change.rename("a/x.rs", "a/x2.rs");
    const resumed = sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed, store }));
    expect(resumed).toEqual(await fresh(root));
    expect(resumed).toEqual(await globbed(root));
    expect(await deriveWorkspaceIndexedV1(root, QUERY, "text", resumed.suffixed, read, { changes: feed, store })).toEqual(await Promise.all(resumed.suffixed.map(read)));
    forgetWorkspaceIndexV1(root);
    feed.refuse = true;
    change.add("a/refused.rs");
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY, { changes: feed, store }))).toEqual(await fresh(root));
  } finally { done(root); rmSync(store, { recursive: true, force: true }); }
});

test("a cold walk reports progress once, stops when cancelled and leaves the index correct for the next call", async () => {
  const root = workspace();
  try {
    let reports = 0;
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY, { report: () => { reports++; } }))).toEqual(await globbed(root));
    expect(reports).toBe(1);
    forgetWorkspaceIndexV1(root);
    const controller = new AbortController();
    controller.abort();
    await expect(listWorkspaceIndexedV1(root, QUERY, { signal: controller.signal })).rejects.toThrow();
    writeFileSync(join(root, "a/after-cancel.rs"), "");
    expect(sorted(await listWorkspaceIndexedV1(root, QUERY))).toEqual(await globbed(root));
  } finally { done(root); }
});

test("a missing root lists nothing", async () => {
  expect(await listWorkspaceIndexedV1(join(tmpdir(), "workspace-index-absent-root"), QUERY)).toEqual({ suffixed: [], named: [] });
});
