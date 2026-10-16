/** 🏷️ Mutation label gate
 *
 * `schema-mutation-label`: every leaf is labelled natively or forwarded, in every locale.
 */

import { type SchemaDiagnostic } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { rustCallArguments, rustFirstLiteral, rustFunctions, rustGroupEnd, rustImplBlocks, rustLex, rustPathAt, type RustToken } from "../🦀️rust-tokens/🟦️.ts";
import { mutationPayloadOwner, rustSources } from "../🌳️mutation-tree/🟦️.ts";

//#region 🏷️MutationLabels
/** 🩺️ The classes of a `schema-mutation-label` finding: an app overriding the leaf label (`labelOverride`), a leaf label that is
 * locale-invariant data (`labelLocaleInvariant`), a locale argument that is the empty literal (`labelLocaleEmpty`), an operation's
 * text line used as a label (`labelOpText`), a label body the gate cannot read as one of the admitted shapes (`labelUnresolved`), and a
 * history-row label written by hand on an emission — `Emit::commit(mutations, label)` or an `Emit { description: … }` literal
 * (`labelHandwritten`, design §20.4). */
export type MutationLabelFindingClass = "labelOverride" | "labelLocaleInvariant" | "labelLocaleEmpty" | "labelOpText" | "labelUnresolved" | "labelHandwritten";

/** 🏷️ How one label body names its operation: `LocalizedLabel::native(en, de)` (`native`), a forward to another label (`forward`), or a finding. */
export type MutationLabelVerdict = "native" | "forward" | MutationLabelFindingClass;

/** 🏷️ One label site of a Rust source: the trait it implements (`MutationKind`, `CompositeMutationKind`, `SemanticMutation`, an app
 * trait for an override, or `data` for a `LocalizedLabel::data` call), its implementor, the verdict and every `[en, de]` template read. */
export type MutationLabelSite = Readonly<{ path: string; trait: string; implementor: string; verdict: MutationLabelVerdict; texts: readonly (readonly [string | null, string | null])[] }>;

/** 📊️ One plugin's share of the label census: label bodies read, `native` and `forward` ones, and the findings per class. */
export type MutationLabelCensusRow = { readonly owner: string; labels: number; native: number; forward: number; findings: number; readonly refused: Record<string, number> };

/** 🏷️ The `schema-mutation-label` lint, its census and every label site read. */
export type MutationLabelReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationLabelCensusRow[]; readonly sites: readonly MutationLabelSite[] };

const MUTATION_LABEL_TRAITS = new Set(["MutationKind", "CompositeMutationKind", "SemanticMutation"]);
export const MUTATION_LABEL_APP_TRAITS = new Set(["ArtifactApp", "ArtifactEditor", "ArtifactViewer"]);

/** 🏷️ The verdict of one label body `tokens[start..end]`: every `LocalizedLabel::native` call must take two locale arguments, neither
 * the empty literal; `LocalizedLabel::data` and `print_op` are findings; a body without `native` must forward to a `label` call. */
export function rustLabelVerdict(tokens: readonly RustToken[], start: number, end: number): { verdict: MutationLabelVerdict; texts: (readonly [string | null, string | null])[] } {
  const texts: (readonly [string | null, string | null])[] = [];
  const found = new Set<MutationLabelVerdict>();
  let forward = false;
  for (let index = start; index < end; index += 1) {
    const token = tokens[index]!;
    if (rustPathAt(tokens, index, ["LocalizedLabel", "native"]) && tokens[index + 4]?.text === "(") {
      const args = rustCallArguments(tokens, index + 4);
      if (args.length !== 2) found.add("labelUnresolved");
      else {
        const [en, de] = args.map(rustFirstLiteral) as [string | null, string | null];
        texts.push([en, de]);
        if (args.some((arg) => arg.filter((part) => part.text !== "&").length === 1 && rustFirstLiteral(arg) === "")) found.add("labelLocaleEmpty");
      }
    } else if (rustPathAt(tokens, index, ["LocalizedLabel", "data"])) found.add("labelLocaleInvariant");
    else if (token.kind === "ident" && token.text === "print_op") found.add("labelOpText");
    else if (token.kind === "ident" && /(^|_)label$/u.test(token.text) && tokens[index + 1]?.text === "(" && tokens[index - 1]?.text !== "fn") forward = true;
  }
  const uninhabited = tokens.slice(start + 1, end).map((token) => token.text).join(" ") === "match * self { }";
  const worst = (["labelOpText", "labelLocaleInvariant", "labelLocaleEmpty", "labelUnresolved"] as const).find((verdict) => found.has(verdict));
  return { verdict: worst ?? (texts.length > 0 ? "native" : forward || uninhabited ? "forward" : "labelUnresolved"), texts };
}

/** 🏷️ Every label site of one Rust source: each `fn label` of a `MutationKind`/`CompositeMutationKind`/`SemanticMutation` impl, each
 * `fn mutation_label` of an app impl (an override, always a finding), and each `LocalizedLabel::data(…)` call fed by `print_op`. */
export function rustMutationLabelSites(path: string, source: string): MutationLabelSite[] {
  const tokens = rustLex(source);
  const sites: MutationLabelSite[] = [];
  for (const block of rustImplBlocks(tokens)) {
    for (const fn of rustFunctions(tokens, block.open, block.close)) {
      if (fn.body === null) continue;
      if (MUTATION_LABEL_TRAITS.has(block.trait) && fn.name === "label") sites.push({ path, trait: block.trait, implementor: block.implementor, ...rustLabelVerdict(tokens, fn.body[0], fn.body[1]) });
      if (MUTATION_LABEL_APP_TRAITS.has(block.trait) && fn.name === "mutation_label") sites.push({ path, trait: block.trait, implementor: block.implementor, verdict: "labelOverride", texts: [] });
    }
  }
  for (let index = 0; index < tokens.length; index += 1) {
    if (!rustPathAt(tokens, index, ["LocalizedLabel", "data"]) || tokens[index + 4]?.text !== "(") continue;
    const close = rustGroupEnd(tokens, index + 4);
    if (tokens.slice(index + 5, close).some((token) => token.kind === "ident" && token.text === "print_op")) sites.push({ path, trait: "data", implementor: "", verdict: "labelOpText", texts: [] });
  }
  return [...sites, ...rustEmitLabelSites(path, tokens)];
}

/** ✍️ Every hand-written history-row label of an emission among `tokens`, in source order (design §20.4): an `Emit::commit(…, label)`
 * call (implementor `commit`) and an `Emit { … description: <anything but None> … }` struct literal (implementor `description`, the
 * shorthand `description` included), the `Emit` path optionally qualified and turbofished; a destructuring pattern (`… } =`, `… } =>`)
 * is no literal. Each is `labelHandwritten`, with the label's first string literal as its text. */
export function rustEmitLabelSites(path: string, tokens: readonly RustToken[]): MutationLabelSite[] {
  const sites: MutationLabelSite[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.kind !== "ident" || tokens[index]!.text !== "Emit") continue;
    let start = index;
    while (tokens[start - 1]?.text === ":" && tokens[start - 2]?.text === ":" && tokens[start - 3]?.kind === "ident") start -= 3;
    const before = tokens[start - 1];
    if (before !== undefined && (["struct", "enum", "impl", "for", "trait", "type", "use", "as"].includes(before.text) || (before.text === ">" && tokens[start - 2]?.text === "-"))) continue;
    let after = index + 1;
    if (tokens[after]?.text === ":" && tokens[after + 1]?.text === ":" && tokens[after + 2]?.text === "<") {
      let depth = 0;
      for (after += 2; after < tokens.length; after += 1) {
        if (tokens[after]!.text === "<") depth += 1;
        else if (tokens[after]!.text === ">" && tokens[after - 1]?.text !== "-" && --depth === 0) break;
      }
      after += 1;
    }
    if (tokens[after]?.text === ":" && tokens[after + 1]?.text === ":" && tokens[after + 2]?.text === "commit" && tokens[after + 3]?.text === "(") {
      sites.push({ path, trait: "Emit", implementor: "commit", verdict: "labelHandwritten", texts: [[rustFirstLiteral(rustCallArguments(tokens, after + 3)[1] ?? []), null]] });
    } else if (tokens[after]?.text === "{" && tokens[rustGroupEnd(tokens, after) + 1]?.text !== "=") {
      const close = rustGroupEnd(tokens, after);
      let depth = 0;
      for (let cursor = after + 1; cursor < close; cursor += 1) {
        const token = tokens[cursor]!;
        if (token.kind === "punct" && "([{".includes(token.text)) depth += 1;
        else if (token.kind === "punct" && ")]}".includes(token.text)) depth -= 1;
        if (depth !== 0 || token.text !== "description" || !["{", ","].includes(tokens[cursor - 1]?.text ?? "")) continue;
        if (cursor + 1 === close || tokens[cursor + 1]?.text === ",") {
          sites.push({ path, trait: "Emit", implementor: "description", verdict: "labelHandwritten", texts: [[null, null]] });
          continue;
        }
        if (tokens[cursor + 1]?.text !== ":" || tokens[cursor + 2]?.text === ":") continue;
        const value: RustToken[] = [];
        let nested = 0;
        for (let part = cursor + 2; part < close; part += 1) {
          const piece = tokens[part]!;
          if (piece.kind === "punct" && "([{".includes(piece.text)) nested += 1;
          else if (piece.kind === "punct" && ")]}".includes(piece.text)) nested -= 1;
          if (nested === 0 && piece.text === ",") break;
          value.push(piece);
        }
        if (!(value.length === 1 && value[0]!.text === "None")) sites.push({ path, trait: "Emit", implementor: "description", verdict: "labelHandwritten", texts: [[rustFirstLiteral(value), null]] });
      }
    }
  }
  return sites;
}

/**
 * 🏷️ Reads every label site of every Rust source under `under` (design §16.2): a history row is labelled by its leaf in every shell
 * locale, so each leaf label is `LocalizedLabel::native(en, de)` or a forward to one, no app overrides the leaf label, no operation's
 * text line is ever a label, and no emission labels its row by hand (§20.4). Every other verdict is one `schema-mutation-label` diagnostic.
 */
export function mutationLabelReport(repoRoot: string, under = ""): MutationLabelReport {
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationLabelCensusRow>();
  const sites: MutationLabelSite[] = [];
  const relevant = (source: string): boolean => !source.includes("quote!") && (source.includes("fn label") || source.includes("fn mutation_label") || (source.includes("LocalizedLabel::data") && source.includes("print_op")) || (source.includes("Emit") && (source.includes("commit") || source.includes("description"))));
  for (const { path, source } of rustSources(repoRoot, under, relevant)) {
    if (path.split("/").includes("🧪️tests")) continue;
    for (const site of rustMutationLabelSites(path, source)) {
      sites.push(site);
      const owner = mutationPayloadOwner(path);
      const row = census.get(owner) ?? { owner, labels: 0, native: 0, forward: 0, findings: 0, refused: {} };
      census.set(owner, row);
      if (MUTATION_LABEL_TRAITS.has(site.trait)) row.labels += 1;
      if (site.verdict === "native") row.native += 1;
      else if (site.verdict === "forward") row.forward += 1;
      else {
        row.findings += 1;
        row.refused[site.verdict] = (row.refused[site.verdict] ?? 0) + 1;
        const subject = site.trait === "Emit" ? (site.implementor === "commit" ? "Emit::commit(mutations, label)" : "Emit { description: … }") : `${site.trait} for ${site.implementor || "a LocalizedLabel::data call"}`;
        diagnostics.push({ code: "schema-mutation-label", scope: null, export: null, format: null, path, detail: `${site.verdict}: ${subject}${site.texts.length > 0 ? ` ${JSON.stringify(site.texts)}` : ""}` });
      }
    }
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.findings - left.findings || left.owner.localeCompare(right.owner)), sites };
}

/**
 * 🏷️ `test schema mutation-labels` — the `schema-mutation-label` gate: every applied leaf of every plugin labels itself in every shell
 * locale, no app overrides that label, no emission labels its row by hand, no history row falls back to an operation's text line.
 * `--census` prints the per-plugin table
 * and always exits 0; without it any finding fails.
 *
 *   bun 📜️script.ts schema mutation-labels [--census] [--under <path>] [--json]
 */
export function runMutationLabels(repoRoot: string, segments: string[]): void {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationLabelReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2));
    {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
  }
  const total = report.census.reduce((sum, row) => ({ labels: sum.labels + row.labels, native: sum.native + row.native, forward: sum.forward + row.forward }), { labels: 0, native: 0, forward: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "labels", "native", "forward", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.labels, row.native, row.forward, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-labels]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-labels] ${total.native} native and ${total.forward} forwarding of ${total.labels} leaf label(s); ${report.diagnostics.length} schema-mutation-label finding(s)${under === "" ? "" : ` under ${under}`}`);
  {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
}
//#endregion 🏷️MutationLabels
