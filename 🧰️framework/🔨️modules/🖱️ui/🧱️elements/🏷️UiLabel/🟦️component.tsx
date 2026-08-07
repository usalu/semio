// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🏷️UiLabel/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

//#region 🎗️UiLabel
/**
 * 🆔️ `UiLabel` + `uiDataLabel`, split out of the ui-react barrel into their own `🧱️elements/` file
 * (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE) — NOT deferred to a later "core extraction" pass
 * like the rest of `🔑️Schema & Keys`, because `VirtualFileSystem` calls `uiDataLabel(...)` at MODULE TOP
 * LEVEL (inside a top-level demo-fixture object literal), not inside a component body. A module-top-level
 * read of a barrel-defined `const` re-exported by a barrel that in turn imports these same elements is a
 * genuine ES-module circular-import initialization-order bug (see
 * `🧱️elements/🔌️Ports/🟦️component.tsx`'s header comment for the sibling `reactHostPort` case).
 * Elements that only call `uiDataLabel(...)` inside function bodies are unaffected — evaluation happens at
 * render time, long after both modules have finished loading — so only this symbol needed to move early.
 */
declare const uiLabelBrand: unique symbol;
/** @emoji 🎗️ A display-ready, locale-resolved string. Component text props (`label`, `title`,
 * `placeholder`, …) should require this instead of `string`, so a hardcoded literal
 * (`label="Close"`) fails to typecheck — only `useLabel`/`useIdLabel` (chrome/product
 * translation lookups) or {@link uiDataLabel} (explicit runtime data) can produce one. */
export type UiLabel = string & { readonly [uiLabelBrand]: true };

/** @emoji 📊️ Genuine runtime data (file names, counts, user content) rendered as a label. Passing a
 * string literal here is a gate violation — see the chrome-i18n lint's `uiDataLabel` literal check. */
export const uiDataLabel = (value: string): UiLabel => value as UiLabel;
//#endregion 🎗️UiLabel
