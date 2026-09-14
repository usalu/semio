//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
type Entity = { kind: string; id: string; value: Record<string, string> };

/** 🔤️ The canonical encoding of one entity value: object keys sorted, exactly as Go and serde emit. */
const canonical = (value: Record<string, string>): string =>
  JSON.stringify(Object.fromEntries(Object.keys(value).sort().map((key) => [key, value[key]])));

/** 🧮️ OpenSSL's SHA-256 over the id-sorted `id NUL kind NUL json(data)` preimage. */
const snapshotOf = (entities: Entity[]) => {
  const inputs = entities
    .map((entity) => ({
      id: `${entity.kind}:${entity.id}`,
      kind: `${entity.kind}.recorded`,
      data: canonical(entity.value),
    }))
    .sort((left, right) => (left.id < right.id ? -1 : left.id > right.id ? 1 : 0));
  const hash = createHash("sha256");
  for (const input of inputs) hash.update(`${input.id}\u0000${input.kind}\u0000${input.data}`, "utf8");
  const snapshot = hash.digest("hex");
  return { snapshot, inputIds: inputs.map((input) => `snapshot:${snapshot}:${input.id}`), inputs };
};

const readEntities = (ctx: { fixtureBytes(uri: string): Uint8Array }): Entity[] =>
  JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📤️export-vectors.json"))).entities;

const countsOf = (entities: Entity[]): Record<string, number> => {
  const counts: Record<string, number> = {};
  for (const entity of entities) counts[entity.kind] = (counts[entity.kind] ?? 0) + 1;
  return counts;
};
//#endregion 🔮️Oracle

//#region 🧭️Adapter
/** 🟦️ node:crypto oracle for the export snapshot identity. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "snapshot-identity": {
      oracle: (ctx) => {
        const entities = readEntities(ctx);
        return { projection: { snapshot: snapshotOf(entities).snapshot, counts: countsOf(entities) } };
      },
    },
    "input-ids-are-namespaced": {
      oracle: (ctx) => ({ projection: { inputIds: snapshotOf(readEntities(ctx)).inputIds } }),
    },
    "unchanged-export-is-refused": {
      oracle: (ctx) => ({
        projection: {
          duplicateRefused: true,
          logUnchanged: true,
          eventCount: readEntities(ctx).length,
        },
      }),
    },
  },
});
//#endregion 🧭️Adapter
