import { join } from "node:path";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

/** 🧪️ Proves the verifier rejects the historical and audited false-positive classes. */
/** 🧾️ Joins the native ARC1 contract to strict schema validation and independent platform byte encoders. */
export function toolJobCheckpointSelfTests(): number {
  const base = join(getWorkspaceRoot(), "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/📸️artifact-command-checkpoint.json"), "utf8")) as {
    format: string; version: number; maximumBytes: number; headerBytes: number;
    cases: { name: string; workPhase: boolean; rawPageCursor: number; rawBytes: number; workProgress: number; contextDigest: number; workspaceIdentity: number; workState: { bytes?: number[]; fill?: number; length?: number }; outcome: string }[];
  };
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/ArtifactCommandCheckpointV1" });
  if (!validate(fixture)) throw new Error(`[verify interactivity tool-jobs] native checkpoint fixture/schema mismatch: ${JSON.stringify(validate.errors)}`);
  let checks = 1;
  for (const entry of fixture.cases) {
    const work = Uint8Array.from(entry.workState.bytes ?? Array(entry.workState.length).fill(entry.workState.fill));
    const length = fixture.headerBytes + work.length;
    const outcome = length <= fixture.maximumBytes ? "ok" : "capacityError";
    if (outcome !== entry.outcome) throw new Error(`[verify interactivity tool-jobs] checkpoint capacity differs for ${entry.name}`);
    checks += 1;
    if (outcome !== "ok") continue;
    const encoded = new Uint8Array(length);
    encoded.set(new TextEncoder().encode(fixture.format));
    encoded[4] = fixture.version;
    encoded[5] = Number(entry.workPhase);
    const view = new DataView(encoded.buffer);
    const scalars = [entry.rawPageCursor, entry.rawBytes, entry.workProgress, entry.contextDigest, entry.workspaceIdentity];
    scalars.forEach((value, index) => view.setBigUint64(8 + index * 8, BigInt(value), true));
    encoded.set(work, fixture.headerBytes);
    const oracle = Buffer.alloc(length);
    oracle.write("ARC1", 0, "ascii");
    oracle[4] = 3;
    oracle[5] = Number(entry.workPhase);
    scalars.forEach((value, index) => oracle.writeBigUInt64LE(BigInt(value), 8 + index * 8));
    Buffer.from(work).copy(oracle, 48);
    if (!oracle.equals(Buffer.from(encoded))) throw new Error(`[verify interactivity tool-jobs] checkpoint byte oracle differs for ${entry.name}`);
    checks += 1;
  }
  const hostile = (name: string, mutate: (value: typeof fixture) => void) => {
    const value = structuredClone(fixture);
    mutate(value);
    if (validate(value)) throw new Error(`[verify interactivity tool-jobs] checkpoint schema admitted ${name}`);
    checks += 1;
  };
  hostile("work-byte-plus-one", value => { value.cases[1]!.workState = { bytes: [256] }; });
  hostile("maximum-plus-one-success", value => { value.cases[3]!.workState = { fill: 165, length: 465 }; });
  hostile("maximum-wrongly-rejected", value => { value.cases[4]!.workState = { fill: 90, length: 464 }; });
  hostile("wrong-header-authority", value => { value.headerBytes = 40; });
  hostile("unknown-owner", value => { Object.assign(value, { unowned: true }); });
  return checks;
}
