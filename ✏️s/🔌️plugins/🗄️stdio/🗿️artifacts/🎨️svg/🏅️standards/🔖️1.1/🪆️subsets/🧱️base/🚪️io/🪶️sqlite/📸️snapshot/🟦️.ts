import {bindSvgDocument,nativeSvgDocument}from "../../📝️text/📸️snapshot/🧮️attributes/🟦️.ts";
/** 🎨️ SVG's persisted typed XML fields projected directly into SVG's own tables. */
import type { SvgSnapshot } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { projectXmlDocument, reconstructXmlDocument, type XmlSqliteTables } from "../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import type { ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted independent SVG schema byte-equal to its adjacent SQL asset. */
export const SVG_SQLITE_SCHEMA = "CREATE TABLE svg_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, root_node_id INTEGER REFERENCES svg_node(id));\nCREATE TABLE svg_node (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK(kind IN ('element','text','cdata','comment','processing_instruction')));\nCREATE TABLE svg_element (node_id INTEGER PRIMARY KEY REFERENCES svg_node(id), name TEXT NOT NULL);\nCREATE TABLE svg_text (node_id INTEGER PRIMARY KEY REFERENCES svg_node(id), text TEXT NOT NULL);\nCREATE TABLE svg_cdata (node_id INTEGER PRIMARY KEY REFERENCES svg_node(id), text TEXT NOT NULL);\nCREATE TABLE svg_comment (node_id INTEGER PRIMARY KEY REFERENCES svg_node(id), text TEXT NOT NULL);\nCREATE TABLE svg_processing_instruction (node_id INTEGER PRIMARY KEY REFERENCES svg_node(id), target TEXT NOT NULL, data TEXT NOT NULL);\nCREATE TABLE svg_attribute (id INTEGER PRIMARY KEY, element_node_id INTEGER NOT NULL REFERENCES svg_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL, value TEXT NOT NULL);\nCREATE TABLE svg_child (id INTEGER PRIMARY KEY, parent_element_node_id INTEGER NOT NULL REFERENCES svg_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), child_node_id INTEGER NOT NULL REFERENCES svg_node(id));\nCREATE TABLE svg_document_misc (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES svg_document(id), position TEXT NOT NULL CHECK(position IN ('prolog','epilog')), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), node_id INTEGER NOT NULL REFERENCES svg_node(id));\nCREATE TABLE svg_declaration (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES svg_document(id), version TEXT NOT NULL, encoding TEXT, standalone INTEGER CHECK(standalone IN (0,1)), quote TEXT NOT NULL CHECK(quote IN ('double','single')));\nCREATE TABLE svg_doctype (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES svg_document(id), prolog_position_decimal TEXT NOT NULL, name TEXT NOT NULL, external_kind TEXT CHECK(external_kind IN ('system','public')), public_id TEXT, system_id TEXT, CHECK((external_kind IS NULL AND public_id IS NULL AND system_id IS NULL) OR (external_kind = 'system' AND public_id IS NULL AND system_id IS NOT NULL) OR (external_kind = 'public' AND public_id IS NOT NULL AND system_id IS NOT NULL)));\nCREATE TABLE svg_entity (id INTEGER PRIMARY KEY, doctype_id INTEGER NOT NULL REFERENCES svg_doctype(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), parameter INTEGER NOT NULL CHECK(parameter IN (0,1)), name TEXT NOT NULL, value TEXT NOT NULL);\n";
const TABLES: XmlSqliteTables = { document: "svg_document", node: "svg_node", element: "svg_element", text: "svg_text", cdata: "svg_cdata", comment: "svg_comment", processingInstruction: "svg_processing_instruction", attribute: "svg_attribute", child: "svg_child", documentMisc: "svg_document_misc", declaration: "svg_declaration", doctype: "svg_doctype", entity: "svg_entity" };
function root(snapshot: SvgSnapshot): void {
  const node = snapshot.doc.root;
  if (node?.kind !== "element" || (node.name !== "svg" && !node.name.endsWith(":svg"))) throw new Error("SVG snapshot requires an svg root element");
}

/** 📤️ Project SVG through explicit table vocabulary while retaining all typed XML metadata. */
export async function svgSnapshotToSqliteDatabase(snapshot: SvgSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  root(snapshot);
  return projectXmlDocument(snapshot.schema, nativeSvgDocument(snapshot.doc), SVG_SQLITE_SCHEMA, TABLES, options);
}
/** 📥️ Reconstruct typed SVG metadata and enforce its required SVG root. */
export async function svgSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<SvgSnapshot> {
  const decoded = await reconstructXmlDocument(database, SVG_SQLITE_SCHEMA, TABLES, options);
  const snapshot={schema:decoded.schema,doc:bindSvgDocument(decoded.doc)};
  root(snapshot);
  return snapshot;
}

