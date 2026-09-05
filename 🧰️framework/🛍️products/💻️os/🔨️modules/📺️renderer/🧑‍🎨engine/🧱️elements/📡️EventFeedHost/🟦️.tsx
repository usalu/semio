// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/📡️EventFeedHost/component.tsx
/** @emoji 📰️ `📡️EventFeedHost` — renders an `EventFeedScene`: a scrollable log of `entriesJson` entries
 * (icon + timestamp + title/detail), auto-scrolling to the newest entry while `follow` is set, and
 * dispatching `activateAction` on entry click. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useMemo, useRef, useState, type MouseEvent } from "react";
import { type ComponentSceneHostProps, type EventFeedEntry, type MutationMessage } from "@semio-tech/framework";
import { cn, ContextMenuController, Icon, interactiveHoverFillClass, useLabel, type ContextMenuItem, type IconName } from "@semio-tech/ui-react";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../🗣️Interpreter/🟦️.tsx";
import { WindowInstanceIdContext } from "../🌐️World3dHost/🟦️.tsx";
import { useMapContextMenuSpecs } from "../🏛️ShellHost/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️EventFeedHost
//#region EventFeedHost
//#region Helpers
const FEED_TONE_CLASS: Record<string, string> = {
  info: "text-foreground",
  success: "text-emerald-400",
  warning: "text-amber-400",
  error: "text-destructive",
  /** ⚖️ `Severity::Fatal` (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
   * CLASS-CONFLICTS` §C1) — bolder than plain `error` since a fatal outcome never partially applied. */
  fatal: "text-destructive font-semibold",
};

function formatFeedTimestamp(timestampMs: number): string {
  try {
    return new Date(timestampMs).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" });
  } catch {
    return "";
  }
}

/** ⚖️ Mints one {@link EventFeedEntry} from a `MutationMessage` (contract freeze §C2/§C9) — `tone`
 * follows `level` 1:1 (`FEED_TONE_CLASS` covers all four, including `fatal`), `title` stays the raw
 * `code` (a consumer localizes it via `ui.mutation.code.*`, never by parsing `message`), `detail`
 * carries the English prose verbatim. */
export function mintMutationMessageFeedEntry(message: MutationMessage, id: string, timestampMs: number, iconId: IconName = "triangle-alert"): EventFeedEntry {
  return { id, timestampMs, iconId, title: message.code, detail: message.message, tone: message.level };
}
//#endregion Helpers

//#region Component
/** @emoji 📰️ Renders an `EventFeedScene`: a scrollable log of `entriesJson` entries (icon + timestamp + title/detail), auto-scrolling to the newest entry while `follow` is set, dispatching `activateAction` on entry click. */
export function EventFeedHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.eventFeed;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.event");
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const entries = useMemo(() => {
    if (!scene?.entriesJson) return [] as EventFeedEntry[];
    try {
      return parseSceneJsonField<EventFeedEntry[]>(scene.entriesJson);
    } catch {
      return [];
    }
  }, [scene?.entriesJson]);
  const listRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!scene?.follow) return;
    const list = listRef.current;
    if (!list) return;
    list.scrollTop = list.scrollHeight;
  }, [entries, scene?.follow]);

  //#region ContextMenu
  /** @emoji 🖱️ `EventFeedScene` tracks no selection concept — each log row's `id` is still known at the click
   * site, so unlike the whole-surface-only hosts this reports the right-clicked entry as a `hit` (see
   * `TableHost`'s `onRowContextMenu` for the analogous per-row convention). */
  const onEntryContextMenu = useCallback(
    (entryId: string, event: MouseEvent<HTMLDivElement>): void => {
      if (!requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "eventFeed", args: null },
            surface: { surfaceId: node.surfaceId, kind: "eventFeed", hits: [{ domain: "entry", id: entryId }], selection: [] },
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

  if (!scene) return <div className="semio-event-feed-empty">{emptySceneLabel}</div>;

  const activateAction = scene.activateAction;
  return (
    <div ref={listRef} className="semio-event-feed-host flex h-full min-h-0 w-full flex-col gap-single overflow-auto p-single" data-surface-id={node.surfaceId}>
      {entries.map((entry) => (
        <div
          key={entry.id}
          className={cn("flex items-start gap-single rounded-md p-single", activateAction && cn(interactiveHoverFillClass, "cursor-pointer"))}
          role={activateAction ? "button" : undefined}
          onClick={
            activateAction
              ? () =>
                  onAction({
                    controllerId: node.controllerId,
                    action: activateAction,
                    args: { surfaceId: node.surfaceId, id: entry.id },
                  })
              : undefined
          }
          onContextMenu={(event) => onEntryContextMenu(entry.id, event)}
        >
          <Icon icon={entry.iconId as IconName} size="small" />
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-single">
              <span className={cn("truncate text-xs font-medium", entry.tone ? FEED_TONE_CLASS[entry.tone] : undefined)}>{entry.title}</span>
              <span className="text-muted-foreground ml-auto shrink-0 text-[10px] tabular-nums">{formatFeedTimestamp(entry.timestampMs)}</span>
            </div>
            {entry.detail ? <p className="text-muted-foreground truncate text-xs">{entry.detail}</p> : null}
          </div>
        </div>
      ))}
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
//#endregion EventFeedHost
//#endregion 🔖️EventFeedHost
