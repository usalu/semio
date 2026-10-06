import { join } from "node:path";
import { readFileSync } from "node:fs";
import { WORKSPACE_ROOT, toolJobProofs, toolJobProofCatalogFailures, toolJobStaticRows, toolJobDispositions } from "../../../../../../../../📜️script.ts";

/** 🧪️ Runs strict language-neutral cross-file factory laws and hostile module substitutions. */
export function toolJobOwnerFactoryResolutionSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🏭️owner-factory-resolution.json"), "utf8"));
  
  
  const compilerProof = toolJobProofs(new Map([[fixture.ownerFile, fixture.ownerSource]]));
  if (compilerProof.length !== 1 || compilerProof[0]?.factoryType !== fixture.factoryType) throw new Error("owner factory compiler witness field was not parsed exactly");
  for (const law of fixture.cases) {
    const candidate = { ...fixture };
    if (law.from && !candidate[law.target].includes(law.from)) throw new Error(`owner factory missing hostile anchor ${law.id}`);
    candidate[law.target] = candidate[law.target].replace(law.from, law.to);
    const files = new Map<string, string>([[candidate.ownerFile, candidate.ownerSource], [candidate.factoryFile, candidate.factorySource], [candidate.glueFile, candidate.glueSource]]);
    const failures = toolJobProofCatalogFailures(files, toolJobStaticRows(files), toolJobDispositions(files), toolJobProofs(files));
    if ((failures.length === 0) !== law.accepted) throw new Error(`owner factory ${law.id}: ${JSON.stringify(failures)}`);
  }
  return fixture.cases.length + 1;
}
