/** 🧵️ Checks codec-local source qualifications without claiming Rust auto-trait execution. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import _ from "lodash";

//#region 🧬️Contract
type SourceName = "plugin" | "builder" | "store" | "mutation";
type Site = { id: string; route: string; source: SourceName; container: string; parents: string[]; name: string; async: boolean; visibility: string; qualification: "associated" | "bare"; header: string; body: string };
type Fixture = {
  access: { sharedMutationAcrossSuspension: string };
  routes: { id: string; sites: string[] }[];
  requiredSites: string[];
  native: {
    sources: Record<SourceName, string>; containers: Record<string, string>; sites: Site[];
    specFields: string; mutationField: string; traits: { name: string; header: string }[];
    protocolTrait: string; codecFields: string[];
  };
};
type Token = { value: string; start: number; end: number };
type Body = { start: number; open: number; close: number };
type Located = Body & { headerStart: number; headerEnd: number; async: boolean; visibility: string };
type Sources = Record<SourceName, string>;
type SourceView = { source: string; items: Token[] };
type Views = Record<SourceName, SourceView>;
type Inspection = { missing: string[]; problems: string[] };
//#endregion 🧬️Contract

//#region 🔎️BoundedSourceSelection
function tokens(source: string): Token[] {
  const result: Token[] = [];
  let position = 0;
  while (position < source.length) {
    const start = position, rest = source.slice(position);
    if (/\s/.test(source[position])) { position++; continue; }
    if (rest.startsWith("//")) {
      const end = source.indexOf("\n", position + 2);
      position = end < 0 ? source.length : end + 1;
      continue;
    }
    if (rest.startsWith("/*")) {
      let depth = 1; position += 2;
      while (position < source.length && depth > 0) {
        if (source.startsWith("/*", position)) { depth++; position += 2; }
        else if (source.startsWith("*/", position)) { depth--; position += 2; }
        else position++;
      }
      assert.equal(depth, 0, "unterminated block comment");
      continue;
    }
    const raw = /^(?:b|c)?r(#{0,255})"/.exec(rest);
    if (raw) {
      const end = source.indexOf('"' + raw[1], position + raw[0].length);
      assert(end >= 0, "unterminated raw string");
      position = end + raw[1].length + 1;
    } else {
      const quoted = /^(?:b|c)?"/.exec(rest);
      const character = /^'(?:[^'\\\r\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.))'/u.exec(rest);
      if (quoted) {
        position += quoted[0].length;
        let closed = false;
        while (position < source.length) {
          if (source[position] === "\\") position += 2;
          else if (source[position++] === '"') { closed = true; break; }
        }
        assert(closed, "unterminated quoted string");
      } else if (character) position += character[0].length;
      else position += /^[A-Za-z_][A-Za-z0-9_]*/.exec(rest)?.[0].length ?? 1;
    }
    result.push({ value: source.slice(start, position), start, end: position });
  }
  return result;
}

function key(source: string): string { return tokens(source).map(token => token.value).join("\0"); }

function sequence(items: Token[], expected: string, start = 0, end = items.length): number[] {
  const wanted = tokens(expected).map(token => token.value), found: number[] = [];
  assert(wanted.length > 0, "empty selector");
  for (let index = start; index + wanted.length <= end; index++) {
    if (wanted.every((value, offset) => items[index + offset].value === value)) found.push(index);
  }
  return found;
}

function closing(items: Token[], open: number): number {
  assert.equal(items[open]?.value, "{", "body opening");
  let depth = 1;
  for (let index = open + 1; index < items.length; index++) {
    if (items[index].value === "{") depth++;
    if (items[index].value === "}" && --depth === 0) return index;
    if (items[index].value !== "}") continue;
  }
  throw new Error("unterminated body");
}

function block(items: Token[], header: string): Body {
  const starts = sequence(items, header);
  assert.equal(starts.length, 1, "unique container: " + header);
  const open = starts[0] + tokens(header).length;
  assert.equal(items[open]?.value, "{", "exact container header: " + header);
  return { start: starts[0], open, close: closing(items, open) };
}

function functionBody(items: Token[], name: string, scope?: Body): Located {
  const matches: number[] = [];
  let depth = 0;
  for (let index = scope ? scope.open + 1 : 0; index < (scope?.close ?? items.length); index++) {
    const value = items[index].value;
    if (value === "{" && scope) depth++;
    else if (value === "}" && scope) depth--;
    if ((!scope || depth === 0) && value === "fn" && items[index + 1]?.value === name) matches.push(index);
  }
  assert.equal(matches.length, 1, "unique function: " + name);
  const start = matches[0];
  const asynchronous = items[start - 1]?.value === "async";
  let preceding = start - (asynchronous ? 2 : 1), visibility = "";
  if (items[preceding]?.value === "pub") visibility = "pub";
  else if (items[preceding]?.value === ")") {
    const end = preceding + 1;
    let parentheses = 1;
    while (--preceding >= 0 && parentheses > 0) {
      if (items[preceding].value === ")") parentheses++;
      else if (items[preceding].value === "(") parentheses--;
    }
    if (items[preceding]?.value === "pub") visibility = items.slice(preceding, end).map(token => token.value).join("");
  }
  let open = start + 2;
  while (open < items.length && items[open].value !== "{") {
    assert.notEqual(items[open].value, ";", "function must have a body");
    open++;
  }
  return { start, open, close: closing(items, open), headerStart: items[start].start, headerEnd: items[open].start, async: asynchronous, visibility };
}

function locate(view: SourceView, fixture: Fixture, site: Site): Located {
  const items = view.items;
  let scope = site.container === "global" ? undefined : block(items, fixture.native.containers[site.container]);
  for (const parent of site.parents) scope = functionBody(items, parent, scope);
  return functionBody(items, site.name, scope);
}

function desiredHeader(site: Site): string {
  if (site.qualification === "associated") return site.header + " where A::Mutation: Sync";
  const boundary = site.header.indexOf("Mutation: ");
  assert(boundary >= 0, "bare mutation bound");
  return site.header.slice(0, boundary) + site.header.slice(boundary).replace("+ Send +", "+ Send + Sync +");
}

function views(sources: Sources): Views {
  return Object.fromEntries(Object.entries(sources).map(([name, source]) => [name, { source, items: tokens(source) }])) as Views;
}

function rewriteHeaders(indexed: Views, fixture: Fixture, sites: Site[], header: (site: Site) => string): Sources {
  const result = Object.fromEntries(Object.entries(indexed).map(([name, view]) => [name, view.source])) as Sources;
  for (const name of ["plugin", "builder"] as const) {
    const edits = sites.filter(site => site.source === name).map(site => ({ found: locate(indexed[name], fixture, site), header: header(site) })).sort((left, right) => right.found.headerStart - left.found.headerStart);
    for (const edit of edits) result[name] = result[name].slice(0, edit.found.headerStart) + edit.header + " " + result[name].slice(edit.found.headerEnd);
  }
  return result;
}

function bodyKey(view: SourceView, found: Body): string {
  return view.items.slice(found.open + 1, found.close).map(token => token.value).join("\0");
}
//#endregion 🔎️BoundedSourceSelection

//#region 🧾️SourceObligations
function requiredSites(fixture: Fixture): string[] {
  assert.equal(fixture.access.sharedMutationAcrossSuspension, "sync");
  const required = new Set<string>();
  for (const route of fixture.routes) for (const site of route.sites) required.add(site);
  return [...required].sort();
}

function inspect(fixture: Fixture, sources: Sources): Inspection {
  const result: Inspection = { missing: [], problems: [] }, indexed = views(sources);
  for (const site of fixture.native.sites) {
    try {
      const source = sources[site.source], found = locate(indexed[site.source], fixture, site);
      const actual = key(source.slice(found.headerStart, found.headerEnd));
      if (actual === key(site.header)) result.missing.push(site.id);
      else if (actual !== key(desiredHeader(site))) result.problems.push(site.id + ":unexpected-header");
      if (found.async !== site.async) result.problems.push(site.id + ":async-changed");
      if (found.visibility !== site.visibility) result.problems.push(site.id + ":visibility-changed");
    } catch (error) { result.problems.push(site.id + ":" + String(error)); }
  }
  try {
    const normalized = rewriteHeaders(indexed, fixture, fixture.native.sites, site => site.header);
    const normalizedViews = { ...indexed, plugin: { source: normalized.plugin, items: tokens(normalized.plugin) }, builder: { source: normalized.builder, items: tokens(normalized.builder) } };
    for (const site of fixture.native.sites) if (bodyKey(normalizedViews[site.source], locate(normalizedViews[site.source], fixture, site)) !== key(site.body)) result.problems.push(site.id + ":body-changed");
  } catch (error) { result.problems.push("body-normalization:" + String(error)); }
  try {
    const plugin = indexed.plugin.items;
    assert.equal(bodyKey(indexed.plugin, block(plugin, "pub struct DocumentCodecSpec")), key(fixture.native.specFields), "erased spec fields");
    for (const trait of fixture.native.traits) {
      const scope = block(plugin, trait.header);
      assert.equal(sequence(plugin, fixture.native.mutationField, scope.open + 1, scope.close).length, 1, trait.name + " mutation qualification");
    }
    block(indexed.mutation.items, fixture.native.protocolTrait);
    const store = indexed.store.items, codec = block(store, "pub struct ArtifactCodec");
    for (const field of fixture.native.codecFields) assert.equal(sequence(store, field, codec.open + 1, codec.close).length, 1, "exact codec field");
  } catch (error) { result.problems.push("preserved-boundary:" + String(error)); }
  result.missing.sort(); result.problems.sort();
  return result;
}
//#endregion 🧾️SourceObligations

//#region 🧪️IndependentOracles
function modelAndSchema(fixture: Fixture, schema: object): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const independent = _.sortBy(_.uniq(_.flatMap(fixture.routes, route => route.sites)));
  assert.equal(_.flatMap(fixture.routes, route => route.sites).length, independent.length, "no duplicated route obligations");
  assert.deepEqual(requiredSites(fixture), independent);
  assert.deepEqual(independent, [...fixture.requiredSites].sort());
  assert.deepEqual(_.sortBy(_.map(fixture.native.sites, "id")), independent);
  assert.equal(_.uniqBy(fixture.routes, "id").length, fixture.routes.length);
  for (const site of fixture.native.sites) {
    assert(fixture.routes.find(route => route.id === site.route)?.sites.includes(site.id), site.id + " route");
    assert.equal(site.qualification, site.route === "bare" ? "bare" : "associated", site.id + " qualification form");
  }
  assert.equal(fixture.native.sites.filter(site => site.qualification === "associated").length, 8);
  assert.equal(fixture.native.sites.filter(site => site.qualification === "bare").length, 4);
  for (const hostile of [
    { ...fixture, requiredSites: fixture.requiredSites.slice(1) },
    { ...fixture, requiredSites: [...fixture.requiredSites.slice(1), fixture.requiredSites[1]] },
    { ...fixture, access: { ...fixture.access, sharedMutationAcrossSuspension: "send" } },
  ]) assert.equal(validate(hostile), false);
}

function hostileSources(fixture: Fixture, sources: Sources): void {
  const candidate = rewriteHeaders(views(sources), fixture, fixture.native.sites, desiredHeader), candidateViews = views(candidate);
  assert.deepEqual(inspect(fixture, candidate), { missing: [], problems: [] }, "in-memory qualified candidate");
  for (const site of fixture.native.sites) {
    const hostile = rewriteHeaders(candidateViews, fixture, [site], target => target.header);
    assert.notEqual(hostile[site.source], candidate[site.source], site.id + " actual mutation");
    assert.deepEqual(inspect(fixture, hostile), { missing: [site.id], problems: [] }, site.id + " exact single-site omission");
  }
  const fakeDeclaration = "pub struct DocumentCodecSpec { fake: bool } fn document_codec<A: ArtifactApp>() {}";
  const copiedComment = "/* outer /* inner */ " + fakeDeclaration + " */";
  assert.deepEqual(inspect(fixture, { ...candidate, plugin: copiedComment + candidate.plugin }), { missing: [], problems: [] }, "comments are not source declarations");
  assert.deepEqual(inspect(fixture, { ...candidate, plugin: 'const _: &str = r###"' + fakeDeclaration + '"###;' + candidate.plugin }), { missing: [], problems: [] }, "raw strings are not source declarations");
  const changedBody = { ...candidate, plugin: candidate.plugin.replace("let codec = DocumentCodecSpec::of::<A>().await;", "let codec = DocumentCodecSpec::of::<A>();") };
  assert.notEqual(changedBody.plugin, candidate.plugin);
  assert(inspect(fixture, changedBody).problems.some(problem => problem.endsWith(":body-changed")), "await removal");
  for (const name of ["ArtifactApp", "ArtifactEditor", "ArtifactViewer"]) {
    const header = fixture.native.traits.find(trait => trait.name === name)!.header;
    const hostile = { ...candidate, plugin: candidate.plugin.replace(header, header + " + Sync") };
    assert.notEqual(hostile.plugin, candidate.plugin);
    assert(inspect(fixture, hostile).problems.length > 0, name + " global widening");
  }
  for (const [id, bound] of [["spec.app", "A: Sync"], ["builder.foreign", "PA: Sync"], ["spec.app", "<A::Mutation as ::protocol::Mutation<A::Snapshot>>::Diff: Sync"]]) {
    const site = fixture.native.sites.find(site => site.id === id)!;
    const hostile = rewriteHeaders(candidateViews, fixture, [site], target => desiredHeader(target) + ", " + bound);
    assert.notEqual(hostile[site.source], candidate[site.source]);
    assert(inspect(fixture, hostile).problems.includes(site.id + ":unexpected-header"), "unreviewed broad local bound: " + bound);
  }
  const broadImpl = { ...candidate, builder: candidate.builder.replace("impl<PA: PluginApp> PluginBuilder<Ready, PA>", "impl<PA: PluginApp + Sync> PluginBuilder<Ready, PA>") };
  assert.notEqual(broadImpl.builder, candidate.builder);
  assert(inspect(fixture, broadImpl).problems.length > 0, "enclosing impl widening");
  const capturing = { ...candidate, plugin: candidate.plugin.replace("codec: fn(String) -> store::ArtifactCodec,", "codec: Box<dyn Fn(String) -> store::ArtifactCodec>,") };
  assert.notEqual(capturing.plugin, candidate.plugin);
  assert(inspect(fixture, capturing).problems.length > 0, "erased fn pointer replacement");
  for (const field of fixture.native.codecFields.slice(0, 2)) {
    const sourceTokens = tokens(candidate.store), positions = sequence(sourceTokens, field);
    assert.equal(positions.length, 1);
    const from = sourceTokens[positions[0]].start, to = sourceTokens[positions[0] + tokens(field).length - 1].end;
    const hostile = { ...candidate, store: candidate.store.slice(0, from) + field.replace(" + Send", "") + candidate.store.slice(to) };
    assert.notEqual(hostile.store, candidate.store);
    assert(inspect(fixture, hostile).problems.length > 0, "erased Send removal");
  }
  console.log("[DEBUG] codec caller source hostiles: 12 exact local omissions; await/global-trait/A/PA/Diff/impl/erased-slot guards; in-memory candidate only");
}

export function testPluginCodecCallerSource(repoRoot: string): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8")) as Fixture;
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  modelAndSchema(fixture, schema);
  const sources = Object.fromEntries(Object.entries(fixture.native.sources).map(([name, path]) => [name, readFileSync(resolve(repoRoot, path), "utf8")])) as Sources;
  hostileSources(fixture, sources);
  const result = inspect(fixture, sources);
  console.log("[DEBUG] codec caller current-source desired law " + JSON.stringify(result));
  assert.deepEqual(result, { missing: [], problems: [] }, "actual Plugin codec qualification source boundary");
}
//#endregion 🧪️IndependentOracles
