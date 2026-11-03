/** 🧪 Proves Go dependency synchronization receives the caller environment, not the ambient process environment. */
import {Database} from "bun:sqlite";
import {expect, test} from "bun:test";
import {mkdtempSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createIssuedResourceControlV1, createRequestedIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {nativeCallerPhaseV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {prepareDependencies} from "../../../🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts";

declareLibraryBuildTestEnvelopeV1({suite: "build-reporting", maximumElapsedMilliseconds: 30000});

test("go dependency synchronization keeps the bounded environment", () => withBuildTestInvocationV1(async original => {
  const root = mkdtempSync(join(tmpdir(), "semio-go-bounded-")), parent = createIssuedResourceControlV1(original, "go-bounded:parent", 0), phase = nativeCallerPhaseV1(original, process.cwd(), createRequestedIssuedResourceControlV1(parent, 20000));
  const bounded = {...process.env, SEMIO_BOUNDED_GO: root};
  let seen = "";
  try {
    await prepareDependencies("go", process.cwd(), original, {version: 1, directory: join(root, "preparation"), leaseDirectory: join(root, "leases"), program: "go"}, phase, async (command, args, _cwd, _signal, _capture, env) => {
      expect([command, ...args]).toEqual(["go", "mod", "download"]);
      seen = env.SEMIO_BOUNDED_GO ?? "";
      expect(env.GOWORK).toBe(join(process.cwd(), "go.work"));
      return "";
    }, bounded, async base => base);
    const oracle = new Database(":memory:");
    try {
      const row = oracle.query("select ?1 = ?2 kept").get(seen, root) as {kept: number};
      expect(row.kept).toBe(1);
    } finally { oracle.close(); }
  } finally { rmSync(root, {recursive: true, force: true}); }
}));
