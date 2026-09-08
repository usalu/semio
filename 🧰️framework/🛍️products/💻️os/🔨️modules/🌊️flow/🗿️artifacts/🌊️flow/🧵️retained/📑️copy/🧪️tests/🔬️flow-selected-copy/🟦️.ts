import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../../../../../../📜️script.ts";

/** 🧪️ Executes flow selected copy policy assertions. */
export function flowSelectedCopySelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained");
  const schema = JSON.parse(readFileSync(join(base, "📑️copy/🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "📑️copy/🧫️fixtures/🔣️.json"), "utf8"));
  const document = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8")).fixture;
  const requireTest = createRequire(import.meta.url);
  const Ajv = requireTest("ajv");
  const stable = requireTest("fast-json-stable-stringify");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/FlowTypedCopyV1" });
  if (!validate(fixture)) throw new Error("Flow selected copy strict schema failed");
  const malformed = structuredClone(fixture); malformed.cases[0].documentJson = "{}";
  const hostiles = [{ ...fixture, extra: true }, malformed, { ...fixture, expected: { ...fixture.expected, framesBeforeRoot: false } }];
  for (const value of hostiles) if (validate(value)) throw new Error("Flow selected copy accepted hostile schema");
  for (const test of fixture.cases) {
    const selected = test.kind === "fixture" ? document : document[test.kind === "widget" ? "widgets" : "synapses"][test.index];
    const copied = structuredClone(selected);
    if (stable(copied) !== stable(JSON.parse(JSON.stringify(selected)))) throw new Error("Flow selected copy third-party canonical oracle disagrees");
    const pointer = test.pointer === "" ? document : test.pointer.slice(1).split("/").reduce((value: any, key: string) => value[key], document);
    if (stable(pointer) !== stable(copied)) throw new Error("Flow selected copy fixture pointer disagrees");
  }
  const source = readFileSync(join(base, "📑️copy/🦀️.rs"), "utf8");
  const exact = (value: string) => value.includes("owned: ManuallyDrop<CopyState<R, T>>")
    && value.includes("root: Arc<dyn Any + Send + Sync>") && value.includes("unsafe impl<T: Sync> Send for Rooted<T>")
    && value.includes("maximum_bytes.min(source.len() - start)") && value.includes("if !state.started")
    && value.includes("state.failed = true") && value.includes("!std::thread::panicking()")
    && value.includes("state.tasks.pop_front()") && value.includes('expect("selected copy retirement factory").retire(root)')
    && value.includes("released_items > 1 || released_bytes > maximum_bytes") && value.includes("state.root_retirement.is_none()")
    && value.includes("target.try_reserve_exact(count)") && value.includes("bytes > self.maximum_single_bytes || total > self.maximum_total_bytes")
    && value.includes("source: Rooted<T>") && value.includes("self.source.get().clone()")
    && !/BTreeMap|BTreeSet|\.nth\(|serde_json::to_|Arc::make_mut|target\.insert\(/.test(value);
  if (!exact(source)) throw new Error("Flow selected copy ownership source linkage missing");
  const mutants = [
    source.replace("owned: ManuallyDrop<CopyState<R, T>>", "owned: CopyState<R, T>"),
    source.replace("maximum_bytes.min(source.len() - start)", "source.len() - start"),
    source.replaceAll("state.failed = true", "state.failed = false"),
    source.replaceAll("!std::thread::panicking()", "true"),
    source.replace('expect("selected copy retirement factory").retire(root)', 'expect("selected copy retirement factory").drop(root)'),
    source.replace("released_items > 1 || released_bytes > maximum_bytes", "false"),
    source.replace("bytes > self.maximum_single_bytes || total > self.maximum_total_bytes", "false"),
  ];
  for (const value of mutants) if (exact(value)) throw new Error("Flow selected copy accepted hostile ownership source");
  return fixture.cases.length + hostiles.length + mutants.length;
}
