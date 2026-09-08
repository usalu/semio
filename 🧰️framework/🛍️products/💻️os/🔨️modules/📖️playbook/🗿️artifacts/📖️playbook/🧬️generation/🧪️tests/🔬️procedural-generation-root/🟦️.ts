import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../../../../../📜️script.ts";

/** 🧪️ Executes procedural generation root policy assertions. */
export function proceduralGenerationRootSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook");
  const schema = JSON.parse(readFileSync(join(base, "🧬️generation/🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "🧬️generation/🧫️fixtures/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/GenerationRootV1" });
  if (!validate(fixture)) throw new Error("generation root fixture failed strict schema");
  const hostiles = [{ ...fixture, extra: true }, { ...fixture, generation: { ...fixture.generation, extra: true } }, { ...fixture, expected: { ...fixture.expected, sharesAllocation: false } }];
  for (const value of hostiles) if (validate(value)) throw new Error("generation root schema accepted hostile input");
  const wire = JSON.stringify(fixture.generation);
  if (Buffer.byteLength(wire) <= 16384 || JSON.stringify(JSON.parse(wire)) !== wire) throw new Error("generation root independent JSON oracle lost large nested content");
  const source = readFileSync(join(base, "🧬️generation/🦀️.rs"), "utf8");
  const modelPath = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs";
  const snapshot = readFileSync(join(WORKSPACE_ROOT, modelPath), "utf8");
  const second = readFileSync(join(WORKSPACE_ROOT, modelPath.replace("🧊️generation3d", "🌀️generation2d")), "utf8");
  const exact = (root: string, model: string) => root.includes("struct GenerationPlayRoot(ManuallyDrop<Option<Arc<GenerationPlayState>>>)")
    && root.includes("Arc::get_mut(self.0.as_mut()") && root.includes("Arc::into_inner(root)")
    && root.includes("owned: ManuallyDrop<GenerationRetirementState>") && root.includes("!std::thread::panicking()")
    && root.includes('panic!("nonempty generation root must be explicitly retired before drop")')
    && root.includes("JsonOwner::Object(value.into_iter())") && root.includes("values.next()")
    && root.includes("bytes.min(value.len())") && !/Arc::make_mut|DerefMut|\.collect\(/.test(root)
    && model.includes("semio_framework_artifact_playbook_playbook::GenerationPlayRoot") && model.includes("pub generation: GenerationPlayRoot");
  if (!exact(source, snapshot) || !exact(source, second)) throw new Error("shared generation root immutable ownership linkage missing");
  const sources = [
    [source.replace("Arc<GenerationPlayState>", "GenerationPlayState"), snapshot],
    [source.replace("Arc::get_mut(self.0.as_mut()", "Arc::make_mut(self.0.as_mut()"), snapshot],
    [source.replaceAll("Arc::into_inner(root)", "Arc::try_unwrap(root).ok()"), snapshot],
    [source, snapshot.replace("pub generation: GenerationPlayRoot", "pub generation: GenerationPlayState")],
    [source, second.replace("pub generation: GenerationPlayRoot", "pub generation: GenerationPlayState")],
    [source.replace("owned: ManuallyDrop<GenerationRetirementState>", "owned: GenerationRetirementState"), snapshot],
    [source.replaceAll("!std::thread::panicking()", "true"), snapshot],
  ];
  for (const [root, model] of sources) if (exact(root, model)) throw new Error("generation root accepted hostile source mutation");
  return 2 + hostiles.length + sources.length;
}
