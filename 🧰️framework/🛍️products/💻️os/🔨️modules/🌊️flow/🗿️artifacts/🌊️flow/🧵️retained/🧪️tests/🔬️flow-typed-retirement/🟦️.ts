import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

const WORKSPACE_ROOT = process.cwd();

/** 🧪️ Executes flow typed retirement policy assertions. */
export function flowTypedRetirementSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained");
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
  const fields = JSON.parse(readFileSync(join(base, "🧫️fixtures/🧬️fields/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/FlowCamera" });
  if (!validate(fixture.hostSnapshot.camera)) throw new Error("Flow camera violates its semantic contract");
  const validateOwned = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/OwnedFields" });
  if (!validateOwned(fields.snapshot)) throw new Error("Flow owned field source violates its production shape");
  const stable = createRequire(import.meta.url)("fast-json-stable-stringify");
  if (stable(JSON.parse(JSON.stringify(fields.snapshot))) !== stable(fields.snapshot)) throw new Error("Flow field independent stable wire differs");
  if (fields.ownedStrings.reduce((bytes: number, text: string) => bytes + Buffer.byteLength(text), 0) !== fields.expectedCopyBytes) throw new Error("Flow field independent UTF8 work census differs");
  
  const document = JSON.parse(JSON.stringify(fixture.hostSnapshot));
  const physical = fixture.physicalRetirement;
  const direct = ["bytes", "strings", "widgets", "specs", "neurons", "synapses", "previews", "layout"];
  const nested = ["orderedSet", "orderedLayoutMap", "orderedNodeMap", "dynamicDictionary", "dynamicValue", "frontierMetadata", "frontierPayload"];
  if (JSON.stringify(physical.directOwners) !== JSON.stringify(direct) || JSON.stringify(physical.nestedOwners) !== JSON.stringify(nested))
    throw new Error("Flow physical retirement owner matrix is incomplete");
  if (physical.oversizedMinimumBytes <= physical.subexactMaximumBytes)
    throw new Error("Flow physical retirement fixture does not cross the legacy 4096-byte boundary");
  const ingress = physical.multiRootIngress;
  if (ingress.ownerCount !== 2 || ingress.firstCapacityBytes <= physical.subexactMaximumBytes || ingress.secondCapacityBytes <= physical.subexactMaximumBytes)
    throw new Error("Flow multi-root ingress fixture does not retain two oversized owners");
  if (Object.values(ingress.laws).some((value) => value !== true))
    throw new Error("Flow multi-root ingress fixture disables an owner-preservation law");
  if (ingress.firstCapacityBytes + ingress.secondCapacityBytes !== 20482)
    throw new Error("Flow multi-root ingress independent capacity oracle disagrees");
  if (Object.values(physical.laws).some((value) => value !== true))
    throw new Error("Flow physical retirement fixture disables a required law");
  const [slider, preview, cluster] = document.widgets;
  const text = [document.schema, slider.id, slider.label, preview.id, ...Object.keys(preview.preview), ...Object.keys(Object.values(preview.preview)[0] as object), "payload", ...preview.expanded,
    cluster.id, cluster.name, ...Object.keys(cluster.flow.nodes), ...Object.values(cluster.flow.nodes).map((value: any) => value.chrome.label),
    ...document.synapses.flatMap((value: any) => [value.id, value.from, value.to, value.fromPort, value.toPort]), ...Object.keys(document.layout)];
  if (text.reduce((total, value) => total + Buffer.byteLength(value), 0) !== fixture.expected.releasedBytes) throw new Error("Flow retirement independent JSON byte oracle disagrees");
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const exact = (value: string) => value.includes("root:Option<ControlledRetirement<FlowOwner>>")
    && value.includes("queue:RetirementQueue") && value.includes("pub fn step(&mut self,grant:RetainedCloneGrant)")
    && value.includes("pub fn next_capacity_byte_demand") && value.includes("pub fn next_release_byte_demand")
    && value.includes("pub fn reserve_push") && value.includes("pub fn admit_owner")
    && value.includes("pub fn push(&mut self,owner:FlowOwner)->Result<(),FlowOwner>") && value.includes("return Err(owner)")
    && value.includes("std::thread::panicking()||self.terminal_is_empty()") && value.includes("owned_variant!(Bytes,Strings,Set,Dictionary,Value,HostSnapshot")
    && !value.includes("maximum_bytes.max") && !value.includes("released_bytes.min")
    && !value.includes("std::mem::forget(owner)") && !/\.clone\(|serde_json::to_/.test(value);
  if (!exact(source)) throw new Error("Flow retirement exact source ownership linkage failed");
  const mutants = [
    source.replace("root:Option<ControlledRetirement<FlowOwner>>", "root:Vec<FlowOwner>"),
    source.replace("pub fn next_release_byte_demand", "fn next_release_byte_demand"),
    source.replace("pub fn reserve_push", "fn reserve_push"),
    source.replace("return Err(owner)", "drop(owner); return Ok(())"),
    source.replace("std::thread::panicking()||self.terminal_is_empty()", "true"),
    source.replace("owned_variant!(Bytes,Strings,Set,Dictionary,Value,HostSnapshot", "owned_variant!(Bytes,Strings,Set,Dictionary,Value"),
  ];
  for (const value of mutants) if (exact(value)) throw new Error("Flow retirement accepted hostile source");
  return 2 + mutants.length;
}
