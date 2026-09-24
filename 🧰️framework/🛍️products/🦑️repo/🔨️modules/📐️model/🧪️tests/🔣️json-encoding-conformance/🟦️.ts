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
 * 🔮️ TypeScript oracle of the JSON encoding conformance case.
 *
 * It is a reference in two independent senses. `ajv` is a real draft 2020-12 validator, so it —
 * not this repository — decides whether a golden actually satisfies `🧬️schema/🔣️.json`; a schema
 * that has drifted from the two implementations is caught here instead of agreeing with itself.
 * And `JSON.parse`/`JSON.stringify` is a third serialiser, written by neither the Go nor the Rust
 * author, that must reproduce the same bytes the two typed encoders do.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "golden-documents-round-trip": {
      oracle: (ctx) => {
        const schema = JSON.parse(readFileSync(ctx.fixture("schema://repo.model/Repo"), "utf8")) as { readonly $schema?: string; readonly $defs: Readonly<Record<string, unknown>> };
        const goldens = JSON.parse(readFileSync(ctx.fixture("shared://🔣️json-encoding-conformance/🔣️goldens.json"), "utf8")) as { goldens: { type: string; json: string }[] };
        const ajv = new Ajv2020({ strict: false, allErrors: true });
        const encoded = goldens.goldens.map((golden) => {
          const definition = (schema.$defs as Record<string, unknown>)[golden.type];
          if (definition === undefined) throw new Error(`🧬️schema/🔣️.json declares no $def for ${golden.type}`);
          const validate = ajv.compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: `#/$defs/${golden.type}` });
          const value: unknown = JSON.parse(golden.json);
          if (!validate(value)) throw new Error(`${golden.type} does not satisfy its schema: ${ajv.errorsText(validate.errors)}`);
          return { type: golden.type, encoded: JSON.stringify(value) };
        });
        return { projection: { encoded } };
      },
    },
  },
});
//#endregion 🔮️Oracle
