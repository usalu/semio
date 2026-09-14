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
const moduleRoot = join(dirname(dirname(import.meta.dir)));
const readJson = (relative: string) => JSON.parse(readFileSync(join(moduleRoot, relative), "utf8"));
const eventSchema = readJson("🧬️schema/🔣️.json");
const kindCatalog = readJson("🧬️schema/🔣️event-kinds.json");
const kinds: string[] = kindCatalog.kinds.map((entry: { kind: string }) => entry.kind);
const validateEnvelope = new Ajv2020({ strict: false }).compile(eventSchema);
//#endregion 🧬️Schema

//#region 🧭️Adapter
/** 🟦️ Ajv oracle for the event kind catalog: the schema file is the only source of truth. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "catalog-is-the-schema-file": {
      oracle: () => ({
        projection: {
          kinds,
          count: kinds.length,
          duplicates: kinds.length - new Set(kinds).size,
          undotted: kinds.filter((kind) => !kind.includes(".")).length,
          schemaVersion: kindCatalog.schemaVersion,
          matchesSchemaFile: true,
          scenarioLevel: "fundamental",
        },
      }),
    },
    "envelope-accepts-only-declared-kinds": {
      oracle: () => {
        const declaredKind = kinds[0];
        const undeclaredKind = "ticket.open.not-a-kind";
        const declared = validateEnvelope({ kind: declaredKind, source: "repo-cli", payload: {} }) === true;
        const undeclared = validateEnvelope({ kind: undeclaredKind, source: "repo-cli", payload: {} }) === true;
        if (undeclared) throw new Error("the schema accepted an undeclared kind");
        return { projection: { declaredKind, undeclaredKind, declared } };
      },
    },
  },
});
//#endregion 🧭️Adapter
