import { dirname, resolve } from "node:path";
import { rustTokenPairs, rustTokenSegments, rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { relativePosix } from "../📝️render/🟦️.ts";

/** 🧾️ One companion leaf that leaves the compiled closure and is served from its repository document. */
export type ExternalizedLeaf = Readonly<{ scope: string; scopeIsExpression: boolean; export: string; format: "typescript" | "graphql" | "protobuf" | "jsonschema" | "rust"; path: string }>;

const FORMAT_FIELDS: Readonly<Record<string, ExternalizedLeaf["format"]>> = { typescript: "typescript", graphql: "graphql", proto: "protobuf" };
const JSON_FORMAT_FIELDS: Readonly<Record<string, ExternalizedLeaf["format"]>> = { ...FORMAT_FIELDS, json_schema: "jsonschema" };
const RUST_FORMAT_FIELDS: Readonly<Record<string, ExternalizedLeaf["format"]>> = { rust: "rust" };
const FORMAT_VARIANTS: Readonly<Record<ExternalizedLeaf["format"], string>> = { typescript: "Typescript", graphql: "Graphql", protobuf: "Protobuf", jsonschema: "JsonSchema", rust: "Rust" };
const FIXED_FACETS: ReadonlySet<string> = new Set(["artifact", "snapshot", "diff", "mutations", "inference", "config", "presence"]);

/** 🔤️ The string literals of a field value that is one `include_str!("..")` or a `concat!` of several, in order; null for any other value. */
function includedStrings(tokens: ReturnType<typeof rustTokens>, pairs: ReturnType<typeof rustTokenPairs>, from: number, to: number): string[] | null {
  const include = (a: number, b: number): string | null =>
    b - a === 5 && tokens[a]?.text === "include_str" && tokens[a + 1]?.text === "!" && tokens[a + 2]?.text === "(" && tokens[a + 3]?.kind === "string" && tokens[a + 4]?.text === ")" ? tokens[a + 3]!.text : null;
  const single = include(from, to);
  if (single) return [single];
  if (tokens[from]?.text !== "concat" || tokens[from + 1]?.text !== "!" || tokens[from + 2]?.text !== "(" || pairs.get(from + 2) !== to - 1) return null;
  const literals: string[] = [];
  for (const [a, b] of rustTokenSegments(tokens, pairs, from + 3, to - 1, ",")) {
    if (a === b) continue;
    const literal = include(a, b);
    if (!literal) return null;
    literals.push(literal);
  }
  return literals.length ? literals : null;
}

/** 🍃️ Replaces the typescript, graphql and proto `include_str!` companions of every `ArtifactSchemaDescriptor` facet (one `include_str!` or a `concat!` of several, served in order) by empty bodies and registers the documents they came from, so the registering crate embeds only its Rust leaf and its normative JSON Schema. With `options.json` the JSON Schema leaf is served too, for a plugin whose guest receives the documents as package assets (`PackageAssetSchemaLeaves`). */
export function externalizeFacetLeaves(text: string, fileAbs: string, repoRoot: string, options: Readonly<{ json?: boolean; rust?: boolean }> = {}): { text: string; leaves: ExternalizedLeaf[] } | null {
  const fields = { ...(options.json === true ? JSON_FORMAT_FIELDS : FORMAT_FIELDS), ...(options.rust === true ? RUST_FORMAT_FIELDS : {}) };
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const edits: { start: number; end: number; text: string }[] = [];
  const leaves: ExternalizedLeaf[] = [];
  const insertions = new Map<number, ExternalizedLeaf[]>();
  for (let index = 0; index + 1 < tokens.length; index += 1) {
    if (tokens[index]!.text !== "FacetLeaves" || tokens[index + 1]!.text !== "{") continue;
    let head = index;
    while (tokens[head - 1]?.text === "::" && tokens[head - 2]?.kind === "identifier") head -= 2;
    if (tokens[head - 1]?.text === "::") head -= 1;
    if (tokens[head - 1]?.text !== ":" || tokens[head - 2]?.kind !== "identifier") continue;
    const facet = tokens[head - 2]!.text;
    if (!FIXED_FACETS.has(facet)) continue;
    let open = head - 3;
    for (; open >= 0; open -= 1) if (tokens[open]!.text === "{" && (pairs.get(open) ?? -1) > index) break;
    if (open < 0) continue;
    let scope: string | null = null;
    let scopeIsExpression = false;
    const close = pairs.get(open)!;
    for (let cursor = open + 1; cursor < close; cursor += 1) {
      const skip = pairs.get(cursor);
      if (skip !== undefined && skip > cursor) { cursor = skip; continue; }
      if (tokens[cursor]!.text !== "id" || tokens[cursor + 1]?.text !== ":") continue;
      if (tokens[cursor + 2]?.kind === "string") { scope = tokens[cursor + 2]!.text.slice(1, -1); break; }
      let end = cursor + 2;
      while (end < close && tokens[end]!.text !== ",") end += 1;
      const expression = tokens.slice(cursor + 2, end);
      if (expression.length && expression.every((token, position) => (position % 2 === 0 ? token.kind === "identifier" : token.text === "::"))) { scope = expression.map((token) => token.text).join(""); scopeIsExpression = true; }
      break;
    }
    if (!scope) continue;
    const literalClose = pairs.get(index + 1)!;
    const local: ExternalizedLeaf[] = [];
    for (const [a, b] of rustTokenSegments(tokens, pairs, index + 2, literalClose, ",")) {
      const field = tokens[a]?.text ?? "";
      const format = fields[field];
      if (!format || tokens[a + 1]?.text !== ":") continue;
      const strings = includedStrings(tokens, pairs, a + 2, b);
      if (!strings) continue;
      edits.push({ start: tokens[a + 2]!.start, end: tokens[b - 1]!.end, text: '""' });
      for (const literal of strings) local.push({ scope, scopeIsExpression, export: facet, format, path: relativePosix(repoRoot, resolve(dirname(fileAbs), literal.slice(1, -1))) });
    }
    if (!local.length) continue;
    leaves.push(...local);
    let start = open;
    while (tokens[start - 1]?.text === "::" || (tokens[start - 1]?.kind === "identifier" && tokens[start - 2]?.text === "::")) start -= tokens[start - 1]?.text === "::" ? 1 : 2;
    insertions.set(tokens[start]!.start, [...(insertions.get(tokens[start]!.start) ?? []), ...local]);
  }
  if (!leaves.length) return null;
  for (const [offset, entries] of insertions) {
    const lineStart = text.lastIndexOf("\n", offset - 1) + 1;
    const indent = /^[ \t]*/u.exec(text.slice(lineStart, offset))![0];
    const rows = entries.map((leaf) => `${indent}    ::semio_framework_schema_registry::SchemaLeafDocument { scope: ${leaf.scopeIsExpression ? leaf.scope : JSON.stringify(leaf.scope)}, export: ${JSON.stringify(leaf.export)}, format: ::semio_framework_schema_registry::SchemaFormat::${FORMAT_VARIANTS[leaf.format]}, path: ${JSON.stringify(leaf.path)} },`);
    edits.push({ start: lineStart, end: lineStart, text: `${indent}::semio_framework_schema_registry::register_schema_leaf_documents(&[\n${rows.join("\n")}\n${indent}]);\n` });
  }
  let out = text;
  for (const edit of edits.sort((x, y) => y.start - x.start || y.end - x.end)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return { text: out, leaves };
}
