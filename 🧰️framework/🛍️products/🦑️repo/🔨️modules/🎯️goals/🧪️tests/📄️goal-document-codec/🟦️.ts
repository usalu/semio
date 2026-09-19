//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
/**
 * 🔮️ TypeScript oracle of the goal document codec case.
 *
 * It is a reference in two independent senses. `ajv` is a real draft 2020-12 validator, so it —
 * not this repository — decides whether a stored document actually satisfies `🧬️schema/🔣️.json`;
 * a schema that has drifted from the two implementations is caught here instead of agreeing with
 * itself. And `JSON.parse`/`JSON.stringify` is a third serialiser, written by neither the Go nor
 * the Rust author, that must reproduce the same bytes the two typed codecs do. The derived parent
 * path is computed here from the identifier alone, so it is an independent statement of the rule
 * rather than a reading of the document's own `parent` member.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "stored-documents-round-trip": {
      oracle: (ctx) => {
        const schema = JSON.parse(readFileSync(ctx.fixture("asset://🧬️schema/🔣️.json"), "utf8")) as { readonly $schema?: string; readonly $defs: Readonly<Record<string, unknown>> };
        const fixture = JSON.parse(readFileSync(ctx.fixture("shared://🎯️goal-documents.json"), "utf8")) as { documents: { id: string; json: string }[] };
        const ajv = new Ajv2020({ strict: false, allErrors: true });
        const validate = ajv.compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/GoalDocument" });
        const documents: string[] = [];
        const members: string[] = [];
        for (const entry of fixture.documents) {
          const value = JSON.parse(entry.json) as Record<string, unknown>;
          if (!validate(value)) throw new Error(`${entry.id} does not satisfy GoalDocument: ${ajv.errorsText(validate.errors)}`);
          documents.push(`${entry.id}=${JSON.stringify(value, null, 2)}\n`);
          const separator = entry.id.lastIndexOf("/");
          const parentPath = separator === -1 ? "" : entry.id.slice(0, separator);
          const dates = (value.dates ?? {}) as { due?: string };
          const management = (value.github ?? {}) as { milestone?: string; issue?: string };
          members.push(
            [
              entry.id,
              value.status ?? "",
              value.llm ?? "",
              value.client ?? "",
              dates.due ?? "",
              parentPath,
              value.parent ?? "",
              management.milestone ?? "",
              management.issue ?? "",
            ].join("|"),
          );
        }
        return { projection: { documents, members } };
      },
    },
  },
});
//#endregion 🔮️Oracle
