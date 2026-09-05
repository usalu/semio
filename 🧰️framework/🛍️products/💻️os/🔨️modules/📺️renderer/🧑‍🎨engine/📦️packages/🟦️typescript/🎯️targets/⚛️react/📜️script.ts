#!/usr/bin/env bun
/** @emoji 🎨️ `@semio-tech/framework-renderer-react` task router. */
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runBunx, runVitest } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

/** 📇️ Proves the hidden retained Home ACK bridge and its language-neutral receipt boundary. */
export function directoryHomeBootstrapOracle(repoRoot: string): number {
  const contractRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap");
  const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8")) as {
    receipt: Readonly<Record<string, unknown>>;
    hostile: readonly { readonly id: string; readonly patch: Readonly<Record<string, unknown>> }[];
    labels: Readonly<Record<"en" | "de", readonly string[]>>;
  };
  const schema = JSON.parse(readFileSync(join(contractRoot, "🧬️.schema.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const receiptKeys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"].sort();
  const receipt = (value: unknown): boolean => {
    if (!value || typeof value !== "object" || Array.isArray(value) || JSON.stringify(Object.keys(value).sort()) !== JSON.stringify(receiptKeys)) return false;
    const row = value as Record<string, unknown>;
    return row.schema === "semio.space.home.directory-projection-receipt.v1"
      && typeof row.sessionBindingSha256 === "string" && /^[0-9a-f]{64}$/u.test(row.sessionBindingSha256)
      && Number.isSafeInteger(row.authorizationGeneration) && Number(row.authorizationGeneration) > 0
      && Number.isSafeInteger(row.throughSeqInclusive) && Number(row.throughSeqInclusive) >= 0
      && typeof row.receiptSha256 === "string" && /^[0-9a-f]{64}$/u.test(row.receiptSha256);
  };
  assert(receipt(fixture.receipt));
  for (const row of fixture.hostile) assert.equal(receipt({ ...structuredClone(fixture.receipt), ...row.patch }), false, row.id);
  const owner = readFileSync(join(contractRoot, "🟦️.tsx"), "utf8");
  const shell = readFileSync(join(contractRoot, "../../🟦️.tsx"), "utf8");
  const runtime = readFileSync(join(contractRoot, "../../../🔌️PluginRuntime/🟦️.tsx"), "utf8");
  assert(owner.includes("await owner.plugin.handleAction") && owner.includes("parseDirectoryProjectionReceiptV1(response.output)"));
  assert(owner.indexOf("await owner.plugin.handleAction") < owner.indexOf('kind: "directory-bootstrap-ack"'));
  assert(owner.includes('kind: "directory-bootstrap-reject"') && owner.includes('kind: "directory-bootstrap-close"'));
  assert(shell.includes('message.kind === "directory-event-page"') && shell.includes("directoryHomeOwnerRef"));
  assert(shell.includes("openDirectoryHomeOwnerV1") && !shell.includes('kind: "directory-open", baseUrl: resolved.hubBaseUrl'));
  assert(runtime.includes("directory projection receipt was exposed before typed-operation terminal publication"));
  assert(runtime.includes("terminalOutputs.length === 1") && runtime.includes("output = terminalOutputs[0]!.val"));
  assert.deepEqual(Object.keys(fixture.labels).sort(), ["de", "en"]);
  assert(fixture.labels.en.every((label: string) => label.length > 0) && fixture.labels.de.every((label: string) => label.length > 0));
  return 21;
}

class DirectoryHomeBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("directory-home-bootstrap-check accepts no arguments");
    console.log(`directory-home-bootstrap-oracle: checks=${directoryHomeBootstrapOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["📇️directory-home-bootstrap.test.tsx"], "🧪️tests/🟦️.ts");
    runVitest(this.root, ["../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx", "--testNamePattern=validates fixed result page authority and preserves document and download effects"], "🧪️tests/🟦️.ts");
  }
}

//#region 🔖️LintScript
const REGION_BALANCE_FILES = ["🟦️.tsx"] as const;

/** 🧭️Counts unmatched `//#region` / `//#endregion` markers per file — a typo'd region silently corrupts the file's canonical structure. */
function collectRegionBalanceViolations(root: string): string[] {
  const violations: string[] = [];
  for (const name of REGION_BALANCE_FILES) {
    const text = readFileSync(join(root, name), "utf8");
    const opens = (text.match(/#region\b/g) ?? []).length;
    const closes = (text.match(/#endregion\b/g) ?? []).length;
    if (opens !== closes) {
      violations.push(`${name}: ${opens} #region marker(s) vs ${closes} #endregion marker(s)`);
    }
  }
  return violations;
}

/** 🧭️Every host registered in `COMPONENT_SCENE_HOSTS` must have exactly one `export function XxxHost(...: ComponentSceneHostProps)` in `🟦️.tsx` — the contract the host registry table dispatches against. */
function collectHostSignatureViolations(root: string): string[] {
  const violations: string[] = [];
  const text = readFileSync(join(root, "🟦️.tsx"), "utf8");
  const registryNames = [...text.matchAll(/lazyHost\(\(\) => Promise\.resolve\(\{ ([A-Z][A-Za-z0-9]*Host) \}\), "\1"\)/g)].map((m) => m[1]!);
  const hostExportCounts = new Map<string, number>();
  for (const m of text.matchAll(/^export function ([A-Z][A-Za-z0-9]*Host)\([^)]*: ComponentSceneHostProps\)/gm)) {
    hostExportCounts.set(m[1]!, (hostExportCounts.get(m[1]!) ?? 0) + 1);
  }
  for (const name of registryNames) {
    const count = hostExportCounts.get(name) ?? 0;
    if (count === 0) {
      violations.push(`🟦️.tsx: no exported component matching ${name}(...: ComponentSceneHostProps)`);
    } else if (count > 1) {
      violations.push(`🟦️.tsx: multiple ${name} exports matching the host contract`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = [...collectRegionBalanceViolations(this.root), ...collectHostSignatureViolations(this.root)];
    if (violations.length === 0) {
      console.log("framework-renderer-react: region/host-contract lint passed");
      return;
    }
    console.error(`framework-renderer-react: found ${violations.length} lint violation(s):`);
    for (const v of violations) console.error(`  ${v}`);
    process.exit(1);
  }
}
//#endregion 🔖️LintScript

const router = new ScriptRouter(fileURLToPath(new URL(".", import.meta.url))).register("test", TestScript).register("lint", LintScript).register("typecheck", TypecheckScript).register("directory-home-bootstrap-check", DirectoryHomeBootstrapCheckScript);

await runBundleScriptMain(router, import.meta.url);
