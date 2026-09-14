// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { loadVizCatalog } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { measurePrintGalleryVariant, printGalleryMatrix, type PrintGalleryFixture } from "../../🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Evidence
/** 🧫️ The committed evidence, read through the plan so an unreferenced fixture can never be used. */
function evidence(ctx: AdapterContext): PrintGalleryFixture {
  return JSON.parse(readFileSync(ctx.fixture("local://🖼️gallery-render.json"), "utf8")) as PrintGalleryFixture;
}

/** 🔀️ Every family whose kinds collide on their option string — the projection the property asserts is empty. */
function optionCollisions(): string[] {
  const seenPerFamily = new Map<string, Map<string, string>>();
  const collisions: string[] = [];
  for (const kind of loadVizCatalog().kinds) {
    const options = JSON.stringify(kind.options);
    const seen = seenPerFamily.get(kind.family) ?? new Map<string, string>();
    const previous = seen.get(options);
    if (previous !== undefined) collisions.push(`${kind.family}: ${previous} and ${kind.slug} both carry the options ${options}`);
    seen.set(options, kind.slug);
    seenPerFamily.set(kind.family, seen);
  }
  return collisions.sort();
}
// #endregion 🧫️Evidence

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "theme-and-language-matrix": {
      /** 🔮️ The specification half: the committed evidence produced by the documented regeneration command. */
      oracle: (ctx: AdapterContext) => ({ projection: evidence(ctx).variants }),
      /** 🎯️ The library half: the matrix rebuilt now, measured the same way. */
      subject: async (ctx: AdapterContext) => {
        const measured: Record<string, unknown> = {};
        for (const variant of printGalleryMatrix()) measured[variant.id] = await measurePrintGalleryVariant(variant);
        return { projection: measured };
      },
    },
    "family-option-distinctness": {
      /** 🔮️ The property: no family may register two kinds with one option string. */
      oracle: () => ({ projection: { collisions: [] as string[] } }),
      /** 🎯️ What the catalogue actually declares. */
      subject: () => ({ projection: { collisions: optionCollisions() } }),
    },
  },
});
// #endregion 🧭️Adapter
