#!/usr/bin/env bun
/** 🧩️ `@semio-tech/puzzle-js` router: `bun ./📜️script.ts test`. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

type PublicationGroup = {
  status: "Migrated" | "BatchOnlyPendingRewrite";
  lanes: ("Artifact" | "Config" | "Draft" | "Presence" | "Transient" | "Child" | "HostOnly")[];
  routes: string[];
  blocker?: string;
};

type PublicationOwner = { owner: "Puzzle2dPlayApp" | "Puzzle3dPlayApp" | "Puzzle5dPlayApp"; source: string; groups: PublicationGroup[] };
type PublicationFixture = { schema: string; closePageBytes: number; owners: PublicationOwner[]; laws: Record<string, boolean> };

const reserved5d = new Set(["copy", "cut", "paste", "import-media"]);

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
  return new Map([...source.matchAll(/\.action_interactive_job\((?:"([^"]+)"|set_fill_count::STEP_ACTION_ID),\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => [match[1] ?? "setFillCountStep", match[2]!]));
}

function exactArray(left: string[], right: string[]): boolean {
  return JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;
}

function publicationContracts(source: string): Map<string, string[]> {
  return new Map([...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((match) => [
    match[1]!,
    [...match[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((lane) => lane[1]!),
  ]));
}

function exactContracts(actual: Map<string, string[]>, groups: PublicationGroup[]): boolean {
  const expected = new Map(groups.flatMap((group) => group.routes.map((route) => [route, group.lanes] as const)));
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
      && (group.status === "Migrated" ? group.blocker === undefined : Boolean(group.blocker))
      && (!group.lanes.includes("HostOnly") || group.lanes.length === 1),
    );
  });
}

function ownerOracle(owner: PublicationOwner, source: string): boolean {
  const production = source.split("//#region 🧪️Testkit")[0]!;
  const pairs = manifestPairs(production);
  const appGroups = owner.groups.map((group) => ({ ...group, routes: group.routes.filter((route) => owner.owner !== "Puzzle5dPlayApp" || !reserved5d.has(route)) }));
  const appRoutes = appGroups.flatMap((group) => group.routes);
  const migrated = appGroups.filter((group) => group.status === "Migrated").flatMap((group) => group.routes);
  const expectedPairs = new Map(appGroups.flatMap((group) => group.routes.map((route) => [route, group.status])));
  if (!exactArray([...pairs.keys()], appRoutes)) return false;
  if (!appRoutes.every((route) => pairs.get(route) === expectedPairs.get(route))) return false;
  if (!exactArray(retainedIds(production, owner.owner), migrated)) return false;
  if (owner.owner === "Puzzle2dPlayApp") {
    return !production.includes("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BoundedFirstStepCommandJobFactory")
      && !production.includes("semio_framework_plugin::bounded_first_step_tool_proofs!")
      && !production.includes("registry.register(BoundedFirstStepCommandJobFactory");
  }
  const factory = owner.owner === "Puzzle5dPlayApp" ? "Puzzle5dRetainedCommandJobFactory" : "Puzzle3dRetainedCommandJobFactory";
  const factoryBlock = production.split(`impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ${factory}`)[1]?.split("//#endregion 🧵️RetainedCommands")[0] ?? "";
  const contracts = publicationContracts(factoryBlock);
  const proofBlock = production.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0] ?? "";
  const proofIds = quotedValues(proofBlock.match(/tools:\s*\[([^\]]*)\]/)?.[1] ?? "");
  const exactFactory = production.includes(`impl semio_framework::ToolJobFactory for ${factory}`)
    && production.includes(`impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ${factory}`)
    && proofBlock.includes(`factory: "${factory}"`)
    && proofBlock.includes(`factory_type: ${factory}`)
    && production.includes(`type Owner = semio_framework_plugin::EditorApp<${owner.owner}>;`)
    && production.includes(owner.owner === "Puzzle5dPlayApp"
      ? `registry.register(${factory}::new(&controller_id))`
      : `registry.register(${factory}::new(&controller))`)
    && (owner.owner === "Puzzle5dPlayApp" || (
      !production.includes("build_artifact_store_one_item_preparation_factory")
      && !production.includes("build_draft_store_one_item_preparation_factory")
      && !production.includes("build_presence_store_one_item_preparation_factory")
      && !production.includes("build_transient_store_one_item_preparation_factory")
    ));
  if (!exactFactory || !exactContracts(contracts, appGroups.filter((group) => group.status === "Migrated")) || !exactArray(proofIds, migrated)) return false;
  if (owner.owner === "Puzzle3dPlayApp") {
    return production.includes("struct Puzzle3dConfigStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory")
      && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation")
      && production.includes("fn build_config_store_one_item_preparation_factory()")
      && production.includes("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))")
      && production.includes('matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")')
      && production.includes('matches!(value.as_str(), "native" | "reuse")')
      && production.includes("request.operation != request.authority.operation()")
      && production.includes("request.generation != request.authority.generation()")
      && production.includes("request.base_revision != request.authority.base_revision()")
      && production.includes("ArtifactStoreOneItemPreparationStep::Progress")
      && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1")
      && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2")
      && production.includes("fn cancel(&mut self)")
      && production.includes("fn begin_close(&mut self)")
      && production.includes("base.return_to_registry()")
      && production.includes("fn terminal_is_empty(&self)");
  }
  if (owner.owner !== "Puzzle5dPlayApp" && production.includes("build_config_store_one_item_preparation_factory")) return false;
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
  async run(): Promise<void> {
    const puzzleRoot = resolve(this.root, "../..");
    const fixture = await Bun.file(resolve(puzzleRoot, "🔣️publication-authority.json")).json() as PublicationFixture;
    const schema = await Bun.file(resolve(puzzleRoot, "🔣️publication-authority.schema.json")).json();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`Puzzle publication fixture failed Ajv validation: ${JSON.stringify(validate.errors)}`);
    if (!fixtureOracle(fixture)) throw new Error("Puzzle publication fixture failed the independent semantic oracle");
    for (const owner of fixture.owners) {
      const source = await Bun.file(resolve(puzzleRoot, owner.source)).text();
      if (!ownerOracle(owner, source)) throw new Error(`${owner.owner} publication authority diverged from the fixture`);
      const blocked = owner.groups.find((group) => group.status === "BatchOnlyPendingRewrite")?.routes[0];
      if (blocked) {
        const hostile = source.replace(
          new RegExp(`(\\.action_interactive_job\\("${blocked}",\\s*(?:semio_framework_plugin::)?InteractiveJobClassification::)BatchOnlyPendingRewrite`),
          "$1Migrated",
        );
        if (hostile === source) throw new Error(`${owner.owner} hostile activation mutation did not apply for ${blocked}`);
        if (ownerOracle(owner, hostile)) throw new Error(`${owner.owner} accepted hostile activation before decode/preparation for ${blocked}`);
      }
      const missingContract = source.replace(/\s*ArtifactToolPublicationContract \{ tool_id: "(?:openAddObjectDialog|setLocale|canvasPointerDown)", lanes: &\[ArtifactToolPublicationLane::(?:HostOnly|Config)\] \},/, "");
      if (missingContract !== source && ownerOracle(owner, missingContract)) throw new Error(`${owner.owner} accepted a missing publication contract`);
      if (owner.owner === "Puzzle3dPlayApp") {
        const missingPreparation = source.replace("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))", "None");
        const widenedTerminology = source.replace('matches!(value.as_str(), "native" | "reuse")', 'matches!(value.as_str(), "native" | "reuse" | "other")');
        const staleAuthority = source.replace("            || request.generation != request.authority.generation()\n", "");
        const missingProgress = source.replaceAll("ArtifactStoreOneItemPreparationStep::Progress", "ArtifactStoreOneItemPreparationStep::Prepared");
        if (ownerOracle(owner, missingPreparation) || ownerOracle(owner, widenedTerminology) || ownerOracle(owner, staleAuthority) || ownerOracle(owner, missingProgress)) {
          throw new Error("Puzzle3d accepted missing Store preparation, a widened mutation envelope, or stale publication authority");
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
      { ...fixture, owners: fixture.owners.map((owner, index) => index === 0 ? { ...owner, groups: [{ ...owner.groups[0]!, status: "Migrated", blocker: owner.groups[0]!.blocker }] } : owner) },
    ];
    if (hostileFixtures.some((hostile) => Boolean(validate(hostile)) || fixtureOracle(hostile))) throw new Error("Puzzle publication fixture accepted a hostile schema/oracle mutation");
    const admitted = fixture.owners.flatMap((owner) => owner.groups.filter((group) => group.status === "Migrated").flatMap((group) => group.routes));
    console.error(`validated Puzzle publication authority; admitted=${admitted.join(",")}; schema=Ajv; oracle=independent`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", PublicationAuthorityAuditScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
