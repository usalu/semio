/** 📥️ Fixture inputs
 *
 * Test input URIs, their resolution and the fixture files below a directory.
 */

//#region 🔌️Adapters
import { type ParsedFeature } from "@semio-tech/framework/test/gherkin";
import { type ResolvedTestInput, TEST_INPUT_URI_RE } from "@semio-tech/framework/test/adapter";
import { existsSync, lstatSync, readdirSync, realpathSync } from "node:fs";
import { join, posix, relative, resolve, sep } from "node:path";
import { testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { fileDigest } from "../🔢️digests/🟦️.ts";
import { type DiscoveredCase } from "../🧭️case-discovery/🟦️.ts";
import { resolveSchemaExport, type SchemaDiagnostic } from "../📐️schema-contracts/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧫️Fixtures
/** 🧫️ One resolved fixture — explicit scheme, never shadow-based, digest pinned at plan time. */


/** 🧫️ Extracts every `shared://` / `local://` / `asset://` / `schema://` reference appearing anywhere in a feature's text. */
export function testInputUrisIn(feature: ParsedFeature): string[] {
  const haystack = [feature.description, ...feature.background.flatMap((step) => [step.text, step.docString ?? "", ...(step.dataTable ?? []).flat()]), ...feature.scenarios.flatMap((scenario) => scenario.steps.flatMap((step) => [step.text, step.docString ?? "", ...(step.dataTable ?? []).flat()]))].join("\n");
  const uris = new Set<string>();
  for (const match of haystack.matchAll(TEST_INPUT_URI_RE)) uris.add(`${match[1]}://${match[2]}`);
  return [...uris].sort();
}

/** 🧭️ Resolves testing data under owner fixtures, static data under owner assets, and contracts by schema export. */
export function resolveTestInputs(repoRoot: string, discovered: DiscoveredCase, uris: readonly string[]): { inputs: ResolvedTestInput[]; missing: string[]; diagnostics: SchemaDiagnostic[] } {
  const inputs: ResolvedTestInput[] = [];
  const missing: string[] = [];
  const diagnostics: SchemaDiagnostic[] = [];
  const taxonomy = testTaxonomy(repoRoot);
  for (const uri of uris) {
    const [scheme, name] = uri.split("://") as [string, string];
    // 🧬️A `schema://` reference is a CONTRACT reference, not a file in this case's fixture directory:
    // it resolves through the derived catalog, so a fixture names the export it is an example of and
    // can never acquire a contract by having one placed next to it.
    if (scheme === "schema") {
      const { resolved, diagnostics: found } = resolveSchemaExport(repoRoot, uri);
      diagnostics.push(...found);
      if (resolved === null) missing.push(uri);
      else inputs.push({ uri, scope: "schema", name, path: resolved.path, digest: resolved.digest });
      continue;
    }
    if ((scheme !== "shared" && scheme !== "asset") || !name || name.includes("\\") || name.split("/").some(part => !part || part === "." || part === "..")) {
      missing.push(uri);
      continue;
    }
    const baseRel = posix.join(discovered.owner, ...(scheme === "shared" ? [taxonomy.testsDirName, taxonomy.testFixturesDirName] : [taxonomy.exampleAssetsDirName]));
    const abs = join(repoRoot, baseRel, name), guard = resolve(repoRoot, baseRel);
    if (!resolve(abs).startsWith(guard + sep) || !existsSync(abs) || !lstatSync(abs).isFile() || realpathSync(abs) !== resolve(abs) || realpathSync(guard) !== guard) {
      missing.push(uri);
      continue;
    }
    inputs.push({ uri, scope: scheme, name, path: `${baseRel}/${name}`, digest: fileDigest(abs) });
  }
  return { inputs, missing, diagnostics };
}

/** 🧫️ Every file under a fixture directory, repo-relative, for orphan detection and immutability proofs. */
export function fixtureFilesUnder(repoRoot: string, relDir: string | null): string[] {
  if (relDir === null) return [];
  const abs = join(repoRoot, relDir);
  if (!existsSync(abs)) return [];
  const out: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else out.push(relative(repoRoot, full).split(sep).join("/"));
    }
  };
  walk(abs);
  return out.sort();
}
//#endregion 🧫️Fixtures
