import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { getWorkspaceRoot, repositoryCargoTestPolicyV1 } from "../../../../../🟦️.ts";

const root = getWorkspaceRoot();
const owner = resolve(import.meta.dir, "../..");
const corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📋️owner-command-policy/🔣️.json"), "utf8")) as { version: number; cases: { id: string; manifest?: string; cwd?: string; manifestText?: string; classification: string; policy: string }[] };
const validateCorpus = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(owner, "🧬️schema/📋️owner-command-policy/🔣️.json"), "utf8")));
const neutralCargo = join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo");
const validatePolicy = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(neutralCargo, "🧬️schema/🔣️.json"), "utf8")));
const foreignPolicy = JSON.parse(readFileSync(join(neutralCargo, "🧫️fixtures/🔣️.json"), "utf8")).policies[0];
const child = 'await Bun.write(process.env.SEMIO_OWNER_POLICY_RECEIPT, JSON.stringify({policy: process.env.SEMIO_CARGO_TEST_POLICY ? JSON.parse(process.env.SEMIO_CARGO_TEST_POLICY) : null, context: JSON.parse(process.env.SEMIO_PROCESS_OWNER_CONTEXT)})); console.log("[native-owner-command-policy] child=" + (process.env.SEMIO_CARGO_TEST_POLICY ? "owned" : "absent"));';

test("native owner command portable corpus admits independent Ajv", () => {
  expect(validateCorpus(corpus), JSON.stringify(validateCorpus.errors)).toBe(true);
  expect(new Set(corpus.cases.map(row => row.id)).size).toBe(corpus.cases.length);
  expect(validatePolicy(foreignPolicy), JSON.stringify(validatePolicy.errors)).toBe(true);
});

for (const row of corpus.cases) test(row.id, () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(artifacts, { recursive: true });
  const output = mkdtempSync(join(artifacts, "native-owner-command-"));
  const receiptPath = join(output, "receipt.json");
  let manifest = row.manifest, cwd = row.cwd;
  if (row.manifestText) {
    const path = join(output, "Cargo.toml");
    writeFileSync(path, row.manifestText);
    manifest = relative(root, path).replaceAll("\\", "/");
    cwd = relative(root, output).replaceAll("\\", "/");
  }
  const document = toml.parse(readFileSync(resolve(root, manifest!), "utf8")) as { package?: { name?: string }; workspace?: object };
  const classification = document.package?.name ? "package" : document.workspace ? "workspace" : "invalid";
  expect(classification).toBe(row.classification);
  const env = { ...process.env, SEMIO_CARGO_TEST_POLICY: JSON.stringify(foreignPolicy), SEMIO_TEST_LEVEL: "fundamental", SEMIO_OWNER_POLICY_RECEIPT: receiptPath, SEMIO_TEST_ARTIFACT_DIR: output };
  const command = [process.execPath, join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", manifest!, "--cwd", cwd!, "--", process.execPath, "--eval", child];
  const result = Bun.spawnSync(command, { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  writeFileSync(join(output, "stdout.log"), result.stdout);
  writeFileSync(join(output, "stderr.log"), result.stderr);
  if (row.policy === "refused") {
    expect(result.exitCode).not.toBe(0);
    expect(new TextDecoder().decode(result.stderr)).toContain("Native owner requires a package or workspace manifest");
    expect(existsSync(receiptPath)).toBe(false);
  } else {
    expect(result.exitCode, new TextDecoder().decode(result.stderr)).toBe(0);
    const receipt = JSON.parse(readFileSync(receiptPath, "utf8"));
    expect(receipt.context.cwd).toBe(resolve(root, cwd!));
    if (row.policy === "owned") {
      expect(validatePolicy(receipt.policy), JSON.stringify(validatePolicy.errors)).toBe(true);
      expect(receipt.policy).toEqual(repositoryCargoTestPolicyV1(manifest!, resolve(root, cwd!), env));
      expect(receipt.policy.manifestPath).toBe(resolve(root, manifest!));
      expect(receipt.policy).not.toEqual(foreignPolicy);
    } else expect(receipt.policy).toBeNull();
  }
  console.log(`[native-owner-command-policy] ${row.id}: ${row.policy}; oracle=${classification}`);
}, 15000);
