#!/usr/bin/env bun
/** 🧩️ `@semio-tech/puzzle-js` router: `bun ./📜️script.ts test`. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "vitest.config.ts");
  }
}

/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type PublicationGroup = {
  status: "migrated" | "batch-only-pending-rewrite";
  lanes: ("artifact" | "config" | "draft" | "presence" | "transient" | "window-config" | "window-transient" | "child" | "interaction" | "host-only")[];
  routes: string[];
  blocker?: string;
};

type PublicationOwner = { owner: "Puzzle2dPlayApp" | "Puzzle3dPlayApp" | "Puzzle5dPlayApp"; source: string; groups: PublicationGroup[] };
type PublicationFixture = { schema: string; closePageBytes: number; owners: PublicationOwner[]; laws: Record<string, boolean> };

const reserved5d = new Set(["copy", "cut", "paste", "import-media"]);
const reserved2d = new Set(["import-media"]);

function quotedValues(source: string): string[] {
  return [...source.matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
}

function retainedIds(source: string, owner: PublicationOwner["owner"]): string[] {
  const dimension = owner.slice(6, 8).toUpperCase();
  const match = source.match(new RegExp(`PUZZLE${dimension}_RETAINED_TOOL_IDS: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`));
  if (!match) throw new Error(`${owner} retained id declaration is missing`);
  return quotedValues(match[1]!);
}

function manifestPairs(source: string): Map<string, string> {
  return new Map([...source.matchAll(/\.action_interactive_job\("([^"]+)",\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => [match[1]!, match[2]!]));
}

function exactArray(left: string[], right: string[]): boolean {
  return JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;
}

function publicationContracts(source: string): Map<string, string[]> {
  return new Map([...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((match) => [
    match[1]!,
    [...match[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|WindowConfig|WindowTransient|Child|Interaction|HostOnly)/g)].map((lane) => lane[1]!),
  ]));
}

function exactContracts(actual: Map<string, string[]>, groups: PublicationGroup[]): boolean {
  const expected = new Map(groups.flatMap((group) => group.routes.map((route) => [route, group.lanes.map(variant)] as const)));
  return exactArray([...actual.keys()], [...expected.keys()])
    && [...expected].every(([route, lanes]) => exactArray(actual.get(route) ?? [], lanes));
}

function fixtureOracle(fixture: PublicationFixture): boolean {
  if (fixture.schema !== "semio.puzzle.publication-authority.v1" || fixture.closePageBytes !== 16_384 || fixture.owners.length !== 3) return false;
  if (!Object.values(fixture.laws).every(Boolean)) return false;
  const owners = fixture.owners.map(({ owner }) => owner);
  if (!exactArray(owners, ["Puzzle2dPlayApp", "Puzzle3dPlayApp", "Puzzle5dPlayApp"])) return false;
  return fixture.owners.every(({ groups }) => {
    const routes = groups.flatMap((group) => group.routes);
    return routes.length > 0 && new Set(routes).size === routes.length && groups.every((group) =>
      group.routes.length > 0
      && group.lanes.length > 0
      && new Set(group.lanes).size === group.lanes.length
      && (group.status === "migrated" ? group.blocker === undefined : Boolean(group.blocker))
      && (!group.lanes.includes("host-only") || group.lanes.length === 1),
    );
  });
}

type WindowOwnershipCase = { definition: string; value: Record<string, unknown>; keys: string[] };

async function validateWindowOwnershipSchemas(puzzleRoot: string): Promise<number> {
  const projection = {
    kind: "threePoint",
    orthographicView: "top",
    axonometricVariant: "isometric",
    axonometricAngleA: 15,
    axonometricAngleB: 12,
    axonometricQuadrant: "ne",
    obliqueVariant: "cavalier",
    obliqueAngle: 45,
    obliqueDepth: 1,
    onePointAxis: "y",
    fov: 50,
    twoPointShift: 0,
    curvilinearFov: 120,
    curvilinearStrength: 1,
    curvilinearMapping: "fisheye",
  };
  const sun = { enabled: false, azimuth: 45, elevation: 35, intensity: 0.85, color: "#ffffff" };
  const schemas: { path: string; cases: WindowOwnershipCase[] }[] = [
    {
      path: "🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧬️schema/🔣️.json",
      cases: [
        { definition: "Puzzle2dWindowConfig", value: { cameraX: 0, cameraY: 0, cameraZoom: 1, lodMode: "automatic", fillCount: 0, gridSnapEnabled: false, gridFactor: 1, suggestionOffset: 80 }, keys: ["cameraX", "cameraY", "cameraZoom", "lodMode", "fillCount", "gridSnapEnabled", "gridFactor", "suggestionOffset"] },
        { definition: "Puzzle2dWindowTransient", value: { engagementInput: "", brushCandidateIndex: 0, brushCandidates: [], brushCandidateSourceHandleId: "" }, keys: ["engagementInput", "brushCandidateIndex", "brushCandidates", "brushCandidateSourceHandleId"] },
      ],
    },
    {
      path: "🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧬️schema/🔣️.json",
      cases: [
        {
          definition: "Puzzle3dWindowConfig",
          value: { lodAutomatic: true, lodDepthVariable: false, gridVisible: true, lodManual: 100, gridSnapEnabled: false, gridSpacing: 10, selectableKinds: { objects: true, vortices: true, attractions: true }, proximityRadius: 0.75, chunkSize: 256, voxelDims: [1, 1, 1], transformMove: true, transformRotate: true, vortexShow: "selected", vortexDirection: "outwards", selectionMethod: "pick", sun, camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1, up: null, projection } },
          keys: ["lodAutomatic", "lodDepthVariable", "gridVisible", "lodManual", "gridSnapEnabled", "gridSpacing", "selectableKinds", "proximityRadius", "chunkSize", "voxelDims", "transformMove", "transformRotate", "vortexShow", "vortexDirection", "selectionMethod", "sun", "camera"],
        },
        { definition: "Puzzle3dWindowTransient", value: { suggestionMenu: null, engagementInput: "", brushCandidateIndex: 0, activation: "" }, keys: ["suggestionMenu", "engagementInput", "brushCandidateIndex", "activation"] },
      ],
    },
    {
      path: "🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧬️schema/🔣️.json",
      cases: [
        { definition: "Puzzle5dBoardWindowConfig", value: { camera2d: { x: 0, y: 0, zoom: 1 }, fillCount: 0, lodMode: "automatic", suggestionOffset: 80, gridSnapEnabled: true, gridFactor: 1 }, keys: ["camera2d", "fillCount", "lodMode", "suggestionOffset", "gridSnapEnabled", "gridFactor"] },
        { definition: "Puzzle5dWorldWindowConfig", value: { camera3d: { position: [8, -8, 8], target: [0, 0, 0], zoom: 1 }, sun }, keys: ["camera3d", "sun"] },
        { definition: "Puzzle5dWindowTransient", value: { engagementInput: "", brushCandidateIndex: 0 }, keys: ["engagementInput", "brushCandidateIndex"] },
      ],
    },
  ];
  let count = 0;
  for (const entry of schemas) {
    const schema = await Bun.file(resolve(puzzleRoot, entry.path)).json() as { $id: string };
    const ajv = new Ajv({ allErrors: true, strict: true });
    ajv.addKeyword({ keyword: "x-semio-state", metaSchema: { type: "string" } });
    ajv.addSchema(schema);
    for (const testCase of entry.cases) {
      const validate = ajv.getSchema(`${schema.$id}#/definitions/${testCase.definition}`);
      if (!validate || !validate(testCase.value)) throw new Error(`${testCase.definition} neutral fixture failed Ajv validation: ${JSON.stringify(validate?.errors)}`);
      const actualKeys = Object.keys(testCase.value).sort();
      if (!exactArray(actualKeys, testCase.keys)) throw new Error(`${testCase.definition} failed the independent exact-record oracle`);
      if (validate({ ...testCase.value, appConfigLeak: true })) throw new Error(`${testCase.definition} accepted an app-config ownership leak`);
      count += 1;
    }
  }
  return count;
}

/** @emoji 🧩️ Whether `production` implements `traitName` for `typeName`, regardless of how the trait
 * path happens to be spelled. Repo-wide import-normalisation sweeps rewrite `semio_framework::Foo`
 * to a bare `Foo` (and back) as a formatting concern; asserting one exact spelling makes this oracle
 * fail on import style rather than on real publication-authority divergence. */
function implementsFor(production: string, traitName: string, typeName: string): boolean {
  return new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*${traitName} for ${typeName}\\b`).test(production);
}

/** @emoji 🌍️ Locale and terminology are OS-owned state projected through
 * `semio_framework_plugin::ViewModel`, never artifact-local settings: no puzzle owner may publish a
 * `setLocale`/`setTerminology` route nor carry a `SetLocale`/`SetTerminology` config mutation. The
 * editors read the locale out of the projected view state (`puzzle2d_config_locale(view_state)`), so
 * an app-local copy would be a second, divergent authority over the same user setting. */
function localeIsOsOwned(production: string): boolean {
  return !/"setLocale"|"setTerminology"|ConfigMutation::Set(?:Locale|Terminology)/.test(production);
}

/** 🗡️ Every hostile source mutation `Puzzle3dPlayApp`'s publication authority must refuse, keyed by
 * the invariant it attacks. The stale-authority and bounded-progress entries use `replaceAll` because
 * both guards are duplicated verbatim across the Config and the Artifact preparation; a
 * single-occurrence replace would leave the other copy intact and the oracle's `.includes` clause
 * would still see the pattern in the mutated source. */
function puzzle3dHostileSources(source: string): Map<string, string> {
  return new Map([
    ["missing Config Store preparation", source.replace("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))", "None")],
    ["missing Artifact Store preparation", source.replace("Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))", "None")],
    ["widened Config mutation envelope", source.replace(
      "    (encoded <= PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded)\n",
      "    Some(0)\n",
    )],
    ["stale publication authority", source.replaceAll("            || request.generation != request.authority.generation()\n", "")],
    ["unbounded preparation progress", source.replaceAll("ArtifactStoreOneItemPreparationStep::Progress", "ArtifactStoreOneItemPreparationStep::Prepared")],
  ]);
}

function ownerOracle(owner: PublicationOwner, source: string): boolean {
  const production = source.split("//#region 🧪️UnitTests")[0]!;
  const pairs = manifestPairs(production);
  const appGroups = owner.groups.map((group) => ({ ...group, routes: group.routes.filter((route) => (owner.owner !== "Puzzle5dPlayApp" || !reserved5d.has(route)) && (owner.owner !== "Puzzle2dPlayApp" || !reserved2d.has(route))) }));
  const appRoutes = appGroups.flatMap((group) => group.routes);
  const migrated = appGroups.filter((group) => group.status === "migrated").flatMap((group) => group.routes);
  const expectedPairs = new Map(appGroups.flatMap((group) => group.routes.map((route) => [route, variant(group.status)])));
  if (!exactArray([...pairs.keys()], appRoutes)) return false;
  if (!appRoutes.every((route) => pairs.get(route) === expectedPairs.get(route))) return false;
  if (!exactArray(retainedIds(production, owner.owner), migrated)) return false;
  const factory = `${owner.owner.slice(0, 8)}RetainedCommandJobFactory`;
  const factoryBlock = production.split(new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for ${factory}\\b`))[1]?.split("//#endregion 🧵️RetainedCommands")[0] ?? "";
  const contracts = publicationContracts(factoryBlock);
  const proofBlock = production.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0] ?? "";
  const proofIds = quotedValues(proofBlock.match(/tools:\s*\[([^\]]*)\]/)?.[1] ?? "");
  const exactFactory = implementsFor(production, "ToolJobFactory", factory)
    && implementsFor(production, "ArtifactOwnedToolJobFactory", factory)
    && proofBlock.includes(`factory: "${factory}"`)
    && proofBlock.includes(`factory_type: ${factory}`)
    && new RegExp(`type Owner = (?:[A-Za-z_][A-Za-z0-9_]*::)*EditorApp<${owner.owner}>;`).test(production)
    && production.includes(owner.owner === "Puzzle5dPlayApp"
      ? `registry.register(${factory}::new(&controller_id))`
      : `registry.register(${factory}::new(&controller))`)
    && production.includes("fn build_artifact_store_one_item_preparation_factory()")
    && production.includes("fn build_config_store_one_item_preparation_factory()")
    && !production.includes("build_draft_store_one_item_preparation_factory")
    && !production.includes("build_presence_store_one_item_preparation_factory")
    && !production.includes("build_transient_store_one_item_preparation_factory")
    && localeIsOsOwned(production);
  if (!exactFactory || !exactContracts(contracts, appGroups.filter((group) => group.status === "migrated")) || !exactArray(proofIds, migrated)) return false;
  if (owner.owner === "Puzzle3dPlayApp") {
    return production.includes("struct Puzzle3dConfigStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation")
      && production.includes("fn build_config_store_one_item_preparation_factory()")
      && production.includes("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))")
      && production.includes("PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 32_768")
      // 📏️ Every `Puzzle3dConfigMutation` variant is admissible, at its own encoded payload size, under
      // the one fixed Config-store envelope. The former allowlist (`Snapshot` only, `_ => None`) made the
      // entire `Puzzle3dScalarConfigWork` family publish nothing — 📓️2026-09-09-wave-D §P3.
      && production.includes("fn puzzle3d_config_store_mutation_bytes(mutation: &Puzzle3dConfigMutation) -> Option<usize> {")
      && production.includes("(encoded <= PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded)")
      && !production.includes("_ => None,\n    }\n}\n\nfn puzzle3d_config_store_edit(")
      && production.includes('return Err("Puzzle3d Config preparation rejected its exact mutation envelope".into());')
      && production.includes("request.operation != request.authority.operation()")
      && production.includes("request.generation != request.authority.generation()")
      && production.includes("request.base_revision != request.authority.base_revision()")
      && production.includes("ArtifactStoreOneItemPreparationStep::Progress")
      && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1")
      && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2")
      && production.includes("fn cancel(&mut self)")
      && production.includes("fn begin_close(&mut self)")
      && production.includes("base.return_to_registry()")
      && production.includes("fn terminal_is_empty(&self)")
      && production.includes("struct Puzzle3dArtifactStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation")
      && production.includes("fn build_artifact_store_one_item_preparation_factory()")
      && production.includes("Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))");
  }
  if (owner.owner === "Puzzle2dPlayApp") {
    return production.includes("struct Puzzle2dConfigStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparation")
      && production.includes("Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))")
      && production.includes("struct Puzzle2dArtifactStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparation")
      && production.includes("Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))")
      && production.includes("PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536")
      && production.includes("request.operation != request.authority.operation()")
      && production.includes("request.generation != request.authority.generation()")
      && production.includes("request.base_revision != request.authority.base_revision()")
      && production.includes("ArtifactStoreOneItemPreparationStep::Progress")
      && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1")
      && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2")
      && production.includes("fn cancel(&mut self)")
      && production.includes("fn begin_close(&mut self)")
      && production.includes("base.return_to_registry()")
      && production.includes("fn terminal_is_empty(&self)")
      // 🎬️ The one dispatch pipeline `handle` and every generic retained reduce share — a second,
      // divergent copy of the scene/host/delta body is exactly what this audit exists to refuse.
      && production.includes("fn puzzle2d_dispatch_emit(")
      && production.includes("puzzle2d_dispatch_emit(command, &snapshot.0, config, &window_config, &window_transient, window_kind, self.view_state.as_ref(), puzzle2d_active_utility(self.view_state.as_ref()), &selection, None)?")
      && production.includes("puzzle2d_dispatch_emit(command, &doc.snapshot.0, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), doc.operation_optional().cloned())")
      && production.includes("PUZZLE2D_SELECTION_BATCH_LIMIT: usize = 1_024")
      && production.includes("(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))");
  }
  if (owner.owner !== "Puzzle5dPlayApp") return true;
  const guard = production.indexOf('if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str())');
  const decode = production.indexOf("puzzle5d_preflight_reserved_wire", guard);
  return [
    'puzzle5d_reserved_factory!(Puzzle5dCopyJobFactory, "copy", "puzzle.5d.reserved.copy.v1")',
    'puzzle5d_reserved_factory!(Puzzle5dCutJobFactory, "cut", "puzzle.5d.reserved.cut.v1")',
    'puzzle5d_reserved_factory!(Puzzle5dPasteJobFactory, "paste", "puzzle.5d.reserved.paste.v1")',
    'puzzle5d_reserved_factory!(Puzzle5dImportJobFactory, "import-media", "puzzle.5d.reserved.import-media.v1")',
  ].every((anchor) => production.includes(anchor))
    && ["Puzzle5dCopyJobFactory", "Puzzle5dCutJobFactory", "Puzzle5dPasteJobFactory", "Puzzle5dImportJobFactory"].every((factory) => production.includes(`registry.register(${factory}::new(&controller_id))`))
    && production.includes("impl ArtifactOwnedToolJobFactory for $factory")
    && production.includes("type Owner = EditorApp<Puzzle5dPlayApp>;")
    && production.includes("fn build_artifact_store_one_item_preparation_factory()")
    && production.includes("Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))")
    && production.includes("fn build_config_store_one_item_preparation_factory()")
    && production.includes("Some(std::sync::Arc::new(Puzzle5dConfigStorePreparationFactory))")
    && production.includes('tool_id: "copy", lanes: &[ArtifactToolPublicationLane::HostOnly]')
    && ["cut", "paste", "import-media"].every((route) => production.includes(`tool_id: "${route}", lanes: &[ArtifactToolPublicationLane::Artifact]`))
    && guard >= 0 && decode > guard;
}

class PublicationAuthorityAuditScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const onlyOwner = segments[0];
    const puzzleRoot = resolve(this.root, "../..");
    const windowOwnershipCases = await validateWindowOwnershipSchemas(puzzleRoot);
    const fixture = await Bun.file(resolve(puzzleRoot, "🧫️fixtures/🔏️publication-authority/🔣️.json")).json() as PublicationFixture;
    const module = await Bun.file(resolve(puzzleRoot, "🧬️schema/🔣️.json")).json() as { $id: string };
    const ajv = new Ajv({ allErrors: true, strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    ajv.addSchema(module);
    const validate = ajv.compile({ $ref: `${module.$id}#/$defs/PuzzlePublicationAuthority` });
    if (!validate(fixture)) throw new Error(`Puzzle publication fixture failed Ajv validation: ${JSON.stringify(validate.errors)}`);
    if (!fixtureOracle(fixture)) throw new Error("Puzzle publication fixture failed the independent semantic oracle");
    const auditedOwners = onlyOwner ? fixture.owners.filter((owner) => owner.owner === onlyOwner) : fixture.owners;
    if (onlyOwner && auditedOwners.length === 0) {
      throw new Error(`unknown owner ${JSON.stringify(onlyOwner)}; expected one of ${fixture.owners.map((owner) => owner.owner).join(", ")}`);
    }
    for (const owner of auditedOwners) {
      const source = await Bun.file(resolve(puzzleRoot, owner.source)).text();
      if (!ownerOracle(owner, source)) throw new Error(`${owner.owner} publication authority diverged from the fixture`);
      const blocked = owner.groups.find((group) => group.status === "batch-only-pending-rewrite")?.routes[0];
      if (blocked) {
        const hostile = source.replace(
          new RegExp(`(\\.action_interactive_job\\("${blocked}",\\s*(?:semio_framework_plugin::)?InteractiveJobClassification::)BatchOnlyPendingRewrite`),
          "$1Migrated",
        );
        if (hostile === source) throw new Error(`${owner.owner} hostile activation mutation did not apply for ${blocked}`);
        if (ownerOracle(owner, hostile)) throw new Error(`${owner.owner} accepted hostile activation before decode/preparation for ${blocked}`);
      }
      const missingContract = source.replace(/\s*ArtifactToolPublicationContract \{ tool_id: "(?:openAddObjectDialog|setCamera|canvasPointerDown)", lanes: &\[ArtifactToolPublicationLane::(?:HostOnly|WindowConfig)\] \},/, "");
      if (missingContract !== source && ownerOracle(owner, missingContract)) throw new Error(`${owner.owner} accepted a missing publication contract`);
      if (owner.owner === "Puzzle3dPlayApp") {
        for (const [invariant, hostile] of puzzle3dHostileSources(source)) {
          if (hostile === source) throw new Error(`Puzzle3d hostile mutation for ${invariant} did not apply`);
          if (ownerOracle(owner, hostile)) throw new Error(`Puzzle3d accepted a hostile mutation for ${invariant}`);
        }
      }
      if (owner.owner === "Puzzle5dPlayApp") {
        const missingReserved = source.replace('puzzle5d_reserved_factory!(Puzzle5dCutJobFactory, "cut", "puzzle.5d.reserved.cut.v1");', "");
        const missingPreparation = source.replace("Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))", "None");
        const decodeBeforeAuthority = source.replace('        if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str()) {\n            return Ok(None);\n        }\n', "");
        if (ownerOracle(owner, missingReserved) || ownerOracle(owner, missingPreparation) || ownerOracle(owner, decodeBeforeAuthority)) {
          throw new Error("Puzzle5d accepted a missing reserved factory, missing Store preparation, or decode before route authority");
        }
      }
    }
    const hostileFixtures: PublicationFixture[] = [
      { ...fixture, closePageBytes: 32_768 },
      { ...fixture, owners: fixture.owners.slice(1) },
      // 🧯️ A `Migrated` group must never carry a blocker — inject one unconditionally so this stays a
      // real hostile mutation for an owner whose first group is already `Migrated` (and therefore has none).
      { ...fixture, owners: fixture.owners.map((owner, index) => index === 0 ? { ...owner, groups: [{ ...owner.groups[0]!, status: "migrated", blocker: owner.groups[0]!.blocker ?? "hostile: a migrated group must carry no blocker" }] } : owner) },
    ];
    if (hostileFixtures.some((hostile) => Boolean(validate(hostile)) || fixtureOracle(hostile))) throw new Error("Puzzle publication fixture accepted a hostile schema/oracle mutation");
    const admitted = auditedOwners.flatMap((owner) => owner.groups.filter((group) => group.status === "migrated").flatMap((group) => group.routes));
    console.error(`validated Puzzle publication authority; owners=${auditedOwners.map((owner) => owner.owner).join(",")}; admitted=${admitted.join(",")}; windowOwnershipCases=${windowOwnershipCases}; schema=Ajv; oracle=independent`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", PublicationAuthorityAuditScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
