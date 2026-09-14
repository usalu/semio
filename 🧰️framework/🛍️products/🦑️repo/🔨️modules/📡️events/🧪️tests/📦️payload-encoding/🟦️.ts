//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Schema
type Vector = { id: string; type: string; input: unknown; encoded: string };

const moduleRoot = dirname(dirname(import.meta.dir));
const eventSchema = JSON.parse(readFileSync(join(moduleRoot, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv2020({ strict: false });
ajv.addSchema(eventSchema, "events");

/** 🔮️ Judges one golden encoding against the payload's own `$defs` subschema. */
const judge = (vector: Vector): boolean => {
  const validate = ajv.getSchema(`events#/$defs/${vector.type}`);
  if (!validate) throw new Error(`🧬️schema/🔣️.json declares no $defs/${vector.type}`);
  const accepted = validate(JSON.parse(vector.encoded)) === true;
  if (!accepted) throw new Error(`${vector.id}: ${ajv.errorsText(validate.errors)}`);
  return accepted;
};
//#endregion 🧬️Schema

//#region 🧭️Adapter
/** 🟦️ Ajv oracle for the payload encodings: every golden must satisfy its own schema. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "golden-encoding-per-payload": {
      oracle: (ctx) => {
        const vectors = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://✉️payload-vectors.json"))).cases as Vector[];
        vectors.forEach(judge);
        return {
          projection: {
            encodings: vectors.map((vector) => ({ id: vector.id, type: vector.type, encoded: vector.encoded })),
            count: vectors.length,
          },
        };
      },
    },
    "omit-empty-and-explicit-null": {
      oracle: (ctx) => {
        const vectors = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://✉️payload-vectors.json"))).cases as Vector[];
        return {
          projection: {
            optional: vectors.map((vector) => ({
              id: vector.id,
              hasEmptyValue: vector.encoded.includes(':""'),
              hasNullValue: vector.encoded.includes(":null"),
            })),
          },
        };
      },
    },
    "envelope-round-trip": {
      oracle: () => {
        const envelope = { kind: "ticket.open.starting", source: "repo-cli", payload: { id: "a" } };
        const validate = ajv.getSchema("events#/$defs/Event");
        if (!validate || validate(envelope) !== true) throw new Error("the schema rejected a well-formed envelope");
        return { projection: { envelope: JSON.stringify(envelope) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
