#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧱 Document fixture recipes — the `blocks` family: the five kinds that add, remove or retype a block
// in the recursive `blocks` tree.
//
// The carrier lists here are the load-bearing part. `set-list-ordered` is md-only because the docx
// serializer FLATTENS a list into bare paragraphs (`DocBlock::List { items, .. } => items.iter()
// .flat_map(..)`), so `ordered` has no WordprocessingML encoding at all and a docx fixture would prove
// nothing about it.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Block, type Doc, type Recipe, baseDoc, heading, para, withBlocks } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
const listBefore: Doc = {
  styles: [],
  blocks: [para("A list follows."), { kind: "list", ordered: false, items: [[para("first item")], [para("second item")]] } satisfies Block],
};

const listAfter: Doc = {
  styles: [],
  blocks: [para("A list follows."), { kind: "list", ordered: true, items: [[para("first item")], [para("second item")]] } satisfies Block],
};

/** 🧱 The `blocks` recipes: insertion, removal, whole-block replacement, heading level and list kind. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "insert-block-appends-a-paragraph",
    family: "blocks",
    mutation: "insert-block",
    property: "block-count",
    carriers: ["docx", "md"],
    notes: "A new paragraph between the body and the closing paragraph. Both carriers count one more block. A PageBreak insert would be invisible in BOTH (each serializer maps it to nothing), which is why this recipe inserts a paragraph and the page-break case is recorded as uncarried rather than faked.",
    before: baseDoc(),
    after: withBlocks(baseDoc(), (blocks) => [...blocks.slice(0, 2), para("The inserted paragraph."), ...blocks.slice(2)]),
  },
  {
    id: "remove-block-drops-a-paragraph",
    family: "blocks",
    mutation: "remove-block",
    property: "block-count",
    carriers: ["docx", "md"],
    notes: "The body paragraph is removed; both carriers count one fewer block and the surviving text entries shift by one.",
    before: baseDoc(),
    after: withBlocks(baseDoc(), (blocks) => [blocks[0]!, blocks[2]!]),
  },
  {
    id: "set-block-content-replaces-a-paragraph",
    family: "blocks",
    mutation: "set-block-content",
    property: "block-text",
    carriers: ["docx", "md"],
    notes: "One block's whole content replaced in place: the block count must not move and exactly one text entry must.",
    before: baseDoc(),
    after: withBlocks(baseDoc(), (blocks) => [blocks[0]!, para("The replaced paragraph."), blocks[2]!]),
  },
  {
    id: "set-heading-level-demotes-the-title",
    family: "blocks",
    mutation: "set-heading-level",
    property: "heading-level",
    carriers: ["docx", "md"],
    notes: "A heading demoted from level 1 to level 3. md carries the level directly; docx carries it only through the HeadingN style-id convention its serializer writes when no explicit style_id was set, so this recipe deliberately leaves the heading unstyled.",
    before: baseDoc(),
    after: withBlocks(baseDoc(), (blocks) => [heading(3, "The Report Title"), blocks[1]!, blocks[2]!]),
  },
  {
    id: "set-list-ordered-numbers-the-list",
    family: "blocks",
    mutation: "set-list-ordered",
    property: "list-ordered",
    carriers: ["md"],
    notes: "A bullet list becomes an ordered list. CommonMark is the ONLY carrier: docx flattens lists into bare paragraphs and drops the flag entirely, so no docx fixture is generated for this kind at all.",
    before: listBefore,
    after: listAfter,
  },
];
//#endregion 🧪️Recipes
