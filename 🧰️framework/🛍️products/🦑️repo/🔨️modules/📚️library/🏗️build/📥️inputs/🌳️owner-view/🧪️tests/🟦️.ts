/** 🧪️ Tests for inputs owner-view. */
import {expect, test} from "bun:test";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import {ownerOfProjectV1, ownerTreeGlobsV1, ownerViewV1} from "../🟨️.mjs";
import corpus from "./🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

const entriesOf = (owner: string) => {
  const names = new Map<string, boolean>();
  for (const path of corpus.tree) if (path.startsWith(`${owner}/`)) { const rest = path.slice(owner.length + 1), [name] = rest.split("/"); names.set(name!, rest.includes("/")); }
  return [...names].map(([name, directory]) => ({name, directory}));
};
const strip = (glob: string) => glob.replace("{workspaceRoot}/", "");
const selected = (globs: readonly string[], path: string) => globs.some((glob) => minimatch(path, strip(glob), {dot: false}));

/** 🌳️ The private owner view loses nothing: shared plus private globs select exactly the files of the plain owner glob outside hidden entries, and only the private directories leave the shared part. */
test("the corpus satisfies its strict schema", () => {
  const ajv = new Ajv({strict: true, allErrors: true});
  expect(ajv.validate(schema, corpus), JSON.stringify(ajv.errors)).toBe(true);
});

test("the owner of a root is the path before its first delivery scope directory", () => {
  for (const row of corpus.owners) expect(ownerOfProjectV1(row.root, corpus.deliveryScopeNames), row.root).toBe(row.owner);
});

test("only an owner with a policy row has a private view", () => {
  for (const row of corpus.owners) expect(ownerViewV1(row.owner, corpus.views) !== undefined, row.root).toBe(row.view);
});

test("shared and private globs together select what the plain owner glob selects", () => {
  const view = ownerViewV1("fw", corpus.views)!, globs = ownerTreeGlobsV1("fw", corpus.extensions, view.private, entriesOf("fw"));
  const plain = [`{workspaceRoot}/fw/**/*.${corpus.extensions}`];
  for (const path of corpus.tree) {
    const hiddenEntry = path.split("/").slice(1).some((part) => part.startsWith("."));
    if (hiddenEntry) { expect(selected([...globs.shared, ...globs.private], path), `hidden ${path}`).toBe(false); continue; }
    expect(selected([...globs.shared, ...globs.private], path), path).toBe(minimatch(path, strip(plain[0]!)));
    expect(selected(globs.private, path), `private ${path}`).toBe(path.startsWith("fw/🛍️products/") && minimatch(path, strip(plain[0]!)));
    expect(selected(globs.shared, path) && path.startsWith("fw/🛍️products/"), `shared ${path}`).toBe(false);
  }
});
