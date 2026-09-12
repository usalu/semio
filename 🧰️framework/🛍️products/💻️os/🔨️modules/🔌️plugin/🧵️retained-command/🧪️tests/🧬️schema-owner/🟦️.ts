import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { Validator } from "jsonschema";

type SchemaModule = { readonly $id: string; readonly $defs: Readonly<Record<string, unknown>>; readonly definitions?: Readonly<Record<string, unknown>> };
type FixtureTarget = { readonly schema: string; readonly definition: string; readonly fixture: string };

const OWNER_ID = "https://semio.tech/schema/os/plugin/retained-command/component.json";
const UI_ID = "https://semio.tech/schema/framework/ui/schema.json";
const LANES = ["HostOnly", "Artifact", "Config", "Draft", "Presence", "Transient", "WindowConfig", "WindowTransient", "Child", "Interaction"] as const;
const CLASSIFICATIONS = ["Unclassified", "Migrated", "BatchOnlyPendingRewrite", "ForbiddenFromUi", "Deleted"] as const;
const MOVED_EXPORTS = [
  "ArtifactToolPublicationLane", "InteractiveJobClassification", "RetainedCommandExecution", "RetainedCommandByteBudget",
  "RetainedCommandStepBudget", "RetainedCommandClassBudget", "RetainedCommandPublicationContract", "RetainedCommandBoundaryCase",
  "RetainedCommandOracle", "RetainedCommandRouteDisposition", "RetainedCommandRouteExecutionLane", "RetainedCommandRouteExecutionFeature",
  "RetainedCommandRouteVariant", "RetainedCommandCorpusLimits", "RetainedCommandDeclaredLimits", "RetainedCommandCohortFactory",
  "RetainedCommandCohortRoute", "RetainedCommandCohortApp", "RetainedCommandCensus", "RetainedCommandLimits",
  "RetainedCommandRoute", "RetainedCommandRoutes", "RetainedCommandCohort", "RetainedCommandRoutesDocument",
] as const;

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
  const root = fileURLToPath(new URL("../../../../../../../../", import.meta.url));
  const readJson = (path: string): any => JSON.parse(readFileSync(join(root, path), "utf8"));
  const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json";
  const uiPath = "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json";
  const owner = readJson(ownerPath) as SchemaModule;
  const ui = readJson(uiPath) as SchemaModule;
  assert.equal(owner.$id, OWNER_ID);
  assert.deepEqual((owner.$defs.ArtifactToolPublicationLane as { enum: string[] }).enum, LANES);
  assert.deepEqual((owner.$defs.InteractiveJobClassification as { enum: string[] }).enum, CLASSIFICATIONS);
  assert.deepEqual((owner.$defs.RetainedCommandAdmissionOutcome as { enum: string[] }).enum, ["FailClosed"]);
  assert.equal(MOVED_EXPORTS.length, 24);
  for (const name of MOVED_EXPORTS) assert(Object.hasOwn(owner.$defs, name), `retained-command owner omits ${name}`);
  assert.equal(Object.keys(owner.$defs).some((name) => name.startsWith("retainedCommand")), false);
  assert.equal(Object.keys(ui.definitions ?? {}).some((name) => name.startsWith("retainedCommand")), false);
  assert.equal(Object.keys(ui.$defs).some((name) => name.startsWith("RetainedCommand")), false);
  assert.equal(existsSync(join(root, "🧰️framework/🔨️modules/🔺️mesh/🟦️.ts")), false);
  assert.equal(existsSync(join(root, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts")), true);
  const scene = readFileSync(join(root, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts"), "utf8");
  const manifest = readFileSync(join(root, "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"), "utf8");
  const barrel = readFileSync(join(root, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts"), "utf8");
  assert(scene.includes('from "../../🛂️manifest/🟦️.ts"'));
  assert(manifest.includes('from "../🖱️ui/🎬️scene/🟦️.ts"'));
  assert(barrel.includes('export * from "../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts"'));
  assert.equal(`${scene}\n${manifest}\n${barrel}`.includes("🔺️mesh/🟦️.ts"), false);

  const targets: FixtureTarget[] = [
    { schema: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", definition: "VcsRetainedCommandRoutes", fixture: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json" },
    { schema: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️schema/🔣️.json", definition: "WiresRetainedCommandRoutes", fixture: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json" },
    { schema: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", definition: "ShootingRetainedCommandLimits", fixture: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", definition: "RemodelingRetainedCommandLimits", fixture: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", definition: "HomeRetainedCommandLimits", fixture: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", definition: "SpaceIndexRetainedCommandLimits", fixture: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
    { schema: "✏️s/🔌️plugins/🪐️space/🧬️schema/🔣️.json", definition: "SpacePlayRetainedCommandLimits", fixture: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧫️fixtures/🧫️retained-command-limits/🔣️.json" },
  ];
  const schemas = targets.map((target) => exportClosure(readJson(target.schema) as SchemaModule, target.definition));
  for (const schema of schemas) assert.equal(JSON.stringify(schema.$defs).includes(`${UI_ID}#/$defs/RetainedCommand`), false);

  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const keyword of ["x-semio-state", "x-semio-child-kind", "x-semio-formats"]) ajv.addKeyword(keyword);
  for (const format of ["base64", "double", "float", "int32", "int64", "uint32", "uint64"]) ajv.addFormat(format, true);
  ajv.addSchema(owner);
  for (const schema of schemas) ajv.addSchema(schema);
  const jsonSchema = new Validator();
  jsonSchema.addSchema(owner as any, owner.$id);
  for (const schema of schemas) jsonSchema.addSchema(schema as any, schema.$id);
  const outcomes: boolean[][] = [];
  for (const [index, target] of targets.entries()) {
    const schema = schemas[index]!;
    const fixture = readJson(target.fixture);
    const reference = `${schema.$id}#/$defs/${target.definition}`;
    const ajvValidate = ajv.compile({ $ref: reference });
    const ajvValid = ajvValidate(fixture) as boolean;
    assert(ajvValid, `${target.definition} Ajv: ${ajv.errorsText(ajvValidate.errors)}`);
    const jsonSchemaValid = jsonSchema.validate(fixture, { $ref: reference } as any).valid;
    assert(jsonSchemaValid, `${target.definition} jsonschema rejected the canonical fixture`);
    const hostile = structuredClone(fixture) as Record<string, any>;
    const firstLane = hostile.routes?.find((route: any) => Array.isArray(route.lanes) && route.lanes.length > 0);
    if (firstLane) firstLane.lanes[0] = "host-only";
    else hostile.routes[0]["unexpected"] = true;
    const ajvHostile = ajvValidate(hostile) as boolean;
    const jsonSchemaHostile = jsonSchema.validate(hostile, { $ref: reference } as any).valid;
    assert.deepEqual([ajvValid, ajvHostile], [jsonSchemaValid, jsonSchemaHostile]);
    assert.equal(ajvHostile, false);
    outcomes.push([ajvValid, ajvHostile]);
  }
  const directFixture = readJson("✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json");
  const directRef = `${OWNER_ID}#/$defs/RetainedCommandRoutesDocument`;
  const directAjv = ajv.compile({ $ref: directRef });
  assert(directAjv(directFixture), ajv.errorsText(directAjv.errors));
  assert(jsonSchema.validate(directFixture, { $ref: directRef } as any).valid);
  for (const lane of LANES) assert(ajv.validate(`${OWNER_ID}#/$defs/ArtifactToolPublicationLane`, lane));
  for (const classification of CLASSIFICATIONS) assert(ajv.validate(`${OWNER_ID}#/$defs/InteractiveJobClassification`, classification));
  assert.equal(ajv.validate(`${OWNER_ID}#/$defs/InteractiveJobClassification`, "FailClosed"), false);
  assert(ajv.validate(`${OWNER_ID}#/$defs/RetainedCommandAdmission`, "FailClosed"));
  console.log(`[DEBUG] retained-command owner validated ${targets.length + 1} fixtures with Ajv and jsonschema; lanes=${LANES.length} classifications=${CLASSIFICATIONS.length}`);
  return { fixtures: targets.length + 1, validators: 2, lanes: LANES.length, classifications: CLASSIFICATIONS.length };
}
