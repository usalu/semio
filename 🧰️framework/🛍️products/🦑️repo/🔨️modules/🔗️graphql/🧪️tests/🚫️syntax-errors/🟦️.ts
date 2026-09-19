//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔮️ ORACLE ONLY. A conforming parser decides whether each corpus input really is malformed, so
// "we reject it" is a claim about GraphQL rather than about our own subset.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { parse } from "graphql";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Adapter
type Corpus = { inputs: { id: string; source: string }[] };

/** 🔮️ TypeScript host of the `graphql` reference parser for the malformed-input case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "malformed-inputs-are-rejected": {
      oracle: (ctx) => {
        const corpus = JSON.parse(Buffer.from(ctx.fixtureBytes("local://🔣️malformed.json")).toString("utf8")) as Corpus;
        return {
          projection: {
            inputs: corpus.inputs.map((entry) => {
              let rejected = false;
              let detail = "";
              try {
                parse(entry.source, { noLocation: true });
              } catch (error) {
                rejected = true;
                detail = (error as Error).message;
              }
              return { input: entry.id, rejected, detail };
            }),
          },
        };
      },
    },
  },
});
//#endregion 🧭️Adapter
