/** ⚙️ Tool-job coverage: fixed-operation registry fixtures and the shared framework action fixture.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { policyReadFileSafe, policyReadModuleTreeSource } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { interactivityCheckedRustBlockScope } from "../../🔬️interactivity-audit/🖥️mounted/🎞️frame-transaction/🟦️.ts";
import { toolJobReservedSpecs } from "../🏭️factory/🟦️.ts";
import { TOOL_JOB_FRAMEWORK_RESERVED_IDS } from "../📋️inventory/🟦️.ts";

export function toolJobFixedOperationRegistryExact(source: string): boolean {
  const required = [
    "pub trait FixedOperationOwner",
    "fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep",
    "fn retained_bytes(&self) -> usize",
    "pub struct FixedOperationKey",
    "pub operation: OperationId",
    "pub generation: Generation",
    "pub struct FixedOperationAdmissionRejected<T>",
    "pub owner: T",
    "pub struct FixedOperationRegistry<T, const CAPACITY: usize>",
    "maximum_bytes: usize",
    "retained_bytes: usize",
    "slots.try_reserve_exact(CAPACITY)",
    "CAPACITY <= Self::MAXIMUM_SLOTS",
    "self.retained_bytes.checked_add(retained_bytes)",
    "next_retained_bytes > self.maximum_bytes",
    "occupied: usize",
    "self.occupied == CAPACITY",
    "self.occupied == 0",
    "entry.key == key && !entry.closing",
    "entry.owner.cancel()",
    "entry.owner.begin_close()",
    "pub fn cancel_stale_step",
    "entry.key.generation == live_generation",
    "entry.owner.close_step(child_grant).admit(child_grant,entry.owner.terminal_is_empty())",
    "entry.owner.terminal_is_empty()",
    "assert_eq!(self.occupied, 0, \"fixed operation registry reached Drop before every exact owner was terminal-empty\")",
    "maximum_plus_one_and_saturation_return_the_exact_owner",
    "stale_generation_interrupted_close_and_aba_preserve_exact_authority",
    "assert!(registry.take(stale).is_none())",
    "assert_eq!(byte_rejected.owner.identity, 12)",
    "assert_eq!(capacity_rejected.owner.identity, 16)",
    "assert!(!registry.is_empty(), \"interrupted close must retain the exact owner\")",
    "maximum_registry_backing_initializes_inside_one_interactive_ceiling_under_concurrent_load",
    "median < u128::from(semio_framework_trace::INTERACTIVE_STEP_CEILING_US)",
    "assert!(matches!(registry.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:8,maximum_depth:64}), InteractiveJobCloseStep::Complete {..}))",
  ];
  return required.every((anchor) => source.includes(anchor));
}



type FixedOperationFixtureCloseGrant = {
  maximumItems: number;
  maximumCopyBytes: number;
  maximumCapacityBytes: number;
  maximumReleaseBytes: number;
  maximumDepth: number;
};

type FixedOperationFixtureStep =
  | { action: "admit"; owner: string; operation: number; generation: number; bytes: number }
  | { action: "take" | "cancel"; operation: number; generation: number }
  | { action: "cancelStaleStep"; operation: number; liveGeneration: number }
  | { action: "close"; grant: FixedOperationFixtureCloseGrant }
  | { action: "inspect" };

type FixedOperationFixtureCase = {
  id: string;
  capacity: number;
  maximumBytes: number;
  steps: FixedOperationFixtureStep[];
  expected: string[];
};

type FixedOperationFixture = {
  schema: string;
  caller:{operation:number;generation:number;clockMicros:number;deadlineMicros:number;fuel:number;admission:FixedOperationFixtureCloseGrant};
  cases: FixedOperationFixtureCase[];
};

export type FixedOperationFixtureOutput = {
  schema: string;
  results: { id: string; output: string[] }[];
};

function toolJobFixedOperationFixtureAssertInteger(value: unknown, minimum: number, maximum: number, field: string): asserts value is number {
  if (!Number.isSafeInteger(value) || Number(value) < minimum || Number(value) > maximum) throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] invalid ${field}.`);
}

function toolJobFixedOperationFixtureAssertKeys(value: Record<string, unknown>, keys: string[], field: string): void {
  if (Object.keys(value).sort().join("\0") !== [...keys].sort().join("\0")) throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] invalid ${field} fields.`);
}

function toolJobFixedOperationFixtureIndex(operation: number, generation: number, capacity: number): number {
  const operation64 = BigInt.asUintN(64, BigInt(operation));
  const generation64 = BigInt.asUintN(64, BigInt(generation));
  const rotated = BigInt.asUintN(64, (generation64 << 17n) | (generation64 >> 47n));
  return Number(BigInt.asUintN(64, operation64 ^ rotated) % BigInt(capacity));
}

export function toolJobFixedOperationFixtureRun(root: string): FixedOperationFixtureOutput {
  const fixturePath = "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📇️fixed-operation-registry-law.json";
  const fixture = JSON.parse(policyReadFileSafe(root, fixturePath)) as FixedOperationFixture;
  if (!fixture || fixture.schema !== "semio.framework.job.fixed-operation-registry-law.v1" || !Array.isArray(fixture.cases))
    throw new Error("[verify interactivity tool-jobs fixed-operation-fixture] fixture root is not exact.");
  toolJobFixedOperationFixtureAssertKeys(fixture as unknown as Record<string,unknown>,["schema","caller","cases"],"root");
  toolJobFixedOperationFixtureAssertKeys(fixture.caller as unknown as Record<string,unknown>,["operation","generation","clockMicros","deadlineMicros","fuel","admission"],"original caller");
  for(const field of ["operation","generation","clockMicros","deadlineMicros","fuel"] as const)toolJobFixedOperationFixtureAssertInteger(fixture.caller[field],0,Number.MAX_SAFE_INTEGER,`original caller.${field}`);
  toolJobFixedOperationFixtureAssertKeys(fixture.caller.admission,["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"],"original caller.admission");
  for(const field of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"] as const)toolJobFixedOperationFixtureAssertInteger(fixture.caller.admission[field],0,Number.MAX_SAFE_INTEGER,`original caller.admission.${field}`);
  if(fixture.caller.clockMicros!==1||fixture.caller.deadlineMicros!==2||fixture.caller.fuel!==1)throw Error("Original fixed fixture caller clock and CPU changed");
  const requiredIds = ["aba", "byte-maximum-plus-one", "cancel-stale", "collision", "empty", "interrupted-repeated-close", "maximum-plus-one", "single"];
  if (fixture.cases.map((entry) => entry.id).sort().join("\0") !== requiredIds.join("\0"))
    throw new Error("[verify interactivity tool-jobs fixed-operation-fixture] law case identities are not an exact bijection.");
  const results = fixture.cases.map((testCase) => {
    toolJobFixedOperationFixtureAssertKeys(testCase as unknown as Record<string, unknown>, ["id", "capacity", "maximumBytes", "steps", "expected"], `case ${testCase.id}`);
    toolJobFixedOperationFixtureAssertInteger(testCase.capacity, 1, 64, `${testCase.id}.capacity`);
    toolJobFixedOperationFixtureAssertInteger(testCase.maximumBytes, 0, 65_536, `${testCase.id}.maximumBytes`);
    if (!Array.isArray(testCase.steps) || testCase.steps.length === 0 || !Array.isArray(testCase.expected) || testCase.expected.some((entry) => typeof entry !== "string" || entry.length === 0 || entry.length > 128))
      throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] invalid ${testCase.id} steps or expected output.`);
    type Owner = { owner: string; operation: number; generation: number; admittedBytes: number; remainingBytes: number; closing: boolean; cancelled: boolean };
    const slots: (Owner | undefined)[] = Array.from({ length: testCase.capacity });
    let retainedBytes = 0;
    let occupied = 0;
    let closeCursor = 0;
    const output: string[] = [];
    for (const step of testCase.steps) {
      if (!step || typeof step !== "object" || typeof step.action !== "string") throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] invalid ${testCase.id} step.`);
      if (step.action === "admit") {
        toolJobFixedOperationFixtureAssertKeys(step, ["action", "owner", "operation", "generation", "bytes"], `${testCase.id}.admit`);
        if (typeof step.owner !== "string" || step.owner.length === 0 || step.owner.length > 64) throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] invalid ${testCase.id}.owner.`);
        toolJobFixedOperationFixtureAssertInteger(step.operation, 0, Number.MAX_SAFE_INTEGER, `${testCase.id}.operation`);
        toolJobFixedOperationFixtureAssertInteger(step.generation, 0, Number.MAX_SAFE_INTEGER, `${testCase.id}.generation`);
        toolJobFixedOperationFixtureAssertInteger(step.bytes, 0, 65_536, `${testCase.id}.bytes`);
        const index = toolJobFixedOperationFixtureIndex(step.operation, step.generation, testCase.capacity);
        const nextBytes = retainedBytes + step.bytes;
        if (!Number.isSafeInteger(nextBytes) || occupied === testCase.capacity || nextBytes > testCase.maximumBytes || slots[index]) {
          output.push(`admit:rejected:${step.owner}`);
        } else {
          slots[index] = { owner: step.owner, operation: step.operation, generation: step.generation, admittedBytes: step.bytes, remainingBytes: step.bytes, closing: false, cancelled: false };
          retainedBytes = nextBytes;
          occupied += 1;
          output.push(`admit:accepted:${step.owner}`);
        }
      } else if (step.action === "take" || step.action === "cancel") {
        toolJobFixedOperationFixtureAssertKeys(step, ["action", "operation", "generation"], `${testCase.id}.${step.action}`);
        toolJobFixedOperationFixtureAssertInteger(step.operation, 0, Number.MAX_SAFE_INTEGER, `${testCase.id}.operation`);
        toolJobFixedOperationFixtureAssertInteger(step.generation, 0, Number.MAX_SAFE_INTEGER, `${testCase.id}.generation`);
        const index = toolJobFixedOperationFixtureIndex(step.operation, step.generation, testCase.capacity);
        const owner = slots[index];
        const exact = owner?.operation === step.operation && owner.generation === step.generation;
        if (step.action === "take") {
          if (!exact || owner!.closing) output.push("take:none");
          else {
            slots[index] = undefined;
            retainedBytes -= owner!.admittedBytes;
            occupied -= 1;
            output.push(`take:${owner!.owner}`);
          }
        } else {
          if (exact) {
            owner!.cancelled = true;
            owner!.closing = true;
          }
          output.push(`cancel:${exact ? "true" : "false"}`);
        }
      } else if (step.action === "cancelStaleStep") {
        toolJobFixedOperationFixtureAssertKeys(step, ["action", "operation", "liveGeneration"], `${testCase.id}.cancelStaleStep`);
        toolJobFixedOperationFixtureAssertInteger(step.operation, 0, Number.MAX_SAFE_INTEGER, `${testCase.id}.operation`);
        toolJobFixedOperationFixtureAssertInteger(step.liveGeneration, 0, Number.MAX_SAFE_INTEGER, `${testCase.id}.liveGeneration`);
        const owner = slots[closeCursor];
        closeCursor = (closeCursor + 1) % testCase.capacity;
        const stale = owner?.operation === step.operation && owner.generation !== step.liveGeneration;
        if (stale) {
          owner!.cancelled = true;
          owner!.closing = true;
        }
        output.push(`stale:${stale ? "true" : "false"}`);
      } else if (step.action === "close") {
        toolJobFixedOperationFixtureAssertKeys(step, ["action", "grant"], `${testCase.id}.close`);
        const grant = step.grant;
        if (!grant || typeof grant !== "object" || Array.isArray(grant)) throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] invalid ${testCase.id}.grant.`);
        toolJobFixedOperationFixtureAssertKeys(grant, ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes", "maximumDepth"], `${testCase.id}.grant`);
        for (const [field, value] of Object.entries(grant)) toolJobFixedOperationFixtureAssertInteger(value, 0, 65_536, `${testCase.id}.grant.${field}`);
        if (grant.maximumItems === 0) {
          output.push("close:blocked");
          continue;
        }
        const index = closeCursor;
        closeCursor = (closeCursor + 1) % testCase.capacity;
        const owner = slots[index];
        if (!owner) output.push(occupied === 0 ? "close:complete" : "close:pending");
        else if (!owner.closing) output.push("close:pending");
        else if (grant.maximumDepth === 0) output.push("close:blocked");
        else if ((owner.remainingBytes > 0 && grant.maximumCopyBytes === 0) || (owner.remainingBytes <= 1 && grant.maximumReleaseBytes < owner.admittedBytes)) output.push("close:pending");
        else {
          if (owner.remainingBytes > 0) owner.remainingBytes -= 1;
          if (owner.remainingBytes === 0) {
            slots[index] = undefined;
            retainedBytes -= owner.admittedBytes;
            occupied -= 1;
          }
          output.push(occupied === 0 ? "close:complete" : "close:pending");
        }
      } else if (step.action === "inspect") {
        toolJobFixedOperationFixtureAssertKeys(step, ["action"], `${testCase.id}.inspect`);
        output.push(`state:${occupied}:${retainedBytes}:${slots.reduce((sum, owner) => sum + (owner?.remainingBytes ?? 0), 0)}`);
      } else {
        throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] unsupported ${testCase.id} action.`);
      }
    }
    if (output.join("\0") !== testCase.expected.join("\0")) throw new Error(`[verify interactivity tool-jobs fixed-operation-fixture] ${testCase.id} output differs from the schema-first law.`);
    return { id: testCase.id, output };
  });
  return { schema: fixture.schema, results };
}

type SharedFrameworkActionRouteFixture = {
  schema: string;
  routes: Array<{ id: string; factory: string; schemaId: string; descriptorDisposition: "migrated"; routeIndex: number; maximumRawBytes: number; maximumItems: number; workUnits: number; maximumOutputBytes: number }>;
  descriptorLaws: Array<{
    id: string;
    ownerKind: "appAction" | "windowAction" | "appCommand" | "modeCommand";
    ownerId: string;
    actionIds: string[];
    initial: "missing" | "migrated";
    expected: "missing" | "migrated";
  }>;
  hostileLaws: Array<{
    id: string;
    kind: "rawAdmission" | "cancellation" | "freshness" | "identity" | "progress" | "close";
    value?: number;
    maximum?: number;
    operation?: number;
    generation?: number;
    liveGeneration?: number;
    maximumItems?: number;
    maximumBytes?: number;
    expected: "accepted" | "rejected" | "cancelled" | "checkpoint" | "retained" | "terminal";
  }>;
};

export function toolJobSharedFrameworkActionFixtureRun(root: string): { schema: string; routes: string[]; descriptor: string[]; hostile: string[] } {
  const fixture = JSON.parse(policyReadFileSafe(root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/⚖️shared-framework-action-routes-law.json")) as SharedFrameworkActionRouteFixture;
  if (fixture.schema !== "semio.framework.plugin.shared-framework-action-routes.v1" || fixture.routes.length !== 12)
    throw new Error("[verify interactivity tool-jobs shared-action-fixture] fixture root is not exact.");
  const expectedIds = TOOL_JOB_FRAMEWORK_RESERVED_IDS.filter((id) => !["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint", "revertToCommand", "configuration-binary"].includes(id)).sort();
  const actualIds = fixture.routes.map((route) => route.id).sort();
  if (actualIds.join("\0") !== expectedIds.join("\0") || new Set(fixture.routes.map((route) => route.factory)).size !== 12 || new Set(fixture.routes.map((route) => route.routeIndex)).size !== 12)
    throw new Error("[verify interactivity tool-jobs shared-action-fixture] route identities are not an exact bijection.");
  const source = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
  const production = new Map(toolJobReservedSpecs(source).map((route) => [route.id, route]));
  for (const route of fixture.routes) {
    const row = production.get(route.id);
    if (!row || row.factory !== route.factory || row.values.join("\0") !== [route.maximumRawBytes, route.maximumItems, route.workUnits, route.maximumOutputBytes].join("\0"))
      throw new Error(`[verify interactivity tool-jobs shared-action-fixture] ${route.id} differs from its production factory contract.`);
    if (route.schemaId !== `framework.reserved.${route.id}.v1` || route.descriptorDisposition !== "migrated" || !source.includes(`registry.register(${route.factory}::<A>::new(&controller_id))`))
      throw new Error(`[verify interactivity tool-jobs shared-action-fixture] ${route.id} lacks exact schema or live registration identity.`);
    if (!source.includes(`FrameworkSharedActionDescriptorRoute { action_id: "${route.id}", schema_id: "${route.schemaId}" }`))
      throw new Error(`[verify interactivity tool-jobs shared-action-fixture] ${route.id} lacks its schema-driven descriptor disposition route.`);
  }
  const sourceFailures: string[] = [];
  const dispositionJoin = interactivityCheckedRustBlockScope(source, "fn join_framework_shared_action_dispositions(definition: &mut AppDefinition)", "shared-action descriptor join", sourceFailures)?.replace(/\s+/g, "");
  const registration = interactivityCheckedRustBlockScope(source, "pub fn register_app_factory(mut self, mut app: App, factory: declarations::AppFactory<PA>) -> Self", "shared-action app registration", sourceFailures)?.replace(/\s+/g, "");
  const exactJoin = "fnjoin_framework_shared_action_dispositions(definition:&mutAppDefinition){foractionindefinition.actions.iter_mut().chain(definition.window_kinds.iter_mut().flat_map(|window|window.actions.iter_mut())){ifframework_shared_action_descriptor_schema_id(&action.id).is_some(){action.semantics.execution.interactive_job=semio_framework::InteractiveJobClassification::Migrated;}}}";
  if (sourceFailures.length || dispositionJoin !== exactJoin || !registration || !/^pubfnregister_app_factory\([^]*?\{letmutfactory=factory;join_framework_shared_action_dispositions\(&mutapp\.definition\);join_framework_shared_action_dispositions\(&mutfactory\.definition\);/.test(registration) || (registration.match(/join_framework_shared_action_dispositions\(/g) ?? []).length !== 2 || !/self\.manifest\.apps\.push\(app\.definition\);[^]*?self\.apps\.insert\(self\.manifest\.apps\.last\(\)\.unwrap\(\)\.id\.clone\(\),factory\);self\}$/.test(registration))
    throw new Error("[verify interactivity tool-jobs shared-action-fixture] descriptor disposition join is not exact across manifest and runtime factory definitions.");
  const dispositionById = new Map(fixture.routes.map((route) => [route.id, route.descriptorDisposition]));
  const requiredDescriptorLawIds = ["exact-app-action-owner", "exact-app-command-owner", "exact-mode-command-owner", "exact-window-owner-primary", "exact-window-owner-secondary", "near-id-app-action-owner", "near-id-window-owner", "unaccepted-app-action-owner", "unaccepted-window-owner"];
  if (fixture.descriptorLaws.map((law) => law.id).sort().join("\0") !== requiredDescriptorLawIds.join("\0"))
    throw new Error("[verify interactivity tool-jobs shared-action-fixture] descriptor law identities are not exact.");
  const descriptor = fixture.descriptorLaws.map((law) => {
    if (!law.ownerId.includes("/") && law.ownerKind === "windowAction") throw new Error(`[verify interactivity tool-jobs shared-action-fixture] ${law.id} lacks an exact window owner placement.`);
    if ((law.id.startsWith("exact-window-owner") || law.id === "exact-app-action-owner") && law.actionIds.slice().sort().join("\0") !== actualIds.join("\0"))
      throw new Error(`[verify interactivity tool-jobs shared-action-fixture] ${law.id} does not cover every accepted shared action id.`);
    for (const actionId of law.actionIds) {
      const result = law.ownerKind === "windowAction" || law.ownerKind === "appAction" ? dispositionById.get(actionId) ?? law.initial : law.initial;
      if (result !== law.expected) throw new Error(`[verify interactivity tool-jobs shared-action-fixture] descriptor law ${law.id} changed ${law.ownerKind} ${law.ownerId}/${actionId} to ${result}.`);
    }
    return `${law.id}:${law.ownerKind}:${law.ownerId}:${law.expected}`;
  });
  const requiredHostileIds = ["aba-generation", "cancel-after-transfer", "cancel-before-transfer", "empty", "exact-generation", "interrupted-close", "maximum", "maximum-plus-one", "one-semantic-unit", "repeated-terminal-close", "wrong-factory", "zero-close-credit"];
  if (fixture.hostileLaws.map((law) => law.id).sort().join("\0") !== requiredHostileIds.join("\0"))
    throw new Error("[verify interactivity tool-jobs shared-action-fixture] hostile law identities are not exact.");
  const hostile = fixture.hostileLaws.map((law) => {
    let result: SharedFrameworkActionRouteFixture["hostileLaws"][number]["expected"];
    if (law.kind === "rawAdmission") result = (law.value ?? -1) <= (law.maximum ?? -1) ? "accepted" : "rejected";
    else if (law.kind === "cancellation") result = "cancelled";
    else if (law.kind === "freshness") result = law.generation === law.liveGeneration ? "accepted" : "rejected";
    else if (law.kind === "identity") result = "rejected";
    else if (law.kind === "progress") result = (law.value ?? 0) > 0 && law.value === law.maximum ? "checkpoint" : "rejected";
    else result = law.maximumItems === 1 && (law.maximumBytes ?? 0) >= 4_096 ? "terminal" : "retained";
    if (result !== law.expected) throw new Error(`[verify interactivity tool-jobs shared-action-fixture] hostile law ${law.id} differs from its expected result.`);
    return `${law.id}:${result}`;
  });
  const shell = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
  const dispatch = interactivityCheckedRustBlockScope(shell, "pub fn dispatch_action<'a>(&'a mut self, mut action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>>", "shared-action accepted tutorial dispatch", sourceFailures)?.replace(/\s+/g, "") ?? "";
  const arming = dispatch.indexOf("letrecord_tutorial_after_acceptance=action.action==semio_framework::RECORD_TUTORIAL_ACTION_ID;");
  const accepted = dispatch.indexOf("letanswer=program.handle_action(session.instance_id,&action_json,&live_view_state).await;letmutresult=answer.map_err(|refusal|self.refused_guest_call(refusal,&session.app))?;", arming);
  const armed = dispatch.indexOf("ifrecord_tutorial_after_acceptance{self.tutorial_start_recording();}", accepted);
  if (sourceFailures.length || arming < 0 || accepted < arming || armed < accepted || (dispatch.match(/self\.tutorial_start_recording\(\);/g) ?? []).length !== 1)
    throw new Error("[verify interactivity tool-jobs shared-action-fixture] recordTutorial arms before the accepted retained route.");
  return { schema: fixture.schema, routes: fixture.routes.map((route) => `${route.id}:${route.factory}:${route.schemaId}:${route.descriptorDisposition}`), descriptor, hostile };
}

export function toolJobFixedOperationRustFixtureSource(root: string): string {
  toolJobFixedOperationFixtureRun(root);
  const fixture = JSON.parse(policyReadFileSafe(root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📇️fixed-operation-registry-law.json")) as FixedOperationFixture;
  const admission=fixture.caller.admission;
  const original=`RetainedCloneGrant{maximum_items:${admission.maximumItems},maximum_copy_bytes:${admission.maximumCopyBytes},maximum_capacity_bytes:${admission.maximumCapacityBytes},maximum_release_bytes:${admission.maximumReleaseBytes},maximum_depth:${admission.maximumDepth}}`;
  const lines = [
    "// 🧪️ Generated from 📇️fixed-operation-registry-law.json by the root verifier.",
    "#[test]",
    "fn language_neutral_fixed_operation_registry_laws_execute_production_registry() {",
    `    let source=${original};let mut caller=OriginalJobFixtureCaller::admit(OperationId(${fixture.caller.operation}),Generation(${fixture.caller.generation}),source,caller_clock,${fixture.caller.deadlineMicros},${fixture.caller.fuel});`,
  ];
  let identity = 100;
  for (const testCase of fixture.cases) {
    lines.push("    {", `        let mut registry = FixedOperationRegistry::<Owner, ${testCase.capacity}>::new(${testCase.maximumBytes});`, "        let mut output = Vec::new();");
    for (const step of testCase.steps) {
      if (step.action === "admit") {
        lines.push(`        fixture_admit(&mut caller,&mut registry, &mut output, OperationId(${step.operation}), Generation(${step.generation}), ${step.bytes}, ${identity}, ${JSON.stringify(step.owner)});`);
        identity += 1;
      } else if (step.action === "take") {
        lines.push(`        fixture_take(&mut caller,&mut registry, &mut output, OperationId(${step.operation}), Generation(${step.generation}));`);
      } else if (step.action === "cancel") {
        lines.push(`        fixture_cancel(&mut registry, &mut output, OperationId(${step.operation}), Generation(${step.generation}));`);
      } else if (step.action === "cancelStaleStep") {
        lines.push(`        fixture_cancel_stale(&mut registry, &mut output, OperationId(${step.operation}), Generation(${step.liveGeneration}));`);
      } else if (step.action === "close") {
        const grant = step.grant;
        lines.push(`        fixture_close(&mut caller,&mut registry, &mut output, RetainedCloneGrant { maximum_items: ${grant.maximumItems}, maximum_copy_bytes: ${grant.maximumCopyBytes}, maximum_capacity_bytes: ${grant.maximumCapacityBytes}, maximum_release_bytes: ${grant.maximumReleaseBytes}, maximum_depth: ${grant.maximumDepth} });`);
      } else if (step.action === "inspect") {
        lines.push("        fixture_inspect(&registry, &mut output);");
      }
    }
    lines.push(`        fixture_assert(${JSON.stringify(testCase.id)}, &mut registry, output, &[${testCase.expected.map((entry) => JSON.stringify(entry)).join(", ")}]);`, "    }");
  }
  lines.push("    caller.retire(source);","}", "");
  return lines.join("\n");
}

export function toolJobDrawingGestureOperationOwnerExact(drawingEditor: string, drawingConfig: string, drawingConfigProto: string, frameworkPlugin: string): boolean {
  const persisted = `${drawingConfig}\n${drawingConfigProto}`;
  const forbiddenPersisted = [
    /gesture_checkpoint_json/,
    /SetGestureCheckpoint/,
    /gesture_(?:operation|generation|session|checkpoint)/,
    /(?:Config|Draft|Snapshot)[\s\S]{0,400}?(?:gesture|pointer)[_A-Za-z0-9]*(?:session|checkpoint)/i,
  ];
  const requiredOwner = [
    "struct DrawingGestureOperationOwner",
    "FixedOperationRegistry<DrawingGestureOperationOwner, DRAWING_GESTURE_OPERATION_SLOTS>",
    "impl FixedOperationOwner for DrawingGestureOperationOwner",
    "struct DrawingInstanceOperationOwner",
    "struct DrawingGestureOperationJob",
    "type Job = DrawingGestureOperationJob",
    "type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>",
    "fn build_instance_operation_owner()",
    "DrawingInstanceOperationOwner::new()",
    "registry.register(DrawingGestureOperationJobFactory::new(&controller))",
    "instance_owner: request.instance_operation_owner",
    "fn preview_projection(&mut self, canonical_base_revision: [u8; 32], active_utility: &str)",
    "fn render_with_instance_operation_owner(",
    "owner.preview_projection(operation.canonical_base_revision, drawing_active_utility(view_state))",
    "DrawingGesturePreview",
    "DRAWING_GESTURE_PREVIEW_POINT_CAPACITY",
    "if observed_revision != base_revision",
    "DrawingRetainedCommandDecoder",
    "decoder.feed_page(page)",
    "UiFixedList<TracePointerWork, TRACE_POINTER_WORK_CAPACITY>",
    "struct DrawingTool",
    "ToolMachineRunner<canvas_tool::CanvasTool, DrawingToolHost>",
    "Emit::commit_transaction(transaction, mutations)",
    "query.cursor.advance(",
    "TracePointerJob::new_marquee",
    "DrawingQueryPublication",
    "query.publication_step()",
    "interaction_select_effect_from_targets",
    "session.trace_pointer",
    "session.point_query",
    "session.tool",
    "DrawingCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle",
    "if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id())",
    "Drawing gesture commands are reachable only through their exact retained factory owner",
    "if session.tool.at_rest() && session.trace_pointer.is_none()",
    "base_revision",
    "fn cancel(&mut self)",
    "fn terminal_is_empty(&self) -> bool",
  ];
  const frameworkOwner = [
    "fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner>",
    "instance_operation_owner: ArtifactInstanceOperationOwnerHandle",
    "A::render_with_request_context(&self.instance_operation_owner",
    "MountedWorkerJobSession::try_new",
  ];
  return (
    forbiddenPersisted.every((pattern) => !pattern.test(persisted)) &&
    requiredOwner.every((anchor) => drawingEditor.includes(anchor)) &&
    frameworkOwner.every((anchor) => frameworkPlugin.includes(anchor)) &&
    !drawingEditor.includes("static DRAWING_SESSIONS") &&
    !drawingEditor.includes("static ACTIVE_DRAW_SESSIONS") &&
    !drawingEditor.includes("checkpoint_from_config") &&
    !drawingEditor.includes("raw_bytes: Vec<u8>") &&
    !drawingEditor.includes("serde_json::from_slice") &&
    !drawingEditor.replaceAll("input.command.dispatch(&doc, &cfg, &mut session)", "").includes("command.dispatch(&doc") &&
    !drawingEditor.includes("owner.session.clone()") &&
    !drawingEditor.includes("flatten_drawing_layers(&doc.layers)") &&
    !drawingEditor.includes("fn resolve_point_pick") &&
    !drawingEditor.includes("fn marquee_layer_hits") &&
    !drawingEditor.includes("fn commit_draft(") &&
    !drawingEditor.includes("mod fsm") &&
    !drawingEditor.includes("Vec<TracePointerWork>") &&
    !drawingEditor.includes("drawing_gesture_preview_payload") &&
    !drawingEditor.includes("interaction_targets_json") &&
    !drawingEditor.includes("TRACE_POINTER_JOBS") &&
    !drawingEditor.includes("ACTIVE_TRACE_POINTER_JOBS")
  );
}
