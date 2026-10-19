/** 🖱️ The language-neutral Node oracles of the `World3dHost` interaction laws: the gesture to action law, the selection-set law and the selection-merge vocabulary, each re-derived from its fixture without React, three or the DOM.
 * @see ../../🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts — the verbs that run it */

import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { dirname, join } from "node:path";
import { readModuleTreeSource } from "../../../../../../../📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { ownedExport } from "../🧬️owned-schema-exports/🟦️.ts";

/** 🌐️ The `World3dHost` source as the one text the unsplit file held: the host root with its mounted module tree, plus the `🖱️interaction-domain-args` child whose action-argument builders only the hooks import. */
function world3dHostSource(hostRoot: string): string {
  return readModuleTreeSource(join(hostRoot, "🟦️.tsx")) + "\n" + readModuleTreeSource(join(hostRoot, "🖱️interaction-domain-args", "🟦️.ts"));
}

/** 🖱️ Re-derives the `World3dHost` gesture→action law from `🖱️pointer-gestures.json` WITHOUT React, three
 * or the DOM — a second, independent implementation of the same fixture the mounted jsdom suite
 * (`🧪️tests/🖱️world3d-interaction`) plays through the real component, so neither side can drift alone. Also
 * pins the source call sites, because a mounted suite driven by a mock still passes when a gesture is
 * quietly re-pointed at a different verb. */
export function world3dPointerGestureOracle(repoRoot: string): number {
  const hostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost");
  type Target = { readonly granularity: string; readonly id: string };
  type Gesture = {
    readonly id: string;
    readonly kind: string;
    readonly instanceId?: string;
    readonly ids?: readonly string[];
    readonly granularity?: string;
    readonly modifiers?: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
    readonly steps?: readonly { readonly position: readonly number[]; readonly target: readonly number[]; readonly zoom: number }[];
    readonly path?: readonly { readonly x: number; readonly y: number }[];
    readonly expect: { readonly action: string; readonly domainId?: string; readonly channel?: string; readonly merge?: string; readonly method?: string; readonly targets?: readonly Target[]; readonly dispatches?: number; readonly camera?: Record<string, unknown> };
  };
  const fixture = JSON.parse(readFileSync(join(hostRoot, "🧪️tests/🧫️fixtures/🖱️pointer-gestures.json"), "utf8")) as {
    scene: { surfaceId: string; controllerId: string; windowInstanceId: string; domainId: string; domainGranularityId: string; instances: readonly { id: string; interactionId: string; interactionGranularityId?: string }[] };
    viewport: { width: number; height: number };
    cameraDebounceMs: number;
    marqueeDragThresholdPx: number;
    gestures: readonly Gesture[];
    wiring: { selectionArgsBuilder: string; selectionTargetsArgsBuilder: string; instanceTargetBuilder: string; hoverArgsBuilder: string; cameraArgsBuilder: string; targetsBuilder: string; selectionTargetsAreASet: string; mergeVocabularyFixture: string; mergeIsNotTranslated: string; deletedMergeBuilders: readonly string[]; deadVerbs: readonly string[] };
  };
  const scene = fixture.scene;
  /** 🖱️ The merge word a modifier set resolves to — the oracle's own copy of `marqueeModeFromModifiers`,
   * written from the documented law rather than imported from the host. It emits the SCHEMA words
   * (`🕹️interaction/🧫️fixtures/🎯️merge-modes.json`): ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted
   * the host's private `add`/`remove`/`toggle` translation, which the framework rejects outright. */
  const mergeWord = (modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean } = {}): string => {
    const shift = modifiers.shiftKey === true;
    const control = modifiers.ctrlKey === true || modifiers.metaKey === true;
    if (shift && control) return "invertive";
    if (shift) return "additive";
    if (control) return "subtractive";
    return "replace";
  };
  /** 🎯️ Resolves the fixture's declared topology targets, deduplicating each granularity/id pair. */
  const topologyTargets = (ids: readonly string[]): readonly Target[] => {
    const seen = new Set<string>();
    const targets: Target[] = [];
    for (const id of ids) {
      const instance = scene.instances.find((instance) => instance.id === id);
      const target = { granularity: instance?.interactionGranularityId ?? scene.domainGranularityId, id: instance?.interactionId ?? id };
      const key = `${target.granularity}\u0000${target.id}`;
      if (seen.has(key)) continue;
      seen.add(key);
      targets.push(target);
    }
    return targets;
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };
  const validateInstance = ownedExport(repoRoot, "renderer", "World3dInstanceInteractionV1");
  const utf8 = new TextEncoder();
  for (const instance of scene.instances) {
    check(validateInstance(instance), `instance ${instance.id} must satisfy World3dInstanceInteractionV1: ${JSON.stringify(validateInstance.errors)}`);
    for (const value of [instance.id, instance.interactionId, instance.interactionGranularityId].filter((value): value is string => value !== undefined)) {
      check(utf8.encode(value).byteLength <= 256, `instance ${instance.id} identifier exceeds the native interaction-id byte authority`);
    }
  }
  check(scene.instances.length > 1 && new Set(scene.instances.map((instance) => instance.interactionId)).size === 1, "the scene must render several instances of ONE topology target, or dedup proves nothing");
  check(scene.instances.some((instance) => instance.interactionGranularityId === undefined), "the scene must keep one instance on the scene granularity fallback");
  check(scene.instances.some((instance) => instance.interactionGranularityId !== undefined), "the scene must declare one per-instance granularity override");
  check(fixture.cameraDebounceMs > 0 && fixture.marqueeDragThresholdPx > 0, "camera debounce and marquee threshold must be positive");
  for (const gesture of fixture.gestures) {
    const expected = gesture.expect;
    if (gesture.kind === "instance-pick") {
      const instance = scene.instances.find((entry) => entry.id === gesture.instanceId);
      assert(instance, `gesture ${gesture.id} names no scene instance`);
      check(expected.action === "interactionSelect", `${gesture.id}: a pick is an interactionSelect`);
      check(expected.method === "pick" && expected.domainId === scene.domainId, `${gesture.id}: a pick carries method=pick on the scene domain`);
      check(expected.merge === mergeWord(gesture.modifiers), `${gesture.id}: merge must be ${mergeWord(gesture.modifiers)}`);
      assert.deepEqual(expected.targets, [{ granularity: instance.interactionGranularityId ?? scene.domainGranularityId, id: instance.interactionId }], `${gesture.id}: a pick preserves the instance target and declared scene fallback`);
      checks += 1;
    } else if (gesture.kind === "selection-args") {
      const ids = gesture.ids ?? [];
      check(new Set(ids).size < ids.length, `${gesture.id}: the builder law must be handed a REPEATED id or it proves nothing`);
      check(expected.action === "interactionSelect" && expected.method === "pick", `${gesture.id}: the args builder builds a pick select`);
      assert.deepEqual(
        expected.targets,
        [...new Set(ids)].map((id) => ({ granularity: gesture.granularity ?? "object", id })),
        `${gesture.id}: a selection is a SET of topology ids — the args builder emits each id at most once, in first-seen order`,
      );
      checks += 1;
    } else if (gesture.kind === "instance-hover") {
      const instance = scene.instances.find((entry) => entry.id === gesture.instanceId);
      assert(instance, `gesture ${gesture.id} names no scene instance`);
      check(expected.action === "interactionHover" && expected.channel === "pointer", `${gesture.id}: hover travels on the pointer channel`);
      assert.deepEqual(expected.targets, [{ granularity: instance.interactionGranularityId ?? scene.domainGranularityId, id: instance.interactionId }], `${gesture.id}: hover preserves the instance target and declared scene fallback`);
      checks += 1;
    } else if (gesture.kind === "background-click") {
      check(expected.action === "interactionSelect" && expected.merge === mergeWord(gesture.modifiers), `${gesture.id}: an empty click still dispatches a select`);
      assert.deepEqual(expected.targets, [], `${gesture.id}: an empty click clears with an EMPTY target list`);
      checks += 1;
    } else if (gesture.kind === "camera-gesture") {
      const steps = gesture.steps ?? [];
      check(steps.length > 1 && expected.dispatches === 1, `${gesture.id}: several steps must debounce into exactly one setCamera`);
      assert.deepEqual(expected.camera, { ...steps[steps.length - 1] }, `${gesture.id}: the dispatched pose is the LAST step, never an intermediate one`);
      checks += 1;
    } else if (gesture.kind === "marquee") {
      const path = gesture.path ?? [];
      const span = Math.hypot(path[path.length - 1]!.x - path[0]!.x, path[path.length - 1]!.y - path[0]!.y);
      check(path.length > 1 && span > fixture.marqueeDragThresholdPx, `${gesture.id}: the path must exceed the drag threshold or it is a click`);
      check(expected.merge === "replace", `${gesture.id}: a marquee release always replaces`);
      check(expected.method === "rectangle", `${gesture.id}: a rectangular marquee retains its selection method`);
      assert.deepEqual(
        expected.targets,
        topologyTargets(scene.instances.map((instance) => instance.id)),
        `${gesture.id}: a full-viewport marquee collapses every rendered instance onto its deduplicated topology targets`,
      );
      checks += 1;
    } else {
      assert.fail(`gesture ${gesture.id} has an unknown kind ${gesture.kind}`);
    }
  }
  const host = world3dHostSource(hostRoot);
  check(host.includes(`dispatch("interactionSelect", ${fixture.wiring.selectionTargetsArgsBuilder}(interactionDomainId, [${fixture.wiring.instanceTargetBuilder}(instances, id, interactionGranularity)], merge))`), "the instance pick must preserve its resolved target through the selection builder");
  check(host.includes(`return dispatchSettled("interactionHover", ${fixture.wiring.hoverArgsBuilder}(interactionDomainId, target.granularity, target.id))`), "the instance hover must preserve its resolved target and hand the coalescing gate its awaitable");
  check(host.includes(`dispatch("interactionSelect", ${fixture.wiring.selectionArgsBuilder}(interactionDomainId, interactionGranularity, [], merge))`), "the empty click must still clear through the selection builder");
  check(host.includes(`dispatch("setCamera", ${fixture.wiring.cameraArgsBuilder}(`), "the debounced camera sync must still build its args through the camera builder");
  check(host.includes(`const domainTargets = ${fixture.wiring.targetsBuilder}(instancesRef.current, preview.mergedInstanceIds, interactionGranularity)`), "the marquee release must still resolve instances onto topology targets");
  check(host.includes(`dispatch("interactionSelect", ${fixture.wiring.selectionTargetsArgsBuilder}(interactionDomainId, domainTargets, "replace", method === "lasso" ? "lasso" : "rectangle"))`), "the marquee release must preserve its shape while replacing on the scene domain");
  check(host.includes(fixture.wiring.selectionTargetsAreASet), "the selection args builder must keep collapsing repeated topology ids — the host never emits a duplicate target");
  check(host.includes('return { domainId, targets: JSON.stringify(targets), merge, method: "pick" };'), "the selection args shape is the wire contract and must stay verbatim");
  check(host.includes('return { domainId, channel: "pointer", targets: JSON.stringify(targets) };'), "the hover args shape is the wire contract and must stay verbatim");
  for (const verb of fixture.wiring.deadVerbs) check(!host.includes(`dispatch("${verb}"`), `${verb} is framework-owned now and must not be dispatched from a world host`);
  const vocabulary = (JSON.parse(readFileSync(join(repoRoot, fixture.wiring.mergeVocabularyFixture), "utf8") ) as { vocabulary: readonly string[]; deletedWords: readonly string[] });
  for (const gesture of fixture.gestures) {
    if (gesture.expect.merge === undefined) continue;
    check(vocabulary.vocabulary.includes(gesture.expect.merge), `${gesture.id}: merge "${gesture.expect.merge}" is outside the ONE schema vocabulary ${vocabulary.vocabulary.join("/")}`);
  }
  check(host.includes(fixture.wiring.mergeIsNotTranslated), "the selection args builder must take a typed MergeMode — a host that re-types it as a bare string can invent a private vocabulary again");
  for (const builder of fixture.wiring.deletedMergeBuilders) check(!host.includes(`function ${builder}(`), `${builder} translated the resolved MergeMode into a private word and is deleted — no adapter`);
  for (const word of vocabulary.deletedWords) check(!host.includes(`return "${word}";`), `the deleted merge word "${word}" must not be produced anywhere in the host`);
  return checks;
}

/** 🧮️ Re-derives the framework selection SET law from `🔌️plugin/🧫️fixtures/🕹️selection-set.json` without
 * Rust — the language-neutral twin of the reserved-job laws in `🔬️plugin-runtime-plugin-builder-contract`.
 * A selection is a set of topology ids: `Select` is idempotent per id whatever the merge, and however
 * many mirror domains (`vortex`) the leftover projection flattens. Also pins the ONE projection the
 * Rust side must keep sharing between its two flatten sites — the live 2026-09-12 defect was exactly
 * two copies of that fold, one deduplicating and one not. */
export function interactionSelectionSetOracle(repoRoot: string): number {
  const pluginRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  type Anchor = { readonly file: string; readonly text: string };
  type Case = { readonly id: string; readonly why: string; readonly seed: readonly string[]; readonly merge: string; readonly targets: readonly string[]; readonly selectedIds: readonly string[] };
  const fixture = JSON.parse(readFileSync(join(pluginRoot, "🧪️tests/🧫️fixtures/🕹️selection-set.json"), "utf8")) as {
    domain: string;
    granularity: string;
    mirrorDomain: string;
    cases: readonly Case[];
    wiring: { projection: Anchor; callSites: readonly Anchor[]; lockedFromSelectedIds: Anchor };
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };
  /** 🧮️ The oracle's own `next_selection` + leftover-flatten, written from the documented law: a
   * replace keeps the batch's distinct ids, an additive appends only genuinely new ones, and the
   * publication flattens the app domain plus its `vortex` mirror into ONE set. */
  const publish = (seed: readonly string[], merge: string, targets: readonly string[]): readonly string[] => {
    const domain = merge === "replace" ? [...new Set(targets)] : [...seed, ...targets.filter((id) => !seed.includes(id))];
    const mirrored = [...domain, ...domain];
    const published: string[] = [];
    for (const id of mirrored) if (!published.includes(id)) published.push(id);
    return published;
  };
  check(fixture.cases.length > 0 && fixture.cases.some((entry) => entry.merge === "replace") && fixture.cases.some((entry) => entry.merge === "additive"), "the set law must cover BOTH the replace and the additive merge");
  check(fixture.cases.some((entry) => new Set(entry.targets).size < entry.targets.length), "at least one case must repeat a target inside one batch or the batch half of the law proves nothing");
  check(fixture.cases.some((entry) => entry.merge === "additive" && entry.targets.every((id) => entry.seed.includes(id))), "at least one case must re-add an already selected id or idempotence proves nothing");
  for (const entry of fixture.cases) {
    assert.deepEqual(publish(entry.seed, entry.merge, entry.targets), [...entry.selectedIds], `${entry.id}: ${entry.why}`);
    checks += 1;
    check(new Set(entry.selectedIds).size === entry.selectedIds.length, `${entry.id}: selectedIds must name every id at most once`);
  }
  const anchored = (anchor: Anchor): boolean => readFileSync(join(repoRoot, anchor.file), "utf8").includes(anchor.text);
  check(anchored(fixture.wiring.projection), "the ONE shared leftover-selection projection must still exist");
  for (const site of fixture.wiring.callSites) check(anchored(site), `every flatten site must go through the shared projection — missing ${site.text}`);
  check(anchored(fixture.wiring.lockedFromSelectedIds), "`locked` must be derived from the deduplicated selectedIds, never from a second flatten");
  return checks;
}

/** 🎯️ Re-derives the ONE selection merge vocabulary from `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`
 * without Rust: every word is validated against the OWNED schema export (`MergeMode` of
 * `🕹️interaction/🧬️schema/🔣️.json`) through ajv — a third-party validator, so the vocabulary is proved
 * against the schema by something other than our own matcher — every deleted word is proved to FAIL
 * that same validator, the per-mode set algebra (including `range` and its documented degradation on
 * an unordered domain) is re-implemented from the fixture's written law, and the source anchors of all
 * four implementations are pinned. Live defect 26/09/09/PROCEDURAL-3D-END-TO-END: two vocabularies in
 * one tree made every modifier-click on a domain-bound world scene fault. */
export function selectionMergeVocabularyOracle(repoRoot: string): number {
  type Row = { readonly id: string; readonly modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean }; readonly pick: string; readonly componentPick: string };
  type Case = { readonly id: string; readonly why: string; readonly orderedTopology?: readonly string[]; readonly seed: readonly string[]; readonly anchor: string | null; readonly merge: string; readonly targets: readonly string[]; readonly selectedIds: readonly string[]; readonly anchorId: string };
  const fixturePath = "🧰️framework/🔨️modules/🕹️interaction/🧪️tests/🧫️fixtures/🎯️merge-modes.json";
  const fixture = JSON.parse(readFileSync(join(repoRoot, fixturePath), "utf8")) as {
    schema: { file: string; export: string; graphql: string };
    vocabulary: readonly string[];
    deletedWords: readonly string[];
    unknownMergeFault: string;
    modifierPolicy: { rows: readonly Row[]; neverEmitted: readonly string[] };
    topology: { domain: string; granularity: string; ordered: readonly string[] };
    range: { decision: string; law: string; unorderedDomains: string };
    cases: readonly Case[];
    wiring: {
      codec: { file: string; toWire: string; fromWire: string };
      domainPath: { decoderFile: string; worldPathFile: string; decoder: string; worldPath: string };
      host: { file: string; selectionArgs: string; componentPick: string; deletedBuilders: readonly string[] };
    };
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };

  const validate = ownedExport(repoRoot, "interaction", fixture.schema.export);
  for (const word of fixture.vocabulary) check(validate(word) === true, `"${word}" is in the fixture vocabulary but the owned ${fixture.schema.export} schema export rejects it`);
  for (const word of fixture.deletedWords) check(validate(word) !== true, `the deleted word "${word}" must NOT validate against the owned ${fixture.schema.export} schema export`);
  const schemaEnum = ((JSON.parse(readFileSync(join(repoRoot, fixture.schema.file), "utf8")) as { $defs: Record<string, { enum?: readonly string[] }> }).$defs[fixture.schema.export]?.enum ?? []) as readonly string[];
  assert.deepEqual([...fixture.vocabulary], [...schemaEnum], "the fixture vocabulary IS the schema enum, in the schema's own order — a second, hand-maintained list is exactly the defect");
  checks += 1;
  const graphql = readFileSync(join(repoRoot, fixture.schema.graphql), "utf8");
  for (const word of fixture.vocabulary) check(graphql.includes(`\n  ${word.toUpperCase()}\n`), `the GraphQL schema leaf must declare ${word.toUpperCase()} — the two schema spellings must not drift`);

  const resolve = (modifiers: Row["modifiers"]): string => {
    const shift = modifiers.shiftKey === true;
    const control = modifiers.ctrlKey === true || modifiers.metaKey === true;
    if (shift && control) return "invertive";
    if (shift) return "additive";
    if (control) return "subtractive";
    return "replace";
  };
  check(fixture.modifierPolicy.rows.length >= 4, "the modifier policy must cover the bare click and every multi-select chord");
  for (const row of fixture.modifierPolicy.rows) {
    check(row.pick === resolve(row.modifiers), `${row.id}: a whole-instance pick resolves to ${resolve(row.modifiers)}`);
    check(row.componentPick === (row.pick === "replace" ? "invertive" : row.pick), `${row.id}: a component pick differs from an instance pick ONLY in that a bare click toggles`);
    check(fixture.vocabulary.includes(row.pick) && fixture.vocabulary.includes(row.componentPick), `${row.id}: both resolved modes must be schema words`);
    for (const never of fixture.modifierPolicy.neverEmitted) check(row.pick !== never && row.componentPick !== never, `${row.id}: a world viewport has no ordered topology, so it must never resolve to "${never}"`);
  }

  const merge = (entry: Case): { readonly ids: readonly string[]; readonly anchorId: string } => {
    const ordered = entry.orderedTopology ?? fixture.topology.ordered;
    const last = entry.targets[entry.targets.length - 1]!;
    if (entry.merge === "range") {
      const anchor = entry.anchor ?? entry.seed[entry.seed.length - 1] ?? last;
      const from = ordered.indexOf(anchor);
      const to = ordered.indexOf(last);
      if (from < 0 || to < 0) return { ids: [last], anchorId: last };
      return { ids: ordered.slice(Math.min(from, to), Math.max(from, to) + 1), anchorId: anchor };
    }
    const distinct = [...new Set(entry.targets)];
    if (entry.merge === "replace") return { ids: distinct, anchorId: last };
    if (entry.merge === "additive") return { ids: [...entry.seed, ...distinct.filter((id) => !entry.seed.includes(id))], anchorId: last };
    if (entry.merge === "subtractive") return { ids: entry.seed.filter((id) => !distinct.includes(id)), anchorId: last };
    const ids = [...entry.seed];
    for (const id of distinct) {
      const at = ids.indexOf(id);
      if (at >= 0) ids.splice(at, 1);
      else ids.push(id);
    }
    return { ids, anchorId: last };
  };
  for (const word of fixture.vocabulary) check(fixture.cases.some((entry) => entry.merge === word), `no case exercises the merge "${word}" — every word of the vocabulary must carry a law`);
  check(fixture.cases.some((entry) => (entry.orderedTopology ?? []).length === 0 && entry.merge === "range"), "the range decision is only pinned if a case runs it against an UNORDERED domain");
  for (const entry of fixture.cases) {
    const produced = merge(entry);
    assert.deepEqual([...produced.ids], [...entry.selectedIds], `${entry.id}: ${entry.why}`);
    checks += 1;
    check(produced.anchorId === entry.anchorId, `${entry.id}: the published anchor must be ${entry.anchorId}`);
    check(new Set(entry.selectedIds).size === entry.selectedIds.length, `${entry.id}: a selection is a SET`);
  }

  const codec = readFileSync(join(repoRoot, fixture.wiring.codec.file), "utf8");
  check(codec.includes(fixture.wiring.codec.toWire), "the ONE wire-label encoder must still exist next to the MergeMode type");
  check(codec.includes(fixture.wiring.codec.fromWire), "the ONE wire-label decoder must still exist next to the MergeMode type");
  const decoderSource = readFileSync(join(repoRoot, fixture.wiring.domainPath.decoderFile), "utf8");
  const worldPathSource = readFileSync(join(repoRoot, fixture.wiring.domainPath.worldPathFile), "utf8");
  const plugin = decoderSource + "\n" + worldPathSource;
  check(decoderSource.includes(fixture.wiring.domainPath.decoder), "the domain path must decode through the shared codec, never through its own match arms");
  check(worldPathSource.includes(fixture.wiring.domainPath.worldPath), "the non-domain world path must take a decoded MergeMode, so a private word is unrepresentable");
  const host = world3dHostSource(dirname(join(repoRoot, fixture.wiring.host.file)));
  check(host.includes(fixture.wiring.host.selectionArgs), "the host's selection args builder must take a typed MergeMode");
  check(host.includes(fixture.wiring.host.componentPick), "the component pick's one deviation must be a named MergeMode→MergeMode function, not a word table");
  for (const builder of fixture.wiring.host.deletedBuilders) check(!host.includes(`function ${builder}(`), `${builder} is deleted — greenfield, no adapter between two vocabularies`);
  for (const word of fixture.deletedWords) {
    check(!plugin.includes(`"${word}" =>`), `the plugin must not match the deleted merge word "${word}"`);
    check(!host.includes(`return "${word}";`), `the host must not produce the deleted merge word "${word}"`);
  }
  check(fixture.unknownMergeFault.includes("unknown merge"), "the fixture must name the fault an out-of-vocabulary merge raises");
  return checks;
}
