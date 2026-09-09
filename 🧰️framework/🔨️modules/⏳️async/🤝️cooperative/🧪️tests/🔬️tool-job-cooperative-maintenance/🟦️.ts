import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../📜️script.ts";

/** 🧪️ Executes tool job cooperative maintenance policy assertions. */
export function toolJobCooperativeMaintenanceSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/⏳️async/🤝️cooperative");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`cooperative maintenance schema: ${JSON.stringify(validate.errors)}`);
  const weights = new Map([["Maintenance", 1], ["Background", 2], ["UserVisible", 4], ["Io", 4], ["Timer", 3], ["Interactive", 8]]);
  const seen = new Set<string>();
  for (const law of fixture.cases) {
    const cost = BigInt(fixture.unitCost), weight = BigInt(law.weight);
    const turns = Number((cost + weight - 1n) / weight);
    if (seen.has(law.lane) || weights.get(law.lane) !== law.weight || turns !== law.selected.length || turns !== law.deficits.length) throw new Error(`cooperative maintenance exact lane: ${law.lane}`);
    seen.add(law.lane);
    for (let turn = 1; turn <= turns; turn++) {
      const accrued = BigInt(turn) * weight;
      if (law.deficits[turn - 1] !== Number(accrued % cost) || law.selected[turn - 1] !== (accrued >= cost)) throw new Error(`cooperative maintenance arithmetic oracle: ${law.lane}/${turn}`);
    }
  }
  for (const mutation of [{ unitCost: 9 }, { synchronousDrain: true }]) {
    if (validate({ ...fixture, ...mutation })) throw new Error("cooperative maintenance accepted changed grant or unknown drain");
  }
  return fixture.cases.length + 2;
}
