// #region 🧲️Header
// 💻️ framework/ui/modules/💾️keybinding-persistence/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type StoragePort } from "@semio-tech/framework";
import { parseKeybindingChords } from "../🔤️keybinding-text-interpretation/🟦️.ts";
import { isElementId } from "../../🧱️elements/🆔️ElementId/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 💾️KeybindingPersistence
const UI_KEYBINDING_OVERRIDES_STORAGE_KEY = "ui.keybindings.overrides";

function parseUiKeybindingOverrides(json: unknown): Record<string, string> {
  if (typeof json !== "object" || json === null) return {};
  const out: Record<string, string> = {};
  for (const [controlId, value] of Object.entries(json as Record<string, unknown>)) {
    if (!isElementId(controlId)) continue;
    if (typeof value !== "string" || !value.trim()) continue;
    if (parseKeybindingChords(value).length === 0) continue;
    out[controlId] = value.trim();
  }
  return out;
}

/** @emoji 💾️ Reads user keybinding overrides from storage. */
export function readStoredUiKeybindingOverrides(storage: StoragePort): Record<string, string> {
  const raw = storage.get(UI_KEYBINDING_OVERRIDES_STORAGE_KEY);
  if (!raw) return {};
  try {
    return parseUiKeybindingOverrides(JSON.parse(raw));
  } catch {
    return {};
  }
}

/** @emoji 💾️ Persists user keybinding overrides. */
export function writeStoredUiKeybindingOverrides(storage: StoragePort, overrides: Record<string, string>): void {
  storage.set(UI_KEYBINDING_OVERRIDES_STORAGE_KEY, JSON.stringify(overrides));
}
// #endregion 💾️KeybindingPersistence
