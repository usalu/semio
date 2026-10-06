/** 🗄️ stdio.xml TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type { RetainedXmlAttribute,RetainedXmlNodeKind,RetainedXmlNode,RetainedXmlExternalId,RetainedXmlDtdDeclaration,RetainedXmlDoctype,RetainedXmlDeclaration,RetainedXmlDocument,XmlAttr,XmlNode,XmlQuote,XmlDeclaration,XmlExternalId,XmlDtdDeclaration,XmlDoctype,XmlDocument,XmlSnapshot } from "./🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export { parseRetainedXmlDocument,retainXmlDocument,materializeRetainedXmlDocument } from "./🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export type { XmlSqliteTables } from "./🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export { XML_SQLITE_SCHEMA,xmlSnapshotToSqliteDatabase,xmlSnapshotFromSqliteDatabase,xmlDocumentsToSqliteDatabase,xmlDocumentsFromSqliteDatabase,projectXmlDocument,projectXmlDocuments,measureXmlDocuments,reconstructXmlDocument,reconstructXmlDocuments } from "./🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
