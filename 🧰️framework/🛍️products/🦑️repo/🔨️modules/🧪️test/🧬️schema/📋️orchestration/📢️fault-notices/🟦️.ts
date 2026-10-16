/** 📢️ Fault notice gate
 *
 * `schema-fault-notice`: every guest fault code reaches the person as a notice in every locale.
 */

import { type SchemaDiagnostic } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { type FaultNoticeDefinition, isFaultNoticeCode, validateFaultNotices } from "@semio-tech/framework/manifest";
import { readJsonObject } from "../📚️schema-documents/🟦️.ts";
import { rustCallArguments, rustFunctions, rustGroupEnd, rustImplBlocks, rustLex, rustPathAt, type RustToken } from "../🦀️rust-tokens/🟦️.ts";
import { rustSources } from "../🌳️mutation-tree/🟦️.ts";

//#region 📢️FaultNotices
/** 🚥️ The classes of a `schema-fault-notice` finding (design §20.12): a guest code no table labels (`faultNoticeMissing`), a guest code
 * that cannot be an app notice code (`faultNoticeSyntax`: fewer than three kebab segments), a guest code in a framework namespace the
 * framework table does not label (`faultNoticeFramework`), an English-only fault without a code (`faultAnonymous`: `Fault::from` of a
 * literal, `format!` or `to_string()`), a declared table breaking a rule (`faultNoticeInvalid`), a `fn fault_notices` body the gate
 * cannot read (`faultNoticeUnresolved`), a declared code no guest source emits (`faultNoticeStale`), and a committed descriptor whose
 * published codes differ from the sources (`faultNoticeDescriptor`, a `describe` is owed). */
export type FaultNoticeFindingClass = "faultNoticeMissing" | "faultNoticeSyntax" | "faultNoticeFramework" | "faultAnonymous" | "faultNoticeInvalid" | "faultNoticeUnresolved" | "faultNoticeStale" | "faultNoticeDescriptor";

/** 📍️ One fault code a guest source emits: `FaultCode::new|from("…")`, `Fault::new(origin, "…", …)`, `fault_from_error!(…, "…")`, a
 * code literal of a `fn code`/`fn fault_code` body (`codeFn`), a call of a refusal helper (`faultHelper`, see {@link rustFaultHelpers}),
 * or an anonymous `Fault::from(…)` (`code` null, `text` its string literal if it has one); `within` names the innermost `fn` around it
 * (`null` outside any), `defaulted` tells whether that `fn` is a trait's default method — the capability-absent contract a host never
 * reaches through an implementor that overrides it. Sites in `#[cfg(test)]`-gated items are not guest sites and are never read. */
export type FaultCodeSite = Readonly<{ path: string; code: string | null; via: "faultCode" | "faultNew" | "faultMacro" | "codeFn" | "faultHelper" | "faultFrom"; text: string | null; within: string | null; defaulted: boolean }>;

/** 🧰️ A refusal helper: the position of the parameter its fault code comes from, or the one fixed code it always refuses with. */
export type FaultHelper = number | string;

/** 📋️ One `fn fault_notices` of an `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer` impl: its implementor and the `(code, en, de)` rows
 * read from `LocalizedLabel::native` tuples (a code is a string literal or a `const NAME: &str` of the plugin) — or, for a body that
 * composes other tables (`forwards`: one `name()` call, or several chained `…fault_notices()` calls), the union of their rows, resolved
 * among the plugin's sources — or `null` when the table is not readable as that shape. */
export type FaultNoticeDeclaration = Readonly<{ path: string; implementor: string; forwards: readonly string[]; notices: readonly Readonly<{ code: string; en: string; de: string }>[] | null }>;

/** 📈️ One plugin's share of the fault-notice census: emitted codes, labelled ones, declared notices and findings per class. */
export type FaultNoticeCensusRow = { readonly owner: string; codes: number; labelled: number; declared: number; anonymous: number; findings: number; readonly refused: Record<string, number> };

/** 📰️ The `schema-fault-notice` lint, its census, every emitted code and every declared table read. */
export type FaultNoticeReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly FaultNoticeCensusRow[]; readonly sites: readonly FaultCodeSite[]; readonly declarations: readonly FaultNoticeDeclaration[] };

const FAULT_NOTICE_APP_TRAITS = new Set(["ArtifactApp", "ArtifactEditor", "ArtifactViewer"]);
/** 🏛️ Code namespaces the framework owns: a guest refusal in one of them is labelled by the framework's table or by no one. */
const FAULT_NOTICE_FRAMEWORK_NAMESPACES = new Set(["app", "plugin", "os", "framework", "module", "mutation", "viewer", "surface", "history", "timeTravel", "toolTransaction", "document", "pure"]);
const FAULT_NOTICE_PLUGINS_ROOT = "✏️s/🔌️plugins";
/** 🔌️ The plugin SDK every guest links: its refusals reach the person like a plugin's own, under the owner `🔌️plugin`; every code it
 * raises is a framework code (labelled by the framework's tables or by no one), and it declares no notice table of its own. */
const FAULT_NOTICE_SDK_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const FAULT_NOTICE_SDK_OWNER = "🔌️plugin";
/** 🫙️ Catch-all codes that name no refusal: a site raising one is an anonymous fault. */
const FAULT_NOTICE_CATCH_ALL = new Set(["plugin.internal"]);
/** 🗄️ The framework's notice tables (history-lane refusals, framework-namespace refusals any app raises): a code they declare is labelled. */
const FAULT_NOTICE_FRAMEWORK_TABLES = ["🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧫️history-notices/🔣️.json", "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧫️framework-notices/🔣️.json"] as const;

/** 🔭️ The findings `schema fault-notices` reports: `all`, or `history-editing` — the refusals a person meets while editing history,
 * loading a document or running a tool (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING scope): every framework-namespace code a plugin
 * raises, every SDK code and every other code or anonymous fault whose code or text has a tool-flow segment or one of whose live sites
 * is a tool-flow site ({@link faultSiteInToolFlow}), and every verdict on a declared table or descriptor ({@link faultNoticeInScope}). */
export type FaultNoticeScope = "all" | "history-editing";
const FAULT_NOTICE_TOOL_FLOW_CODE = /(?:^|[.-])(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|timeTravel|toolRun|toolTransaction)(?:[.-]|$)/u;
const FAULT_NOTICE_TOOL_FLOW_PATH = /🛠️|tool|gesture|gumball|scrub|press|transaction|time-travel/iu;
const FAULT_NOTICE_TOOL_FLOW_FN = /(?:^|_)(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|time_travel)(?:_|$)|(?:^|_)(?:document|archive|envelope|retained)_load(?:_|$)|(?:^|_)load_(?:document|archive|envelope)(?:_|$)/u;
const FAULT_NOTICE_TOOL_FLOW_SOURCE = /\bChildEmit\b|\bEmit::[a-z_]*drag[a-z_]*\b/u;

/** 🪪️ What {@link faultNoticeInScope} weighs for one finding: the code or anonymous text it names, whether one of its live sites is a
 * tool-flow site, whether the SDK raises it, and whether any of its sites is live (outside a trait's default method). */
export type FaultNoticeSubject = Readonly<{ named: string | null; toolFlow: boolean; sdk: boolean; live: boolean }>;
const FAULT_NOTICE_DECLARED_SUBJECT: FaultNoticeSubject = { named: null, toolFlow: false, sdk: false, live: true };

/** 🧲️ Whether one finding belongs to `scope` — see {@link FaultNoticeScope}: under `history-editing` a code or anonymous fault is kept
 * when it is live and in the flow (a tool-flow site or a tool-flow code or text), a plugin's framework-namespace code whenever it is
 * live, and every verdict on a declared table or descriptor always. */
export function faultNoticeInScope(scope: FaultNoticeScope, verdict: FaultNoticeFindingClass, subject: FaultNoticeSubject = FAULT_NOTICE_DECLARED_SUBJECT): boolean {
  if (scope === "all") return true;
  const flow = subject.live && (subject.toolFlow || (subject.named !== null && FAULT_NOTICE_TOOL_FLOW_CODE.test(subject.named)));
  switch (verdict) {
    case "faultNoticeFramework":
      return subject.sdk ? flow : subject.live;
    case "faultNoticeMissing":
    case "faultNoticeSyntax":
    case "faultAnonymous":
      return flow;
    default:
      return true;
  }
}

/** 🛤️ Whether a site is a tool-flow site: its path has a tool-flow segment, it sits inside a tool-flow `fn` (`build_tool_job`,
 * `begin_gesture`, `apply_time_travel_action`, …) or a document-load `fn` (`begin_document_archive_load`, `advance_artifact_envelope_load`,
 * …), or its plugin source publishes a child emit or a drag emit (`ChildEmit`, `Emit::*drag*`). */
function faultSiteInToolFlow(site: FaultCodeSite, emitting: ReadonlySet<string>): boolean {
  return emitting.has(site.path) || (site.within !== null && FAULT_NOTICE_TOOL_FLOW_FN.test(site.within)) || site.path.split("/").some((segment) => FAULT_NOTICE_TOOL_FLOW_PATH.test(segment));
}
const DOTTED_CODE = /^[A-Za-z][\w-]*(?:\.[A-Za-z0-9][\w-]*)+$/u;

/** 🔡️ The value of one Rust string literal token: quotes and raw hashes stripped, the common escapes resolved. */
export function rustStringValue(literal: string): string {
  const raw = /^b?r(#*)"/u.exec(literal);
  if (raw !== null) return literal.slice(raw[0].length, literal.length - 1 - raw[1]!.length);
  return literal
    .replace(/^b?"|"$/gu, "")
    .replace(/\\u\{([0-9a-fA-F]+)\}|\\(["'\\nrt0])/gu, (_, hex: string | undefined, escaped: string | undefined) => (hex !== undefined ? String.fromCodePoint(Number.parseInt(hex, 16)) : ({ n: "\n", r: "\r", t: "\t", "0": "\0" } as Record<string, string>)[escaped!] ?? escaped!));
}

/** 🔖️ Every `const NAME: &str = "…";` (also `&'static str`, any visibility) of one Rust source, by name. */
export function rustStringConsts(source: string): Map<string, string> {
  const tokens = rustLex(source);
  const consts = new Map<string, string>();
  for (let index = 0; index + 1 < tokens.length; index += 1) {
    if (tokens[index]!.text !== "const" || tokens[index + 1]?.kind !== "ident" || tokens[index + 2]?.text !== ":") continue;
    let at = index + 3;
    while (at < tokens.length && tokens[at]!.text !== "=" && tokens[at]!.text !== ";") at += 1;
    const type = tokens.slice(index + 3, at).map((token) => token.text).join("");
    if (!/^&(?:'static)?str$/u.test(type) || tokens[at]?.text !== "=" || tokens[at + 1]?.kind !== "literal" || tokens[at + 2]?.text !== ";" || !/^b?r?#*"/u.test(tokens[at + 1]!.text)) continue;
    consts.set(tokens[index + 1]!.text, rustStringValue(tokens[at + 1]!.text));
  }
  return consts;
}

/** 🪝️ The refusal helpers of one Rust source, by name — free `fn`s (no `self`), nested ones included: one whose body builds its fault
 * from a parameter (`FaultCode::new|from(code)` or `Fault::new(origin, code, …)`) is that parameter's position; one returning `Fault`
 * whose body builds exactly one fault, from a string literal or a named `const` of `consts`, is that fixed code. */
export function rustFaultHelpers(source: string, consts: ReadonlyMap<string, string> = new Map()): Map<string, FaultHelper> {
  const tokens = rustLex(source);
  const helpers = new Map<string, FaultHelper>();
  for (const fn of rustFnBodies(tokens)) {
    const params = rustCallArguments(tokens, fn.params).map((param) => (param[0]?.kind === "ident" && param[1]?.text === ":" && param[2]?.text !== ":" ? param[0].text : null));
    if (rustCallArguments(tokens, fn.params)[0]?.some((token) => token.text === "self")) continue;
    const codeArgs: (readonly RustToken[])[] = [];
    for (let index = fn.open; index < fn.close; index += 1) {
      const codeArg = (rustPathAt(tokens, index, ["FaultCode", "new"]) || rustPathAt(tokens, index, ["FaultCode", "from"])) && tokens[index + 4]?.text === "(" ? rustCallArguments(tokens, index + 4)[0] : rustPathAt(tokens, index, ["Fault", "new"]) && tokens[index + 4]?.text === "(" ? rustCallArguments(tokens, index + 4)[1] : undefined;
      if (codeArg !== undefined && codeArg.every((token) => token.text !== "FaultCode")) codeArgs.push(codeArg);
    }
    const position = codeArgs.map((arg) => (arg.length === 1 && arg[0]!.kind === "ident" ? params.indexOf(arg[0]!.text) : -1)).find((at) => at >= 0);
    const returnsFault = /^->(?:\w+::)*Fault$/u.test(tokens.slice(rustGroupEnd(tokens, fn.params) + 1, fn.open).map((token) => token.text).join(""));
    const fixed = codeArgs.length === 1 && codeArgs[0]!.length === 1 ? (codeArgs[0]![0]!.kind === "literal" && /^b?r?#*"/u.test(codeArgs[0]![0]!.text) ? rustStringValue(codeArgs[0]![0]!.text) : (consts.get(codeArgs[0]![0]!.text) ?? null)) : null;
    if (position !== undefined) helpers.set(fn.name, position);
    else if (returnsFault && fixed !== null && DOTTED_CODE.test(fixed)) helpers.set(fn.name, fixed);
  }
  return helpers;
}

/** 🗺️ Every `fn` with a body in a token stream — methods and nested ones included — by name with the `(` of its parameters, its `{…}`
 * span and whether it is a trait's default method (its innermost enclosing block is a `trait` body), in source order. */
function rustFnBodies(tokens: readonly RustToken[]): { readonly name: string; readonly params: number; readonly open: number; readonly close: number; readonly defaulted: boolean }[] {
  const traits: (readonly [number, number])[] = [];
  const bodies: { readonly name: string; readonly params: number; readonly open: number; readonly close: number; defaulted: boolean }[] = [];
  for (let at = 0; at + 1 < tokens.length; at += 1) {
    if (tokens[at]!.text === "trait" && tokens[at + 1]!.kind === "ident") {
      let open = at + 2;
      while (open < tokens.length && tokens[open]!.text !== "{" && tokens[open]!.text !== ";") open += 1;
      if (tokens[open]?.text === "{") traits.push([open, rustGroupEnd(tokens, open)]);
    }
    if (tokens[at]!.text !== "fn" || tokens[at + 1]!.kind !== "ident") continue;
    let params = at + 2;
    while (params < tokens.length && tokens[params]!.text !== "(") params += 1;
    let open = rustGroupEnd(tokens, params) + 1;
    while (open < tokens.length && tokens[open]!.text !== "{" && tokens[open]!.text !== ";") open += 1;
    if (tokens[open]?.text === "{") bodies.push({ name: tokens[at + 1]!.text, params, open, close: rustGroupEnd(tokens, open), defaulted: false });
  }
  for (const body of bodies) {
    const trait = traits.findLast(([open, close]) => open < body.open && body.close <= close);
    body.defaulted = trait !== undefined && !bodies.some((outer) => outer !== body && trait[0] < outer.open && outer.open < body.open && body.close <= outer.close);
  }
  return bodies;
}

/** 🧪️ The token spans of every `#[cfg(…test…)]`-gated item of a token stream (`cfg(test)`, `cfg(any(test, …))`, not `cfg(not(test))`) —
 * test code, never a guest source; an inner `#![cfg(test)]` gates the whole source. */
function rustTestOnlySpans(tokens: readonly RustToken[]): (readonly [number, number])[] {
  const spans: (readonly [number, number])[] = [];
  const testOnly = (open: number): boolean => {
    const attribute = tokens.slice(open + 1, rustGroupEnd(tokens, open)).map((token) => token.text).join("");
    return /^cfg\(/u.test(attribute) && /\btest\b/u.test(attribute) && !/not\(test\)/u.test(attribute);
  };
  for (let at = 0; at + 2 < tokens.length; at += 1) {
    if (tokens[at]!.text !== "#") continue;
    if (tokens[at + 1]!.text === "!" && tokens[at + 2]!.text === "[" && testOnly(at + 2)) return [[0, tokens.length]];
    if (tokens[at + 1]!.text !== "[" || !testOnly(at + 1)) continue;
    let item = rustGroupEnd(tokens, at + 1) + 1;
    while (tokens[item]?.text === "#" && tokens[item + 1]?.text === "[") item = rustGroupEnd(tokens, item + 1) + 1;
    while (item < tokens.length && tokens[item]!.text !== "{" && tokens[item]!.text !== ";") item = (tokens[item]!.text === "(" || tokens[item]!.text === "[" ? rustGroupEnd(tokens, item) : item) + 1;
    spans.push([at, tokens[item]?.text === "{" ? rustGroupEnd(tokens, item) : item]);
  }
  return spans;
}

/** 🎯️ Every fault code site of one Rust source, in source order — see {@link FaultCodeSite}; a code argument is a string literal or a
 * (path to a) `const` named in `consts`; a call of a refusal helper in `helpers` carries its code at the helper's position; a
 * `fn code`/`fn fault_code` is read in any impl, inherent ones included (a named refusal enum's `fn code(&self) -> &'static str`),
 * literals and named consts alike; a site inside a `#[cfg(test)]`-gated item is not read. */
export function rustFaultCodeSites(path: string, source: string, consts: ReadonlyMap<string, string> = new Map(), helpers: ReadonlyMap<string, FaultHelper> = new Map()): FaultCodeSite[] {
  const tokens = rustLex(source);
  const bodies = rustFnBodies(tokens);
  const tests = rustTestOnlySpans(tokens);
  const sites: (Omit<FaultCodeSite, "within" | "defaulted"> & { readonly at: number })[] = [];
  const literal = (arg: readonly RustToken[] | undefined): string | null => {
    if (arg === undefined) return null;
    if (arg.length === 1 && arg[0]!.kind === "literal" && /^b?r?#*"/u.test(arg[0]!.text)) return rustStringValue(arg[0]!.text);
    const last = arg.at(-1);
    return last?.kind === "ident" && arg.every((token, at) => (at % 3 === 0 ? token.kind === "ident" : token.text === ":")) ? (consts.get(last.text) ?? null) : null;
  };
  for (let index = 0; index < tokens.length; index += 1) {
    if ((rustPathAt(tokens, index, ["FaultCode", "new"]) || rustPathAt(tokens, index, ["FaultCode", "from"])) && tokens[index + 4]?.text === "(") {
      const code = literal(rustCallArguments(tokens, index + 4)[0]);
      if (code !== null) sites.push({ path, code, via: "faultCode", text: null, at: index });
    } else if (rustPathAt(tokens, index, ["Fault", "new"]) && tokens[index + 4]?.text === "(") {
      const code = literal(rustCallArguments(tokens, index + 4)[1]);
      if (code !== null) sites.push({ path, code, via: "faultNew", text: null, at: index });
    } else if (rustPathAt(tokens, index, ["Fault", "from"]) && tokens[index + 4]?.text === "(") {
      const arg = rustCallArguments(tokens, index + 4)[0] ?? [];
      const text = arg.map((token) => token.text).join("");
      if (literal(arg) !== null || /^format!/u.test(text) || /\.(to_string|to_owned)\(\)$/u.test(text)) sites.push({ path, code: null, via: "faultFrom", text: literal(arg), at: index });
    } else if (tokens[index]!.text === "fault_from_error" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "(") {
      const code = literal(rustCallArguments(tokens, index + 2)[2]);
      if (code !== null) sites.push({ path, code, via: "faultMacro", text: null, at: index });
    } else if (tokens[index]!.kind === "ident" && helpers.has(tokens[index]!.text) && tokens[index + 1]?.text === "(" && tokens[index - 1]?.text !== "fn" && tokens[index - 1]?.text !== ".") {
      const helper = helpers.get(tokens[index]!.text)!;
      const code = typeof helper === "string" ? helper : literal(rustCallArguments(tokens, index + 1)[helper]);
      if (code !== null) sites.push({ path, code, via: "faultHelper", text: null, at: index });
    }
  }
  for (const body of bodies.filter((candidate) => candidate.name === "code" || candidate.name === "fault_code")) {
    for (let index = body.open; index < body.close; index += 1) {
      const token = tokens[index]!;
      if (rustPathAt(tokens, index - 4, ["FaultCode", "new"]) || rustPathAt(tokens, index - 4, ["FaultCode", "from"])) continue;
      const code = token.kind === "literal" && /^b?r?#*"/u.test(token.text) ? rustStringValue(token.text) : token.kind === "ident" ? (consts.get(token.text) ?? null) : null;
      if (code !== null && DOTTED_CODE.test(code)) sites.push({ path, code, via: "codeFn", text: null, at: index });
    }
  }
  const within = (at: number) => bodies.findLast((body) => body.open < at && at < body.close);
  return sites
    .filter((site) => !tests.some(([open, close]) => open <= site.at && site.at <= close))
    .sort((left, right) => left.at - right.at)
    .map(({ at, ...site }) => ({ ...site, within: within(at)?.name ?? null, defaulted: within(at)?.defaulted ?? false }));
}

/** 🧮️ The `(code, en, de)` rows of one table body `tokens[open..close]`: each `LocalizedLabel::native(en, de)` call preceded by `"code",`
 * or by `NAME,` of a const in `consts` is one row; `null` when a call is not that shape or the body names none. */
function rustFaultNoticeRows(tokens: readonly RustToken[], open: number, close: number, consts: ReadonlyMap<string, string> = new Map()): { code: string; en: string; de: string }[] | null {
  const notices: { code: string; en: string; de: string }[] = [];
  for (let index = open; index < close; index += 1) {
    if (!rustPathAt(tokens, index, ["LocalizedLabel", "native"]) || tokens[index + 4]?.text !== "(") continue;
    let start = index;
    while (tokens[start - 1]?.text === ":" && tokens[start - 2]?.text === ":" && tokens[start - 3]?.kind === "ident") start -= 3;
    const texts = rustCallArguments(tokens, index + 4).map((arg) => (arg.length === 1 && arg[0]!.kind === "literal" ? rustStringValue(arg[0]!.text) : null));
    const key = tokens[start - 1]?.text === "," ? tokens[start - 2] : undefined;
    const code = key?.kind === "literal" ? rustStringValue(key.text) : key?.kind === "ident" ? (consts.get(key.text) ?? null) : null;
    if (code === null || texts.length !== 2 || texts.some((text) => text === null)) return null;
    notices.push({ code, en: texts[0]!, de: texts[1]! });
  }
  return notices.length > 0 ? notices : null;
}

/** 📜️ Every `fn fault_notices` declaration of one Rust source — see {@link FaultNoticeDeclaration}: an empty `&[]` body declares none, a
 * body that is one call `path::name()` forwards to `name`, a body without rows of its own that chains `…fault_notices()` calls forwards to
 * each of them (rows resolved by {@link rustFaultNoticeTable}), any other body is a table; an impl in a `#[cfg(test)]`-gated item is
 * not read. */
export function rustFaultNoticeDeclarations(path: string, source: string, consts: ReadonlyMap<string, string> = new Map()): FaultNoticeDeclaration[] {
  const tokens = rustLex(source);
  const tests = rustTestOnlySpans(tokens);
  const declarations: FaultNoticeDeclaration[] = [];
  for (const block of rustImplBlocks(tokens)) {
    if (!FAULT_NOTICE_APP_TRAITS.has(block.trait) || tests.some(([open, close]) => open <= block.open && block.open <= close)) continue;
    for (const fn of rustFunctions(tokens, block.open, block.close)) {
      if (fn.body === null || fn.name !== "fault_notices") continue;
      const [open, close] = fn.body;
      const body = tokens.slice(open + 1, close).map((token) => token.text).join("");
      const single = /^(?:[\p{L}\p{N}_]+::)*([\p{L}_][\p{L}\p{N}_]*)\(\)$/u.exec(body)?.[1];
      const rows = single === undefined && body !== "&[]" ? rustFaultNoticeRows(tokens, open, close, consts) : null;
      const chained = tokens.slice(open + 1, close).flatMap((token, at, all) => (token.kind === "ident" && token.text.endsWith("fault_notices") && all[at + 1]?.text === "(" && all[at + 2]?.text === ")" ? [token.text] : []));
      const forwards = single !== undefined ? [single] : rows === null && !body.includes("LocalizedLabel::native") ? [...new Set(chained)] : [];
      declarations.push({ path, implementor: block.implementor, forwards, notices: body === "&[]" ? [] : forwards.length > 0 ? null : rows });
    }
  }
  return declarations;
}

/** 📑️ The rows of the free function `name` of one Rust source (a forwarded table), or `undefined` when the source defines none. */
export function rustFaultNoticeTable(source: string, name: string, consts: ReadonlyMap<string, string> = new Map()): { code: string; en: string; de: string }[] | null | undefined {
  const tokens = rustLex(source);
  const fn = rustFunctions(tokens, -1, tokens.length).find((candidate) => candidate.name === name && candidate.body !== null);
  return fn === undefined ? undefined : rustFaultNoticeRows(tokens, fn.body![0], fn.body![1], consts);
}

/** 🧩️ The plugin directory segment of a repository-relative path under `✏️s/🔌️plugins`, or `null`. */
function faultNoticePlugin(path: string): string | null {
  if (path.startsWith(`${FAULT_NOTICE_SDK_ROOT}/`)) return FAULT_NOTICE_SDK_OWNER;
  const segments = path.split("/");
  const plugins = segments.indexOf("🔌️plugins");
  return plugins >= 0 ? (segments[plugins + 1] ?? null) : null;
}

/** 🏘️ The crate-sized unit consts, refusal helpers and forwarded tables resolve within: the `🗿️artifacts/<artifact>` root of a path, else
 * its plugin directory. */
function faultNoticeUnit(path: string): string {
  if (path.startsWith(`${FAULT_NOTICE_SDK_ROOT}/`)) return FAULT_NOTICE_SDK_ROOT;
  const segments = path.split("/");
  const artifacts = segments.indexOf("🗿️artifacts");
  return artifacts >= 0 && segments[artifacts + 1] !== undefined ? segments.slice(0, artifacts + 2).join("/") : segments.slice(0, segments.indexOf("🔌️plugins") + 2).join("/");
}

/** 🗣️ A declared `(code, en, de)` row as the definition its descriptor publishes (`LocalizedLabel::native` is terminology-invariant). */
function faultNoticeDefinition(row: Readonly<{ code: string; en: string; de: string }>): FaultNoticeDefinition {
  return { code: row.code, label: { native: { en: row.en, de: row.de }, reuse: { en: row.en, de: row.de } } };
}

/**
 * 📢️ Reads every guest fault code and every declared notice table of every plugin under `under` (design §20.12): each code a guest
 * emits is labelled by the framework's tables ({@link FAULT_NOTICE_FRAMEWORK_TABLES}) or by one of its plugin's `fault_notices`
 * tables in every locale; every declared table is valid (`validateFaultNotices`) and names only emitted codes; every committed
 * descriptor (`🌎️hub/🧩️compositions/<plugin>/🔣️.json`) publishes exactly the declared codes, each table valid. Every other verdict
 * in `scope` ({@link faultNoticeInScope}) is one `schema-fault-notice` diagnostic.
 */
export function faultNoticeReport(repoRoot: string, under = "", scope: FaultNoticeScope = "all"): FaultNoticeReport {
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, FaultNoticeCensusRow>();
  const sites: FaultCodeSite[] = [];
  const declarations: FaultNoticeDeclaration[] = [];
  const framework = new Set(FAULT_NOTICE_FRAMEWORK_TABLES.flatMap((table) => ((readJsonObject(repoRoot, table)?.notices ?? []) as { readonly code: string }[]).map((notice) => notice.code)));
  const relevant = (source: string): boolean => source.includes("Fault") || source.includes("fn code") || source.includes("fault_notices");
  const sources: { readonly path: string; readonly source: string }[] = [];
  for (const root of under === "" ? [FAULT_NOTICE_PLUGINS_ROOT, FAULT_NOTICE_SDK_ROOT] : [under]) {
    for (const { path, source } of rustSources(repoRoot, root, relevant)) if (!path.split("/").includes("🧪️tests") && faultNoticePlugin(path) !== null) sources.push({ path, source });
  }
  const unitConsts = new Map<string, Map<string, string | null>>();
  const unitHelpers = new Map<string, Map<string, FaultHelper | null>>();
  for (const { path, source } of sources) {
    const consts = unitConsts.get(faultNoticeUnit(path)) ?? new Map<string, string | null>();
    unitConsts.set(faultNoticeUnit(path), consts);
    for (const [name, value] of rustStringConsts(source)) consts.set(name, consts.has(name) && consts.get(name) !== value ? null : value);
  }
  const resolved = <T,>(table: Map<string, T | null> | undefined): Map<string, T> => new Map([...(table ?? [])].filter((entry): entry is [string, T] => entry[1] !== null));
  const constsOf = (path: string): Map<string, string> => resolved(unitConsts.get(faultNoticeUnit(path)));
  for (const { path, source } of sources) {
    const helpers = unitHelpers.get(faultNoticeUnit(path)) ?? new Map<string, FaultHelper | null>();
    unitHelpers.set(faultNoticeUnit(path), helpers);
    for (const [name, helper] of rustFaultHelpers(source, constsOf(path))) helpers.set(name, helpers.has(name) && helpers.get(name) !== helper ? null : helper);
  }
  const helpersOf = (path: string): Map<string, FaultHelper> => resolved(unitHelpers.get(faultNoticeUnit(path)));
  const emitting = new Set(sources.filter(({ path, source }) => faultNoticePlugin(path) !== FAULT_NOTICE_SDK_OWNER && FAULT_NOTICE_TOOL_FLOW_SOURCE.test(source)).map(({ path }) => path));
  for (const { path, source } of sources) sites.push(...rustFaultCodeSites(path, source, constsOf(path), helpersOf(path)));
  for (const { path, source } of sources) {
    const consts = constsOf(path);
    for (const declaration of faultNoticePlugin(path) === FAULT_NOTICE_SDK_OWNER ? [] : rustFaultNoticeDeclarations(path, source, consts)) {
      const holders = (sameUnit: boolean) => sources.filter((candidate) => (sameUnit ? faultNoticeUnit(candidate.path) === faultNoticeUnit(path) : faultNoticePlugin(candidate.path) === faultNoticePlugin(path)) && candidate.source.includes("fn "));
      const table = (forward: string): { code: string; en: string; de: string }[] | null => [...holders(true), ...holders(false)].filter((candidate) => candidate.source.includes(`fn ${forward}`)).map((candidate) => rustFaultNoticeTable(candidate.source, forward, constsOf(candidate.path))).find((rows) => rows !== undefined) ?? null;
      const tables = declaration.forwards.map(table);
      const notices = declaration.forwards.length === 0 ? declaration.notices : tables.some((rows) => rows === null) ? null : tables.flatMap((rows) => rows!);
      declarations.push({ ...declaration, notices });
    }
  }
  const row = (owner: string): FaultNoticeCensusRow => {
    const found = census.get(owner) ?? { owner, codes: 0, labelled: 0, declared: 0, anonymous: 0, findings: 0, refused: {} };
    census.set(owner, found);
    return found;
  };
  const finding = (owner: string, verdict: FaultNoticeFindingClass, path: string, detail: string, subject: FaultNoticeSubject = FAULT_NOTICE_DECLARED_SUBJECT): void => {
    if (!faultNoticeInScope(scope, verdict, subject)) return;
    const target = row(owner);
    target.findings += 1;
    target.refused[verdict] = (target.refused[verdict] ?? 0) + 1;
    diagnostics.push({ code: "schema-fault-notice", scope: null, export: null, format: null, path, detail: `${verdict}: ${detail}` });
  };
  const declared = new Map<string, Set<string>>();
  for (const declaration of declarations) {
    const plugin = faultNoticePlugin(declaration.path)!;
    const codes = declared.get(plugin) ?? new Set<string>();
    declared.set(plugin, codes);
    if (declaration.notices === null) {
      finding(plugin, "faultNoticeUnresolved", declaration.path, `fn fault_notices of ${declaration.implementor}${declaration.forwards.length === 0 ? "" : ` (forward to ${declaration.forwards.join(", ")})`} is not a table of (code, LocalizedLabel::native(en, de)) rows`);
      continue;
    }
    row(plugin).declared += declaration.notices.length;
    for (const notice of declaration.notices) codes.add(notice.code);
    for (const error of validateFaultNotices(declaration.notices.map(faultNoticeDefinition))) finding(plugin, "faultNoticeInvalid", declaration.path, `${error.rule} ${error.code}${"locale" in error ? ` (${error.terminology}/${error.locale})` : ""} in ${declaration.implementor}`);
  }
  const emitted = new Map<string, Set<string>>();
  const told = new Set<string>();
  const flows = new Map<string, { toolFlow: boolean; live: boolean }>();
  for (const site of sites) {
    if (site.code === null) continue;
    const key = `${faultNoticePlugin(site.path)!}\u0000${site.code}`;
    const flow = flows.get(key) ?? { toolFlow: false, live: false };
    flows.set(key, { toolFlow: flow.toolFlow || (!site.defaulted && faultSiteInToolFlow(site, emitting)), live: flow.live || !site.defaulted });
  }
  for (const site of sites) {
    const plugin = faultNoticePlugin(site.path)!;
    const sdk = plugin === FAULT_NOTICE_SDK_OWNER;
    if (site.code === null || FAULT_NOTICE_CATCH_ALL.has(site.code)) {
      if (site.code !== null && typeof helpersOf(site.path).get(site.within ?? "") === "string") continue;
      row(plugin).anonymous += 1;
      finding(plugin, "faultAnonymous", site.path, site.code !== null ? `${site.code} (${site.via}) names no refusal — name it and declare its notice` : `Fault::from(${site.text === null ? "text" : JSON.stringify(site.text)}) carries no code — name the refusal and declare its notice`, { named: site.text, toolFlow: faultSiteInToolFlow(site, emitting), sdk, live: !site.defaulted });
      continue;
    }
    const codes = emitted.get(plugin) ?? new Set<string>();
    emitted.set(plugin, codes);
    if (codes.has(site.code)) continue;
    codes.add(site.code);
    row(plugin).codes += 1;
    if (framework.has(site.code) || declared.get(plugin)?.has(site.code)) {
      row(plugin).labelled += 1;
      continue;
    }
    const key = `${plugin}\u0000${site.code}`;
    if (told.has(key)) continue;
    told.add(key);
    const namespace = site.code.split(".")[0]!;
    finding(plugin, sdk || FAULT_NOTICE_FRAMEWORK_NAMESPACES.has(namespace) ? "faultNoticeFramework" : isFaultNoticeCode(site.code) ? "faultNoticeMissing" : "faultNoticeSyntax", site.path, `${site.code} (${site.via})`, { named: site.code, sdk, ...flows.get(key)! });
  }
  for (const [plugin, codes] of declared) for (const code of codes) if (!emitted.get(plugin)?.has(code)) finding(plugin, "faultNoticeStale", declarations.find((declaration) => faultNoticePlugin(declaration.path) === plugin)!.path, `${code} is declared but no guest source of ${plugin} emits it`);
  for (const plugin of new Set([...census.keys()])) {
    const path = `🌎️hub/🧩️compositions/${plugin}/🔣️.json`;
    const descriptor = readJsonObject(repoRoot, path);
    if (descriptor === null || (under !== "" && !under.startsWith(`${FAULT_NOTICE_PLUGINS_ROOT}/${plugin}`) && under !== FAULT_NOTICE_PLUGINS_ROOT)) continue;
    const apps = ((descriptor.manifest as { readonly apps?: readonly { readonly id?: string; readonly faultNotices?: readonly FaultNoticeDefinition[] }[] } | undefined)?.apps ?? []);
    const published = new Set(apps.flatMap((app) => (app.faultNotices ?? []).map((notice) => notice.code)));
    for (const app of apps) for (const error of validateFaultNotices(app.faultNotices ?? [])) finding(plugin, "faultNoticeInvalid", path, `${error.rule} ${error.code}${"locale" in error ? ` (${error.terminology}/${error.locale})` : ""} in published app ${app.id ?? "?"}`);
    const sources = declared.get(plugin) ?? new Set<string>();
    const missing = [...sources].filter((code) => !published.has(code)).sort();
    const extra = [...published].filter((code) => !sources.has(code)).sort();
    if (missing.length + extra.length > 0) finding(plugin, "faultNoticeDescriptor", path, `describe owed — unpublished ${JSON.stringify(missing)}, no longer declared ${JSON.stringify(extra)}`);
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.findings - left.findings || left.owner.localeCompare(right.owner)), sites, declarations };
}

/**
 * 🖥️ `test schema fault-notices` — the `schema-fault-notice` gate (design §20.12): every fault code a guest can refuse with reaches the
 * person as a notice in every shell locale, never as a raw code. `--census` prints the per-plugin table (owner, distinct codes, labelled
 * ones, declared notices, anonymous faults, findings per class) and always exits 0; without it any finding fails. `--scope
 * history-editing` keeps the refusals of the history-editing and tool flows ({@link faultNoticeInScope}).
 *
 *   bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema fault-notices [--census] [--under <path>] [--scope all|history-editing] [--json]
 */
export function runFaultNotices(repoRoot: string, segments: string[]): void {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const scope = segments.includes("--scope") ? segments[segments.indexOf("--scope") + 1] : "all";
  if (scope !== "all" && scope !== "history-editing") throw new Error(`[schema fault-notices] --scope expects all or history-editing, got ${JSON.stringify(scope)}`);
  const report = faultNoticeReport(repoRoot, under, scope);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2));
    {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
  }
  const total = report.census.reduce((sum, row) => ({ codes: sum.codes + row.codes, labelled: sum.labelled + row.labelled, declared: sum.declared + row.declared, anonymous: sum.anonymous + row.anonymous }), { codes: 0, labelled: 0, declared: 0, anonymous: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "codes", "labelled", "declared", "anonymous", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.codes, row.labelled, row.declared, row.anonymous, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema fault-notices]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema fault-notices] ${total.labelled} labelled of ${total.codes} guest fault code(s), ${total.declared} declared notice(s), ${total.anonymous} anonymous fault(s); ${report.diagnostics.length} schema-fault-notice finding(s)${under === "" ? "" : ` under ${under}`}${scope === "all" ? "" : ` in scope ${scope}`}`);
  {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
}
//#endregion 📢️FaultNotices
