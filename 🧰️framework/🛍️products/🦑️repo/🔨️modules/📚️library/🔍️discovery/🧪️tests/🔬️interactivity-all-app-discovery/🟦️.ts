import { INTERACTIVITY_ALL_APP_REQUIRED_GATES, interactivityAllAppDescriptorFromSource, interactivityAllAppOracleJson, INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY, interactivityAllAppActionDispositionFailures, interactivityAllAppActionProductionFailures, INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY, interactivityAllAppLaunchesFromSource, INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY, interactivityAllAppLaunchCoverageFailures, interactivityAllAppPlaygroundLaunchNames } from "../../../../../../../../📜️script.ts";

/** 🧪️Runs empty/single/max/max-plus-one, mutation, JSONC, and third-party-oracle discovery laws. */
export async function interactivityAllAppDiscoverySelfTests(): Promise<number> {
  const action = (id: string, disposition?: string) => ({ id, label: { native: { en: "Action", de: "Aktion" }, reuse: { en: "Action", de: "Aktion" } }, semantics: { execution: { ...(disposition ? { interactiveJob: disposition } : {}) } } });
  const app = (id: string, actions: unknown[] = []) => ({ id, label: { native: { en: "Editor", de: "Editor" }, reuse: { en: "Editor", de: "Editor" } }, windowKinds: actions.length > 0 ? [{ id: "main", actions }] : [] });
  const descriptor = (apps: unknown[]) => JSON.stringify({ descriptorVersion: 1, role: "plugin", manifest: { pluginId: "fixture", apps } });
  const gates = INTERACTIVITY_ALL_APP_REQUIRED_GATES.map((gate) => ({ ...gate, cwd: "${workspaceFolder}", presentation: { group: "4_gate" } }));
  const launch = (configurations: unknown[]) => JSON.stringify({ version: "0.2.0", configurations });
  const single = descriptor([app("s.fixture.fixture@1/*#editor")]);
  if ((interactivityAllAppDescriptorFromSource("single.json", single).failures.length) !== 0) throw new Error("[verify interactivity apps] single descriptor self-test was falsely rejected");
  if (JSON.stringify(JSON.parse(single)) !== await interactivityAllAppOracleJson(single)) throw new Error("[verify interactivity apps] owned descriptor parse disagrees with the TypeScript oracle");
  if (!interactivityAllAppDescriptorFromSource("empty.json", descriptor([])).failures.some((failure) => failure.includes("manifest.apps is empty"))) throw new Error("[verify interactivity apps] empty descriptor self-test was falsely accepted");
  const extensionSource = `const EXTENSION_ID: &str = "fixture-extension"; fn bundle() { ExtensionBundle::new(EXTENSION_ID, "Fixture", "1.0.0").extends("fixture"); }`;
  if (interactivityAllAppDescriptorFromSource("fixture/🧩️extensions/one/🔣️.json", descriptor([]), extensionSource).failures.length !== 0) throw new Error("[verify interactivity apps] parent-activated extension self-test was falsely rejected");
  const maximum = Array.from({ length: INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY }, (_, index) => app(`s.fixture.${index}@1/*#editor`));
  if (interactivityAllAppDescriptorFromSource("maximum.json", descriptor(maximum)).failures.length !== 0) throw new Error("[verify interactivity apps] maximum descriptor self-test was falsely rejected");
  if (!interactivityAllAppDescriptorFromSource("plus-one.json", descriptor([...maximum, app("s.fixture.plus-one@1/*#editor")])).failures.some((failure) => failure.includes("exceed fixed capacity"))) throw new Error("[verify interactivity apps] maximum-plus-one descriptor self-test was falsely accepted");
  const missingGerman = JSON.stringify({ descriptorVersion: 1, role: "plugin", manifest: { pluginId: "fixture", apps: [{ id: "fixture", label: { native: { en: "Editor" } } }] } });
  if (!interactivityAllAppDescriptorFromSource("missing-de.json", missingGerman).failures.some((failure) => failure.includes("lacks equivalent en/de labels"))) throw new Error("[verify interactivity apps] missing German label self-test was falsely accepted");
  const migratedAction = interactivityAllAppDescriptorFromSource("migrated-action.json", descriptor([app("s.fixture.fixture@1/*#editor", [action("run", "migrated")])])).row!;
  if (interactivityAllAppActionDispositionFailures([migratedAction]).length !== 0) throw new Error("[verify interactivity apps] migrated action self-test was falsely rejected");
  const ownedMigratedAction = { ...migratedAction, file: "✏️s/🔌️plugins/fixture/🔣️.json" };
  if (interactivityAllAppActionProductionFailures([ownedMigratedAction], [{ file: "✏️s/🔌️plugins/fixture/🦀️.rs", id: "run", source: "literal" }], []).length !== 0) throw new Error("[verify interactivity apps] owner-local accepted production action self-test was falsely rejected");
  if (!interactivityAllAppActionProductionFailures([ownedMigratedAction], [{ file: "✏️s/🔌️plugins/other/🦀️.rs", id: "run", source: "literal" }], []).some((failure) => failure.includes("without an accepted owner-local production command"))) throw new Error("[verify interactivity apps] wrong-owner production action self-test was falsely accepted");
  if (interactivityAllAppActionProductionFailures([ownedMigratedAction], [], ["run"]).length !== 0) throw new Error("[verify interactivity apps] accepted shared reserved action self-test was falsely rejected");
  const missingAction = interactivityAllAppDescriptorFromSource("missing-action.json", descriptor([app("s.fixture.fixture@1/*#editor", [action("run")])])).row!;
  if (!interactivityAllAppActionDispositionFailures([missingAction]).some((failure) => failure.includes("interactiveJob=\"missing\""))) throw new Error("[verify interactivity apps] missing action disposition self-test was falsely accepted");
  const actionMaximum = Array.from({ length: INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY }, (_, index) => action(`run-${index}`, "migrated"));
  if (!interactivityAllAppDescriptorFromSource("action-plus-one.json", descriptor([app("s.fixture.fixture@1/*#editor", [...actionMaximum, action("plus-one", "migrated")])])).failures.some((failure) => failure.includes("action rows exceeding fixed capacity"))) throw new Error("[verify interactivity apps] maximum-plus-one action self-test was falsely accepted");
  const validLaunch = launch([...gates, { name: "🛠️dev fixture", command: "bun ./📜️script.ts dev fixture", cwd: "${workspaceFolder}" }]);
  if (interactivityAllAppLaunchesFromSource(validLaunch).failures.length !== 0) throw new Error("[verify interactivity apps] valid launch self-test was falsely rejected");
  if (JSON.stringify(Bun.JSONC.parse(validLaunch)) !== await interactivityAllAppOracleJson(validLaunch)) throw new Error("[verify interactivity apps] owned launch parse disagrees with the TypeScript oracle");
  if (!interactivityAllAppLaunchesFromSource(launch([...gates.slice(1), { name: "🛠️dev fixture", command: "true", cwd: "${workspaceFolder}" }])).failures.some((failure) => failure.includes(INTERACTIVITY_ALL_APP_REQUIRED_GATES[0].name))) throw new Error("[verify interactivity apps] missing gate self-test was falsely accepted");
  const overCapacity = [...gates, ...Array.from({ length: INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY + 1 - gates.length }, (_, index) => ({ name: `fixture-${index}`, command: "true", cwd: "${workspaceFolder}" }))];
  if (!interactivityAllAppLaunchesFromSource(launch(overCapacity)).failures.some((failure) => failure.includes("exceed fixed capacity"))) throw new Error("[verify interactivity apps] maximum-plus-one launch self-test was falsely accepted");
  const fixtureDescriptor = { ...interactivityAllAppDescriptorFromSource("fixture.json", descriptor([app("s.fixture.fixture@1/*#editor")])).row!, file: "✏️s/🔌️plugins/fixture/🔣️.json" };
  const fixturePlayground = [{ variant: "fixture", pluginId: "fixture", appId: "s.fixture.fixture@1/*#editor" }];
  const fixtureSeed = JSON.stringify({ devLaunchers: { fixture: { namePrefix: "🧪️fixture" } } });
  const fixtureLaunches = [
    { name: "🛠️dev🧪️fixture⚛️react", command: "bun ./📜️script.ts dev fixture", cwd: "${workspaceFolder}", env: { SEMIO_RENDERER: "react" } },
    { name: "🛠️dev🧪️fixture🧊️wgpu🌐️wasm", command: "bun ./📜️script.ts dev fixture", cwd: "${workspaceFolder}", env: { SEMIO_RENDERER: "wgpu" } },
    { name: "🛠️dev🧪️fixture🧊️wgpu🖥️native", command: "bun ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native fixture", cwd: "${workspaceFolder}", env: {} },
  ];
  if (interactivityAllAppLaunchCoverageFailures([fixtureDescriptor], fixturePlayground, fixtureSeed, fixtureLaunches).length !== 0) throw new Error("[verify interactivity apps] complete owner-qualified launch self-test was falsely rejected");
  if (!interactivityAllAppLaunchCoverageFailures([fixtureDescriptor], fixturePlayground, fixtureSeed, fixtureLaunches.slice(0, 2)).some((failure) => failure.includes("WGPU native"))) throw new Error("[verify interactivity apps] missing native launch self-test was falsely accepted");
  if (!interactivityAllAppLaunchCoverageFailures([fixtureDescriptor], [{ ...fixturePlayground[0]!, pluginId: "other" }], fixtureSeed, fixtureLaunches).some((failure) => failure.includes("owner-qualified"))) throw new Error("[verify interactivity apps] wrong-owner launch self-test was falsely accepted");
  if (!interactivityAllAppLaunchCoverageFailures([fixtureDescriptor], fixturePlayground, fixtureSeed, fixtureLaunches.map((row, index) => index === 0 ? { ...row, command: "true" } : row)).some((failure) => failure.includes("owner-qualified"))) throw new Error("[verify interactivity apps] wrong-command launch self-test was falsely accepted");
  if (!interactivityAllAppLaunchCoverageFailures([fixtureDescriptor], fixturePlayground, fixtureSeed, fixtureLaunches.map((row, index) => index === 1 ? { ...row, env: { SEMIO_RENDERER: "react" } } : row)).some((failure) => failure.includes("owner-qualified"))) throw new Error("[verify interactivity apps] wrong-renderer launch self-test was falsely accepted");
  const viewerDescriptor = { ...fixtureDescriptor, appIds: ["s.fixture.fixture@1/*#viewer"] };
  if (interactivityAllAppLaunchCoverageFailures([viewerDescriptor], fixturePlayground, fixtureSeed, fixtureLaunches).length !== 0) throw new Error("[verify interactivity apps] shared dialect editor/viewer launch self-test was falsely rejected");
  const fixtureLaunchNames = interactivityAllAppPlaygroundLaunchNames(fixturePlayground, fixtureSeed);
  if (fixtureLaunchNames.size !== 3 || fixtureLaunches.some((row) => !fixtureLaunchNames.has(row.name))) throw new Error("[verify interactivity apps] generated playground launches were falsely classified as launch-only products");
  if (fixtureLaunchNames.has("🛠️dev🧪️launch-only-product")) throw new Error("[verify interactivity apps] launch-only product was falsely classified as a generated playground launch");
  return 25;
}
