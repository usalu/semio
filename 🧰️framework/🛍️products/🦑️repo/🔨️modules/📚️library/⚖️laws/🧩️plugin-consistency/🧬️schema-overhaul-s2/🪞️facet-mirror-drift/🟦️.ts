import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { canonicalPrimaryFilenameForKind, loadCatalogTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { policySnakeToCamel } from "../../../../🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts";
import { policyStripTsCommentsAndStrings } from "../../🚧️authority-bans/🟦️.ts";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { policyListStdioSchemaOwningEntries } from "../📇️stdio-entries/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyNormalizeRelPath } from "../../🏛️foundation/🟦️.ts";

/**
 * 📏️S-8 rule 1 (`POLICY_FACET_MIRROR_DRIFT`): for every stdio standard's snapshot/diff/mutations
 * facet, the camelCased public field identifiers found in the Rust `.rs` leaf should also appear
 * (textually) in the sibling `.ts`/`.graphql`/`.json`/`.proto` leaves — textual heuristic (substring
 * presence), not a real cross-language type check, matching this repo's other facet-parity policies.
 * Bidirectional: this also harvests each sibling leaf's OWN declared field names (interface members,
 * GraphQL type/input fields, JSON Schema `properties` keys, proto message fields) and flags any that
 * have no corresponding Rust field — this is what catches a sibling still shaped like the generic W1b
 * scaffold (`entries: [{key, value}]`) sitting where real fields belong; the forward direction alone
 * would pass that scaffold as long as it also happened to textually contain the real field names.
 * Seeded with the current census: ALL 93 checked (standard,facet) pairs drift today (31 standards × 3
 * facets — none of this program's facet mirrors have been rewritten yet; e.g. gif's TS mirror is
 * still literally zip's, per the master plan's own opening finding). F-wave agents shrink this to
 * zero as they rewrite each artifact's facets for real; this wave does NOT fix any facet itself.
 */
const POLICY_FACET_MIRROR_DRIFT_FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;

const POLICY_FACET_MIRROR_DRIFT_SIBLINGS = ["typescript-source", "graphql", "json", "protobuf"].map((kindId) => canonicalPrimaryFilenameForKind(kindId, loadCatalogTaxonomy()));

const POLICY_FACET_MIRROR_DRIFT_FIELD_RE = /(?:^|[\s{,(])(?:pub\s+)?([a-z][a-z0-9_]*)\s*:\s*[A-Za-z_&\[<('"]/gm;

const POLICY_FACET_MIRROR_DRIFT_KEYWORDS = new Set(["self", "where", "if", "else", "match", "for", "while", "let", "fn", "return", "in", "as", "dyn", "mut", "ref", "impl", "type"]);

/** 🔎️CamelCased public field/variant identifiers referenced in a schema facet's Rust leaf (textual heuristic, not a parser — matches struct field decls AND enum struct-variant field decls). */
function policyFacetRustFieldNames(content: string): string[] {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_FIELD_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_FIELD_RE.exec(stripped))) {
    const n = m[1]!;
    if (POLICY_FACET_MIRROR_DRIFT_KEYWORDS.has(n) || n.startsWith("r#")) continue;
    names.add(n);
  }
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

//#region 🔧️PolicyFacetMirrorDriftReverse
const POLICY_FACET_MIRROR_DRIFT_TS_RE = /(?:^|[\s{;(])([A-Za-z_][A-Za-z0-9_]*)\??\s*:\s*[^;\n=]+[;\n]/gm;

const POLICY_FACET_MIRROR_DRIFT_TS_KEYWORDS = new Set([
  "interface", "type", "export", "import", "from", "extends", "implements", "class", "function", "const", "let", "var",
  "namespace", "module", "declare", "readonly", "public", "private", "protected", "static", "abstract", "new", "this",
  "super", "typeof", "keyof", "case", "default", "switch", "try", "catch", "finally", "throw", "async", "await", "yield",
  "get", "set", "constructor", "void", "never", "unknown", "any", "undefined", "null", "true", "false", "satisfies",
  "asserts", "is", "infer", "of", "in", "as", "if", "else", "for", "while", "return",
]);

const POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE = /^\s*([a-z][A-Za-z0-9_]*)\s*:/gm;

const POLICY_FACET_MIRROR_DRIFT_PROTO_RE = /^\s*(?:optional\s+|repeated\s+)?(?:map\s*<[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/gm;

const POLICY_FACET_MIRROR_DRIFT_PROTO_SKIP_RE = /^\s*(enum|oneof|message|package|syntax|import)\b/;

const POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE = /#\[serde\([^\]]*\btag\s*=\s*"([a-zA-Z_][a-zA-Z0-9_]*)"/g;

/** ⚖️Below-which a sibling field name is too short to trust as a substring match against a real Rust field (avoids "id"/"at"-style accidental hits) — see `policyFacetMirrorDriftBreaches`'s extraFields step. */
const POLICY_FACET_MIRROR_DRIFT_SUBSTRING_MIN_LEN = 4;

/**
 * 🔎️Serde internally-tagged-enum discriminant values (`#[serde(tag = "kind")]`) found in a schema
 * facet's Rust leaf. These are real fields every non-Rust mirror must carry (GraphQL/JSON-Schema/
 * proto have no native tagged-union sugar, so they always spell the discriminant out as an actual
 * field) even though `policyFacetRustFieldNames` never sees them — there is no `kind: SomeType`
 * struct-field declaration in the Rust source, just this attribute. Read raw (not comment/string
 * stripped) since the tag value IS a string literal; the regex is narrow enough not to misfire
 * inside a line or block comment in practice.
 */
function policyFacetRustTagFieldNames(content: string): string[] {
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE.exec(content))) names.add(m[1]!);
  return [...names];
}

const POLICY_FACET_MIRROR_DRIFT_VARIANT_RE = /^\s*([A-Z][A-Za-z0-9]*)\s*[{(]/gm;

/**
 * 🔎️PascalCase enum variant names declared in a schema facet's Rust leaf (e.g. `enum BmpMutation {
 * SetHeaderFields { .. }, SetPixelData { .. }, .. }`, or a plain `Rgb { r, g, b }` color variant).
 * `policyFacetRustFieldNames` only sees the FIELDS a struct/struct-variant declares, never the
 * variant name itself — but externally-tagged (the serde default, no `#[serde(tag = "…")]`) or
 * `oneof`-shaped mirrors spell the variant name out as a real field/message/oneof-arm identifier
 * (proto's `SetHeaderFields set_header_fields = 3;`, a GraphQL union member, a JSON discriminant
 * value, …), same problem `policyFacetRustTagFieldNames` solves for the internally-tagged case.
 * Line-start heuristic (`PascalCaseIdent` immediately followed by `{`/`(`) — true parsing would
 * need brace-tracked enum-body awareness this file's other policy rules don't attempt either; a
 * stray match (an `Ok(`/`Some(`/struct-literal-construction hit) only ever ADDS a permitted name,
 * never suppresses a real breach, so the cost of over-matching here is small.
 */
function policyFacetRustVariantFieldNames(content: string): string[] {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_VARIANT_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_VARIANT_RE.exec(stripped))) {
    const n = m[1]!;
    names.add(n.charAt(0).toLowerCase() + n.slice(1));
  }
  return [...names];
}

/** 🔎️Field names a `.ts` sibling mirror leaf itself declares (interface/type member lines), for the reverse extra-field drift check. */
function policyFacetTsFieldNames(content: string): string[] {
  const stripped = policyStripTsCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_TS_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_TS_RE.exec(stripped))) {
    const n = m[1]!;
    if (POLICY_FACET_MIRROR_DRIFT_TS_KEYWORDS.has(n)) continue;
    names.add(n);
  }
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.graphql` sibling mirror leaf itself declares (type/input field lines), for the reverse extra-field drift check. */
function policyFacetGraphqlFieldNames(content: string): string[] {
  const noComments = content.replace(/#.*$/gm, "");
  const noArgs = noComments.replace(/\([^)]*\)/g, "");
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE.exec(noArgs))) names.add(m[1]!);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.json` sibling mirror leaf itself declares (every key under any `properties` object, including nested under `definitions`/`$defs`), for the reverse extra-field drift check. Throws if the file isn't valid JSON — callers must treat that as its own breach, not a crash. */
function policyFacetJsonFieldNames(content: string): string[] {
  const root: unknown = JSON.parse(content);
  const names = new Set<string>();
  const visit = (node: unknown): void => {
    if (!node || typeof node !== "object") return;
    if (Array.isArray(node)) {
      for (const item of node) visit(item);
      return;
    }
    const obj = node as Record<string, unknown>;
    if (obj.properties && typeof obj.properties === "object" && !Array.isArray(obj.properties)) {
      for (const key of Object.keys(obj.properties as Record<string, unknown>)) names.add(key);
    }
    for (const val of Object.values(obj)) visit(val);
  };
  visit(root);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.proto` sibling mirror leaf itself declares (numbered message fields, skipping enum/oneof/message/package/syntax/import lines), for the reverse extra-field drift check. */
function policyFacetProtoFieldNames(content: string): string[] {
  const bodyLines = content.split(/\r?\n/).filter((line) => !POLICY_FACET_MIRROR_DRIFT_PROTO_SKIP_RE.test(line));
  const body = bodyLines.join("\n");
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_PROTO_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_PROTO_RE.exec(body))) names.add(m[1]!);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Dispatches to the right per-language harvester by the sibling's own filename suffix. Returns `[]` for a suffix none of the harvesters own (defensive; every current entry in `POLICY_FACET_MIRROR_DRIFT_SIBLINGS` is covered). */
function policyFacetSiblingFieldNames(sib: string, content: string): string[] {
  if (sib.endsWith(".ts")) return policyFacetTsFieldNames(content);
  if (sib.endsWith(".graphql")) return policyFacetGraphqlFieldNames(content);
  if (sib.endsWith(".json")) return policyFacetJsonFieldNames(content);
  if (sib.endsWith(".proto")) return policyFacetProtoFieldNames(content);
  return [];
}

//#endregion 🔧️PolicyFacetMirrorDriftReverse

const POLICY_FACET_MIRROR_DRIFT_ALLOWLIST = new Set<string>([
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-component",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-component",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-component",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-component",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-component",
  "stdio/bmp/standards#v3-subsets-any-schema-snapshot-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-component",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-component",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-component",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-component",
  "stdio/gif/standards#87a-subsets-any-schema-diff-component",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-component",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-component",
  "stdio/gif/standards#89a-subsets-any-schema-diff-component",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-component",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-diff-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-snapshot-component",
  "stdio/ifc/standards#4-subsets-any-schema-diff-component",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-component",
  "stdio/ifc/standards#4-subsets-any-schema-snapshot-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-snapshot-component",
  "stdio/las/standards#1.0-subsets-any-schema-diff-component",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/las/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/md/standards#commonmark-subsets-any-schema-diff-component",
  "stdio/md/standards#commonmark-subsets-any-schema-mutations-component",
  "stdio/md/standards#commonmark-subsets-any-schema-snapshot-component",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-component",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-component",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-diff-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-mutations-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-component",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-component",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/ply/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/png/standards#1.2-subsets-any-schema-diff-component",
  "stdio/png/standards#1.2-subsets-any-schema-mutations-component",
  "stdio/png/standards#1.2-subsets-any-schema-snapshot-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/step/standards#ap214-subsets-any-schema-diff-component",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-component",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-component",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-component",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-component",
  "stdio/stl/standards#ascii-subsets-any-schema-snapshot-component",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-component",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-component",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-diff-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-mutations-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-snapshot-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/binary/standards#raw-subsets-any-schema-diff-component",
  "stdio/binary/standards#raw-subsets-any-schema-mutations-component",
  "stdio/binary/standards#raw-subsets-any-schema-snapshot-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-diff-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-mutations-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-snapshot-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-diff-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-mutations-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-snapshot-component",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-component",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-component",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-component",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-component",
]);

export function policyFacetMirrorDriftBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_FACET_MIRROR_DRIFT_FACETS) {
      const facetRel = `${entry.subsetRel}/🧬️schema/${facet}`;
      const rustRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (!existsSync(join(repoRoot, rustRel))) continue;
      const rustContent = policyReadFileSafe(repoRoot, rustRel);
      const camelFields = policyFacetRustFieldNames(rustContent);
      // 🩹 compareFields also folds in serde tag discriminants (`#[serde(tag = "kind")]`) and
      // camelCased enum variant names — real fields every non-Rust mirror must spell out even
      // though no Rust struct literally declares them as a named field (see
      // policyFacetRustTagFieldNames's and policyFacetRustVariantFieldNames's docstrings;
      // confirmed against stdio.json/png/bmp's real, handcrafted mirrors during this rule's own
      // verification).
      const compareFields = [...camelFields, ...policyFacetRustTagFieldNames(rustContent), ...policyFacetRustVariantFieldNames(rustContent)];
      const missingBySibling: string[] = [];
      for (const sib of POLICY_FACET_MIRROR_DRIFT_SIBLINGS) {
        const sibAbs = join(repoRoot, facetRel, sib);
        if (!existsSync(sibAbs)) {
          missingBySibling.push(`${sib}:MISSING_FILE`);
          continue;
        }
        const sibContent = readFileSync(sibAbs, "utf8");
        const missingFields = camelFields.filter((f) => !sibContent.includes(f));
        if (missingFields.length > 0) missingBySibling.push(`${sib}:${missingFields.length}`);
        let siblingFields: string[];
        try {
          siblingFields = policyFacetSiblingFieldNames(sib, sibContent);
        } catch {
          missingBySibling.push(`${sib}:PARSE_ERROR`);
          continue;
        }
        // 🩹 A sibling field counts as "real" if it exactly matches a Rust/tag field, OR visibly
        // CONTAINS one as a substring (e.g. GraphQL/proto disambiguate same-named enum-variant
        // fields with a type prefix — stdio.json's real mirror spells Rust's plain `value` field
        // as `boolValue`/`stringValue` per variant — mirroring the forward check's own substring
        // leniency, just in the other direction). Case-INSENSITIVE on purpose: the disambiguating
        // prefix capitalizes the embedded real field's first letter (`arrayItems`, not
        // `arrayitems`), so a case-sensitive substring test would miss it entirely. The length
        // floor keeps this from rubber-stamping every sibling field against a trivially short real
        // field name.
        const extraFields = siblingFields.filter((f) => {
          if (f === "schema" || compareFields.includes(f)) return false;
          const fLower = f.toLowerCase();
          return !compareFields.some((cf) => cf.length >= POLICY_FACET_MIRROR_DRIFT_SUBSTRING_MIN_LEN && fLower.includes(cf.toLowerCase()));
        });
        if (extraFields.length > 0) missingBySibling.push(`${sib}:extra:${extraFields.length}`);
      }
      const normalized = policyNormalizeRelPath(rustRel);
      const allowlisted = POLICY_FACET_MIRROR_DRIFT_ALLOWLIST.has(normalized);
      if (missingBySibling.length > 0) {
        if (allowlisted) continue;
        breaches.push({
          id: `facet-mirror-drift-${rustRel}`,
          summary: `"${facetRel}" siblings drift from the Rust leaf's real fields — missing and/or extra (${missingBySibling.join(", ")})`,
          kind: "stdio-artifacts/facet-mirror-drift",
          scope: entry.artRel,
          priority: "low",
          reason: "Every camelCased field the Rust leaf declares should also appear in its sibling .ts/.graphql/.json/.proto leaves, AND every field a sibling declares should correspond to a real Rust field — a stale/copy-pasted mirror (missing real fields) or a leftover generic scaffold (extra fields with no Rust counterpart, e.g. entries/key/value) both silently drift from the real shape otherwise (see the master plan's gif-TS-mirror-is-literally-zip's finding).",
          solution: `Rewrite ${facetRel}'s sibling leaves to mirror the Rust leaf's real fields exactly (no missing, no extra), or if this artifact/facet hasn't been reached yet, add "${normalized}" to POLICY_FACET_MIRROR_DRIFT_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `facet-mirror-drift-stale-${rustRel}`,
          summary: `"${facetRel}" is allowlisted in POLICY_FACET_MIRROR_DRIFT_ALLOWLIST but its siblings no longer drift`,
          kind: "stdio-artifacts/facet-mirror-drift",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_FACET_MIRROR_DRIFT_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}
