//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔮️ ORACLE ONLY. The `graphql` package's own `parse` and `valueFromASTUntyped` coerce the corpus;
// only the DEFAULT rule — a default fills an absent argument NAME and nothing else — is restated
// here, because it belongs to this executor and to no library.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { Kind, parse, valueFromASTUntyped, type OperationDefinitionNode } from "graphql";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔢️Coercion
type Case = { id: string; source: string; variables: Record<string, unknown>; defaults: Record<string, unknown> };

/** 🔢️ One root selection's coerced argument map, sorted by argument name. */
function coerceSelection(definition: OperationDefinitionNode, defaults: Record<string, unknown>, variables: Record<string, unknown>): unknown[] {
  return definition.selectionSet.selections.map((selection) => {
    if (selection.kind !== Kind.FIELD) throw new Error(`the corpus must contain only field selections, found ${selection.kind}`);
    const args = new Map<string, unknown>();
    for (const argument of selection.arguments ?? []) {
      const value = valueFromASTUntyped(argument.value, variables);
      args.set(argument.name.value, value === undefined ? null : value);
    }
    for (const [name, fallback] of Object.entries(defaults)) {
      if (!args.has(name) && fallback !== null) args.set(name, fallback);
    }
    return {
      key: selection.alias?.value ?? selection.name.value,
      arguments: [...args.keys()].sort().map((name) => ({ name, value: args.get(name) ?? null })),
    };
  });
}

function operationOf(source: string): OperationDefinitionNode {
  const [definition] = parse(source, { noLocation: true }).definitions;
  if (definition === undefined || definition.kind !== Kind.OPERATION_DEFINITION) throw new Error("the corpus must contain exactly one operation definition");
  return definition;
}

function project(corpus: { cases: Case[] }, onlyWithDefaults: boolean): unknown {
  return {
    cases: corpus.cases
      .filter((entry) => !onlyWithDefaults || Object.keys(entry.defaults).length > 0)
      .map((entry) => ({ id: entry.id, selections: coerceSelection(operationOf(entry.source), entry.defaults, entry.variables) })),
  };
}
//#endregion 🔢️Coercion

//#region 🧭️Adapter
/** 🔮️ TypeScript host of the `graphql` reference coercion for the variable-coercion case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "arguments-resolve-against-variables": {
      oracle: (ctx) => ({ projection: project(JSON.parse(Buffer.from(ctx.fixtureBytes("shared://🔀️variable-coercion/🔣️coercions.json")).toString("utf8")) as { cases: Case[] }, false) }),
    },
    "defaults-fill-only-absent-arguments": {
      oracle: (ctx) => ({ projection: project(JSON.parse(Buffer.from(ctx.fixtureBytes("shared://🔀️variable-coercion/🔣️coercions.json")).toString("utf8")) as { cases: Case[] }, true) }),
    },
  },
});
//#endregion 🧭️Adapter
