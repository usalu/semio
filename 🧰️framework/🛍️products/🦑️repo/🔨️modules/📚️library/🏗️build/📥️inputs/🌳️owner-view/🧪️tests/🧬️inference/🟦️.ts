import {isOpaqueSourcePathV1,opaqueSourceInputExclusionsV1} from "../../../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts";
/** 🧬️ Tests for inputs owner-view inference. */
import {expect, test} from "bun:test";
import {readFileSync} from "node:fs";
import {join, relative} from "node:path";
import {runInNewContext} from "node:vm";
import Ajv from "ajv";
import ts from "typescript";
import {ownerOfProjectV1, ownerTreeGlobsV1, ownerViewV1} from "../../🟨️.mjs";
import {toolchainsActivatedByV1} from "../../../🪛️toolchain-activation/🟨️.mjs";
import corpus from "./🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

const source = readFileSync(new URL("../../../🟨️.mjs", import.meta.url), "utf8"), tree = ts.createSourceFile("inputs.mjs", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
const declaration = tree.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "inferProjectInputs");

/** 🌳️ The real input inference splits the owner glob of a project with a private view and activates a toolchain through a provisioning dependency: the corpus is strict and every case is run through the declaration text of `inferProjectInputs`. */
test("the corpus satisfies its strict schema", () => {
  const ajv = new Ajv({strict: false, allErrors: true});
  expect(ajv.validate(schema, corpus), JSON.stringify(ajv.errors)).toBe(true);
});

test("each case yields the expected shared globs, private globs and toolchain files", async () => {
  if (!declaration) throw Error("Actual input inference declaration required");
  const workspaceRoot = "/__owner_view_fixture__";
  for (const row of corpus.cases) {
    const context = {sourceRepresentation: "source", POLICY: {generatedDirectories: ["node_modules", ".cache"], toolchains: corpus.toolchains, ownerViews: corpus.views, runtimeStateDirectories: corpus.runtimeState}, TAXONOMY: {testDeliveryScopeDirectoryNames: corpus.scopeNames}, LIBRARY_ROOT: join(workspaceRoot, "runner"), SCRIPT_BASENAME: "script.ts", cargoSourceInputs: () => undefined, goManifest: () => false, buildPath: (value: string) => value.replaceAll("\\", "/"), relativeScriptInputs: async () => [], closureOf: async () => [], resolveOutputPath: () => undefined, exampleCollectionInputs: () => [], workspaceNegationClosed: (values: unknown[]) => values, declaredSourceInputs: async () => ({}), owned: () => false};
    const entries = row.entries.map(([name, directory]) => ({name, isDirectory: () => directory, isFile: () => !directory}));
    const result = await runInNewContext("(" + declaration.getText(tree).replace(/^export\s+/u, "") + ")(json,root,workspaceRoot,{}, {},context)", {isOpaqueSourcePathV1,opaqueSourceInputExclusionsV1,json: {name: "p", targets: row.targets}, root: row.root, workspaceRoot, context, join, relative, existsSync: () => false, readdirSync: (_path: string, options?: {withFileTypes?: boolean}) => (options?.withFileTypes ? entries : []), cargoManifestInputV1: (root: string) => `${root}/Cargo.toml`, warmCargoResolutionV1: async () => undefined, cargoLockInput: () => ({env: "lock"}), deduplicateProjectInputs: (values: unknown[]) => values, compactSourceInputs: (values: unknown[]) => values, internProjectInputGroups: (groups: Record<string, unknown[]>) => groups, ownerOfProjectV1, ownerViewV1, ownerTreeGlobsV1, toolchainsActivatedByV1, SCRIPT_BASENAME: "script.ts"}) as Record<string, unknown[]>;
    const shared = (result.default ?? []).filter((item): item is string => typeof item === "string" && !item.startsWith("!"));
    for (const has of row.sharedHas) expect(shared, `${row.id} has ${has}`).toContain(has);
    for (const lacks of row.sharedLacks) expect(shared.some((item) => item.includes(lacks)), `${row.id} lacks ${lacks}`).toBe(false);
    expect<unknown[] | null>(result.ownerPrivate ?? null, row.id).toEqual(row.ownerPrivate);
    for (const path of corpus.runtimeState) expect((result.default ?? []).includes(`!{workspaceRoot}/${path}/**/*`), `${row.id} negates ${path}`).toBe(true);
    expect(shared.includes("{workspaceRoot}/tool/🔧️toolchain/🔣️.json"), `${row.id} tex`).toBe(row.tex);
  }
});
