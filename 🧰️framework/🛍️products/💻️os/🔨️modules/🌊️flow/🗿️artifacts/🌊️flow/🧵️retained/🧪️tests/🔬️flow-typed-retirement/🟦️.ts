import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../../../../../📜️script.ts";

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
  const [slider, preview, cluster] = document.widgets;
  const text = [document.schema, slider.id, slider.label, preview.id, ...Object.keys(preview.preview), ...Object.keys(Object.values(preview.preview)[0] as object), "payload", ...preview.expanded,
    cluster.id, cluster.name, ...Object.keys(cluster.flow.nodes), ...Object.values(cluster.flow.nodes).map((value: any) => value.chrome.label),
    ...document.synapses.flatMap((value: any) => [value.id, value.from, value.to, value.fromPort, value.toPort]), ...Object.keys(document.layout)];
  if (text.reduce((total, value) => total + Buffer.byteLength(value), 0) !== fixture.expected.releasedBytes) throw new Error("Flow retirement independent JSON byte oracle disagrees");
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const exact = (value: string) => value.includes("owners: ManuallyDrop<LinkedList<FlowOwner>>")
    && value.includes("maximum_bytes.min(bytes.len())") && value.includes("!std::thread::panicking()")
    && value.includes("FlowOwner::Fixture(value)") && !/\.clone\(|serde_json::to_|\.collect\(/.test(value);
  if (!exact(source)) throw new Error("Flow retirement exact source ownership linkage failed");
  const mutants = [
    source.replace("owners: ManuallyDrop<LinkedList<FlowOwner>>", "owners: LinkedList<FlowOwner>"),
    source.replace("maximum_bytes.min(bytes.len())", "bytes.len()"),
    source.replace("!std::thread::panicking()", "true"),
    source.replace("FlowOwner::Fixture(value)", "FlowOwner::Fixture(_value)"),
  ];
  for (const value of mutants) if (exact(value)) throw new Error("Flow retirement accepted hostile source");
  return 2 + hostiles.length + mutants.length;
}
