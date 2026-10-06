import Ajv from "ajv";
import { Validator } from "jsonschema";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

type SchemaModule = { readonly $id: string; readonly $defs: Readonly<Record<string, unknown>>; readonly definitions?: Readonly<Record<string, unknown>> };
type ExampleTarget = { readonly schema: string; readonly fixture: string };

const OWNER_ID = "https://json.schemas.assets.semio-tech.com/os/plugin/retained-command/component.json";
const UI_ID = "https://json.schemas.assets.semio-tech.com/framework/ui/schema.json";
const LANES = ["HostOnly", "Artifact", "Config", "Draft", "Presence", "Transient", "WindowConfig", "WindowTransient", "Child", "Interaction"] as const;
const CLASSIFICATIONS = ["Unclassified", "Migrated", "BatchOnlyPendingRewrite", "ForbiddenFromUi", "Deleted"] as const;
const MOVED_EXPORTS = ["ArtifactToolPublicationLane", "InteractiveJobClassification", "RetainedCommandExecution", "RetainedCommandByteBudget", "RetainedCommandStepBudget", "RetainedCommandClassBudget", "RetainedCommandPublicationContract", "RetainedCommandRouteDisposition", "RetainedCommandRouteExecutionLane", "RetainedCommandRouteExecutionFeature", "RetainedCommandRouteVariant", "RetainedCommandCohortFactory", "RetainedCommandCohortRoute", "RetainedCommandCohortApp", "RetainedCommandBudget", "RetainedCommandRoute", "RetainedCommandRoutes", "RetainedCommandRoutesDocument"] as const;

/** 🪢️ Isolates one consumer export and its local `$defs` closure from unrelated sibling dependencies. */
function exportClosure(schema: SchemaModule, exportId: string): SchemaModule {
  const closure: Record<string, unknown> = {};
  const visit = (name: string): void => {
    if (Object.hasOwn(closure, name)) return;
    const definition = schema.$defs[name];
    assert(definition, `${schema.$id} declares no $defs/${name}`);
    closure[name] = definition;
    const scan = (value: unknown): void => {
      if (Array.isArray(value)) for (const child of value) scan(child);
      else if (value && typeof value === "object") for (const [key, child] of Object.entries(value)) {
        if (key === "$ref" && typeof child === "string" && child.startsWith("#/$defs/")) visit(child.slice("#/$defs/".length));
        else scan(child);
      }
    };
    scan(definition);
  };
  visit(exportId);
  return { $id: schema.$id, $defs: closure };
}

/** 🧬️ Proves retained-command vocabulary, reachable consumers, and fixtures have one OS-plugin owner. */
export function testRetainedCommandSchemaOwnership(): { readonly fixtures: number; readonly validators: number; readonly lanes: number; readonly classifications: number } {
  const root = fileURLToPath(new URL("../../../../../../../..", import.meta.url));
  const readJson = (path: string): any => JSON.parse(readFileSync(join(root, path), "utf8"));
  const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json";
  const uiPath = "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json";
  const owner = readJson(ownerPath) as SchemaModule;
  assert.equal(existsSync(join(root, uiPath)), false);
  assert.equal(owner.$id, OWNER_ID);
  assert.deepEqual((owner.$defs.ArtifactToolPublicationLane as { enum: string[] }).enum, LANES);
  assert.deepEqual((owner.$defs.InteractiveJobClassification as { enum: string[] }).enum, CLASSIFICATIONS);
  assert.deepEqual((owner.$defs.RetainedCommandAdmissionOutcome as { enum: string[] }).enum, ["FailClosed"]);
  assert.equal(MOVED_EXPORTS.length, 18);
  for (const name of MOVED_EXPORTS) assert(Object.hasOwn(owner.$defs, name), `retained-command owner omits ${name}`);
  assert.equal(Object.keys(owner.$defs).some((name) => name.startsWith("retainedCommand")), false);
  assert.equal(existsSync(join(root, "🧰️framework/🔨️modules/🔺️mesh/🟦️.ts")), false);
  assert.equal(existsSync(join(root, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts")), true);
  const scene = readFileSync(join(root, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts"), "utf8");
  const manifest = readFileSync(join(root, "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"), "utf8");
  const barrel = readFileSync(join(root, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts"), "utf8");
  assert(scene.includes('from "../../🛂️manifest/🟦️.ts"'));
  assert(manifest.includes('from "../🖱️ui/🎬️scene/🟦️.ts"'));
  assert(barrel.includes('export * from "../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts"'));
  assert.equal(`${scene}\n${manifest}\n${barrel}`.includes("🔺️mesh/🟦️.ts"), false);

  const targets: ExampleTarget[] = [
    { schema: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", fixture: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json" },
    { schema: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️schema/🔣️.json", fixture: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json" },
    { schema: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", fixture: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", fixture: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", fixture: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", fixture: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/🪐️space/🧬️schema/🔣️.json", fixture: "🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
  ];
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const keyword of ["x-semio-state", "x-semio-child-kind", "x-semio-formats"]) ajv.addKeyword(keyword);
  for (const format of ["base64", "double", "float", "int32", "int64", "uint32", "uint64"]) ajv.addFormat(format, true);
  ajv.addSchema(owner);
  const jsonSchema = new Validator();
  const draft7Owner = JSON.parse(JSON.stringify(owner).replaceAll("\"$defs\":", "\"definitions\":").replaceAll("#/$defs/", "#/definitions/"));
  delete draft7Owner.$ref;
  jsonSchema.addSchema(draft7Owner, owner.$id);
  const validateValue = (definition: string, value: any): void => {
    const reference = `${OWNER_ID}#/$defs/${definition}`;
    const independent = ajv.compile({ $ref: reference });
    assert(independent(value), ajv.errorsText(independent.errors));
    assert.equal(jsonSchema.validate(value, { $ref: reference.replace("#/$defs/", "#/definitions/") } as any).valid, true);
    const hostile = structuredClone(value);
    if (definition === "RetainedCommandBudget") {
      if ("rawBytes" in hostile) hostile.rawBytes = -1;
      else hostile.bounded.maxRawBytes = -1;
    } else if (Array.isArray(hostile.lanes) && hostile.lanes.length) hostile.lanes[0] = "host-only";
    else hostile.unexpected = true;
    assert.equal(independent(hostile), false);
    assert.equal(jsonSchema.validate(hostile, { $ref: reference.replace("#/$defs/", "#/definitions/") } as any).valid, false);
  };
  for (const target of targets) {
    const example = readJson(target.fixture);
    if (example.limits) validateValue("RetainedCommandBudget", example.limits);
    for (const route of example.routes) validateValue("RetainedCommandRoute", route);
    for (const contract of example.publicationContracts ?? []) validateValue("RetainedCommandPublicationContract", contract);
  }
  const directFixture = readJson("✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json");
  const classificationRef = `${OWNER_ID}#/$defs/InteractiveJobClassification`;
  for (const route of directFixture.routes) {
    assert(ajv.validate(classificationRef, route.disposition), ajv.errorsText());
    assert(jsonSchema.validate(route.disposition, { $ref: classificationRef.replace("#/$defs/", "#/definitions/") } as any).valid);
  }
  assert.equal(new Set(directFixture.routes.map((route: { id: string }) => route.id)).size, directFixture.routes.length);
  for (const lane of LANES) assert(ajv.validate(`${OWNER_ID}#/$defs/ArtifactToolPublicationLane`, lane));
  for (const classification of CLASSIFICATIONS) assert(ajv.validate(`${OWNER_ID}#/$defs/InteractiveJobClassification`, classification));
  assert.equal(ajv.validate(`${OWNER_ID}#/$defs/InteractiveJobClassification`, "FailClosed"), false);
  assert(ajv.validate(`${OWNER_ID}#/$defs/RetainedCommandAdmission`, "FailClosed"));
  return { fixtures: targets.length + 1, validators: 2, lanes: LANES.length, classifications: CLASSIFICATIONS.length };
}
