#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ✍️ Document fixture recipes — the `runs` family: the three kinds that rewrite inline content.
//
// `set-run-text-rewrites-the-body-copy` is the GATE recipe. It ships a deliberately wrong after
// alongside the right one, so the same reading comparison can be proved to accept the good pair and
// reject the bad one. A gate only ever tested on good input is not a gate.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Doc, type Recipe, baseDoc, para, withBlocks } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
const emphasisBefore: Doc = { styles: [], blocks: [para("An emphasised word follows."), { kind: "paragraph", runs: [{ text: "emphasis" }] }] };
const emphasisAfter: Doc = { styles: [], blocks: [para("An emphasised word follows."), { kind: "paragraph", runs: [{ text: "emphasis", bold: true, italic: true }] }] };

const figureBefore: Doc = { styles: [], blocks: [para("A figure follows."), { kind: "image", imageId: "figure-one", alt: "the original caption" }] };
const figureAfter: Doc = { styles: [], blocks: [para("A figure follows."), { kind: "image", imageId: "figure-one", alt: "the corrected caption" }] };

/** ✍️ The `runs` recipes: literal text, character formatting, and an image block's alt text. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "set-run-text-rewrites-the-body-copy",
    family: "runs",
    mutation: "set-run-text",
    property: "block-text",
    carriers: ["docx", "md"],
    notes: "One run's literal text. THE GATE RECIPE: the committed counterexample is a single-character corruption of the correct after (`revised` → `rev1sed`), close enough that only a reader that genuinely decoded the text can tell them apart, so accepting the good pair and rejecting the bad one is a real measurement rather than a formality.",
    before: baseDoc(),
    after: withBlocks(baseDoc(), (blocks) => [blocks[0]!, para("The revised body paragraph."), blocks[2]!]),
    counterexample: withBlocks(baseDoc(), (blocks) => [blocks[0]!, para("The rev1sed body paragraph."), blocks[2]!]),
  },
  {
    id: "set-run-style-emphasises-a-run",
    family: "runs",
    mutation: "set-run-style",
    property: "run-emphasis",
    carriers: ["docx", "md"],
    notes: "Bold and italic on one run. docx witnesses w:b/w:i/w:u and md witnesses Strong/Emphasis/Link, so the two carriers cover different halves of RunStyle; neither witnesses size, font or colour, and both serializers say so in their own headers.",
    before: emphasisBefore,
    after: emphasisAfter,
  },
  {
    id: "set-image-block-corrects-the-caption",
    family: "runs",
    mutation: "set-image-block",
    property: "image-alt",
    carriers: ["docx", "md"],
    notes: "An image block's alt text. md keeps a real image node carrying both the alt and the image id as its URL; docx keeps the alt as ordinary paragraph text, which still witnesses the change even though the carrier cannot say that it IS an alt. Neither carries width or height.",
    before: figureBefore,
    after: figureAfter,
  },
];
//#endregion 🧪️Recipes
