//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔮️ ORACLE ONLY. This repository ships no TypeScript executor; this adapter hosts the `graphql`
// npm package so the corpus is judged by a conforming executor rather than by a second reading of
// our own source. Everything below the `🖼️Sources` region is a DECLARED mapping of the frozen
// records onto the committed SDL's field names — it restates the derivations, and `graphql-js`
// alone decides what executing a selection set against them means. Nothing here walks a selection
// set, resolves an alias, answers a `__typename` or coerces an argument: that is the executor's
// job, and it is exactly what this case measures.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { buildSchema, execute, parse } from "graphql";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔤️Identity
/** 🔤️ The code points whose default presentation is text, and therefore carry U+FE0F in an id. */
const TEXT_DEFAULT = ["🏗", "⌨", "🖱", "🗃", "⚙", "⚖", "🏷", "🛠", "✂", "🛡", "🗑", "☀", "⏱", "✏", "👮", "⬅", "⬆", "⬇"];

/** 😀️ Normalises an emoji to the presentation form an identifier carries. */
function emojiText(emoji: string): string {
  const base = [...emoji].filter((character) => character !== "︎" && character !== "️").join("");
  for (const textDefault of TEXT_DEFAULT) if (base.includes(textDefault)) return base.replace(textDefault, `${textDefault}️`);
  return base;
}

/** 🔤️ The flattened, lower-cased ASCII core of a name. */
function flat(value: string): string {
  return [...value.toLowerCase()].filter((character) => /[a-z0-9]/.test(character)).join("");
}

/** 🔤️ Title-cases one dash separated slug, word by word. */
function titleizeSlug(slug: string): string {
  return slug
    .split("-")
    .map((word) => (word === "" ? "" : word[0]!.toUpperCase() + word.slice(1).toLowerCase()))
    .join(" ");
}

const ENTITY = { ticket: "🎫", contributor: "🧑‍💻", todo: "📝", breach: "🚫", section: "🔖", statute: "" } as const;
const TECHNOLOGY_EMOJI: Record<string, string> = { user: "👤️", infrastructure: "🧰️", research: "🔬️", mono: "🌱️" };
const BUNDLE_EMOJI: Record<string, string> = { library: "📚", schema: "🛂", binary: "⌨", ui: "🖱", site: "🌐", assets: "🏪", repo: "🪆" };
const DEFINITION_KIND: Record<string, string> = { implementation: "IMPLEMENTATION", interface: "INTERFACE", constant: "CONSTANT" };
const PRIORITY: Record<string, string> = { high: "HIGH", medium: "MEDIUM", low: "LOW" };
const TICKET_STATUS: Record<string, string> = { open: "OPEN", closed: "CLOSED" };
const TICKET_CLIENT: Record<string, string> = {
  "copilot-chat": "COPILOT_CHAT",
  windsurf: "WINDSURF",
  "windsurf-chat": "WINDSURF_CHAT",
  antigravity: "ANTIGRAVITY",
  "antigravity-chat": "ANTIGRAVITY_CHAT",
  cursor: "CURSOR",
  "cursor-chat": "CURSOR_CHAT",
  vscode: "VSCODE",
  "claude-code": "CLAUDE_CODE",
  codex: "CODEX",
  droid: "DROID",
  "kiro-cli": "KIRO_CLI",
};

/** 🏷️ The technology family a bare technology name belongs to. */
function technologyKind(name: string): string {
  if (name === "compose") return TECHNOLOGY_EMOJI.user!;
  if (name === "repo") return TECHNOLOGY_EMOJI.infrastructure!;
  if (name === "coda") return TECHNOLOGY_EMOJI.research!;
  return name.startsWith("@") ? technologyKind(name.slice(1)) : TECHNOLOGY_EMOJI.user!;
}
//#endregion 🔤️Identity

//#region 🖼️Sources
type Row = Record<string, any>;

/** 🔤️ A blank string reads as an absent value. */
const optional = (value: unknown): string | null => (typeof value === "string" && value !== "" ? value : null);

/** 📁️ Joins a repository-relative path onto the root, always with forward slashes. */
function fileUri(rootDir: string, relative: string): string {
  const root = rootDir.replace(/\\/g, "/");
  const tail = (relative ?? "").replace(/\\/g, "/");
  return tail === "" ? `file://${root}` : `file://${root.replace(/\/+$/, "")}/${tail.replace(/^\/+/, "")}`;
}

/** 🕰️ Normalises a recorded timestamp into the rendering the `DateTime` scalar emits. */
function rfc3339(raw: string): string | null {
  const trimmed = (raw ?? "").trim();
  if (trimmed.length < 19 || (trimmed[10] !== " " && trimmed[10] !== "T")) return null;
  const time = trimmed.slice(11).split(/[+Z.]/)[0]!;
  return time.length === 8 ? `${trimmed.slice(0, 10)}T${time}Z` : null;
}

/** 🎫️ Whether an interaction kind is the expected one, tolerating the `.ended` suffix. */
const isKind = (kind: string, expected: string): boolean => kind.trim() === expected || kind.trim().replace(/\.ended$/, "") === expected;

/** 🧹️ Whether a text survives one filter input. */
function matchesFilter(text: string, filter: Row | null | undefined): boolean {
  const needle = filter?.filter;
  if (typeof needle !== "string" || needle === "") return true;
  const [haystack, sought] = filter?.matchCase === true ? [text, needle] : [text.toLowerCase(), needle.toLowerCase()];
  if (filter?.matchWholeWord === true) return haystack.split(/[^\p{L}\p{N}]+/u).includes(sought);
  return haystack.includes(sought);
}

/** 🗄️ Builds every source object the committed SDL's default resolution reads from. */
function sources(records: Row): Row {
  const rootDir: string = records.rootDir ?? "";
  const technologies: Row[] = records.technologies ?? [];
  const statutes: Row[] = records.statutes ?? [];

  const technologyId = (technology: Row): string => emojiText(technology.emoji !== "" ? technology.emoji : technology.name.includes("repo") ? TECHNOLOGY_EMOJI.infrastructure! : technology.name.startsWith("coda") ? TECHNOLOGY_EMOJI.research! : "") + flat(technology.name);
  const bundleId = (bundle: Row): string => {
    const emoji = bundle.emoji !== "" && bundle.emoji !== undefined ? bundle.emoji : (BUNDLE_EMOJI[bundle.kind] ?? "");
    const [technologyCode, bundleCode] = bundle.name.includes("/") ? [bundle.name.slice(0, bundle.name.indexOf("/")), bundle.name.slice(bundle.name.indexOf("/") + 1)] : [bundle.name, bundle.name];
    const owner = technologies.find((candidate) => candidate.name === technologyCode && candidate.emoji !== "" && candidate.emoji !== undefined);
    return emojiText(owner === undefined ? technologyKind(technologyCode) : owner.emoji) + flat(technologyCode) + emojiText(emoji) + flat(bundleCode);
  };

  const packageSource = (row: Row): Row => ({ __typename: "Package", name: row.name, version: row.version, path: row.path, kind: row.kind });
  const bundleSource = (bundle: Row): Row => ({
    __typename: "Bundle",
    id: bundleId(bundle),
    name: bundle.name,
    root: bundle.root,
    sourceRoot: optional(bundle.sourceRoot),
    projectType: null,
    tags: bundle.tags ?? [],
    packages: (bundle.packages ?? []).map(packageSource),
    kind: bundle.kind,
    uri: fileUri(rootDir, bundle.root),
    folders: [],
    files: [],
    breachs: [],
  });
  const technologySource = (technology: Row): Row => ({
    __typename: "Technology",
    id: technologyId(technology),
    name: technology.name,
    root: technology.root,
    kind: technology.kind,
    bundles: (technology.bundles ?? []).map(bundleSource),
    uri: `repo://technology/${technologyId(technology)}`,
  });

  const folderSource = (folder: Row): Row => ({ __typename: "Folder", ...folder, parent: null, bundle: null, breachs: [], children: () => (records.folders ?? []).filter((candidate: Row) => candidate.parentId === folder.id).map(folderSource), files: () => (records.files ?? []).filter((candidate: Row) => candidate.folderId === folder.id).map(fileSource) });
  const fileSource = (file: Row): Row => ({
    __typename: "File",
    ...file,
    folder: null,
    bundle: null,
    content: null,
    breachs: [],
    contributors: [],
    sections: () => (records.sections ?? []).filter((candidate: Row) => candidate.filePath === file.path).map(sectionSource),
    definitions: () => (records.definitions ?? []).filter((candidate: Row) => candidate.filePath === file.path).map(definitionSource),
  });
  const fileByPath = (path: string): Row | null => {
    const found = (records.files ?? []).find((candidate: Row) => candidate.path === path.replace(/\\/g, "/"));
    return found === undefined ? null : fileSource(found);
  };

  const range = (start: number, end: number): Row => ({ __typename: "Range", start, end });
  const definitionSource = (definition: Row): Row => ({
    __typename: "Definition",
    id: definition.id,
    name: definition.name,
    kind: DEFINITION_KIND[definition.kind] ?? definition.kind,
    startLine: definition.startLine,
    startIndex: definition.startIndex,
    file: () => fileByPath(definition.filePath ?? ""),
    section: () => (definition.sectionPath === undefined || definition.sectionPath === "" ? null : sectionSource({ name: definition.sectionPath, path: definition.sectionPath, filePath: definition.filePath, emoji: "", startLine: 0, endLine: 0, startIndex: 0, endIndex: 0, children: [], definitions: [] })),
    breachs: [],
    range: range(definition.startLine ?? 0, definition.endLine ?? 0),
  });
  const sectionSource = (section: Row): Row => {
    const children = (section.children ?? []).map(sectionSource);
    const definitions = (section.definitions ?? []).map(definitionSource);
    return {
      __typename: "Section",
      id: section.id !== undefined && section.id !== "" ? section.id : emojiText(section.emoji !== "" ? section.emoji : ENTITY.section) + flat(section.name),
      name: section.name,
      path: section.path ?? "",
      startLine: section.startLine ?? 0,
      startIndex: section.startIndex ?? 0,
      file: () => (section.filePath === undefined || section.filePath === "" ? null : fileByPath(section.filePath)),
      parent: null,
      children: [...children, ...definitions].sort((left, right) => left.startLine - right.startLine || left.startIndex - right.startIndex),
      definitions,
      breachs: [],
      range: range(section.startLine ?? 0, section.endLine ?? 0),
    };
  };

  const statuteMetaOf = (kind: string): Row => statutes.find((candidate) => candidate.kind === kind) ?? { kind, policyId: "", priority: "low", reason: "Unknown breach", solution: "Fix the breach", autofixable: false };
  const statuteSource = (meta: Row): Row => ({
    __typename: "Statute",
    id: emojiText(ENTITY.statute) + meta.kind.split("/").map(titleizeSlug).join("#"),
    priority: PRIORITY[meta.priority] ?? meta.priority,
    autofixable: meta.autofixable,
    reason: meta.reason,
    solution: meta.solution,
    policy: { __typename: "Policy", id: "/policies/lint-scripts", name: "Lint scripts", description: null, scopes: ["**/*"], groups: [], statutes: [] },
    breachs: null,
  });
  const breachSource = (breach: Row): Row => {
    const meta = { ...statuteMetaOf(breach.kind) };
    if (breach.priority !== undefined) meta.priority = breach.priority;
    if (breach.autofixable !== undefined) meta.autofixable = breach.autofixable;
    if (optional(breach.reason) !== null) meta.reason = breach.reason;
    if (optional(breach.solution) !== null) meta.solution = breach.solution;
    return {
      __typename: "Breach",
      id: emojiText(ENTITY.breach) + breach.id,
      kindId: breach.kind,
      kind: statuteSource(meta),
      scope: breach.scope,
      file: null,
      folder: null,
      line: breach.line ?? 0,
      column: breach.column ?? 0,
      excerpt: optional(breach.excerpt),
      summary: breach.summary,
      priority: PRIORITY[meta.priority] ?? meta.priority,
      autofixable: meta.autofixable,
    };
  };
  const territorySource = (territory: Row): Row => ({ __typename: "Territory", name: territory.name, description: territory.description, scopes: territory.scopes ?? [], groups: (territory.groups ?? []).map(territorySource), kinds: (territory.kinds ?? []).map((kind: string) => statuteSource(statuteMetaOf(kind))) });
  const policySource = (policy: Row): Row => ({ __typename: "Policy", id: policy.id, name: policy.name, description: policy.description ?? null, scopes: policy.scopes ?? [], groups: (policy.groups ?? []).map(territorySource), statutes: (policy.statutes ?? []).map(statuteSource) });

  const interactionSource = (interaction: Row): Row => ({ __typename: "Interaction", kind: interaction.kind, prompt: interaction.prompt ?? "", checkpoint: interaction.checkpoint ?? "", llm: interaction.llm ?? "", effort: interaction.effort ?? "", date: interaction.date, system: interaction.system ?? "", client: interaction.client ?? "", author: interaction.author ?? "" });
  const interactionResourceSource = (interaction: Row): Row => ({ ...interactionSource(interaction), __typename: "InteractionResource", sourceKind: interaction.sourceKind, sourceId: interaction.sourceId, goalId: interaction.goalId ?? "", ticketId: interaction.ticketId ?? "" });

  const contributorSource = (contributor: Row): Row => ({
    __typename: "Contributor",
    id: emojiText(ENTITY.contributor) + flat(contributor.alias),
    github: contributor.github,
    emoji: optional(contributor.emoji),
    name: contributor.name,
    names: contributor.names ?? [],
    email: contributor.email,
    emails: contributor.emails ?? [],
    fingerprint: optional(contributor.fingerprint),
    fingerprints: contributor.fingerprints ?? [],
    links: Object.entries(contributor.links ?? {})
      .map(([name, url]) => ({ __typename: "ContributorLink", name, url }))
      .sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0)),
    icons: null,
    contributions: null,
    bundles: null,
    files: null,
    tickets: null,
  });

  const started = (ticket: Row): string => {
    for (const interaction of ticket.interactions ?? []) if (isKind(interaction.kind, "ticket.open")) { const parsed = rfc3339(interaction.date); if (parsed !== null) return parsed; }
    const first = (ticket.interactions ?? [])[0];
    const parsed = first === undefined ? null : rfc3339(first.date);
    return parsed ?? `${String(ticket.year).padStart(4, "0")}-${String(ticket.month).padStart(2, "0")}-${String(ticket.day).padStart(2, "0")}T00:00:00Z`;
  };
  const finished = (ticket: Row): string | null => {
    const closing = [...(ticket.interactions ?? [])].reverse().find((interaction: Row) => isKind(interaction.kind, "ticket.close"));
    return closing === undefined ? null : rfc3339(closing.date);
  };
  const latest = (ticket: Row, field: string): string | null => {
    const interactions = ticket.interactions ?? [];
    if (interactions.length > 0) return optional(interactions[interactions.length - 1][field]);
    const agents = ticket.agents ?? [];
    return agents.length > 0 ? optional(agents[agents.length - 1][field]) : null;
  };
  const ticketSource = (ticket: Row): Row => ({
    __typename: "Ticket",
    id: emojiText(ENTITY.ticket) + flat(ticket.slug),
    year: ticket.year,
    month: ticket.month,
    day: ticket.day,
    slug: ticket.slug,
    path: optional(ticket.folderPath) ?? ticket.jsonPath ?? "",
    llm: latest(ticket, "llm"),
    effort: latest(ticket, "effort"),
    client: (() => { const client = latest(ticket, "client"); return client === null ? null : (TICKET_CLIENT[client] ?? client); })(),
    checkpoint: optional((ticket.interactions ?? [])[0]?.checkpoint),
    uri: fileUri(rootDir, ticket.folderPath ?? ""),
    title: ticket.title,
    emoji: optional(ticket.emoji),
    prompt: optional(ticket.description) ?? (ticket.interactions ?? [])[0]?.prompt ?? "",
    summary: null,
    status: TICKET_STATUS[ticket.status] ?? ticket.status,
    interactions: (ticket.interactions ?? []).map(interactionSource),
    author: () => {
      const interactions = ticket.interactions ?? [];
      if (interactions.length === 0) return null;
      const raw: string = interactions[interactions.length - 1].author ?? "";
      const [name, email] = raw.includes(" <") ? [raw.slice(0, raw.indexOf(" <")).trim(), raw.slice(raw.indexOf(" <") + 2).replace(/>$/, "")] : [raw, ""];
      const author = email === "" ? name : email;
      const found = (records.contributors ?? []).find((candidate: Row) => (candidate.emails ?? []).some((mail: string) => mail === author || author.includes(mail)) || candidate.name === author);
      return contributorSource(found ?? { alias: author, emoji: "", github: author, name: author, names: [], email: "", emails: [author], links: {}, fingerprint: "" });
    },
    dates: { __typename: "TicketDate", started: started(ticket), finished: finished(ticket) },
    goal: optional(ticket.goal),
    parent: optional(ticket.parent),
    bundles: null,
    files: null,
  });

  const goalSource = (goal: Row): Row => ({
    __typename: "Goal",
    id: goal.id,
    title: goal.title,
    description: goal.description,
    prompt: goal.prompt,
    dueDate: optional(goal.dates?.due),
    createdAt: null,
    client: goal.client,
    llm: goal.llm,
    effort: optional(goal.effort),
    status: goal.status,
    milestone: (() => {
      const milestone: string = goal.github?.milestone ?? "";
      const direct = Number.parseInt(milestone, 10);
      if (!Number.isNaN(direct) && String(direct) === milestone) return direct;
      const tail = Number.parseInt(milestone.slice(milestone.lastIndexOf("/") + 1), 10);
      return Number.isNaN(tail) ? null : tail;
    })(),
    issue: optional(goal.github?.issue),
    parent: optional(goal.parent),
    interactions: [],
  });

  const todoSource = (todo: Row): Row => ({ __typename: "Todo", id: emojiText(ENTITY.todo) + flat(todo.id), name: todo.name, description: optional(todo.description), parentId: todo.parentId, location: todo.location === undefined ? null : { __typename: "Location", ...todo.location } });
  const analyzeSource = (scope: string | null | undefined): Row => {
    const analyze = records.analyze ?? { breachs: [], metrics: { total: 0, byPriority: { high: 0, medium: 0, low: 0 }, autofixable: 0 } };
    const breachs = (analyze.breachs ?? []).filter((breach: Row) => scope === null || scope === undefined || String(breach.scope).startsWith(scope));
    return { __typename: "AnalyzeResult", breachs: breachs.map(breachSource), metrics: { __typename: "AnalyzeMetrics", total: analyze.metrics?.total ?? 0, autofixable: analyze.metrics?.autofixable ?? 0, byPriority: { __typename: "PriorityCount", ...(analyze.metrics?.byPriority ?? { high: 0, medium: 0, low: 0 }) } } };
  };

  const ticketRows = (args: Row): Row[] =>
    (records.tickets ?? [])
      .filter((ticket: Row) => args.year === undefined || args.year === null || ticket.year === args.year)
      .filter((ticket: Row) => args.month === undefined || args.month === null || ticket.month === args.month)
      .filter((ticket: Row) => args.day === undefined || args.day === null || ticket.day === args.day)
      .filter((ticket: Row) => args.status === undefined || args.status === null || TICKET_STATUS[ticket.status] === args.status)
      .filter((ticket: Row) => matchesFilter(ticket.slug, args.filter))
      .map(ticketSource);

  const repoSource: Row = {
    __typename: "Repo",
    id: "repo:compose",
    name: "compose",
    path: rootDir,
    technologies: () => [...technologies].sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0)).map(technologySource),
    checkpoints: (args: Row) => (typeof args.limit === "number" ? (records.checkpoints ?? []).slice(0, args.limit) : (records.checkpoints ?? [])).map((checkpoint: Row) => ({ __typename: "Checkpoint", ...checkpoint })),
    bundles: () => (records.bundles ?? []).map(bundleSource),
    folders: () => (records.folders ?? []).map(folderSource),
    files: () => (records.files ?? []).map(fileSource),
    sections: () => (records.sections ?? []).map(sectionSource),
    definitions: () => (records.definitions ?? []).map(definitionSource),
    contributors: () => (records.contributors ?? []).map(contributorSource),
    goals: () => (records.goals ?? []).map(goalSource),
    tickets: (args: Row) => ticketRows(args),
    policies: () => (records.policies ?? []).map(policySource),
    statutes: () => statutes.map(statuteSource),
    breachs: (args: Row) => analyzeSource(args.scope).breachs,
  };

  return {
    node: () => null,
    repo: () => repoSource,
    technologies: (args: Row) => [...technologies].sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0)).filter((technology) => matchesFilter(technology.name, args.filter) || matchesFilter(technologyId(technology), args.filter)).map(technologySource),
    bundles: (args: Row) => (records.bundles ?? []).filter((bundle: Row) => matchesFilter(bundle.name, args.filter)).map(bundleSource),
    folders: () => (records.folders ?? []).map(folderSource),
    files: () => (records.files ?? []).map(fileSource),
    sections: () => (records.sections ?? []).map(sectionSource),
    definitions: () => (records.definitions ?? []).map(definitionSource),
    contributors: (args: Row) => (records.contributors ?? []).filter((contributor: Row) => matchesFilter(contributor.github, args.filter)).map(contributorSource),
    todos: (args: Row) => (records.todos ?? []).filter((todo: Row) => matchesFilter(todo.name, args.filter)).map(todoSource),
    tickets: (args: Row) => ticketRows(args),
    interactions: () => (records.interactions ?? []).map(interactionResourceSource),
    drafts: () => (records.drafts ?? []).map((draft: Row) => ({ __typename: "Draft", ...draft })),
    policies: (args: Row) => (records.policies ?? []).filter((policy: Row) => matchesFilter(policy.name, args.filter)).map(policySource),
    statutes: () => statutes.map(statuteSource),
    breachs: (args: Row) => analyzeSource(args.scope).breachs,
    bundle: (args: Row) => bundleSource((records.bundles ?? []).find((bundle: Row) => bundle.name === args.name || bundleId(bundle) === args.name) ?? { name: args.name, root: "", technologyName: "", tags: [], kind: "library", emoji: "", packages: [] }),
    folder: (args: Row) => { const found = (records.folders ?? []).find((folder: Row) => folder.path === String(args.path).replace(/\\/g, "/")); return found === undefined ? null : folderSource(found); },
    file: (args: Row) => fileByPath(String(args.path)),
    section: (args: Row) => { const joined = (args.sectionPath ?? []).join("#"); const found = (records.sections ?? []).find((section: Row) => section.filePath === args.path && section.path === joined); return sectionSource(found ?? { name: joined, path: "", filePath: "", emoji: "", startLine: 0, endLine: 0, startIndex: 0, endIndex: 0, children: [], definitions: [] }); },
    definition: (args: Row) => { const found = (records.definitions ?? []).find((definition: Row) => definition.filePath === args.path && definition.name === args.name); return definitionSource(found ?? { id: "", name: args.name, kind: "implementation", filePath: "", sectionPath: "", emoji: "", startLine: 0, endLine: 0, startIndex: 0, endIndex: 0 }); },
    contributor: (args: Row) => contributorSource((records.contributors ?? []).find((contributor: Row) => contributor.github === args.id) ?? { alias: args.id, emoji: "", github: args.id, name: "", names: [], email: "", emails: [], links: {}, fingerprint: "" }),
    ticket: (args: Row) => { const found = ticketRows({ year: args.year, month: args.month, day: args.day }).find((ticket: Row) => ticket.slug === args.slug); return found ?? null; },
    policy: (args: Row) => { const found = (records.policies ?? []).find((policy: Row) => policy.name === args.id || policy.id === args.id || policy.name.toLowerCase() === String(args.id).toLowerCase() || policy.id.toLowerCase() === String(args.id).toLowerCase()); return policySource(found ?? { id: `repo/policy/${args.id}`, name: args.id, description: null, scopes: [], groups: [], statutes: [] }); },
    statute: (args: Row) => statuteSource(statuteMetaOf(String(args.id))),
    analyze: (args: Row) => analyzeSource(args.scope),
  };
}
//#endregion 🖼️Sources

//#region 🧭️Adapter
type Corpus = { queries: { id: string; source: string; variables?: Record<string, unknown> }[] };

/** ⚙️ Executes the corpus through the reference executor over the committed SDL. */
function runCorpus(ctx: any, keep: (entry: Corpus["queries"][number]) => boolean): { projection: unknown } {
  const schema = buildSchema(Buffer.from(ctx.fixtureBytes("shared://📜️served-schema/🔗️.graphql")).toString("utf8"));
  const records = JSON.parse(Buffer.from(ctx.fixtureBytes("shared://🔣️repo-records.json")).toString("utf8")) as Row;
  const rootValue = sources(records);
  const corpus = JSON.parse(Buffer.from(ctx.fixtureBytes("shared://▶️query-execution/🔣️queries.json")).toString("utf8")) as Corpus;
  const queries = corpus.queries.filter(keep).map((entry) => {
    const result = execute({ schema, document: parse(entry.source), rootValue, variableValues: entry.variables ?? {} });
    if ("then" in (result as object)) throw new Error(`${entry.id}: the reference executor went asynchronous`);
    const sync = result as { data?: unknown; errors?: readonly { message: string }[] };
    if (sync.errors !== undefined && sync.errors.length > 0) throw new Error(`${entry.id}: ${sync.errors.map((error) => error.message).join("; ")}`);
    return { id: entry.id, data: sync.data };
  });
  return { projection: { queries } };
}

/** 🔷️ Whether a query carries an argument or a variable. */
const parameterised = (entry: Corpus["queries"][number]): boolean => entry.variables !== undefined || entry.source.includes("(");

/** 🔮️ TypeScript host of the `graphql` reference executor for the query-execution case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "corpus-executes-identically": { oracle: (ctx) => runCorpus(ctx, () => true) },
    "arguments-coerce-before-resolution": { oracle: (ctx) => runCorpus(ctx, parameterised) },
  },
});
//#endregion 🧭️Adapter
