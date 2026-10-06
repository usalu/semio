/** 🌐️ HTML's native typed elements, raw text, boolean attributes and child ownership. */
import type { HtmlSnapshot, HtmlNode } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, artifactSqliteValueByteLengthControlled, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow, SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted schema byte-equal to the adjacent SQL asset. */
export const HTML_SQLITE_SCHEMA = "CREATE TABLE html_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, doctype TEXT, root_node_id INTEGER NOT NULL REFERENCES html_node(id));\nCREATE TABLE html_node (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK(kind IN ('element','text','comment','raw_text')));\nCREATE TABLE html_element (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), name TEXT NOT NULL);\nCREATE TABLE html_text (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), text TEXT NOT NULL);\nCREATE TABLE html_comment (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), text TEXT NOT NULL);\nCREATE TABLE html_raw_text (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), parent_kind TEXT NOT NULL CHECK(parent_kind IN ('script','style')), text TEXT NOT NULL);\nCREATE TABLE html_attribute (id INTEGER PRIMARY KEY, element_node_id INTEGER NOT NULL REFERENCES html_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL, value TEXT);\nCREATE TABLE html_child (id INTEGER PRIMARY KEY, parent_element_node_id INTEGER NOT NULL REFERENCES html_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), child_node_id INTEGER NOT NULL REFERENCES html_node(id));\n";
const TABLES = ["html_document", "html_node", "html_element", "html_text", "html_comment", "html_raw_text", "html_attribute", "html_child"] as const;
const COMPONENTS = [["html_element","element"],["html_text","text"],["html_comment","comment"],["html_raw_text","raw_text"]] as const;
function kind(node: HtmlNode): string { return node.kind === "rawText" ? "raw_text" : node.kind; }
async function measure(snapshot: HtmlSnapshot, options: ArtifactSqliteOptions): Promise<number> {
  let rows = 1, checked = 0;
  let bytes = 16 + await artifactSqliteValueByteLengthControlled(snapshot.schema, options, "projectSnapshot", 16);
  if (snapshot.doctype !== undefined) bytes += await artifactSqliteValueByteLengthControlled(snapshot.doctype, options, "projectSnapshot", bytes);
  const check = (): void => { if (rows > (options.maxRows ?? 1_000_000)) throw new Error("HTML SQLite row limit"); artifactSqliteValueBudget(bytes, options); };
  check();
  const stack = [{ node: snapshot.root, finish: false }];
  const active = new Set<HtmlNode>();
  while (stack.length) {
    const { node, finish } = stack.pop()!;
    if (finish) { active.delete(node); continue; }
    if (active.has(node)) throw new Error("HTML snapshot contains a cycle");
    active.add(node); stack.push({ node, finish: true });
    rows += 2; bytes += 16 + artifactSqliteTextBytes(kind(node));
    if (node.kind === "element") {
      bytes += await artifactSqliteValueByteLengthControlled(node.name, options, "projectSnapshot", bytes);
      rows += node.attributes.length + node.children.length;
      if (rows + 2 * node.children.length > (options.maxRows ?? 1_000_000)) throw new Error("HTML SQLite row limit");
      bytes += 32 * node.children.length;
      check();
      for (const attr of node.attributes) {
        bytes += 24; bytes += await artifactSqliteValueByteLengthControlled(attr.name, options, "projectSnapshot", bytes);
        if (attr.value !== undefined) bytes += await artifactSqliteValueByteLengthControlled(attr.value, options, "projectSnapshot", bytes); check();
        if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
      }
      for (let index = node.children.length - 1; index >= 0; index--) stack.push({ node: node.children[index]!, finish: false });
    } else if (node.kind === "text" || node.kind === "comment") bytes += await artifactSqliteValueByteLengthControlled(node.text, options, "projectSnapshot", bytes);
    else if (node.kind === "rawText") {
      if (node.parentKind !== "script" && node.parentKind !== "style") throw new Error("HTML raw text parent kind is invalid");
      bytes += node.parentKind.length; bytes += await artifactSqliteValueByteLengthControlled(node.text, options, "projectSnapshot", bytes);
    } else throw new Error("HTML node kind is invalid");
    check();
    if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
  }
  return rows;
}

/** 📤️ Project HTML's own node variants without serializing native HTML markup. */
export async function htmlSnapshotToSqliteDatabase(snapshot: HtmlSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  const total = await measure(snapshot, options);
  const rows = new Map<string,SqliteRow[]>(TABLES.map(name => [name,[]]));
  let completed = 0;
  const insert = (name: string, cells: SqliteValue[], identity?: bigint): bigint => {
    const table = rows.get(name)!;
    const id = identity ?? BigInt(table.length + 1);
    table.push({ rowid: id, values: [id,...cells] }); completed++;
    return id;
  };
  const tick = async (): Promise<void> => { if (completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total); };
  insert("html_document",[snapshot.schema,snapshot.doctype ?? null,1n]);
  const stack: { node: HtmlNode; parent?: bigint; ordinal?: number }[] = [{ node: snapshot.root }];
  while (stack.length) {
    const { node,parent,ordinal } = stack.pop()!;
    const id = insert("html_node",[kind(node)]);
    switch (node.kind) {
      case "element":
        insert("html_element",[node.name],id);
        for (let ordinal = 0; ordinal < node.attributes.length; ordinal++) {
          const attr = node.attributes[ordinal]!;
          insert("html_attribute",[id,BigInt(ordinal),attr.name,attr.value ?? null]); await tick();
        }
        for (let ordinal = node.children.length - 1; ordinal >= 0; ordinal--) stack.push({ node: node.children[ordinal]!, parent: id, ordinal });
        break;
      case "text": insert("html_text",[node.text],id); break;
      case "comment": insert("html_comment",[node.text],id); break;
      case "rawText": insert("html_raw_text",[node.parentKind,node.text],id); break;
    }
    if (parent !== undefined) insert("html_child",[parent,BigInt(ordinal!),id]);
    await tick();
  }
  const database = await artifactSqliteDatabase(HTML_SQLITE_SCHEMA,TABLES.map(name => rows.get(name)!),options);
  await artifactSqliteCheckpoint(options,"projectSnapshot",total,total);
  return database;
}
function group(groups: Map<bigint,SqliteRow[]>, owner: bigint, row: SqliteRow): void { const list = groups.get(owner) ?? []; list.push(row); groups.set(owner,list); }

/** 📥️ Restore optional attributes and raw text through checked native HTML ownership. */
export async function htmlSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<HtmlSnapshot> {
  const total = database.tables.reduce((sum,table) => sum + table.rows.length,0);
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,total);
  if (total > (options.maxRows ?? 1_000_000)) throw new Error("HTML SQLite row limit");
  let fieldBytes = 0;
  for (const table of database.tables) for (const row of table.rows) for (const value of row.values) fieldBytes += await artifactSqliteValueByteLengthControlled(value, options, "reconstructSnapshot", fieldBytes);
  const tableRows = await artifactSqliteTables(database,HTML_SQLITE_SCHEMA,options);
  const rows = new Map<string,readonly SqliteRow[]>(TABLES.map((name,index) => [name,tableRows[index]!]));
  const document = artifactSqliteDocument(rows.get("html_document")!);
  const root = artifactSqliteInteger(document,3);
  const nodes = new Map<bigint,SqliteRow>(), components = new Map<bigint,SqliteRow>();
  let checked = 0, completed = 1;
  const prepare = async (): Promise<void> => { if (++checked % 256 === 0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,total); };
  const tick = async (): Promise<void> => { if (++completed % 256 === 0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total); };
  for (const row of rows.get("html_node")!) {
    if (!COMPONENTS.some(([,tag]) => artifactSqliteText(row,1) === tag)) throw new Error("HTML node kind is invalid");
    nodes.set(row.rowid,row); await prepare();
  }
  if (!nodes.has(root)) throw new Error("HTML document root is dangling");
  for (const [table,tag] of COMPONENTS) for (const row of rows.get(table)!) {
    if (!nodes.has(row.rowid) || artifactSqliteText(nodes.get(row.rowid)!,1) !== tag || components.has(row.rowid)) throw new Error("HTML node requires exactly its typed component");
    artifactSqliteText(row,1);
    if (tag === "raw_text") {
      const parent = artifactSqliteText(row,1);
      if (parent !== "script" && parent !== "style") throw new Error("HTML raw text parent kind is invalid");
      artifactSqliteText(row,2);
    }
    components.set(row.rowid,row); await prepare();
  }
  if (components.size !== nodes.size) throw new Error("HTML node lacks its typed component");
  const owners = new Set([root]);
  const children = new Map<bigint,SqliteRow[]>(), attributes = new Map<bigint,SqliteRow[]>();
  for (const row of rows.get("html_child")!) {
    const parent = artifactSqliteInteger(row,1), child = artifactSqliteInteger(row,3);
    if (!nodes.has(parent) || artifactSqliteText(nodes.get(parent)!,1) !== "element" || !nodes.has(child) || owners.has(child)) throw new Error("HTML child requires an element parent and unique node ownership");
    owners.add(child); group(children,parent,row); await prepare();
  }
  for (const row of rows.get("html_attribute")!) {
    const parent = artifactSqliteInteger(row,1);
    if (!nodes.has(parent) || artifactSqliteText(nodes.get(parent)!,1) !== "element") throw new Error("HTML attribute requires an element parent");
    artifactSqliteText(row,3);
    if (row.values[4] !== null) artifactSqliteText(row,4);
    group(attributes,parent,row); await prepare();
  }
  for (const groups of [children,attributes]) for (const [owner,list] of groups) { groups.set(owner,artifactSqliteOrderedRows(list,2)); await prepare(); }
  if (owners.size !== nodes.size) throw new Error("HTML node lacks a document or element owner");
  const stack = [{ id: root, finish: false }];
  const visited = new Set<bigint>(), built = new Map<bigint,HtmlNode>();
  while (stack.length) {
    const { id,finish } = stack.pop()!;
    if (!finish) {
      if (visited.has(id)) throw new Error("HTML node graph contains a cycle");
      visited.add(id); stack.push({ id,finish: true });
      const list = children.get(id) ?? [];
      for (let index = list.length - 1; index >= 0; index--) stack.push({ id: artifactSqliteInteger(list[index]!,3), finish: false });
      if (visited.size % 256 === 0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total);
      continue;
    }
    const component = components.get(id)!;
    let node: HtmlNode;
    switch (artifactSqliteText(nodes.get(id)!,1)) {
      case "element":
        node = { kind: "element",name: artifactSqliteText(component,1),attributes: [],children: [] };
        for (const row of attributes.get(id) ?? []) { node.attributes.push({ name: artifactSqliteText(row,3), value: row.values[4] === null ? undefined : artifactSqliteText(row,4) }); await tick(); }
        for (const row of children.get(id) ?? []) {
          const child = artifactSqliteInteger(row,3);
          const value = built.get(child);
          if (!value) throw new Error("HTML child was not reconstructed");
          node.children.push(value); built.delete(child); await tick();
        }
        attributes.delete(id); children.delete(id);
        break;
      case "text": node = { kind: "text",text: artifactSqliteText(component,1) }; break;
      case "comment": node = { kind: "comment",text: artifactSqliteText(component,1) }; break;
      case "raw_text": node = { kind: "rawText",parentKind: artifactSqliteText(component,1) as "script" | "style",text: artifactSqliteText(component,2) }; break;
      default: throw new Error("HTML node kind is invalid");
    }
    built.set(id,node); await tick(); await tick();
  }
  if (visited.size !== nodes.size || children.size || attributes.size) throw new Error("HTML node graph is disconnected or cyclic");
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",total,total);
  return { schema: artifactSqliteText(document,1),doctype: document.values[2] === null ? undefined : artifactSqliteText(document,2),root: built.get(root)! };
}

