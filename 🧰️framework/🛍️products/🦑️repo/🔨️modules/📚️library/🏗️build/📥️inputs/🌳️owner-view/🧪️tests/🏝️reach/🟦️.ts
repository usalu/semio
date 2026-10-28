/** 🧪️ Tests for inputs owner-view reach. */
import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import corpus from "./🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

/** 🏝️ Contract: the premise of the private owner view is no longer "a framework module cannot depend on a product" (131 framework files mention the product subtree) but "every reach of a framework module into a product is one its own task or its crate closure covers". The census reads every tracked file below the shared root and classifies each reach; a reach in none of the covered classes must be listed in the corpus with its reason, and a listed exception that is no longer a reach is refused, so the list can only shrink. */
const repositoryRoot = (from: string): string => { let at = from; while (!existsSync(join(at, ".git"))) { const up = dirname(at); if (up === at) throw Error("The checkout of the repository is required"); at = up; } return at; };
const checkout = repositoryRoot(dirname(fileURLToPath(import.meta.url)));
const tracked = (): string[] => {
  const run = spawnSync("git", ["ls-files", "-z", "--", corpus.sharedRoot], {cwd: checkout, encoding: "utf8", maxBuffer: 1 << 28});
  if (run.status !== 0) throw Error(run.stderr);
  return run.stdout.split("\0").filter((path) => path !== "" && !path.includes("/node_modules/"));
};
const isTest = (path: string) => path.includes(corpus.testDirectory) || /\.test\.[a-z]+$/u.test(path);
const comment = (line: string) => /^\s*(?:\/\/|\/\*|\*|#)/u.test(line);

/** 🔎️ Names the files below the shared root that reach into the private directory in a way no mechanism covers. */
export function uncoveredReaches(files: readonly string[], read: (path: string) => string): string[] {
  const found: string[] = [], followed = new RegExp(corpus.followedRust, "u");
  for (const path of files) {
    if (isTest(path)) continue;
    let text: string;
    try { text = read(path); } catch { continue; }
    if (!text.includes(corpus.privateDirectory)) continue;
    if (path.endsWith(".rs")) {
      if (text.split("\n").some((line) => line.includes(corpus.privateDirectory) && !comment(line) && !followed.test(line))) found.push(path);
    } else if (path.endsWith("Cargo.toml")) {
      if (text.split("\n").some((line) => line.includes(corpus.privateDirectory) && !comment(line) && !/\bpath\s*=/u.test(line))) found.push(path);
    } else if (corpus.codeExtensions.some((extension) => path.endsWith(extension))) {
      const imported = ts.preProcessFile(text, true, true).importedFiles.map((row) => row.fileName);
      if (imported.some((specifier) => specifier.includes(corpus.privateDirectory))) found.push(path);
    }
  }
  return found.sort();
}

test("the corpus is closed by its schema", () => {
  const ajv = new Ajv({strict: false});
  expect(ajv.validate(schema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(schema, {...corpus, extra: 1})).toBe(false);
  expect(ajv.validate(schema, {...corpus, exceptions: [{path: "a"}]})).toBe(false);
});

test("the classifier sees every uncovered kind of reach and none of the covered ones", () => {
  const dir = corpus.privateDirectory, files: Record<string, string> = {
    "a/imports.ts": `import x from "../../${dir}/p/🟦️.ts";`,
    "a/dynamic.mjs": `const m = await import("../${dir}/q.mjs");`,
    "a/requires.js": `const m = require("../${dir}/r.js");`,
    "a/comment-only.ts": `/** @see ../${dir}/doc */\nexport const x = 1;`,
    "a/data.json": `{"path": "${dir}/x"}`,
    "b/followed.rs": `#[path = "../${dir}/m.rs"]\nmod m;\nconst T: &str = include_str!("../${dir}/f.json");`,
    "b/loose.rs": `const ROOT: &str = "../${dir}/x";`,
    "b/doc.rs": `// see ../${dir}/m.rs\nfn f() {}`,
    "b/Cargo.toml": `[dependencies]\nsemio-os = { path = "../${dir}/os" }`,
    "b/Loose/Cargo.toml": `[package]\ndescription = "${dir}"`,
    "c/some.test.ts": `import x from "../${dir}/p.ts";`,
    [`d${corpus.testDirectory}t.ts`]: `import x from "../${dir}/p.ts";`,
  };
  expect(uncoveredReaches(Object.keys(files), (path) => files[path]!)).toEqual(["a/dynamic.mjs", "a/imports.ts", "a/requires.js", "b/Loose/Cargo.toml", "b/loose.rs"]);
});

test("Law: every reach of a framework module into the product subtree is covered by a Cargo path dependency, a followed Rust source, an own-task test, or listed with its reason; the list holds no stale entry", () => {
  const exceptions: readonly {path: string; reason: string}[] = corpus.exceptions, reaches = uncoveredReaches(tracked(), (path) => readFileSync(join(checkout, path), "utf8")), listed = exceptions.map((row) => row.path).sort();
  expect(reaches.filter((path) => !listed.includes(path)), "an unlisted reach: its dependents would not key the product files it reads").toEqual([]);
  expect(listed.filter((path) => !reaches.includes(path)), "a listed exception that is no longer a reach").toEqual([]);
}, 120000);
