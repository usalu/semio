// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/DiffViewHost/component.tsx
/** @emoji 🆚️ `DiffViewHost` — the text-diff scene host: a minimal, dependency-free O(before·after)
 * LCS-based line diff between `before`/`after`, rendered unified (default) or split per `mode`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useMemo, useState, type MouseEvent, type ReactElement } from "react";
import { type ComponentSceneHostProps, type Conflict } from "@semio-tech/framework";
import { cn, ContextMenuController, useLabel, type ContextMenuItem } from "@semio-tech/ui-react";
import { openSurfaceContextMenu, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../Interpreter/🟦️.tsx";
import { WindowInstanceIdContext } from "../World3dHost/🟦️.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️DiffViewHost
//#region DiffViewHost
//#region Types
type DiffLineKind = "equal" | "add" | "remove";
type DiffLine = { readonly kind: DiffLineKind; readonly beforeNo?: number; readonly afterNo?: number; readonly text: string };
type SplitRow = { readonly left?: DiffLine; readonly right?: DiffLine };
//#endregion Types

//#region LineDiff
/** 🔍️ Minimal O(before·after) LCS-based line diff — no external dependency, adequate for the moderate-sized before/after buffers a `DiffViewScene` carries. */
function diffLines(before: readonly string[], after: readonly string[]): DiffLine[] {
  const beforeLen = before.length;
  const afterLen = after.length;
  const lcs: number[][] = Array.from({ length: beforeLen + 1 }, () => new Array<number>(afterLen + 1).fill(0));
  for (let i = beforeLen - 1; i >= 0; i--) {
    for (let j = afterLen - 1; j >= 0; j--) {
      lcs[i][j] = before[i] === after[j] ? lcs[i + 1][j + 1] + 1 : Math.max(lcs[i + 1][j], lcs[i][j + 1]);
    }
  }
  const lines: DiffLine[] = [];
  let i = 0;
  let j = 0;
  while (i < beforeLen && j < afterLen) {
    if (before[i] === after[j]) {
      lines.push({ kind: "equal", beforeNo: i + 1, afterNo: j + 1, text: before[i] });
      i += 1;
      j += 1;
    } else if (lcs[i + 1][j] >= lcs[i][j + 1]) {
      lines.push({ kind: "remove", beforeNo: i + 1, text: before[i] });
      i += 1;
    } else {
      lines.push({ kind: "add", afterNo: j + 1, text: after[j] });
      j += 1;
    }
  }
  while (i < beforeLen) {
    lines.push({ kind: "remove", beforeNo: i + 1, text: before[i] });
    i += 1;
  }
  while (j < afterLen) {
    lines.push({ kind: "add", afterNo: j + 1, text: after[j] });
    j += 1;
  }
  return lines;
}

/** 🪞️ Pairs consecutive remove/add runs into aligned rows for the split-pane layout; equal lines mirror onto both sides. */
function buildSplitRows(diff: readonly DiffLine[]): SplitRow[] {
  const rows: SplitRow[] = [];
  let i = 0;
  while (i < diff.length) {
    const line = diff[i];
    if (line.kind === "equal") {
      rows.push({ left: line, right: line });
      i += 1;
      continue;
    }
    const removes: DiffLine[] = [];
    while (i < diff.length && diff[i].kind === "remove") {
      removes.push(diff[i]);
      i += 1;
    }
    const adds: DiffLine[] = [];
    while (i < diff.length && diff[i].kind === "add") {
      adds.push(diff[i]);
      i += 1;
    }
    const pairCount = Math.max(removes.length, adds.length);
    for (let pair = 0; pair < pairCount; pair += 1) {
      rows.push({ left: removes[pair], right: adds[pair] });
    }
  }
  return rows;
}
//#endregion LineDiff

//#region Rendering
const DIFF_LINE_CLASS: Record<DiffLineKind, string> = {
  equal: "text-foreground",
  add: "text-emerald-400",
  remove: "text-destructive",
};

const DIFF_LINE_PREFIX: Record<DiffLineKind, string> = { equal: " ", add: "+", remove: "-" };

function UnifiedDiff({ lines }: { readonly lines: readonly DiffLine[] }) {
  return (
    <div className="semio-diff-view-unified font-mono text-xs">
      {lines.map((line, index) => (
        <div key={index} className={cn("flex gap-single whitespace-pre-wrap px-single", DIFF_LINE_CLASS[line.kind])}>
          <span className="text-muted-foreground w-10 shrink-0 select-none text-right tabular-nums">{line.beforeNo ?? ""}</span>
          <span className="text-muted-foreground w-10 shrink-0 select-none text-right tabular-nums">{line.afterNo ?? ""}</span>
          <span className="w-3 shrink-0 select-none">{DIFF_LINE_PREFIX[line.kind]}</span>
          <span>{line.text}</span>
        </div>
      ))}
    </div>
  );
}

function SplitDiffPane({ rows, side }: { readonly rows: readonly SplitRow[]; readonly side: "left" | "right" }) {
  return (
    <div className="semio-diff-view-split-pane min-w-0 flex-1 font-mono text-xs">
      {rows.map((row, index) => {
        const line = row[side];
        return (
          <div key={index} className={cn("flex gap-single whitespace-pre-wrap px-single", line ? DIFF_LINE_CLASS[line.kind] : "text-muted-foreground")}>
            <span className="text-muted-foreground w-10 shrink-0 select-none text-right tabular-nums">{line ? (side === "left" ? line.beforeNo : line.afterNo) : ""}</span>
            <span>{line?.text ?? ""}</span>
          </div>
        );
      })}
    </div>
  );
}
//#endregion Rendering

//#region 🔖️ConflictDiff
/** ⚔️ Builds this host's `before`/`after` pair for a selected open {@link Conflict} (contract freeze
 * §C5/§C9) — `before` is the caller-supplied current-document text (empty when no synchronous
 * snapshot is in hand yet), `after` is the incoming `Quarantined` envelopes pretty-printed, or a
 * placeholder naming the touched edits for a `Degraded` conflict (there is no separate "incoming"
 * payload to diff against once a degraded batch already applied). */
export function conflictDiffText(conflict: Conflict, current: string): { readonly before: string; readonly after: string } {
  if (conflict.kind.kind === "quarantined") return { before: current, after: JSON.stringify(conflict.kind.envelopes, null, 2) };
  return { before: current, after: `// degraded edits: ${conflict.kind.edit_ids.join(", ")}` };
}

/** ⚔️ Renders a `before`/`after` conflict pair through this host's own unified diff renderer —
 * `ChromePanels`' Conflicts panel reuses it directly for "incoming vs current" without needing a
 * full `DiffViewScene` component-scene node (see {@link conflictDiffText}). */
export function ConflictDiffPreview({ before, after }: { readonly before: string; readonly after: string }): ReactElement {
  const lines = useMemo(() => diffLines(before.split("\n"), after.split("\n")), [before, after]);
  return <UnifiedDiff lines={lines} />;
}
//#endregion 🔖️ConflictDiff

//#region Component
/** @emoji 🆚️ Renders a `DiffViewScene`: a minimal, dependency-free line-based diff between `before`/`after`, unified (default) or split per `mode`. */
export function DiffViewHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.diffView;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.diff");
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const before = scene?.before ?? "";
  const after = scene?.after ?? "";
  const lines = useMemo(() => diffLines(before.split("\n"), after.split("\n")), [before, after]);
  const splitRows = useMemo(() => (scene?.mode === "split" ? buildSplitRows(lines) : []), [lines, scene?.mode]);

  //#region ContextMenu
  /** @emoji 🖱️ `DiffViewScene` carries only `before`/`after`/`mode` — no per-line pick/selection state reaches this
   * host — so `hits`/`selection` stay empty per surface convention (see `GraphTimelineHost`). */
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "diffView", args: null },
            surface: { surfaceId: node.surfaceId, kind: "diffView", hits: [], selection: [] },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
      })();
    },
    [mapContextMenu, node.surfaceId, requestContextMenu, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion ContextMenu

  if (!scene) return <div className="semio-diff-view-empty">{emptySceneLabel}</div>;

  return (
    <div
      className="semio-diff-view-host h-full min-h-0 w-full overflow-auto p-single"
      data-surface-id={node.surfaceId}
      data-diff-language={scene.language}
      onContextMenu={onContextMenu}
    >
      {scene.mode === "split" ? (
        <div className="flex min-h-0 w-full gap-single">
          <SplitDiffPane rows={splitRows} side="left" />
          <div className="border-border w-px shrink-0 border-l" />
          <SplitDiffPane rows={splitRows} side="right" />
        </div>
      ) : (
        <UnifiedDiff lines={lines} />
      )}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Component
//#endregion DiffViewHost
//#endregion 🔖️DiffViewHost
