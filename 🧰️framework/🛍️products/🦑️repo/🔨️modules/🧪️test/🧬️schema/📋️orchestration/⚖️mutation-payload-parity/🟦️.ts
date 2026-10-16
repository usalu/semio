/** ⚖️ Mutation payload parity gate
 *
 * `schema-mutation-payload-parity`: every mutation leaf and aggregate schema describes the wire its committed fixtures and the `#[derive(Mutations)]` enum witness.
 */

import { type FeatureStep } from "@semio-tech/framework/test/adapter";
import { type SchemaDiagnostic, readSchemaCatalog } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { parseFeature } from "@semio-tech/framework/test/gherkin";
import { mutationInputInstance } from "@semio-tech/framework/manifest";
import { parseSchemaInvariants } from "@semio-tech/framework/manifest/schema";
import { readFileSync } from "node:fs";
import { Validator } from "jsonschema";
import { join } from "node:path";
import { canonicalJson, catalogSchemaDocuments, escapePointer, isRecord, readJsonObject, resolveReferencedSchemaDocuments, schemaKind, type SchemaNode, unescapePointer } from "../📚️schema-documents/🟦️.ts";
import { type MutationAggregate, type MutationAggregateLayout, mutationArtifactScope, type MutationLeafRecord, mutationPayloadOwner, mutationTree, sharedSegments } from "../🌳️mutation-tree/🟦️.ts";

//#region ⚖️MutationPayloadParity
/** 🩺️ The classes of a `schema-mutation-payload-parity` finding. */
export type MutationPayloadFindingClass = "invalid" | "undescribed" | "opaque" | "layout" | "aggregate" | "unmapped" | "unresolved" | "unwitnessed" | "negative";

/**
 * 🚫️ How a committed fixture witnesses its leaf schema. `positive`: the wire must satisfy the schema. `negative`: its committed
 * outcome (`<case>/🎯️outcome/🔣️.json`) is a refusal with `mutation.invariant`, a payload the domain never admits; `invariant` is the
 * id the outcome names for a payload-intrinsic rule JSON Schema cannot express (declared in the leaf's `x-semio-invariant`).
 */
export type MutationFixtureOutcome = Readonly<{ direction: "positive" | "negative"; invariant: string | null }>;

/** 🚫️ The witness direction of a committed outcome document: `negative` for `{"status": "rejected", "code": "mutation.invariant"}`. */
export function mutationFixtureOutcome(outcome: unknown): MutationFixtureOutcome {
  const negative = isRecord(outcome) && outcome.status === "rejected" && outcome.code === "mutation.invariant";
  return { direction: negative ? "negative" : "positive", invariant: negative && typeof outcome.invariant === "string" ? outcome.invariant : null };
}

/**
 * 🚫️ The verdict on a negative witness: clean when its leaf schema rejects it (`invalid`), or when it is schema-valid but its
 * outcome names an invariant the leaf declares in `x-semio-invariant` (`declared`, the ids of `mutationSchemaInvariants`); else
 * one `negative` finding. Never `invalid`.
 */
export function mutationNegativeVerdict(payload: readonly MutationPayloadFinding[], outcome: MutationFixtureOutcome, declared: ReadonlySet<string>): MutationPayloadFinding[] {
  if (payload.some((finding) => finding.class === "invalid") || (outcome.invariant !== null && declared.has(outcome.invariant))) return [];
  const detail = outcome.invariant === null ? "the committed outcome refuses this payload with mutation.invariant and names no invariant, but the leaf schema accepts it: state the rule in the schema, or declare it in x-semio-invariant and name its id in the outcome" : `the committed outcome names the invariant ${JSON.stringify(outcome.invariant)}, which the leaf schema neither enforces nor declares in x-semio-invariant (declared: ${[...declared].map((id) => JSON.stringify(id)).join(", ") || "none"})`;
  return [{ class: "negative", pointer: "", detail }];
}

/** 🧷️ The invariant ids a schema document declares in `x-semio-invariant` on any of its nodes (read by `parseSchemaInvariants`; a malformed
 * value declares nothing here — the strict vocabulary oracle reports it). */
export function mutationSchemaInvariants(document: unknown): ReadonlySet<string> {
  const ids = new Set<string>();
  const declared = (value: unknown): readonly { readonly id: string }[] => {
    try {
      return parseSchemaInvariants(value);
    } catch {
      return [];
    }
  };
  const walk = (node: unknown): void => {
    if (Array.isArray(node)) for (const item of node) walk(item);
    else if (isRecord(node))
      for (const [key, value] of Object.entries(node)) {
        if (key === "x-semio-invariant") for (const invariant of declared(value)) ids.add(invariant.id);
        else if (key !== "const" && key !== "enum" && key !== "examples" && key !== "default") walk(value);
      }
  };
  walk(document);
  return ids;
}

/** 🔎️ One parity finding: its class, an RFC 6901 pointer (into the payload, or `<document>#<pointer>` into a schema for `opaque`), and why. */
export type MutationPayloadFinding = Readonly<{ class: MutationPayloadFindingClass; pointer: string; detail: string }>;

/** 📊️ One plugin's share of the payload-parity census: leaves, committed fixtures (of them `negatives`, and of those `invariants` naming a
 * declared `x-semio-invariant`), fixtures without a finding, findings per class. */
export type MutationPayloadCensusRow = { readonly owner: string; leaves: number; witnessed: number; fixtures: number; negatives: number; invariants: number; rows: number; clean: number; readonly findings: Record<string, number> };

/** 🥒️ One wire-form row of a feature: the leaf `kind` and its wire payload `params`, at the 1-based line of its scenario. */
export type MutationFeatureRow = Readonly<{ line: number; kind: string; params: unknown }>;

/** ⚖️ The `schema-mutation-payload-parity` lint and its census. */
export type MutationPayloadParityReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationPayloadCensusRow[] };

/** 🐫 The TypeScript twin of value_derive `variant_wire_name` (`🌱️value/✨️derive/⚙️expansion/🦀️.rs`): an enum variant's wire name. */
export function mutationVariantWireName(ident: string, rename: string | null, renameAll: string | null): string {
  if (rename !== null) return rename;
  const words = ident.split(/(?=\p{Lu})/u).map((word) => word.toLowerCase());
  if (renameAll === "camelCase") return words.map((word, index) => (index === 0 ? word : word.charAt(0).toUpperCase() + word.slice(1))).join("");
  if (renameAll === "kebab-case") return words.join("-");
  if (renameAll === "snake_case") return words.join("_");
  if (renameAll === "lowercase") return words.join("");
  return ident;
}

/**
 * 🧾️ The leaf payload `payload_value()` yields for the variant wired as `wireName`, cut out of the aggregate wire `value` that
 * `layout` encodes: the one-key wrapper of an externally tagged enum, the `content` member of an adjacently tagged one, the
 * members beside the tag of an internally tagged one — or why `value` is not that variant in that layout.
 */
export function mutationLeafPayload(layout: MutationAggregateLayout, wireName: string, value: unknown): { payload: unknown } | { refused: string } {
  if (!isRecord(value)) return { refused: "the aggregate wire value is not an object" };
  const keys = Object.keys(value);
  if (layout.tag === null) return keys.length === 1 && keys[0] === wireName ? { payload: value[wireName] } : { refused: `an externally tagged aggregate wires this variant as {${JSON.stringify(wireName)}: …}, found the keys ${JSON.stringify(keys)}` };
  if (value[layout.tag] !== wireName) return { refused: `the tag ${JSON.stringify(layout.tag)} must be ${JSON.stringify(wireName)}, found ${JSON.stringify(value[layout.tag] ?? keys)}` };
  if (layout.content === null) return { payload: Object.fromEntries(Object.entries(value).filter(([key]) => key !== layout.tag)) };
  const extra = keys.filter((key) => key !== layout.tag && key !== layout.content);
  return extra.length > 0 ? { refused: `an adjacently tagged aggregate carries only ${JSON.stringify(layout.tag)} and ${JSON.stringify(layout.content)}, found ${JSON.stringify(extra)}` } : { payload: value[layout.content] ?? null };
}

/**
 * 🧺️ The aggregate-schema branch of one variant, as the repo-wide rule derives it from the wire layout: the leaf `$ref` itself
 * (internally tagged; the leaf declares the tag `const`), `{W: $ref}` (externally tagged) or `{tag: const W, content: $ref}`
 * (adjacently tagged), each closed with `additionalProperties: false`. A wrapped leaf (`#[mutation_leaf(payload = <Variant>)]`)
 * describes its editable payload at the document root and its whole wire enum under `$defs/<wrapper>`, so the branch refers to
 * that enum: the root alone would refuse the wrapper's other variants and the variant envelope itself.
 */
export function mutationAggregateBranch(layout: MutationAggregateLayout, wireName: string, leafId: string, wrapper: string | null = null): Record<string, unknown> {
  const leaf = { $ref: wrapper === null ? leafId : `${leafId}#/$defs/${escapePointer(wrapper)}` };
  if (layout.tag !== null && layout.content === null) return leaf;
  if (layout.tag === null) return { type: "object", additionalProperties: false, required: [wireName], properties: { [wireName]: leaf } };
  return { type: "object", additionalProperties: false, required: [layout.tag, layout.content], properties: { [layout.tag]: { const: wireName }, [layout.content!]: leaf } };
}

/** 🎁️ A wrapped leaf: the leaf enum's own wire layout and the wire name of its `#[mutation_leaf(payload = <Variant>)]` variant. */
export type MutationLeafWrapper = Readonly<{ layout: MutationAggregateLayout; wireName: string }>;

/**
 * 🎁️ The editable leaf payload of one committed fixture, as `payload_value()` yields it: the aggregate wrapper is cut first
 * (`mutationLeafPayload`); a wrapped leaf then yields the content of its `payload` variant, and any other variant of it is
 * `inert` (not editable, so no witness of the leaf schema). `refused` when the fixture is not the variant in the layout.
 */
export function mutationInputPayload(layout: MutationAggregateLayout, wireName: string, value: unknown, wrapper: MutationLeafWrapper | null): { payload: unknown } | { refused: string } | { inert: string } {
  const cut = mutationLeafPayload(layout, wireName, mutationFixtureWire(value));
  if ("refused" in cut || wrapper === null) return cut;
  const inner = mutationLeafPayload(wrapper.layout, wrapper.wireName, cut.payload);
  return "payload" in inner ? inner : { inert: inner.refused };
}

/**
 * 🎬️ The step vocabulary of a mutation row: a `When` step that applies, attempts or replays the kind it names (`When the <kind>
 * mutation is applied with its parameters`, `… is attempted …`, `When <kind> is replayed against its vector …`). A `Then` step's
 * expectation docstring and a codec step (`When it is parsed … and printed back`) never apply a mutation.
 */
export function mutationFeatureStep(step: FeatureStep, kind: string): boolean {
  return step.keyword === "When" && step.text.includes(kind) && /\b(?:applied|attempted|replayed)\b/u.test(step.text);
}

/**
 * 🥒️ The wire-form rows of a `🥒️.feature`, read through the harness's own `parseFeature` (the plan every native host receives):
 * every mutation step (`mutationFeatureStep`) of an expanded scenario whose docstring is a JSON object `{"kind": <leaf kind>,
 * "params": <wire payload>, …}`, at the scenario's line. A Scenario Outline row whose template carries `"kind": "<id>"` and
 * `"params": <params>` is one such step; a round-trip, codec or expectation step carrying the same shape is not a row.
 */
export function mutationFeatureRows(source: string): MutationFeatureRow[] {
  return parseFeature(source).scenarios.flatMap((scenario) =>
    scenario.steps.flatMap((step) => {
      const value: unknown = (() => {
        try {
          return step.docString === undefined ? undefined : JSON.parse(step.docString);
        } catch {
          return undefined;
        }
      })();
      return isRecord(value) && typeof value.kind === "string" && "params" in value && mutationFeatureStep(step, value.kind) ? [{ line: scenario.line, kind: value.kind, params: value.params }] : [];
    }),
  );
}

/** 📦️ The aggregate wire value a committed `🦠️mutation/🔣️.json` holds: the whole document, or the `mutation` member of an exhaustive
 * case record `{kind, mutation, before, after}` (the stdio `mutate-*` cases). */
export function mutationFixtureWire(value: unknown): unknown {
  return isRecord(value) && isRecord(value.mutation) && "before" in value && "after" in value ? value.mutation : value;
}

/**
 * ⚖️ The parity checks of one leaf payload schema over the `schema://` document universe, validated by the third-party npm
 * `jsonschema`: `invalid` (the payload fails its leaf schema), `undescribed` (a payload member no `properties`,
 * `patternProperties` or `additionalProperties` describes, walked through `$ref`, `allOf` and the matching `oneOf`/`anyOf`
 * branch), and `opaque` (an object node of the schema that declares no members at all).
 */
export function mutationPayloadChecker(documents: ReadonlyMap<string, Record<string, unknown>>) {
  const validator = new Validator();
  for (const [id, document] of documents) {
    try {
      validator.addSchema(document, id);
    } catch {
      validator.unresolvedRefs = [];
    }
    validator.schemas[id] = document;
  }
  const at = (document: string, pointer: string): unknown => {
    let current: unknown = documents.get(document);
    for (const segment of pointer.split("/").slice(1).map(unescapePointer)) current = isRecord(current) ? current[segment] : Array.isArray(current) && /^\d+$/u.test(segment) ? current[Number(segment)] : undefined;
    return current;
  };
  const resolve = (document: string, pointer: string): SchemaNode | null => {
    for (let hop = 0; hop < 32; hop += 1) {
      const node = at(document, pointer);
      if (!isRecord(node)) return null;
      if (typeof node.$ref !== "string") return { document, pointer, node };
      const hash = node.$ref.indexOf("#");
      const id = hash < 0 ? node.$ref : node.$ref.slice(0, hash);
      try {
        document = id === "" ? document : new URL(id, document).href;
      } catch {
        return null;
      }
      pointer = hash < 0 ? "" : decodeURIComponent(node.$ref.slice(hash + 1));
      if (!documents.has(document)) return null;
    }
    return null;
  };
  const reference = (document: string, pointer: string): string => `${document}#${pointer.split("/").slice(1).map((segment) => `/${encodeURIComponent(escapePointer(unescapePointer(segment)))}`).join("")}`;
  const valid = (instance: unknown, document: string, pointer: string): boolean => {
    try {
      return validator.validate(instance, { $ref: reference(document, pointer) }).valid;
    } catch {
      return false;
    }
  };
  const members = (instance: unknown, document: string, pointer: string, seen: Set<string>): SchemaNode[] => {
    const resolved = resolve(document, pointer);
    if (resolved === null || seen.has(`${resolved.document}#${resolved.pointer}`)) return [];
    seen.add(`${resolved.document}#${resolved.pointer}`);
    const node = resolved.node;
    const found = [resolved];
    const branch = (keyword: string, index: number | null): void => void found.push(...members(instance, resolved.document, `${resolved.pointer}/${keyword}${index === null ? "" : `/${index}`}`, seen));
    if (Array.isArray(node.allOf)) node.allOf.forEach((_, index) => branch("allOf", index));
    for (const keyword of ["oneOf", "anyOf"]) {
      if (!Array.isArray(node[keyword])) continue;
      const matching = (node[keyword] as unknown[]).map((_, index) => index).filter((index) => valid(instance, resolved.document, `${resolved.pointer}/${keyword}/${index}`));
      for (const index of keyword === "oneOf" ? matching.slice(0, 1) : matching) branch(keyword, index);
    }
    if (node.if !== undefined) {
      const chosen = valid(instance, resolved.document, `${resolved.pointer}/if`) ? "then" : "else";
      if (node[chosen] !== undefined) branch(chosen, null);
    }
    return found;
  };
  const where = (node: SchemaNode): string => `${node.document}#${node.pointer}`;
  const describe = (instance: unknown, document: string, pointer: string, path: string, findings: MutationPayloadFinding[], depth: number): void => {
    const nodes = depth > 64 ? [] : members(instance, document, pointer, new Set());
    if (nodes.length === 0) return;
    if (isRecord(instance)) {
      const declared = nodes.some(({ node }) => node.properties !== undefined || node.patternProperties !== undefined || node.additionalProperties !== undefined);
      const typed = nodes.some(({ node }) => node.type === "object" || (Array.isArray(node.type) && node.type.includes("object")));
      if (!declared) {
        if (typed) for (const key of Object.keys(instance)) findings.push({ class: "undescribed", pointer: `${path}/${escapePointer(key)}`, detail: `${where(nodes[0]!)} declares an object without members, yet the payload carries ${JSON.stringify(key)}` });
        return;
      }
      for (const [key, value] of Object.entries(instance)) {
        const child = `${path}/${escapePointer(key)}`;
        const targets: [string, string][] = [];
        for (const { document: owner, pointer: base, node } of nodes) {
          if (isRecord(node.properties) && key in node.properties) targets.push([owner, `${base}/properties/${escapePointer(key)}`]);
          if (isRecord(node.patternProperties)) for (const pattern of Object.keys(node.patternProperties)) if (new RegExp(pattern, "u").test(key)) targets.push([owner, `${base}/patternProperties/${escapePointer(pattern)}`]);
        }
        const additional = nodes.find(({ node }) => node.additionalProperties !== undefined);
        if (targets.length === 0 && additional === undefined) findings.push({ class: "undescribed", pointer: child, detail: `${JSON.stringify(key)} is no declared member of ${where(nodes[0]!)}` });
        else if (targets.length === 0 && isRecord(additional?.node.additionalProperties)) targets.push([additional.document, `${additional.pointer}/additionalProperties`]);
        for (const [owner, base] of targets) describe(value, owner, base, child, findings, depth + 1);
      }
    } else if (Array.isArray(instance))
      for (const { document: owner, pointer: base, node } of nodes) {
        if (isRecord(node.items)) instance.forEach((item, index) => describe(item, owner, `${base}/items`, `${path}/${index}`, findings, depth + 1));
        else if (Array.isArray(node.items)) instance.forEach((item, index) => describe(item, owner, index < (node.items as unknown[]).length ? `${base}/items/${index}` : `${base}/additionalItems`, `${path}/${index}`, findings, depth + 1));
      }
  };
  const opaque = (document: string, pointer: string, root: string, findings: MutationPayloadFinding[], seen: Set<string>): void => {
    const resolved = resolve(document, pointer);
    if (resolved === null || seen.has(where(resolved))) return;
    seen.add(where(resolved));
    const node = resolved.node;
    const typed = node.type === "object" || (Array.isArray(node.type) && node.type.includes("object"));
    const composed = ["allOf", "oneOf", "anyOf", "if"].some((keyword) => node[keyword] !== undefined);
    const described = node.patternProperties !== undefined || node.additionalProperties !== undefined || (isRecord(node.properties) && Object.keys(node.properties).length > 0);
    if (typed && !composed && !described) findings.push({ class: "opaque", pointer: resolved.document === root ? `#${resolved.pointer}` : where(resolved), detail: "an object schema declares no members (properties, patternProperties or additionalProperties), so its payload fields are undescribed" });
    const child = (suffix: string): void => opaque(resolved.document, `${resolved.pointer}/${suffix}`, root, findings, seen);
    for (const keyword of ["properties", "patternProperties"]) if (isRecord(node[keyword])) for (const key of Object.keys(node[keyword] as object)) child(`${keyword}/${escapePointer(key)}`);
    for (const keyword of ["additionalProperties", "additionalItems", "then", "else"]) if (isRecord(node[keyword])) child(keyword);
    if (isRecord(node.items)) child("items");
    for (const keyword of ["items", "allOf", "oneOf", "anyOf"]) if (Array.isArray(node[keyword])) (node[keyword] as unknown[]).forEach((_, index) => child(`${keyword}/${index}`));
  };
  const checks = {
    /** 🔎️ The `invalid` and `undescribed` findings of one normalized leaf payload against the leaf document `id`. */
    payload(id: string, instance: unknown): MutationPayloadFinding[] {
      const findings: MutationPayloadFinding[] = [];
      try {
        const errors = validator.validate(instance, { $ref: id }).errors;
        if (errors.length > 0) findings.push({ class: "invalid", pointer: "", detail: errors.slice(0, 4).map((error) => `${error.property.replace(/^instance/u, "") || "/"} ${error.message}`).join("; ") });
      } catch (error) {
        return [{ class: "unresolved", pointer: "", detail: error instanceof Error ? error.message : String(error) }];
      }
      describe(instance, id, "", "", findings, 0);
      return findings;
    },
    /** 🧺️ The `aggregate` finding when the aggregate document `id` rejects the whole aggregate wire value of a fixture. */
    aggregate(id: string, wire: unknown): MutationPayloadFinding[] {
      try {
        const errors = validator.validate(wire, { $ref: id }).errors;
        return errors.length === 0 ? [] : [{ class: "aggregate", pointer: "", detail: `${id} rejects the aggregate wire value: ${errors.slice(0, 2).map((error) => `${error.property.replace(/^instance/u, "") || "/"} ${error.message}`).join("; ")}` }];
      } catch (error) {
        return [{ class: "aggregate", pointer: "", detail: `${id} does not resolve: ${error instanceof Error ? error.message : String(error)}` }];
      }
    },
    /** 🧷️ The invariant ids the leaf document `id` declares in `x-semio-invariant`. */
    invariants(id: string): ReadonlySet<string> {
      return mutationSchemaInvariants(documents.get(id));
    },
    /** 🕳️ The `opaque` findings of the leaf document `id`, walked through every reachable subschema and `$ref`. */
    opaque(id: string): MutationPayloadFinding[] {
      const findings: MutationPayloadFinding[] = [];
      opaque(id, "", id, findings, new Set());
      return findings;
    },
    /**
     * 🧫️ Every finding of one committed fixture `value` of the variant wired as `wireName`: `layout` when it is not that
     * variant in `layout`, else the payload checks of the editable leaf payload (`mutationInputPayload`; root `const`s spliced
     * back by the manifest reader `mutationInputInstance`) against the leaf document `leafId` — skipped for a non-editable
     * variant of a wrapped leaf — and the aggregate check against `aggregateId`.
     */
    fixture(layout: MutationAggregateLayout, wireName: string, leafId: string, aggregateId: string | null, value: unknown, wrapper: MutationLeafWrapper | null = null, outcome: MutationFixtureOutcome = { direction: "positive", invariant: null }): MutationPayloadFinding[] {
      const wire = mutationFixtureWire(value);
      const cut = mutationInputPayload(layout, wireName, value, wrapper);
      if ("refused" in cut) return [{ class: "layout", pointer: "", detail: cut.refused }];
      if ("inert" in cut) return aggregateId === null ? [] : checks.aggregate(aggregateId, wire);
      const instance = (() => {
        try {
          return isRecord(cut.payload) ? mutationInputInstance(documents.get(leafId), (reference) => documents.get(reference), cut.payload) : cut.payload;
        } catch {
          return cut.payload;
        }
      })();
      const payload = checks.payload(leafId, instance);
      if (outcome.direction === "negative") return mutationNegativeVerdict(payload, outcome, checks.invariants(leafId));
      return [...payload, ...(aggregateId === null ? [] : checks.aggregate(aggregateId, wire))];
    },
  };
  return checks;
}

/**
 * ⚖️ Reads every committed mutation fixture (`<owner>/🧫️fixtures/🧬️mutations/<leaf>/<case>/🦠️mutation/🔣️.json` and every
 * other `…fixtures/…/🦠️mutation` or `🧪️tests/…/🦠️mutation` layout), maps it to its leaf (by the leaf directory the fixture path
 * names, else by the variant its payload names), cuts the leaf payload out of the aggregate wire value exactly as
 * `payload_value()` does (the layout read off the aggregate's `#[value(...)]` attribute), splices the leaf schema's root
 * `const`s back in with the manifest reader `mutationInputInstance`, and validates the result against the leaf payload schema.
 * The whole aggregate wire value is validated against the aggregate's own `🧬️mutations/🔣️.json` document where it exists (an
 * internally tagged aggregate's leaf schemas declare the tag `const`, so its `oneOf` of leaf `$ref`s admits the wire; an externally
 * or adjacently tagged aggregate wraps each leaf `$ref` in its variant key or content member). Every leaf schema under `under` is
 * also walked for `opaque` object nodes. The fixture is the witness of the Rust wire shape. A fixture whose committed outcome
 * (`<case>/🎯️outcome/🔣️.json`) refuses it with `mutation.invariant` is a negative witness (`mutationFixtureOutcome`): its leaf
 * schema must reject it, or — for a payload-intrinsic rule JSON Schema cannot express — its outcome must name an `invariant` the leaf
 * declares in `x-semio-invariant` (counted in `invariants`); else `negative`. It is counted in `negatives`, never reported
 * `invalid`, and never witnesses the leaf.
 */
export function mutationPayloadParityReport(repoRoot: string, under = ""): MutationPayloadParityReport {
  const tree = mutationTree(repoRoot);
  const documents = catalogSchemaDocuments(repoRoot);
  const { catalog } = readSchemaCatalog(repoRoot);
  const scopes = new Map(Object.entries(catalog?.scopes ?? {}).map(([id, scope]) => [scope.path, id]));
  const leafDocument = new Map<string, string>();
  for (const leaf of tree.leaves) {
    const document = readJsonObject(repoRoot, leaf.schemaPath);
    if (document === null) continue;
    const declared = typeof document.$id === "string" ? document.$id : null;
    const id = declared !== null && (!documents.has(declared) || JSON.stringify(documents.get(declared)) === JSON.stringify(document)) ? declared : `urn:semio:mutation-leaf:${leafDocument.size}`;
    documents.set(id, document);
    leafDocument.set(leaf.directory, id);
  }
  const aggregateDocument = new Map<string, string>();
  const aggregateFile = new Map<string, string | null>();
  for (const { path, name } of tree.aggregates) {
    const schemaPath = `${path.slice(0, path.lastIndexOf("/"))}/🔣️.json`;
    if (!aggregateFile.has(path)) {
      const document = readJsonObject(repoRoot, schemaPath);
      const id = document === null ? null : typeof document.$id === "string" && (!documents.has(document.$id) || JSON.stringify(documents.get(document.$id)) === JSON.stringify(document)) ? document.$id : `urn:semio:mutation-aggregate:${aggregateFile.size}`;
      if (document !== null && id !== null) documents.set(id, document);
      aggregateFile.set(path, id);
    }
    const id = aggregateFile.get(path);
    if (id === null || id === undefined) continue;
    const definitions = documents.get(id)?.$defs;
    aggregateDocument.set(`${path}|${name}`, isRecord(definitions) && isRecord(definitions[name]) ? `${id}#/$defs/${escapePointer(name)}` : id);
  }
  resolveReferencedSchemaDocuments(repoRoot, documents, tree.schemas);
  const checker = mutationPayloadChecker(documents);
  const kindPath = (segments: readonly string[]): string => segments.map(schemaKind).join("/");
  const leafByKind = new Map(tree.leaves.map((leaf) => [`${leaf.root}|${kindPath(leaf.directory.slice(leaf.root.length + 1).split("/"))}`, leaf]));
  const leafBySegments = new Map(tree.leaves.map((leaf) => [`${leaf.root}|${leaf.directory.slice(leaf.root.length + 1)}`, leaf]));
  const leafByDirectory = new Map(tree.leaves.map((leaf) => [leaf.directory, leaf]));
  const nearest = <T,>(items: readonly T[], path: (item: T) => string, target: string): T[] => {
    const scope = mutationArtifactScope(target);
    const scored = items.filter((item) => mutationArtifactScope(path(item)) === scope).map((item) => [sharedSegments(path(item), target), item] as const);
    const best = Math.max(0, ...scored.map(([score]) => score));
    return scored.filter(([score]) => score === best && best > 0).map(([, item]) => item);
  };
  const witnesses = new Map<string, number>();
  const wrapperOf = (leaf: MutationLeafRecord, aggregate: MutationAggregate): MutationLeafWrapper | null => {
    const wrapper = tree.wrappers.get(leaf.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(leaf.variant));
    return wrapper === undefined || wrapper.payloadVariant === null ? null : { layout: wrapper.layout, wireName: mutationVariantWireName(wrapper.payloadVariant, wrapper.variants.get(wrapper.payloadVariant) ?? null, wrapper.layout.renameAll) };
  };
  const aggregateOf = (leaf: MutationLeafRecord): MutationAggregate | null => {
    const candidates = nearest(tree.aggregates.filter((aggregate) => aggregate.variants.has(leaf.variant)), (aggregate) => aggregate.path, leaf.directory);
    return candidates.length > 0 && candidates.every((candidate) => JSON.stringify(candidate.layout) === JSON.stringify(candidates[0]!.layout)) ? candidates[0]! : null;
  };
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationPayloadCensusRow>();
  const row = (path: string): MutationPayloadCensusRow => {
    const owner = mutationPayloadOwner(path);
    const existing = census.get(owner) ?? { owner, leaves: 0, witnessed: 0, fixtures: 0, negatives: 0, invariants: 0, rows: 0, clean: 0, findings: {} };
    census.set(owner, existing);
    return existing;
  };
  const report = (path: string, schemaPath: string | null, findings: readonly MutationPayloadFinding[]): void => {
    const target = row(path);
    for (const finding of findings) {
      target.findings[finding.class] = (target.findings[finding.class] ?? 0) + 1;
      const scope = schemaPath === null ? null : (scopes.get(schemaPath.slice(0, schemaPath.lastIndexOf("/"))) ?? null);
      diagnostics.push({ code: "schema-mutation-payload-parity", scope, export: null, format: "🔣️jsonschema", path, detail: `${finding.class} at ${JSON.stringify(finding.pointer)}: ${finding.detail}${schemaPath === null || schemaPath === path ? "" : ` (leaf schema ${schemaPath})`}` });
    }
  };
  for (const leaf of tree.leaves) {
    if (!leaf.directory.startsWith(under)) continue;
    row(leaf.directory).leaves += 1;
    const id = leafDocument.get(leaf.directory);
    report(leaf.schemaPath, leaf.schemaPath, id === undefined ? [{ class: "unresolved", pointer: "", detail: `${leaf.schemaPath} is absent or not a JSON object` }] : checker.opaque(id));
  }
  const aggregateNode = (reference: string): Record<string, unknown> | null => {
    const [id, pointer = ""] = reference.split("#");
    let node: unknown = documents.get(id!);
    for (const segment of pointer.split("/").slice(1)) node = isRecord(node) ? node[unescapePointer(segment)] : undefined;
    return isRecord(node) ? node : null;
  };
  for (const aggregate of tree.aggregates) {
    if (!aggregate.path.startsWith(under)) continue;
    const schemaPath = `${aggregate.path.slice(0, aggregate.path.lastIndexOf("/"))}/🔣️.json`;
    const reference = aggregateDocument.get(`${aggregate.path}|${aggregate.name}`);
    const findings: MutationPayloadFinding[] = [];
    const refuse = (detail: string): void => void findings.push({ class: "aggregate", pointer: "", detail });
    const expected: Record<string, unknown>[] = [];
    for (const [variant, rename] of aggregate.variants) {
      const wireName = mutationVariantWireName(variant, rename, aggregate.layout.renameAll);
      const leaves = nearest(tree.leaves.filter((leaf) => leaf.variant === variant), (leaf) => leaf.directory, aggregate.path);
      const id = leaves.length === 1 ? leafDocument.get(leaves[0]!.directory) : undefined;
      if (leaves.length !== 1) refuse(`${aggregate.name}::${variant} has ${leaves.length} mutation leaves near ${aggregate.path}, not one`);
      else if (id === undefined || id.startsWith("urn:semio:")) refuse(`the leaf of ${aggregate.name}::${variant} (${leaves[0]!.schemaPath}) declares no unique $id`);
      else {
        const tag = aggregate.layout.tag;
        const leafSchema = documents.get(id);
        const pinsTag = (node: unknown): boolean => { const properties=isRecord(node)?node.properties:undefined;const property=isRecord(properties)&&tag!==null?properties[tag]:undefined;return isRecord(property)&&property.const===wireName; };
        const variants = isRecord(leafSchema) ? (Array.isArray(leafSchema.oneOf) ? leafSchema.oneOf : Array.isArray(leafSchema.anyOf) ? leafSchema.anyOf : []) : [];
        if (tag !== null && aggregate.layout.content === null && !(pinsTag(leafSchema) || (variants.length > 0 && variants.every(pinsTag)))) refuse(`the leaf of ${aggregate.name}::${variant} does not declare the tag ${JSON.stringify(tag)} as const ${JSON.stringify(wireName)}`);
        const wrapper = tree.wrappers.get(leaves[0]!.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(variant) && candidate.payloadVariant !== null);
        expected.push(mutationAggregateBranch(aggregate.layout, wireName, id, wrapper?.name ?? null));
      }
    }
    const node = reference === undefined ? null : aggregateNode(reference);
    if (reference === undefined) refuse(`#[derive(Mutations)] enum ${aggregate.name} has no aggregate schema ${schemaPath}`);
    else if (reference.startsWith("urn:semio:")) refuse(`${schemaPath} declares no $id`);
    else if (findings.length === 0 && canonicalJson(node?.oneOf) !== canonicalJson(expected)) refuse(`the oneOf of ${reference} is not the ${expected.length}-branch union the #[value(...)] layout of ${aggregate.name} and its leaf $ids derive`);
    report(aggregate.path, schemaPath, findings);
  }
  for (const fixture of tree.fixtures.sort()) {
    if (!fixture.startsWith(under)) continue;
    const target = row(fixture);
    target.fixtures += 1;
    const outcome = mutationFixtureOutcome(readJsonObject(repoRoot, `${fixture.slice(0, fixture.lastIndexOf("/🦠️mutation/"))}/🎯️outcome/🔣️.json`));
    if (outcome.direction === "negative") target.negatives += 1;
    const segments = fixture.split("/").slice(0, -2);
    const anchor = segments.findLastIndex((segment) => segment.endsWith("fixtures") || segment === "🧪️tests");
    const owner = segments.slice(0, anchor).join("/");
    const value: unknown = (() => {
      try {
        return JSON.parse(readFileSync(join(repoRoot, fixture), "utf8"));
      } catch {
        return undefined;
      }
    })();
    const named = segments.slice(anchor + 1).filter((segment) => segment !== "🧬️mutations");
    const windows = named.flatMap((_, start) => named.map((__, end) => named.slice(start, named.length - end)).filter((window) => window.length > 0));
    const shortened = (window: readonly string[]): MutationLeafRecord | undefined => {
      const root = `${owner}/🧬️schema/🧬️mutations`;
      const matches = tree.leaves.filter((candidate) => candidate.root === root && candidate.directory.slice(root.length + 1).startsWith(`${window.join("/")}-`));
      return matches.length === 1 ? matches[0] : undefined;
    };
    let leaf =
      leafByDirectory.get(owner) ??
      windows.map((window) => leafBySegments.get(`${owner}/🧬️schema/🧬️mutations|${window.join("/")}`)).find((found) => found !== undefined) ??
      windows.map(shortened).find((found) => found !== undefined) ??
      windows.map((window) => leafByKind.get(`${owner}/🧬️schema/🧬️mutations|${kindPath(window)}`)).find((found) => found !== undefined);
    if (leaf === undefined)
      for (const aggregate of nearest(tree.aggregates, (candidate) => candidate.path, fixture))
        for (const [variant, rename] of aggregate.variants)
          if ("payload" in mutationLeafPayload(aggregate.layout, mutationVariantWireName(variant, rename, aggregate.layout.renameAll), mutationFixtureWire(value))) leaf ??= nearest(tree.leaves.filter((candidate) => candidate.variant === variant), (candidate) => candidate.directory, aggregate.path)[0];
    if (leaf === undefined) {
      report(fixture, null, [{ class: "unmapped", pointer: "", detail: value === undefined ? "the fixture is not JSON" : "no mutation leaf matches the fixture's path, and no nearby aggregate declares the variant its payload names" }]);
      continue;
    }
    const aggregate = aggregateOf(leaf);
    const id = leafDocument.get(leaf.directory);
    if (aggregate === null && id !== undefined && readJsonObject(repoRoot, `${leaf.directory}/🔣️.json`)?.composition === "composite") {
      const payload = checker.payload(id, value);
      const findings = outcome.direction === "negative" ? mutationNegativeVerdict(payload, outcome, checker.invariants(id)) : payload;
      if (outcome.direction === "negative" && outcome.invariant !== null && checker.invariants(id).has(outcome.invariant)) target.invariants += 1;
      if (outcome.direction === "positive") witnesses.set(leaf.directory, (witnesses.get(leaf.directory) ?? 0) + 1);
      if (findings.length === 0) target.clean += 1;
      report(fixture, leaf.schemaPath, findings);
      continue;
    }
    if (aggregate === null || id === undefined) {
      report(fixture, leaf.schemaPath, [{ class: aggregate === null ? "unmapped" : "unresolved", pointer: "", detail: aggregate === null ? `no single #[derive(Mutations)] aggregate near ${leaf.directory} declares the variant ${leaf.variant}` : `${leaf.schemaPath} is absent or not a JSON object` }]);
      continue;
    }
    const wireName = mutationVariantWireName(leaf.variant, aggregate.variants.get(leaf.variant) ?? null, aggregate.layout.renameAll);
    const wrapper = wrapperOf(leaf, aggregate);
    const findings = checker.fixture(aggregate.layout, wireName, id, aggregateDocument.get(`${aggregate.path}|${aggregate.name}`) ?? null, value, wrapper, outcome).map((finding) => (finding.class === "layout" ? { ...finding, detail: `${finding.detail} (${aggregate.name} in ${aggregate.path})` } : finding));
    if (outcome.direction === "negative" && outcome.invariant !== null && checker.invariants(id).has(outcome.invariant)) target.invariants += 1;
    if (outcome.direction === "positive" && "payload" in mutationInputPayload(aggregate.layout, wireName, value, wrapper)) witnesses.set(leaf.directory, (witnesses.get(leaf.directory) ?? 0) + 1);
    if (findings.length === 0) target.clean += 1;
    report(fixture, leaf.schemaPath, findings);
  }
  const standardScope = (path: string): string => {
    const segments = path.split("/");
    const standard = segments.indexOf("🏅️standards");
    return standard >= 0 && standard + 1 < segments.length ? segments.slice(0, standard + 2).join("/") : path;
  };
  const aggregatePathById = new Map([...aggregateFile].flatMap(([path, id]) => (id === null ? [] : [[id, path] as const])));
  const reexportedVocabularies = (owner: string): string[] => {
    const document = readJsonObject(repoRoot, `${owner}/🧬️schema/🧬️mutations/🔣️.json`);
    if (document === null || document.oneOf !== undefined || !Array.isArray(document.allOf)) return [];
    return document.allOf.flatMap((branch) => (isRecord(branch) && typeof branch.$ref === "string" ? [aggregatePathById.get(branch.$ref)] : [])).filter((path): path is string => path !== undefined);
  };
  for (const feature of tree.features.sort()) {
    if (!feature.startsWith(under)) continue;
    const owner = feature.slice(0, feature.indexOf("/🧪️tests/"));
    const scopes = [owner, ...reexportedVocabularies(owner)].map(standardScope);
    for (const entry of mutationFeatureRows(readFileSync(join(repoRoot, feature), "utf8"))) {
      const target = row(feature);
      target.rows += 1;
      const where = `row ${JSON.stringify(entry.kind)} at line ${entry.line}`;
      const leaf = nearest(tree.leaves.filter((candidate) => candidate.kind === entry.kind && scopes.some((scope) => candidate.directory.startsWith(scope))), (candidate) => candidate.directory, owner)[0];
      const id = leaf === undefined ? undefined : leafDocument.get(leaf.directory);
      if (leaf === undefined || id === undefined) {
        report(feature, null, [{ class: "unmapped", pointer: "", detail: `${where}: no mutation leaf of kind ${JSON.stringify(entry.kind)} under ${scopes.join(" or ")}` }]);
        continue;
      }
      const payload = entry.params;
      const instance = (() => {
        try {
          return isRecord(payload) ? mutationInputInstance(documents.get(id), (reference) => documents.get(reference), payload) : payload;
        } catch {
          return payload;
        }
      })();
      const findings = checker.payload(id, instance).map((finding) => ({ ...finding, detail: `${where}: ${finding.detail}` }));
      witnesses.set(leaf.directory, (witnesses.get(leaf.directory) ?? 0) + 1);
      if (findings.length === 0) target.clean += 1;
      report(feature, leaf.schemaPath, findings);
    }
  }
  for (const leaf of tree.leaves) {
    if (!leaf.directory.startsWith(under)) continue;
    if (witnesses.has(leaf.directory)) row(leaf.directory).witnessed += 1;
    else report(leaf.schemaPath, leaf.schemaPath, [{ class: "unwitnessed", pointer: "", detail: `no committed wire fixture (…fixtures/…/🦠️mutation/🔣️.json, the Rust ToValue of an aggregate op of this variant, editable payload variant for a wrapped leaf) witnesses ${leaf.directory}, so its schema is unchecked against the wire` }]);
  }
  for (const aggregate of tree.aggregates) {
    if (!aggregate.path.startsWith(under)) continue;
    const covered = [...aggregate.variants.keys()].some((variant) => nearest(tree.leaves.filter((leaf) => leaf.variant === variant), (leaf) => leaf.directory, aggregate.path).some((leaf) => witnesses.has(leaf.directory)));
    if (!covered) report(aggregate.path, null, [{ class: "unwitnessed", pointer: "", detail: `no committed wire fixture witnesses any of the ${aggregate.variants.size} variants of #[derive(Mutations)] enum ${aggregate.name}` }]);
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => Object.values(right.findings).reduce((sum, count) => sum + count, 0) - Object.values(left.findings).reduce((sum, count) => sum + count, 0) || left.owner.localeCompare(right.owner)) };
}

/**
 * ⚖️ `test schema mutation-payloads` — the `schema-mutation-payload-parity` gate: every committed mutation fixture's leaf payload
 * validates against, and is fully described by, its leaf payload schema, and no leaf schema holds an opaque object node.
 * `--census` prints the per-plugin table and always exits 0; without it any finding fails.
 *
 *   bun 📜️script.ts schema mutation-payloads [--census] [--under <path>] [--json]
 */
export function runMutationPayloadParity(repoRoot: string, segments: string[]): void {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationPayloadParityReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : report, null, 2));
    {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
  }
  const total = report.census.reduce((sum, row) => ({ leaves: sum.leaves + row.leaves, witnessed: sum.witnessed + row.witnessed, fixtures: sum.fixtures + row.fixtures + row.rows, negatives: sum.negatives + row.negatives, invariants: sum.invariants + row.invariants, clean: sum.clean + row.clean }), { leaves: 0, witnessed: 0, fixtures: 0, negatives: 0, invariants: 0, clean: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.findings)))].sort();
    console.log(["owner", "leaves", "witnessed", "fixtures", "negatives", "invariants", "rows", "clean", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.leaves, row.witnessed, row.fixtures, row.negatives, row.invariants, row.rows, row.clean, ...classes.map((name) => row.findings[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-payloads]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-payloads] ${total.clean}/${total.fixtures} fixture payload(s) and wire-form feature row(s) meet their leaf schema (${total.negatives} negative witness(es) it must reject, ${total.invariants} of them by a declared x-semio-invariant); ${total.witnessed}/${total.leaves} leaves witnessed; ${report.diagnostics.length} schema-mutation-payload-parity finding(s)${under === "" ? "" : ` under ${under}`}`);
  {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
}
//#endregion ⚖️MutationPayloadParity
