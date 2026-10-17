/** 📱️ Settings chrome panel, Default apps and Conflicts tabs: the default app row codec and tree, and the conflict resolution tree. */
import { Button, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, type TreePanelConfig } from "@semio-tech/ui-react";
import { type AppRef, type AppRole, type ArtifactDialect, dialectCoordinate, type Conflict, type ConflictResolution } from "@semio-tech/framework";
import { ConflictDiffPreview, conflictDiffText } from "../../🔺️DiffViewHost/🟦️.tsx";
import { defaultAppsSettingsTabText, noneOptionText, shellLabel, surfaceRoleChipText } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";

//#region 🔖️os-chrome-panels
//#region SettingsPanel
//#region 🔖️DefaultAppsPanel
/** 👁️✏️ One `AppRef` option in a default-apps `Select` — `"none"` clears the pin. */
export const DEFAULT_APP_NONE_VALUE = "none";

export function encodeDefaultAppValue(app: AppRef): string {
  return `${app.pluginId} ${app.appId}`;
}

function decodeDefaultAppValue(value: string): AppRef | null {
  if (value === DEFAULT_APP_NONE_VALUE) return null;
  const separator = value.indexOf(" ");
  if (separator < 0) return null;
  return { pluginId: value.slice(0, separator), appId: value.slice(separator + 1) };
}

/** 👁️✏️ One dialect's viewer/editor `AppRouter` options for the `SettingsDefaultApps` table — built by
 * `ShellHost` from `AppRouter.entriesFor` + `OpeningPreferences`, one row per role per dialect. */
export type DefaultAppRow = {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly options: readonly { readonly value: string; readonly label: string }[];
  readonly value: string;
};

export type DefaultAppsHostApi = {
  readonly rows: readonly DefaultAppRow[];
  readonly locale: string;
  readonly setDefault: (dialect: ArtifactDialect, role: AppRole, app: AppRef) => void;
  readonly clearDefault: (dialect: ArtifactDialect, role: AppRole) => void;
};

export function buildSettingsDefaultAppsTree(host: DefaultAppsHostApi): TreePanelConfig {
  if (host.rows.length === 0) {
    return { sections: [{ id: "framework.settings.defaultApps.empty", items: [{ id: "framework.settings.defaultApps.empty.row", label: shellLabel("ui.settings.unavailable") }] }] };
  }
  return {
    sections: [
      {
        id: "framework.settings.defaultApps.table",
        label: defaultAppsSettingsTabText(host.locale),
        defaultOpen: true,
        items: host.rows.map((row) => {
          const rowId = `framework.settings.defaultApps.${dialectCoordinate(row.dialect)}.${row.role}`;
          return {
            id: rowId,
            label: `${dialectCoordinate(row.dialect)} — ${surfaceRoleChipText(row.role, host.locale)}`,
            control: (
              <Select id={rowId}
                value={row.value}
                onValueChange={(value) => {
                  if (value === DEFAULT_APP_NONE_VALUE) {
                    host.clearDefault(row.dialect, row.role);
                    return;
                  }
                  const app = decodeDefaultAppValue(value);
                  if (app) host.setDefault(row.dialect, row.role, app);
                }}
              >
                <SelectTrigger id={rowId} className="h-small w-48" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value={DEFAULT_APP_NONE_VALUE}>{noneOptionText(host.locale)}</SelectItem>
                  {row.options.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          };
        }),
      },
    ],
  };
}

//#endregion 🔖️DefaultAppsPanel
//#region 🔖️ConflictsPanel
/** ⚖️ `📌️ChromePanels`' read/write surface for the Conflicts settings tab (contract freeze §C5/§C9) —
 * `ShellHost` builds `conflicts` from `🐚️Shell`'s `selectOpenConflicts`, `messageText`/`kindLabel`
 * localize a `Conflict`'s own `ConflictKind`/`MutationMessage`s (never parsed from `message` prose).
 * `onSelect` drives the `🔺️DiffViewHost`-backed preview for the currently expanded conflict. */
export type ConflictsHostApi = {
  readonly conflicts: readonly Conflict[];
  readonly locale: string;
  readonly selectedConflictId: string | null;
  readonly kindLabel: (conflict: Conflict) => string;
  readonly messageText: (conflict: Conflict) => string;
  readonly onSelect: (conflictId: string) => void;
  readonly onResolve: (conflictId: string, resolution: ConflictResolution) => void;
  /** ⚖️ Current-document text for the `🔺️DiffViewHost`-backed "incoming vs current" preview (see
   * `conflictDiffText`) — `""` when no synchronous document snapshot is in hand. */
  readonly currentDocumentText: string;
};

export function buildConflictsTree(host: ConflictsHostApi): TreePanelConfig {
  if (host.conflicts.length === 0) {
    return { sections: [{ id: "framework.settings.conflicts.empty", items: [{ id: "framework.settings.conflicts.empty.row", label: shellLabel("ui.settings.unavailable") }] }] };
  }
  return {
    sections: [
      {
        id: "framework.settings.conflicts.table",
        label: shellLabel("ui.conflict.panel"),
        defaultOpen: true,
        items: host.conflicts.map((conflict) => {
          const rowId = `framework.settings.conflicts.${conflict.id}`;
          const selected = conflict.id === host.selectedConflictId;
          const diff = selected ? conflictDiffText(conflict, host.currentDocumentText) : null;
          return {
            id: rowId,
            label: `${host.kindLabel(conflict)} — ${host.messageText(conflict)}`,
            onClick: () => host.onSelect(conflict.id),
            control: (
              <div className="flex flex-col gap-single">
                <div className="flex items-center gap-single">
                  <Button icon="check" id={`${rowId}.accept`} text={shellLabel("ui.conflict.accept")} onClick={() => host.onResolve(conflict.id, "accept")} />
                  <Button icon="x" id={`${rowId}.discard`} text={shellLabel("ui.conflict.discard")} onClick={() => host.onResolve(conflict.id, "discard")} />
                </div>
                {diff ? (
                  <div className="max-h-40 max-w-96 overflow-auto rounded-sm border border-border">
                    <ConflictDiffPreview before={diff.before} after={diff.after} />
                  </div>
                ) : null}
              </div>
            ),
          };
        }),
      },
    ],
  };
}
//#endregion 🔖️ConflictsPanel
//#endregion SettingsPanel
//#endregion 🔖️os-chrome-panels
