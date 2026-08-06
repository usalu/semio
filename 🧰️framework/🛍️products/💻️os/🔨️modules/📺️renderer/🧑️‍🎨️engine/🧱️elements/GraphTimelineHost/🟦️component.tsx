// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/GraphTimelineHost/component.tsx
/** @emoji 🕰️ `GraphTimelineHost` — the graph-history/checkpoint timeline scene host: renders a
 * `HistoryTable` from the program-supplied `columnsJson` and dispatches `checkoutCheckpoint` on row
 * selection. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useMemo, useState, type MouseEvent } from "react";
import { type ComponentSceneHostProps } from "@semio-tech/framework-core";
import { childElementId, ContextMenuController, HistoryTable, useLabel, type ContextMenuItem, type HistoryColumn } from "@semio-tech/ui-react";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback } from "../Interpreter/🟦️component.tsx";
import { WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️GraphTimelineHost
//#region GraphTimelineHost
export function GraphTimelineHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.graphTimeline;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const contextMenuTitleLabel = useLabel("ui.surfaceContextMenu.history");
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const columns = useMemo(() => {
    if (!scene) return [] as HistoryColumn[];
    try {
      return parseSceneJsonField<HistoryColumn[]>(scene.columnsJson);
    } catch {
      return [];
    }
  }, [scene]);

  //#region ContextMenu
  /** @emoji 🖱️ `GraphTimelineScene` carries only `columnsJson` — no per-row pick/selection state reaches this host (`HistoryTable` doesn't expose a row-context-menu hook either) — so `hits`/`selection` stay empty per surface convention. */
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const items = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "graphTimeline" },
            surface: { surfaceId: node.surfaceId, kind: "graphTimeline", hits: [], selection: [] },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, items });
      })();
    },
    [mapContextMenu, node.surfaceId, requestContextMenu, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion ContextMenu

  if (!scene) return <div className="semio-graph-timeline-empty">{emptySceneLabel}</div>;

  return (
    <div className="semio-graph-timeline-host h-full min-h-0 w-full overflow-auto p-single" data-surface-id={node.surfaceId} onContextMenu={onContextMenu}>
      <HistoryTable
        id={childElementId(node.surfaceId, "table")}
        columns={columns}
        onSelectCheckpoint={(checkpointId) =>
          onAction({
            controllerId: node.controllerId,
            action: "checkoutCheckpoint",
            args: { checkpointId },
          })
        }
      />
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
//#endregion GraphTimelineHost
//#endregion 🔖️GraphTimelineHost
