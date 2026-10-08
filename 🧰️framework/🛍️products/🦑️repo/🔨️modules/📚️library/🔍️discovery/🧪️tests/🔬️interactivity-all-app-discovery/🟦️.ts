import { INTERACTIVITY_ALL_APP_REQUIRED_GATES, interactivityAllAppDescriptorFromSource, interactivityAllAppOracleJson, INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY, interactivityAllAppActionDispositionFailures, interactivityAllAppActionProductionFailures, INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY, interactivityAllAppGateFailures, interactivityAllAppPlaygroundsFromSource, interactivityAllAppRenderersFromSource, interactivityAllAppPlaygroundSurfaces, interactivityAllAppSurfaceCoverageFailures } from "../../../../../../../../📜️script.ts";

/** 🧪️Runs empty/single/max/max-plus-one, mutation, dashboard-declaration, and third-party-oracle discovery laws. */
export async function interactivityAllAppDiscoverySelfTests(): Promise<number> {
  let checks = 0;
  const expect = (condition: boolean, message: string): void => {
    checks += 1;
    if (!condition) throw new Error(`[verify interactivity apps] ${message}`);
  };
  const action = (id: string, disposition?: string) => ({ id, label: { native: { en: "Action", de: "Aktion" }, reuse: { en: "Action", de: "Aktion" } }, semantics: { execution: { ...(disposition ? { interactiveJob: disposition } : {}) } } });
  const app = (id: string, actions: unknown[] = []) => ({ id, label: { native: { en: "Editor", de: "Editor" }, reuse: { en: "Editor", de: "Editor" } }, windowKinds: actions.length > 0 ? [{ id: "main", actions }] : [] });
  const descriptor = (apps: unknown[]) => JSON.stringify({ descriptorVersion: 1, role: "plugin", manifest: { pluginId: "fixture", apps } });
  const single = descriptor([app("s.fixture.fixture@1/*#editor")]);
  expect(interactivityAllAppDescriptorFromSource("single.json", single).failures.length === 0, "single descriptor self-test was falsely rejected");
  expect(JSON.stringify(JSON.parse(single)) === await interactivityAllAppOracleJson(single), "owned descriptor parse disagrees with the TypeScript oracle");
  expect(interactivityAllAppDescriptorFromSource("empty.json", descriptor([])).failures.some((failure) => failure.includes("manifest.apps is empty")), "empty descriptor self-test was falsely accepted");
  const extensionDescriptor = (apps: unknown[]) => JSON.stringify({ descriptorVersion: 1, role: "extension", manifest: { pluginId: "fixture", apps } });
  const extensionSource = `const EXTENSION_ID: &str = "fixture-extension"; fn bundle() { ExtensionBundle::new(EXTENSION_ID, "Fixture", "1.0.0").extends("fixture"); }`;
  const constantBundle = interactivityAllAppDescriptorFromSource("fixture/🧩️extensions/one/🔣️.json", extensionDescriptor([]), extensionSource);
  expect(constantBundle.failures.length === 0, "parent-activated extension self-test was falsely rejected");
  expect(constantBundle.row?.pluginId === "fixture-extension", "extension bundle id was not resolved through its declared constant");
  expect(interactivityAllAppDescriptorFromSource("fixture/🧩️extensions/one/🔣️.json", descriptor([]), extensionSource).failures.some((failure) => failure.includes("role must be extension")), "plugin-role extension descriptor self-test was falsely accepted");
  const literalBundleSource = `const EXTENSION_ID: &str = "draw"; fn bundle() { ExtensionBundle::new("flow-extension-draw", "Draw", "0.1.0").extends("flow"); }`;
  const literalBundle = interactivityAllAppDescriptorFromSource("fixture/🧩️extensions/two/🔣️.json", extensionDescriptor([]), literalBundleSource);
  expect(literalBundle.failures.length === 0, "literal-bundle extension self-test was falsely rejected");
  expect(literalBundle.row?.pluginId === "flow-extension-draw", "extension bundle id was read from EXTENSION_ID instead of ExtensionBundle::new");
  expect(interactivityAllAppDescriptorFromSource("fixture/🧩️extensions/three/🔣️.json", extensionDescriptor([]), `fn bundle() { }`).failures.some((failure) => failure.includes("no resolvable ExtensionBundle::new")), "bundle-less extension self-test was falsely accepted");
  const maximum = Array.from({ length: INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY }, (_, index) => app(`s.fixture.${index}@1/*#editor`));
  expect(interactivityAllAppDescriptorFromSource("maximum.json", descriptor(maximum)).failures.length === 0, "maximum descriptor self-test was falsely rejected");
  expect(interactivityAllAppDescriptorFromSource("plus-one.json", descriptor([...maximum, app("s.fixture.plus-one@1/*#editor")])).failures.some((failure) => failure.includes("exceed fixed capacity")), "maximum-plus-one descriptor self-test was falsely accepted");
  const missingGerman = JSON.stringify({ descriptorVersion: 1, role: "plugin", manifest: { pluginId: "fixture", apps: [{ id: "fixture", label: { native: { en: "Editor" } } }] } });
  expect(interactivityAllAppDescriptorFromSource("missing-de.json", missingGerman).failures.some((failure) => failure.includes("lacks equivalent en/de labels")), "missing German label self-test was falsely accepted");
  const migratedAction = interactivityAllAppDescriptorFromSource("migrated-action.json", descriptor([app("s.fixture.fixture@1/*#editor", [action("run", "migrated")])])).row!;
  expect(interactivityAllAppActionDispositionFailures([migratedAction]).length === 0, "migrated action self-test was falsely rejected");
  const ownedMigratedAction = { ...migratedAction, file: "✏️s/🔌️plugins/fixture/🔣️.json" };
  expect(interactivityAllAppActionProductionFailures([ownedMigratedAction], [{ file: "✏️s/🔌️plugins/fixture/🦀️.rs", id: "run", source: "literal" }], []).length === 0, "owner-local accepted production action self-test was falsely rejected");
  expect(interactivityAllAppActionProductionFailures([ownedMigratedAction], [{ file: "✏️s/🔌️plugins/other/🦀️.rs", id: "run", source: "literal" }], []).some((failure) => failure.includes("without an accepted owner-local production command")), "wrong-owner production action self-test was falsely accepted");
  expect(interactivityAllAppActionProductionFailures([ownedMigratedAction], [], ["run"]).length === 0, "accepted shared reserved action self-test was falsely rejected");
  const missingAction = interactivityAllAppDescriptorFromSource("missing-action.json", descriptor([app("s.fixture.fixture@1/*#editor", [action("run")])])).row!;
  expect(interactivityAllAppActionDispositionFailures([missingAction]).some((failure) => failure.includes("interactiveJob=\"missing\"")), "missing action disposition self-test was falsely accepted");
  const actionMaximum = Array.from({ length: INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY }, (_, index) => action(`run-${index}`, "migrated"));
  expect(interactivityAllAppDescriptorFromSource("action-plus-one.json", descriptor([app("s.fixture.fixture@1/*#editor", [...actionMaximum, action("plus-one", "migrated")])])).failures.some((failure) => failure.includes("action rows exceeding fixed capacity")), "maximum-plus-one action self-test was falsely accepted");

  const schema = JSON.stringify({ $defs: { Parameter: { properties: { kind: { enum: ["choice", "text", "flag"] }, valuePositional: { const: true } } } } });
  const verifyTarget = (target: Record<string, unknown>) => JSON.stringify({ name: "workspace", targets: { verify: target } });
  const declaredVerify = { options: { command: "bun ./📜️script.ts verify", forwardAllArgs: true }, metadata: { semio: { dashboard: { parameters: [{ id: "check", kind: "text", valuePositional: true }] } } } };
  const declared = verifyTarget(declaredVerify);
  expect(interactivityAllAppGateFailures(declared, schema).length === 0, "declared dashboard verify command was falsely rejected");
  expect(JSON.stringify(JSON.parse(declared)) === await interactivityAllAppOracleJson(declared), "owned manifest parse disagrees with the TypeScript oracle");
  expect(interactivityAllAppGateFailures(JSON.stringify({ name: "workspace", targets: {} }), schema).some((failure) => failure.includes("workspace:verify is missing")), "undeclared verify command self-test was falsely accepted");
  expect(interactivityAllAppGateFailures(verifyTarget({ ...declaredVerify, options: { command: "bun ./📜️script.ts verify" } }), schema).some((failure) => failure.includes("forward free arguments")), "non-forwarding verify command self-test was falsely accepted");
  expect(interactivityAllAppGateFailures(verifyTarget({ ...declaredVerify, metadata: {} }), schema).some((failure) => failure.includes("positional text parameter")), "parameterless verify command self-test was falsely accepted");
  expect(interactivityAllAppGateFailures(declared, JSON.stringify({ $defs: {} })).some((failure) => failure.includes("declares no text kind")), "schema without text parameters self-test was falsely accepted");
  expect(interactivityAllAppGateFailures("{", schema).some((failure) => failure.includes("invalid JSON")), "invalid manifest self-test was falsely accepted");
  const firstGate = INTERACTIVITY_ALL_APP_REQUIRED_GATES[0]!;
  expect(INTERACTIVITY_ALL_APP_REQUIRED_GATES.length > 0 && interactivityAllAppGateFailures(declared, schema, INTERACTIVITY_ALL_APP_REQUIRED_GATES).length === 0, "required gates were falsely rejected");
  expect(interactivityAllAppGateFailures(declared, schema, [firstGate, firstGate]).some((failure) => failure.includes("duplicate check")), "duplicate gate self-test was falsely accepted");
  expect(interactivityAllAppGateFailures(declared, schema, [{ id: "empty", check: [] }]).some((failure) => failure.includes("non-empty words")), "empty gate self-test was falsely accepted");

  const axisSource = `pub const PLAYGROUND_SOURCE: &str = "x";\nconst RENDERERS: &[&str] = &["react", "wgpu-wasm", "wgpu-native"];`;
  const axis = interactivityAllAppRenderersFromSource(axisSource);
  expect(axis.failures.length === 0 && axis.renderers.join(",") === "react,wgpu-wasm,wgpu-native", "renderer axis was not read from the registry source");
  expect(interactivityAllAppRenderersFromSource(`const RENDERERS: &[&str] = &["react", "wgpu-wasm"];`).failures.some((failure) => failure.includes("exactly one React")), "renderer axis without a native surface self-test was falsely accepted");
  expect(interactivityAllAppRenderersFromSource("fn nothing() {}").failures.some((failure) => failure.includes("not readable")), "missing renderer axis self-test was falsely accepted");
  const catalog = (rows: unknown[]) => JSON.stringify(rows);
  const fixtureRow = { variant: "fixture", pluginId: "fixture", app: "s.fixture.fixture@1/*#editor", ports: { react: 6001, wgpu: 6101 } };
  const catalogSource = catalog([fixtureRow]);
  const fixturePlaygrounds = interactivityAllAppPlaygroundsFromSource(catalogSource);
  expect(fixturePlaygrounds.failures.length === 0 && fixturePlaygrounds.rows.length === 1 && fixturePlaygrounds.rows[0]!.wgpu === 6101, "playground catalog was falsely rejected");
  const oracleCatalog = JSON.stringify({ playgrounds: JSON.parse(catalogSource) });
  expect(JSON.stringify(JSON.parse(oracleCatalog).playgrounds) === JSON.stringify(JSON.parse(await interactivityAllAppOracleJson(oracleCatalog)).playgrounds) && fixturePlaygrounds.rows[0]!.react === JSON.parse(await interactivityAllAppOracleJson(oracleCatalog)).playgrounds[0].ports.react, "owned playground catalog parse disagrees with the TypeScript oracle");
  expect(interactivityAllAppPlaygroundsFromSource(catalog([fixtureRow, fixtureRow])).failures.some((failure) => failure.includes("duplicate variant")), "duplicate playground variant self-test was falsely accepted");
  expect(interactivityAllAppPlaygroundsFromSource(catalog([{ variant: "" }])).failures.some((failure) => failure.includes("malformed playground row")), "malformed playground row self-test was falsely accepted");
  expect(interactivityAllAppPlaygroundsFromSource("[]").failures.some((failure) => failure.includes("no playground rows")), "empty playground catalog self-test was falsely accepted");
  expect(interactivityAllAppPlaygroundsFromSource("{").failures.some((failure) => failure.includes("invalid JSON")), "invalid playground catalog self-test was falsely accepted");

  const fixtureDescriptor = { ...interactivityAllAppDescriptorFromSource("fixture.json", descriptor([app("s.fixture.fixture@1/*#editor")])).row!, file: "✏️s/🔌️plugins/fixture/🔣️.json" };
  const playground = fixturePlaygrounds.rows[0]!;
  expect(interactivityAllAppPlaygroundSurfaces(playground, axis.renderers).length === 3, "playground with both ports does not offer every renderer");
  expect(interactivityAllAppPlaygroundSurfaces({ ...playground, react: 0 }, axis.renderers).join(",") === "wgpu-wasm,wgpu-native", "playground without a React port still offers React");
  expect(interactivityAllAppSurfaceCoverageFailures([fixtureDescriptor], [playground], axis.renderers).length === 0, "complete owner-qualified surface self-test was falsely rejected");
  expect(interactivityAllAppSurfaceCoverageFailures([fixtureDescriptor], [{ ...playground, wgpu: 0 }], axis.renderers).some((failure) => failure.includes("WGPU native")), "missing WGPU surface self-test was falsely accepted");
  expect(interactivityAllAppSurfaceCoverageFailures([fixtureDescriptor], [{ ...playground, react: 0 }], axis.renderers).some((failure) => failure.includes("owner-qualified")), "missing React surface self-test was falsely accepted");
  expect(interactivityAllAppSurfaceCoverageFailures([fixtureDescriptor], [{ ...playground, pluginId: "other" }], axis.renderers).some((failure) => failure.includes("owner-qualified")), "wrong-owner surface self-test was falsely accepted");
  expect(interactivityAllAppSurfaceCoverageFailures([fixtureDescriptor], [{ ...playground, appId: "s.other.other@1/*#editor" }], axis.renderers).some((failure) => failure.includes("owner-qualified")), "wrong-app surface self-test was falsely accepted");
  const viewerDescriptor = { ...fixtureDescriptor, appIds: ["s.fixture.fixture@1/*#viewer"] };
  expect(interactivityAllAppSurfaceCoverageFailures([viewerDescriptor], [playground], axis.renderers).length === 0, "shared dialect editor/viewer surface self-test was falsely rejected");
  return checks;
}
