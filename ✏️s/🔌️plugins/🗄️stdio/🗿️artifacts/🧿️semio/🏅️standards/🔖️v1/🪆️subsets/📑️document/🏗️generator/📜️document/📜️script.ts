#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 📄️ Document fixture recipes — the `document` family: the two WHOLE-DOCUMENT kinds. `set-snapshot`
// is this subset's one generic production entry point and, applied with an unchanged snapshot, also
// serves as the identity element (`NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every
// variant to wrap a leaf payload, and `no` is not an approved semantic verb), so this family is where
// the cross-carrier agreement invariant is exercised on a deliberately flat document (no lists, no
// quotes) that docx and md both encode the same way.
//
// A recipe DESCRIBES two documents; it computes nothing. `../📜️script.ts` encodes them with the
// third-party writers and `../../🔬️probes/📜️script.ts` reads them back with different libraries.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Doc, type Recipe, baseDoc, heading, para, style } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
const draft: Doc = {
  styles: [style("Body", "Body Text")],
  blocks: [heading(2, "Draft Section"), para("Draft body."), { kind: "code", language: "rust", text: "fn main() {}" }],
};

const final: Doc = {
  styles: [style("Body", "Body Text"), style("Quote", "Quotation", "Body")],
  blocks: [heading(2, "Final Section"), para("Final body."), { kind: "code", language: "rust", text: "fn main() {}" }],
};

/** 📄️ The `document` recipes: the identity element and the generic whole-snapshot replacement. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "no-mutation-leaves-the-document-untouched",
    family: "document",
    mutation: "set-snapshot",
    property: "block-text",
    carriers: ["docx", "md"],
    notes: "The identity element. `NoMutation` was dropped from `SemioDocumentMutation`, so identity is now `SetSnapshot` applied with the base document unchanged — `set-snapshot`'s own diff helper returns `SemioDocumentDiff::default()` (with a `mutation.no-op` warning) whenever `snapshot == base`, so its only reachable outcome is a no-op, and the evidence for it is that two independent readers recover exactly the same block-text sequence from the before and the after carrier.",
    before: baseDoc(),
    after: baseDoc(),
  },
  {
    id: "set-snapshot-replaces-the-whole-document",
    family: "document",
    mutation: "set-snapshot",
    property: "block-text",
    carriers: ["docx", "md"],
    notes: "The generic entry point this subset actually dispatches today: a whole-snapshot replacement that rewrites a heading and a paragraph, adds a named style, and leaves the fenced code block alone — so the reading must show movement in exactly two text entries and none in the third.",
    before: draft,
    after: final,
  },
];
//#endregion 🧪️Recipes
