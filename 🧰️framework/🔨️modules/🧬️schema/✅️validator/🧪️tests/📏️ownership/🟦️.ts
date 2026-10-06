import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import law from "../../📏️ownership/🧫️fixtures/🔣️.json";

/** 🧭️ Proves the validator and document transport have separate physical owners. */
export function proveSchemaValidatorOwnershipV1(repoRoot: string): void {
  const read = (path: string) => readFileSync(resolve(repoRoot, path), "utf8");
  assert(existsSync(resolve(repoRoot, law.validatorPackage)), "neutral validator package must exist");
  assert(!existsSync(resolve(repoRoot, law.retiredHttpModule)), "general Schema must relinquish document HTTP");
  assert(existsSync(resolve(repoRoot, law.httpModule)), "Directory client must own its compiled HTTP provider");
  assert(existsSync(resolve(repoRoot, law.httpFixture)), "Directory client must own unchanged HTTP vectors");
  assert(!read(law.genericFacade).includes("document_http"), "general Schema must expose no document HTTP provider");
  const manifest = read(law.validatorPackage);
  for (const dependency of law.neutralDependencies) assert(manifest.includes(dependency));
  assert(!manifest.includes("products"), "validator package must declare no product dependency");
  const source = read(law.validatorPackage.replace("📦️packages/🦀️rust/Cargo.toml", "🦀️.rs"));
  assert(!source.includes("semio_framework_os"), "validator source must consume neutral values directly");
}
