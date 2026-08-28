//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync } from "node:fs";
import { join, sep } from "node:path";
import { defineTestAdapter, digest } from "../../📦️packages/🟦️typescript/📦️index.ts";
//#endregion 🔌️Adapters

//#region 🧭️Adapter
/** 🟦️ TypeScript side of the host protocol conformance case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "digest-and-fixture-resolution": {
      subject: (ctx) => ({
        projection: {
          vectorDigest: digest(ctx.fixtureBytes("shared://📄️protocol-vector.txt")),
          literalDigest: digest("semio"),
          fixtureName: "📄️protocol-vector.txt",
          seed: Number(ctx.seed),
          level: ctx.scenario.level,
          steps: ctx.scenario.steps.length,
        },
      }),
    },
    "fixture-not-in-plan-is-an-error": {
      subject: (ctx) => {
        let reported = false;
        try {
          ctx.fixture("shared://this-fixture-is-not-declared");
        } catch {
          reported = true;
        }
        return { projection: { resolverReportedFailure: reported } };
      },
    },
    "work-directory-is-cache-local": {
      subject: (ctx) => ({
        projection: {
          insideTestCache: ctx.workDir.split(sep).join("/").includes("/.🧬semio/🦑️repo/⚡️cache/tests/"),
          hasOwnershipMarker: existsSync(join(ctx.workDir, "🧾️marker.json")),
        },
      }),
    },
  },
});
//#endregion 🧭️Adapter
