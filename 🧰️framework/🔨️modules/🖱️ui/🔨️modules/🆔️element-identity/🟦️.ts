// #region 🧲️Header
// 💻️ framework/ui/modules/🆔️element-identity/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🆔️ElementIdentity
/** 🆔️ Qualified identity shared by UI elements. */
export interface ElementProps {
  readonly id: string;
}
// #endregion 🆔️ElementIdentity

// #region 🆔️ElementIds
/** 🆔️ Normalizes arbitrary input into a single camelCase element-id segment — byte-for-byte mirror of
 * `element_id_segment` in `framework/core/rs/lib.rs` (core/js stays DOM-free, so the DOM-facing
 * `elementIdSelector`/alias helpers live in `framework/ui/js/react` instead). */
function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) {
      segment += ch.toLowerCase();
    } else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else {
      segment += ch;
    }
  }
  return segment;
}

/** 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`. */
export function windowElementId(kindId: string): string {
  return `framework.window.${elementIdSegment(kindId)}`;
}

/** 🆔️ Element id of a panel tab's uncollapsed panel body; `tabId` is already dotted, appended verbatim. */
export function panelTabElementId(tabId: string): string {
  return `framework.panelTab.${tabId}`;
}

/** 🆔️ Alias id of the first draggable tree row inside a panel tab, stamped via `data-element-alias`. */
export function panelTabFirstDraggableElementId(tabId: string): string {
  return `framework.panelTab.${tabId}.firstDraggable`;
}
// #endregion 🆔️ElementIds
