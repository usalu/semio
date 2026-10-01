/** 📰️ Handcrafted XML document/node/component/attribute/declaration/entity relations. */
import type { XmlSnapshot, XmlNode, XmlDocument, XmlExternalId } from "../🟦️.ts";
import { parseXmlDeclaration } from "../🟦️.ts";
import { artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow, SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const XML_SQLITE_SCHEMA = "CREATE TABLE xml_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, root_node_id INTEGER REFERENCES xml_node(id));\nCREATE TABLE xml_node (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK(kind IN ('element','text','cdata','comment','processing_instruction')));\nCREATE TABLE xml_element (node_id INTEGER PRIMARY KEY REFERENCES xml_node(id), name TEXT NOT NULL);\nCREATE TABLE xml_text (node_id INTEGER PRIMARY KEY REFERENCES xml_node(id), text TEXT NOT NULL);\nCREATE TABLE xml_cdata (node_id INTEGER PRIMARY KEY REFERENCES xml_node(id), text TEXT NOT NULL);\nCREATE TABLE xml_comment (node_id INTEGER PRIMARY KEY REFERENCES xml_node(id), text TEXT NOT NULL);\nCREATE TABLE xml_processing_instruction (node_id INTEGER PRIMARY KEY REFERENCES xml_node(id), target TEXT NOT NULL, data TEXT NOT NULL);\nCREATE TABLE xml_attribute (id INTEGER PRIMARY KEY, element_node_id INTEGER NOT NULL REFERENCES xml_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL, value TEXT NOT NULL);\nCREATE TABLE xml_child (id INTEGER PRIMARY KEY, parent_element_node_id INTEGER NOT NULL REFERENCES xml_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), child_node_id INTEGER NOT NULL REFERENCES xml_node(id));\nCREATE TABLE xml_document_misc (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES xml_document(id), position TEXT NOT NULL CHECK(position IN ('prolog','epilog')), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), node_id INTEGER NOT NULL REFERENCES xml_node(id));\nCREATE TABLE xml_declaration (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES xml_document(id), version TEXT NOT NULL, encoding TEXT, standalone INTEGER CHECK(standalone IN (0,1)), quote TEXT NOT NULL CHECK(quote IN ('double','single')));\nCREATE TABLE xml_doctype (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES xml_document(id), prolog_position INTEGER NOT NULL CHECK(prolog_position >= 0), name TEXT NOT NULL, external_kind TEXT CHECK(external_kind IN ('system','public')), public_id TEXT, system_id TEXT, CHECK((external_kind IS NULL AND public_id IS NULL AND system_id IS NULL) OR (external_kind = 'system' AND public_id IS NULL AND system_id IS NOT NULL) OR (external_kind = 'public' AND public_id IS NOT NULL AND system_id IS NOT NULL)));\nCREATE TABLE xml_entity (id INTEGER PRIMARY KEY, doctype_id INTEGER NOT NULL REFERENCES xml_doctype(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), parameter INTEGER NOT NULL CHECK(parameter IN (0,1)), name TEXT NOT NULL, value TEXT NOT NULL);\n";

/** 🗂️ Explicit table vocabulary for persisted typed XML documents. */
export interface XmlSqliteTables {
  readonly document: string;
  readonly node: string;
  readonly element: string;
  readonly text: string;
  readonly cdata: string;
  readonly comment: string;
  readonly processingInstruction: string;
  readonly attribute: string;
  readonly child: string;
  readonly documentMisc: string;
  readonly declaration: string;
  readonly doctype: string;
  readonly entity: string;
}
const XML_TABLES: XmlSqliteTables = { document: "xml_document", node: "xml_node", element: "xml_element", text: "xml_text", cdata: "xml_cdata", comment: "xml_comment", processingInstruction: "xml_processing_instruction", attribute: "xml_attribute", child: "xml_child", documentMisc: "xml_document_misc", declaration: "xml_declaration", doctype: "xml_doctype", entity: "xml_entity" };
function tableNames(tables: XmlSqliteTables): string[] { return [tables.document, tables.node, tables.element, tables.text, tables.cdata, tables.comment, tables.processingInstruction, tables.attribute, tables.child, tables.documentMisc, tables.declaration, tables.doctype, tables.entity]; }

function kind(node: XmlNode): string { return node.kind === "cData" ? "cdata" : node.kind === "processingInstruction" ? "processing_instruction" : node.kind; }
function boundaries(doc: XmlDocument): void {
  if (doc.declaration) parseXmlDeclaration(doc.declaration);
  for (const nodes of [doc.prolog, doc.epilog]) if (nodes.some(node => node.kind !== "comment" && node.kind !== "processingInstruction")) throw new Error("XML document boundaries require miscellaneous nodes");
  const position = doc.doctype?.prologPosition ?? 0;
  if (!Number.isSafeInteger(position) || position < 0 || position > doc.prolog.length) throw new Error("XML doctype position exceeds prolog");
}
function rowLimit(rows: number, options: ArtifactSqliteOptions): void {
  if (!Number.isSafeInteger(rows) || rows > (options.maxRows ?? 1_000_000)) throw new Error("XML SQLite row limit");
}
async function measure(snapshot: XmlSnapshot, options: ArtifactSqliteOptions): Promise<number> {
  const doc = snapshot.doc;
  let rows = 1;
  let bytes = artifactSqliteTextBytes(snapshot.schema) + (doc.root ? 16 : 8);
  let visited = 0;
  const check = (): void => { rowLimit(rows, options); artifactSqliteValueBudget(bytes, options); };
  check();
  if (doc.declaration) {
    const declaration = doc.declaration;
    rows++;
    bytes += 16 + artifactSqliteTextBytes(declaration.version) + (declaration.encoding === undefined ? 0 : artifactSqliteTextBytes(declaration.encoding)) + (declaration.standalone === undefined ? 0 : 8) + 6;
    check();
  }
  if (doc.doctype) {
    const doctype = doc.doctype;
    rows++;
    bytes += 24 + artifactSqliteTextBytes(doctype.name);
    if (doctype.externalId) {
      bytes += 6 + artifactSqliteTextBytes(doctype.externalId.systemId);
      if (doctype.externalId.kind === "public") bytes += artifactSqliteTextBytes(doctype.externalId.publicId);
      else if (doctype.externalId.kind !== "system") throw new Error("XML external identifier kind is invalid");
    }
    check();
    for (const entity of doctype.declarations) {
      if (entity.kind !== "entity" || typeof entity.parameter !== "boolean") throw new Error("XML entity fields are invalid");
      rows++;
      bytes += 32 + artifactSqliteTextBytes(entity.name) + artifactSqliteTextBytes(entity.value);
      check();
      if (++visited % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
    }
  }
  const boundaryCount = doc.prolog.length + doc.epilog.length;
  rows += boundaryCount;
  bytes += 38 * boundaryCount;
  rowLimit(rows + 2 * boundaryCount, options);
  check();
  const stack: { node: XmlNode; exit: boolean }[] = [...doc.prolog, ...doc.epilog, ...(doc.root ? [doc.root] : [])].map(node => ({ node, exit: false }));
  const active = new Set<XmlNode>();
  while (stack.length) {
    const { node, exit } = stack.pop()!;
    if (exit) { active.delete(node); continue; }
    if (active.has(node)) throw new Error("XML snapshot contains a cycle");
    active.add(node);
    stack.push({ node, exit: true });
    rows += 2;
    bytes += 16 + artifactSqliteTextBytes(kind(node));
    switch (node.kind) {
      case "element":
        bytes += artifactSqliteTextBytes(node.name);
        rows += node.attrs.length + node.children.length;
        rowLimit(rows + 2 * node.children.length, options);
        bytes += 32 * node.children.length;
        check();
        for (const attr of node.attrs) {
          bytes += 24 + artifactSqliteTextBytes(attr.name) + artifactSqliteTextBytes(attr.value);
          check();
          if (++visited % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
        }
        for (let index = node.children.length - 1; index >= 0; index--) stack.push({ node: node.children[index]!, exit: false });
        break;
      case "text": case "cData": case "comment": bytes += artifactSqliteTextBytes(node.text); break;
      case "processingInstruction": bytes += artifactSqliteTextBytes(node.target) + artifactSqliteTextBytes(node.data); break;
      default: throw new Error("XML node kind is invalid");
    }
    check();
    if (++visited % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
  }
  return rows;
}
interface Parent { node: XmlNode; parent?: bigint; position?: "prolog" | "epilog"; ordinal?: number }

/** 📤️ Project typed XML components and explicit ownership without encoding a native XML payload. */
export async function projectXmlDocument(schema: string, doc: XmlDocument, sql: string, tables: XmlSqliteTables, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  const snapshot = { schema, doc };
  const TABLES = tableNames(tables);
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  boundaries(snapshot.doc);
  const total = await measure(snapshot, options);
  const rows = new Map<string, SqliteRow[]>(TABLES.map(name => [name, []]));
  let completed = 0;
  const insert = (name: string, cells: SqliteValue[], identity?: bigint): bigint => {
    const table = rows.get(name)!;
    const id = identity ?? BigInt(table.length + 1);
    table.push({ rowid: id, values: [id, ...cells] });
    completed++;
    return id;
  };
  const tick = async (): Promise<void> => { if (completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total); };
  insert(tables.document, [snapshot.schema, doc.root ? 1n : null]);
  if (doc.declaration) {
    const declaration = doc.declaration;
    insert(tables.declaration, [1n, declaration.version, declaration.encoding ?? null, declaration.standalone === undefined ? null : BigInt(Number(declaration.standalone)), declaration.quote ?? "double"]);
  }
  if (doc.doctype) {
    const doctype = doc.doctype;
    const external = doctype.externalId;
    insert(tables.doctype, [1n, BigInt(doctype.prologPosition ?? 0), doctype.name, external?.kind ?? null, external?.kind === "public" ? external.publicId : null, external?.systemId ?? null]);
    for (let ordinal = 0; ordinal < doctype.declarations.length; ordinal++) {
      const entity = doctype.declarations[ordinal]!;
      insert(tables.entity, [1n, BigInt(ordinal), BigInt(Number(entity.parameter)), entity.name, entity.value]);
      await tick();
    }
  }
  const stack: Parent[] = [];
  for (const position of ["epilog", "prolog"] as const) for (let ordinal = doc[position].length - 1; ordinal >= 0; ordinal--) stack.push({ node: doc[position][ordinal]!, position, ordinal });
  if (doc.root) stack.push({ node: doc.root });
  while (stack.length) {
    const { node, parent, position, ordinal } = stack.pop()!;
    const id = insert(tables.node, [kind(node)]);
    switch (node.kind) {
      case "element":
        insert(tables.element, [node.name], id);
        for (let ordinal = 0; ordinal < node.attrs.length; ordinal++) {
          const attr = node.attrs[ordinal]!;
          insert(tables.attribute, [id, BigInt(ordinal), attr.name, attr.value]);
          await tick();
        }
        for (let ordinal = node.children.length - 1; ordinal >= 0; ordinal--) stack.push({ node: node.children[ordinal]!, parent: id, ordinal });
        break;
      case "text": insert(tables.text, [node.text], id); break;
      case "cData": insert(tables.cdata, [node.text], id); break;
      case "comment": insert(tables.comment, [node.text], id); break;
      case "processingInstruction": insert(tables.processingInstruction, [node.target, node.data], id); break;
    }
    if (position) insert(tables.documentMisc, [1n, position, BigInt(ordinal!), id]);
    else if (parent !== undefined) insert(tables.child, [parent, BigInt(ordinal!), id]);
    await tick();
  }
  const database = artifactSqliteDatabase(sql, TABLES.map(name => rows.get(name)!), options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

function optionalText(row: SqliteRow, column: number): string | undefined { return row.values[column] === null ? undefined : artifactSqliteText(row, column); }
function group(map: Map<bigint, SqliteRow[]>, id: bigint, row: SqliteRow): void { const rows = map.get(id) ?? []; rows.push(row); map.set(id, rows); }

/** 📥️ Reconstruct XML ownership and typed optional metadata, rejecting dangling components and cycles. */
export async function reconstructXmlDocument(database: SqliteDatabase, sql: string, tables: XmlSqliteTables, options: ArtifactSqliteOptions = {}): Promise<XmlSnapshot> {
  const TABLES = tableNames(tables);
  const COMPONENTS = [[tables.element, "element"], [tables.text, "text"], [tables.cdata, "cdata"], [tables.comment, "comment"], [tables.processingInstruction, "processing_instruction"]] as const;
  const total = database.tables.reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const tableRows = await artifactSqliteTables(database, sql, options);
  const rows = new Map(TABLES.map((name, index) => [name, tableRows[index]!]));
  const document = artifactSqliteDocument(rows.get(tables.document)!);
  const root = document.values[2] === null ? undefined : artifactSqliteInteger(document, 2);
  const nodes = new Map<bigint, SqliteRow>();
  const components = new Map<bigint, SqliteRow>();
  let completed = 0;
  const tick = async (): Promise<void> => { if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", Math.min(completed, total), total); };
  for (const row of rows.get(tables.node)!) {
    if (!COMPONENTS.some(([, tag]) => artifactSqliteText(row, 1) === tag)) throw new Error("XML node kind is invalid");
    nodes.set(row.rowid, row);
    await tick();
  }
  for (const [table, tag] of COMPONENTS) {
    for (const row of rows.get(table)!) {
      if (!nodes.has(row.rowid) || artifactSqliteText(nodes.get(row.rowid)!, 1) !== tag || components.has(row.rowid)) throw new Error("XML typed component is dangling or mismatched");
      artifactSqliteText(row, 1);
      if (tag === "processing_instruction") artifactSqliteText(row, 2);
      components.set(row.rowid, row);
      await tick();
    }
  }
  if (components.size !== nodes.size) throw new Error("XML node lacks its typed component");
  const owners = new Set<bigint>();
  if (root !== undefined) { if (!nodes.has(root)) throw new Error("XML root is dangling"); owners.add(root); }
  const misc = new Map<string, SqliteRow[]>();
  for (const row of rows.get(tables.documentMisc)!) {
    artifactSqliteDocumentReference(row, 1);
    const position = artifactSqliteText(row, 2);
    const id = artifactSqliteInteger(row, 4);
    if (!["prolog", "epilog"].includes(position) || !nodes.has(id) || owners.has(id)) throw new Error("XML boundary ownership is invalid");
    owners.add(id);
    const list = misc.get(position) ?? [];
    list.push(row);
    misc.set(position, list);
    await tick();
  }
  const children = new Map<bigint, SqliteRow[]>();
  for (const row of rows.get(tables.child)!) {
    const parent = artifactSqliteInteger(row, 1);
    const child = artifactSqliteInteger(row, 3);
    if (!nodes.has(parent) || artifactSqliteText(nodes.get(parent)!, 1) !== "element" || !nodes.has(child) || owners.has(child)) throw new Error("XML child ownership is invalid");
    owners.add(child);
    group(children, parent, row);
    await tick();
  }
  const attributes = new Map<bigint, SqliteRow[]>();
  for (const row of rows.get(tables.attribute)!) {
    const parent = artifactSqliteInteger(row, 1);
    if (!nodes.has(parent) || artifactSqliteText(nodes.get(parent)!, 1) !== "element") throw new Error("XML attribute parent is dangling or not an element");
    artifactSqliteText(row, 3);
    artifactSqliteText(row, 4);
    group(attributes, parent, row);
    await tick();
  }
  for (const [position, list] of misc) { misc.set(position, artifactSqliteOrderedRows(list, 3)); await tick(); }
  for (const collection of [children, attributes]) for (const [id, list] of collection) { collection.set(id, artifactSqliteOrderedRows(list, 2)); await tick(); }
  if (owners.size !== nodes.size) throw new Error("XML node has no document or element owner");
  const roots = root === undefined ? [] : [root];
  for (const position of ["prolog", "epilog"]) for (const row of misc.get(position) ?? []) roots.push(artifactSqliteInteger(row, 4));
  const stack = roots.slice().reverse().map(id => ({ id, finish: false }));
  const visited = new Set<bigint>();
  const built = new Map<bigint, XmlNode>();
  while (stack.length) {
    const { id, finish } = stack.pop()!;
    const childRows = children.get(id) ?? [];
    if (!finish) {
      if (visited.has(id)) throw new Error("XML child graph contains a cycle");
      visited.add(id);
      stack.push({ id, finish: true });
      for (let index = childRows.length - 1; index >= 0; index--) stack.push({ id: artifactSqliteInteger(childRows[index]!, 3), finish: false });
      await tick();
      continue;
    }
    const component = components.get(id)!;
    let node: XmlNode;
    switch (artifactSqliteText(nodes.get(id)!, 1)) {
      case "element":
        node = { kind: "element", name: artifactSqliteText(component, 1), attrs: [], children: [] };
        for (const row of attributes.get(id) ?? []) { node.attrs.push({ name: artifactSqliteText(row, 3), value: artifactSqliteText(row, 4) }); await tick(); }
        for (const row of childRows) {
          const child = artifactSqliteInteger(row, 3);
          if (!built.has(child)) throw new Error("XML child was not reconstructed");
          node.children.push(built.get(child)!);
          built.delete(child);
          await tick();
        }
        attributes.delete(id);
        children.delete(id);
        break;
      case "text": node = { kind: "text", text: artifactSqliteText(component, 1) }; break;
      case "cdata": node = { kind: "cData", text: artifactSqliteText(component, 1) }; break;
      case "comment": node = { kind: "comment", text: artifactSqliteText(component, 1) }; break;
      case "processing_instruction": node = { kind: "processingInstruction", target: artifactSqliteText(component, 1), data: artifactSqliteText(component, 2) }; break;
      default: throw new Error("XML node kind is invalid");
    }
    built.set(id, node);
    await tick();
  }
  if (visited.size !== nodes.size || children.size || attributes.size) throw new Error("XML child graph is disconnected or cyclic");
  const doc: XmlDocument = { prolog: [], epilog: [] };
  if (root !== undefined) doc.root = built.get(root)!;
  for (const position of ["prolog", "epilog"] as const) for (const row of misc.get(position) ?? []) doc[position].push(built.get(artifactSqliteInteger(row, 4))!);
  const declarations = rows.get(tables.declaration)!;
  if (declarations.length > 1) throw new Error("XML document has multiple declarations");
  if (declarations.length) {
    const declaration = artifactSqliteDocument(declarations);
    artifactSqliteDocumentReference(declaration, 1);
    const quote = artifactSqliteText(declaration, 5);
    if (quote !== "double" && quote !== "single") throw new Error("XML declaration quote is invalid");
    doc.declaration = { version: artifactSqliteText(declaration, 2), encoding: optionalText(declaration, 3), standalone: declaration.values[4] === null ? undefined : artifactSqliteBoolean(declaration, 4), quote };
  }
  const doctypes = rows.get(tables.doctype)!;
  if (doctypes.length > 1) throw new Error("XML document has multiple doctypes");
  if (doctypes.length) {
    const doctype = artifactSqliteDocument(doctypes);
    artifactSqliteDocumentReference(doctype, 1);
    const position = artifactSqliteInteger(doctype, 2);
    if (position < 0n || position > BigInt(doc.prolog.length)) throw new Error("XML doctype position exceeds prolog");
    const external = optionalText(doctype, 4);
    const publicId = optionalText(doctype, 5);
    const systemId = optionalText(doctype, 6);
    let externalId: XmlExternalId | undefined;
    if (external === undefined && publicId === undefined && systemId === undefined) externalId = undefined;
    else if (external === "system" && publicId === undefined && systemId !== undefined) externalId = { kind: "system", systemId };
    else if (external === "public" && publicId !== undefined && systemId !== undefined) externalId = { kind: "public", publicId, systemId };
    else throw new Error("XML external identifier shape is invalid");
    doc.doctype = { prologPosition: Number(position), name: artifactSqliteText(doctype, 3), externalId, declarations: [] };
    for (const entity of artifactSqliteOrderedRows(rows.get(tables.entity)!, 2)) {
      artifactSqliteDocumentReference(entity, 1);
      doc.doctype.declarations.push({ kind: "entity", parameter: artifactSqliteBoolean(entity, 3), name: artifactSqliteText(entity, 4), value: artifactSqliteText(entity, 5) });
      await tick();
    }
  } else if (rows.get(tables.entity)!.length) throw new Error("XML entities lack their doctype");
  boundaries(doc);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), doc };
}

/** 📰️ Project the XML artifact through its explicit native table vocabulary. */
export function xmlSnapshotToSqliteDatabase(snapshot: XmlSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  return projectXmlDocument(snapshot.schema, snapshot.doc, XML_SQLITE_SCHEMA, XML_TABLES, options);
}
/** 📰️ Reconstruct the XML artifact from its handcrafted relational schema. */
export function xmlSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<XmlSnapshot> {
  return reconstructXmlDocument(database, XML_SQLITE_SCHEMA, XML_TABLES, options);
}
