/** 🔑️ Contract: the pass of the module closure graph and the traversal over its stored nodes derive one key per reached node, so a question with a resource that a module's URL import points at is answered from the stored nodes with the same reached names the pass and the command walk give (the module is stored only under its resource-varied key). */
import { expect, setDefaultTimeout, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { closureGraphAnswerV1, closureGraphPassV1 } from "../../../🟨️.mjs";

setDefaultTimeout(180000);
const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));

test("a question with a resource a module imports by URL is answered from the stored nodes with the reached names of the pass and of the walk", async () => {
  const inference = await import("../../../../🟨️.mjs") as any;
  await inference.libraryBootstrap;
  const scripts = await import("../../../🟨️.mjs") as any, root = realpathSync(mkdtempSync(join(tmpdir(), "closure-resource-key-")));
  try {
    for (const [name, text] of Object.entries<string>(fixture.files)) { mkdirSync(dirname(join(root, name)), { recursive: true }); writeFileSync(join(root, name), text); }
    const at = (name: string) => join(root, name), question = { roots: fixture.question.roots.map(at), requiredSources: fixture.question.requiredSources, resources: fixture.question.resources.map(at), selectors: [], stops: [] };
    const fresh = () => { const cache = inference.createScriptInputCache({ workspaceRoot: root }); cache.pass = {}; cache.generatedOutput = () => undefined; return cache; };
    const pass = await closureGraphPassV1(fresh(), root, [question]);
    const key = (name: string, resource = "") => JSON.stringify(["{workspaceRoot}/" + name, 1, 1, resource, null]);
    expect(pass.graph.has(key("loader.ts", "{workspaceRoot}/engine.wasm"))).toBe(true);
    const fact = await closureGraphAnswerV1(fresh(), root, question);
    expect(fact.digest).toBe(pass.digests[0]!);
    expect([...new Set(pass.graph.reach([key("entry.ts")], { redirect: new Map([[key("loader.ts"), key("loader.ts", "{workspaceRoot}/engine.wasm")]]) }).names as string[])].sort()).toEqual(fixture.reached);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
