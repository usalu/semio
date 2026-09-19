//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔮️ ORACLE ONLY. This repository ships no TypeScript implementation of the grammar; this adapter
// hosts the `graphql` npm package so the corpus is judged by a conforming parser rather than by a
// second reading of our own source.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { Kind, parse, type ASTNode, type OperationDefinitionNode, type SelectionNode, type ValueNode } from "graphql";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🖼️Projection
/**
 * 🖼️ The declared, lossy mapping from a conforming GraphQL AST to `🧬️schema/🔣️.json`.
 *
 * Lossy on purpose and in exactly the places the grammar is: variable definitions and directives are
 * dropped, an alias-free field carries a null alias, and a bare enum name projects as a string
 * literal. Nothing here reimplements a parser — the shape is a rename of what `parse` returned.
 */
function projectValue(node: ValueNode): unknown {
  switch (node.kind) {
    case Kind.VARIABLE:
      return { kind: "variable", name: node.name.value };
    case Kind.LIST:
      return { kind: "list", items: node.values.map(projectValue) };
    case Kind.OBJECT:
      return { kind: "object", fields: [...node.fields].sort((a, b) => a.name.value.localeCompare(b.name.value)).map((field) => ({ name: field.name.value, value: projectValue(field.value) })) };
    case Kind.NULL:
      return { kind: "null" };
    case Kind.INT:
      return { kind: "literal", value: Number.parseInt(node.value, 10) };
    case Kind.FLOAT:
      return { kind: "literal", value: Number.parseFloat(node.value) };
    case Kind.BOOLEAN:
      return { kind: "literal", value: node.value };
    case Kind.STRING:
    case Kind.ENUM:
      return { kind: "literal", value: node.value };
    default:
      throw new Error(`unmapped value node ${(node as ASTNode).kind}`);
  }
}

function projectSelection(node: SelectionNode): unknown {
  if (node.kind !== Kind.FIELD) throw new Error(`the corpus must contain only field selections, found ${node.kind}`);
  return {
    name: node.name.value,
    alias: node.alias?.value ?? null,
    arguments: [...(node.arguments ?? [])].sort((a, b) => a.name.value.localeCompare(b.name.value)).map((argument) => ({ name: argument.name.value, value: projectValue(argument.value) })),
    fields: (node.selectionSet?.selections ?? []).map(projectSelection),
  };
}

function operationOf(source: string): OperationDefinitionNode {
  const document = parse(source, { noLocation: true });
  const [definition, ...rest] = document.definitions;
  if (definition === undefined || definition.kind !== Kind.OPERATION_DEFINITION) throw new Error("the corpus must contain exactly one operation definition");
  if (rest.length > 0) throw new Error("the corpus must contain exactly one operation definition");
  return definition;
}

function projectDocument(source: string): unknown {
  const definition = operationOf(source);
  return { operation: definition.operation, selections: definition.selectionSet.selections.map(projectSelection) };
}
//#endregion 🖼️Projection

//#region 🧭️Adapter
type Corpus = { documents: { id: string; source: string }[] };

/** 🔮️ TypeScript host of the `graphql` reference parser for the document-parsing case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "corpus-projects-identically": {
      oracle: (ctx) => {
        const corpus = JSON.parse(Buffer.from(ctx.fixtureBytes("local://🔣️documents.json")).toString("utf8")) as Corpus;
        return { projection: { documents: corpus.documents.map((entry) => ({ id: entry.id, document: projectDocument(entry.source) })) } };
      },
    },
    "operation-kind-is-recovered": {
      oracle: (ctx) => {
        const corpus = JSON.parse(Buffer.from(ctx.fixtureBytes("local://🔣️documents.json")).toString("utf8")) as Corpus;
        return { projection: { documents: corpus.documents.map((entry) => ({ id: entry.id, operation: operationOf(entry.source).operation })) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
