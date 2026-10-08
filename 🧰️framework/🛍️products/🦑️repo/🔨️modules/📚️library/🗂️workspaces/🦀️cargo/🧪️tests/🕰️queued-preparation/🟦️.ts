import {cargoPreparationRuntimeV1,observedCargoPreparationV1} from "../🧰️runtime/🟦️.ts";
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { setTimeout as delay } from "node:timers/promises";
import { acquireQueuedResourceLease } from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🕰️queued-preparation/🔣️.json", import.meta.url), "utf8"));



test("queued native preparation survives waiting beyond one active recipe budget", async () => {
  expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["expectedMachineOutput"]["prepared"]).toEqual(true);
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for queued native proofs");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "cargo-queued-preparation-"));
  const put = (path: string, source: string): void => {
    mkdirSync(join(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), source);
  };
  put("Cargo.toml", '[workspace]\nresolver="2"\nmembers=["kernel"]\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=["kernel/Cargo.toml"]\n[workspace.package]\nedition="2021"\n');
  put("kernel/Cargo.toml", '[package]\nname="queued-kernel"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="🦀️.rs"\n[package.metadata.semio.preparation]\nscript="../📜️script.ts"\ncommand=["publish"]\n');
  put("kernel/🦀️.rs", "pub fn kernel() {}\n");
  put("📜️script.ts", observedCargoPreparationV1(root,`import {writeFileSync} from "node:fs";import {join} from "node:path";writeFileSync(join(process.env.NX_WORKSPACE_ROOT!,"published.txt"),${JSON.stringify(fixture.expectedPublication)});\n`));
  const library = cargoPreparationRuntimeV1(root);
  put("proof/📜️script.ts", `import {prepareCargoWorkspaceInvocation} from ${JSON.stringify(library)};prepareCargoWorkspaceInvocation(process.env.NX_WORKSPACE_ROOT!,["test","--manifest-path","kernel/Cargo.toml"],process.env.NX_WORKSPACE_ROOT!);console.log(JSON.stringify({prepared:true}));\n`);
  const holder = await acquireQueuedResourceLease({ directory: join(root, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), resource: `cargo-preparation:${root}`, mode: "exclusive", owner: "independent-owner", signal: new AbortController().signal });
  const child = Bun.spawn([process.execPath, join(root, "proof/📜️script.ts")], { cwd: root, env: { ...process.env, NX_WORKSPACE_ROOT: root }, stdout: "pipe", stderr: "pipe" });
  const output = new Response(child.stdout).text(), diagnostic = new Response(child.stderr).text();
  try {
    await delay(fixture.waitMillis);
    holder.release();
    expect(await child.exited, await diagnostic).toBe(0);
    expect(JSON.parse(await output)).toEqual(fixture.expectedMachineOutput);
    expect(readFileSync(join(root, "published.txt"), "utf8")).toBe(fixture.expectedPublication);
    expect(await diagnostic).toContain(fixture.expectedWait);
  } finally {
    holder.release();
    if (child.exitCode === null) child.kill();
    await child.exited;
  }
}, 45_000);
