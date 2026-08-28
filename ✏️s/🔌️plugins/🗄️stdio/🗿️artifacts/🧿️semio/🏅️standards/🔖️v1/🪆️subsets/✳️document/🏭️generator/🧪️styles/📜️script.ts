#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🎨️ Document fixture recipes — the `styles` family: the five kinds that touch a named style, either
// on a paragraph or in the style table itself.
//
// Every recipe here is docx-ONLY, and that is a finding rather than an omission. The md serializer's own
// header states it: "`styles`/`DocStyle` are dropped entirely — CommonMark has no named-style concept,
// so `style_id` on `Paragraph`/`Heading` is ignored on export". A CommonMark fixture for these kinds
// would carry no trace of the mutation at all.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Recipe, baseDoc, style, withBlocks, withStyles } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🎨️ The `styles` recipes: a paragraph's style reference, and the four style-table kinds. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "set-paragraph-style-names-the-body-style",
    family: "styles",
    mutation: "set-paragraph-style",
    property: "paragraph-style",
    carriers: ["docx"],
    notes: "A named paragraph style set on the body paragraph, read back out of w:pStyle. CommonMark drops style_id on export, so WordprocessingML is the only witness.",
    before: baseDoc(),
    after: withBlocks(baseDoc(), (blocks) => [blocks[0]!, { kind: "paragraph", style: "Body", runs: [{ text: "The body paragraph." }] }, blocks[2]!]),
  },
  {
    id: "insert-style-adds-a-named-style",
    family: "styles",
    mutation: "insert-style",
    property: "style-table",
    carriers: ["docx"],
    notes: "A new row in the named-style table, carrying an id, a display name and a basedOn parent. Its diff arm is unconditional, so `applied` is its only reachable outcome.",
    before: baseDoc(),
    after: withStyles(baseDoc(), (styles) => [...styles, style("Quote", "Quotation", "Body")]),
  },
  {
    id: "remove-style-drops-a-named-style",
    family: "styles",
    mutation: "remove-style",
    property: "style-table",
    carriers: ["docx"],
    notes: "A named style removed from the table. Also unconditional in its diff arm, hence `applied` only.",
    before: baseDoc(),
    after: withStyles(baseDoc(), (styles) => [styles[0]!]),
  },
  {
    id: "set-style-name-renames-a-style",
    family: "styles",
    mutation: "set-style-name",
    property: "style-table",
    carriers: ["docx"],
    notes: "A style's display name changed while its id stays put — visible in w:name and nowhere else in this subset's carriers. The id column must not move, which is what separates a rename from a replace.",
    before: baseDoc(),
    after: withStyles(baseDoc(), (styles) => [style("Body", "Running Text"), styles[1]!]),
  },
  {
    id: "set-style-based-on-reparents-a-style",
    family: "styles",
    mutation: "set-style-based-on",
    property: "style-table",
    carriers: ["docx"],
    notes: "A style's w:basedOn parent set. Only WordprocessingML carries style inheritance at all.",
    before: baseDoc(),
    after: withStyles(baseDoc(), (styles) => [styles[0]!, style("Heading1", "Heading 1", "Body")]),
  },
];
//#endregion 🧪️Recipes
