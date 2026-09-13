import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

const WORKSPACE_ROOT = process.cwd();

/** 🧪️ Executes flow typed retirement policy assertions. */
export function flowTypedRetirementSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained");
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/FlowRetirementV1" });
  if (!validate(fixture)) throw new Error("Flow retirement strict fixture schema failed");
  const malformed = structuredClone(fixture); malformed.fixture.widgets[0].extra = true;
  const hostiles = [{ ...fixture, extra: true }, malformed, { ...fixture, expected: { ...fixture.expected, terminalEmpty: false } }];
  for (const value of hostiles) if (validate(value)) throw new Error("Flow retirement schema accepted hostile payload");
  const document = JSON.parse(JSON.stringify(fixture.fixture));
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
  const exact = (value: string) => value.includes("frontier: ManuallyDrop<PagedList<FlowOwner")
    && value.includes("pub fn next_allocation_bytes") && value.includes("pub fn reserve_allocation")
    && value.includes("pub fn next_push_allocation_bytes") && value.includes("pub fn reserve_push_allocation")
    && value.includes("pub fn push(&mut self, owner: FlowOwner) -> Result<(), FlowOwner>") && value.includes("return Err(owner)")
    && value.includes("pub fn next_close_byte_demand") && value.includes("release_backing")
    && value.includes("!std::thread::panicking()") && value.includes("FlowOwner::Fixture(value)")
    && !value.includes("maximum_bytes.min(bytes.len())") && !value.includes("close_step(1, 4096)")
    && !value.includes("std::mem::forget(owner)") && !/\.clone\(|serde_json::to_/.test(value);
  if (!exact(source)) throw new Error("Flow retirement exact source ownership linkage failed");
  const mutants = [
    source.replace("frontier: ManuallyDrop<PagedList<FlowOwner", "frontier: ManuallyDrop<Vec<FlowOwner"),
    source.replace("pub fn next_close_byte_demand", "fn next_close_byte_demand"),
    source.replace("pub fn next_push_allocation_bytes", "fn next_push_allocation_bytes"),
    source.replace("return Err(owner)", "drop(owner); return Ok(())"),
    source.replace("!std::thread::panicking()", "true"),
    source.replace("FlowOwner::Fixture(value)", "FlowOwner::Fixture(_value)"),
  ];
  for (const value of mutants) if (exact(value)) throw new Error("Flow retirement accepted hostile source");
  return 2 + hostiles.length + mutants.length;
}
