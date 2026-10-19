import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { readSourceFamilySource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { WORKSPACE_ROOT } from "@semio-tech/repo-lib/inventory";
import { toolJobRuntimeProofQualified } from "@semio-tech/repo-lib/laws";

/** 🧪️ Compares the runtime join fixture with an independent Ajv exact-authority oracle and hostile source variants. */
export function toolJobFactoryProofJoinSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔬️tool-factory-proof.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const ajv = new Ajv({ strict: true, allErrors: true });
  
  
  const exact = { ownerType: "TestApp", controller: "s.test.synthetic@1/*#editor", documentSchema: "semio.test/v1", tool: "setLabel", factoryType: "TestRetainedCommandFactory", factoryTypeName: "plugin::TestRetainedCommandFactory", payloadSchema: "semio.test.retained-command.v1", executionContract: "resumable:4096:4:1:4096:7500:1:1", liveBusRegistration: true, factoryName: "TestRetainedCommandFactory", registered: true, unique: true };
  const oracle = ajv.compile({ const: exact });
  const field = { wrongType: "factoryType", sameOwnerDifferentFactory: "factoryType", wrongTypeName: "factoryTypeName", wrongFactory: "factoryName", wrongOwner: "ownerType", wrongController: "controller", wrongDocumentSchema: "documentSchema", wrongTool: "tool", wrongContract: "executionContract", wrongPayloadSchema: "payloadSchema", differentBus: "liveBusRegistration" };
  for (const law of fixture.cases) {
    const candidate: Record<string, unknown> = { ...exact };
    if (law.change === "missingType") delete candidate.factoryType;
    else if (law.change === "missingRegistration") candidate.registered = false;
    else if (law.change === "sentinel") { candidate.factoryName = "BoundedFirstStepCommandJobFactory"; delete candidate.factoryType; delete candidate.factoryTypeName; }
    else if (law.change === "duplicate") candidate.unique = false;
    else if (law.change !== "none") candidate[field[law.change as keyof typeof field]] = "other-authority";
    if (oracle(candidate) !== law.accepted) throw new Error(`factory runtime oracle disagrees for ${law.id}`);
  }
  const owners = ["🦀️.rs", "🧵️instance/🦀️.rs", "👷️job/🏛️reserved/🦀️.rs", "📜️declaration/👷️job/🦀️.rs", "⚛️reactor/🔌️app/🦀️.rs", "⚛️reactor/🏃️runtime/🦀️.rs", "👷️job/🦀️.rs"] as const;
  const rootSource = readSourceFamilySource(base);
  for (const owner of owners.slice(1)) if (!rootSource.includes(`#[path = "${owner}"]`)) throw new Error(`factory runtime owner is not mounted: ${owner}`);
  const source = owners.map(owner => readFileSync(join(base, owner), "utf8")).join("\n");
  if (!toolJobRuntimeProofQualified(source)) throw new Error("production runtime factory join is not exact");
  const hostile = [
    "A::register_tool_job_factories(&mut app_tool_registry)",
    "crate::reserved_jobs::tool_job_registration::<A>(&registry, &app_id, A::DOCUMENT_SCHEMA,",
    "row.factory_type_id == Some(registration.factory_type_id)",
    "row.factory_type_name == Some(registration.factory_type_name)",
    "registration.owner == owner",
    "registration.key.controller_id == runtime_controller_id",
    "registration.key.tool_id == row.tool_id",
    "registration.contract == row.contract",
    "bus.admit_exact_wire(runtime_controller_id, row.tool_id, &registration.schema_id, &[])",
    "QualifiedToolProof::AppOwned(registration.clone()).admits::<A>(&admission)",
    "proof.with_factory_type::<$owner, $factory_type>()",
    "&& registered.is_none()",
  ];
  for (const anchor of hostile) {
    if (!source.includes(anchor)) throw new Error(`factory runtime hostile anchor missing: ${anchor}`);
    if (toolJobRuntimeProofQualified(source.replaceAll(anchor, "unqualified_authority"))) throw new Error(`factory runtime accepts missing authority: ${anchor}`);
  }
  return fixture.cases.length + hostile.length + 1;
}
