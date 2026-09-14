//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔮️ ORACLE ONLY. `buildSchema` refuses a document that is not GraphQL, so simply reaching the
// inventory below already decides the first half of this case. The inventory itself is read off the
// schema graphql-js built — never off our own type table, which this adapter cannot see.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { buildSchema, isEnumType, isInputObjectType, isInterfaceType, isObjectType, isScalarType, isUnionType, type GraphQLNamedType } from "graphql";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🖼️Inventory
const BUILT_IN = ["String", "Int", "Boolean", "ID", "Float"];

/** 🖼️ The language-neutral inventory of one named type of a built schema. */
function inventoryOf(declared: GraphQLNamedType): unknown {
  if (isObjectType(declared)) {
    return {
      name: declared.name,
      kind: "object",
      interfaces: declared.getInterfaces().map((entry) => entry.name),
      fields: Object.values(declared.getFields()).map((field) => ({ name: field.name, type: field.type.toString(), args: field.args.map((argument) => ({ name: argument.name, type: argument.type.toString() })) })),
    };
  }
  if (isInterfaceType(declared)) {
    return { name: declared.name, kind: "interface", fields: Object.values(declared.getFields()).map((field) => ({ name: field.name, type: field.type.toString(), args: field.args.map((argument) => ({ name: argument.name, type: argument.type.toString() })) })) };
  }
  if (isUnionType(declared)) return { name: declared.name, kind: "union", possibleTypes: declared.getTypes().map((entry) => entry.name) };
  if (isEnumType(declared)) return { name: declared.name, kind: "enum", values: declared.getValues().map((value) => value.name) };
  if (isInputObjectType(declared)) return { name: declared.name, kind: "input", fields: Object.values(declared.getFields()).map((field) => ({ name: field.name, type: field.type.toString() })) };
  return { name: declared.name, kind: "scalar" };
}
//#endregion 🖼️Inventory

//#region 🧭️Adapter
/** 🔮️ TypeScript host of the `graphql` reference reader for the sdl-dump case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "served-schema-matches-the-committed-sdl": {
      oracle: (ctx) => {
        const schema = buildSchema(ctx.fixtureBytes("asset://🧬️schema/🔣️schema.graphql").toString("utf8"));
        const named = Object.values(schema.getTypeMap())
          .filter((declared) => !declared.name.startsWith("__"))
          .filter((declared) => !(isScalarType(declared) && BUILT_IN.includes(declared.name)))
          .sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0));
        return { projection: { query: schema.getQueryType()?.name ?? null, mutation: schema.getMutationType()?.name ?? null, types: named.map(inventoryOf) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
