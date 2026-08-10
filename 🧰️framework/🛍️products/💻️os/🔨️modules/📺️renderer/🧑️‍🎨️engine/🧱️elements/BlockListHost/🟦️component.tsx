// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/BlockListHost/component.tsx
/** @emoji 🧱️ `BlockListHost` — the step/block-list scene host: dnd-kit sortable steps and blocks,
 * a drag-and-drop block palette (native-drag and driver-arm-drag), and step/block add/remove/move
 * action dispatch. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { useCallback, useContext, useMemo, useState, type MouseEvent } from "react";
import { type ActionDescriptor, type ComponentSceneHostProps } from "@semio-tech/framework";
import {
  Button,
  cn,
  closestCenter,
  ContextMenuController,
  DndContext,
  DndCSS,
  DragHandle,
  Icon,
  SortableContext,
  useLabel,
  useNativeDragArm,
  useSortable,
  useUiDriverDragSurface,
  verticalListSortingStrategy,
  type ContextMenuItem,
  type DragEndEvent,
  type IconName,
} from "@semio-tech/ui-react";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../Interpreter/🟦️component.tsx";
import { WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️BlockListHost
//#region BlockListHost
//#region Types
type BlockRecord = { readonly id: string; readonly label: string; readonly kind: string; readonly description?: string };
type StepRecord = { readonly id: string; readonly title: string; readonly description?: string; readonly blocks: readonly BlockRecord[] };
type PaletteEntryRecord = { readonly blockKind: string; readonly label: string; readonly iconId: IconName };
const PALETTE_DRAG_MIME = "application/x-semio-block-list-block-kind";
//#endregion Types

//#region Helpers
function dispatchBlockListAction(onAction: (action: ActionDescriptor) => void, controllerId: string, action: string, args: Record<string, unknown>): void {
  onAction({ controllerId, action, args });
}
//#endregion Helpers

//#region SortableRow
/** 🧩️ Wraps a row in dnd-kit's sortable machinery so it can be reordered within its enclosing `SortableContext`. */
function SortableRow({ id, children }: { readonly id: string; readonly children: (dragHandleProps: { readonly attributes: React.HTMLAttributes<HTMLElement>; readonly listeners: Record<string, unknown> | undefined; readonly style: React.CSSProperties }) => React.ReactNode }) {
  const surfaceDrag = useUiDriverDragSurface();
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  const style: React.CSSProperties = { transform: DndCSS.Transform.toString(transform), transition, opacity: isDragging ? 0.5 : 1 };
  return (
    <div ref={setNodeRef} style={style} {...attributes} {...(surfaceDrag ? listeners : {})}>
      {children({ attributes, listeners: surfaceDrag ? undefined : listeners, style })}
    </div>
  );
}
//#endregion SortableRow

//#region Block
function BlockCard({ block, stepId, controllerId, onAction }: { readonly block: BlockRecord; readonly stepId: string; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const surfaceDrag = useUiDriverDragSurface();
  return (
    <SortableRow id={block.id}>
      {({ attributes, listeners }) => (
        <div className={cn("semio-block-card flex items-center gap-2 rounded border border-border bg-background p-single", surfaceDrag && "cursor-grab active:cursor-grabbing")} data-block-id={block.id}>
          {!surfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(event) => event.stopPropagation()} /> : <Icon icon="grip-vertical" size="small" />}
          <div className="min-w-0 flex-1">
            <div className="truncate text-xs font-medium">{block.label}</div>
            <div className="truncate text-xs text-muted-foreground">{block.kind}</div>
          </div>
          <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, controllerId, "removeBlock", { stepId, blockId: block.id })} />
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Block

//#region Step
function StepCard({ step, palette, controllerId, onAction }: { readonly step: StepRecord; readonly palette: readonly PaletteEntryRecord[]; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const surfaceDrag = useUiDriverDragSurface();
  const blockIds = useMemo(() => step.blocks.map((block) => block.id), [step.blocks]);

  function handleBlockDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = step.blocks.findIndex((block) => block.id === over.id);
    if (index === -1) return;
    dispatchBlockListAction(onAction, controllerId, "moveBlock", { blockId: active.id, fromStepId: step.id, toStepId: step.id, index });
  }

  return (
    <SortableRow id={step.id}>
      {({ attributes, listeners }) => (
        <div
          className={cn("semio-step-card flex flex-col gap-2 rounded border border-border bg-background p-single", surfaceDrag && "cursor-grab active:cursor-grabbing")}
          data-step-id={step.id}
          onDragOver={(event) => {
            event.preventDefault();
            event.dataTransfer.dropEffect = "copy";
          }}
          onDrop={(event) => {
            event.preventDefault();
            const kind = event.dataTransfer.getData(PALETTE_DRAG_MIME);
            if (!kind) return;
            dispatchBlockListAction(onAction, controllerId, "addBlock", { stepId: step.id, kind });
          }}
        >
          <div className="flex items-center gap-2">
            {!surfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(event) => event.stopPropagation()} /> : <Icon icon="grip-vertical" size="small" />}
            <div className="min-w-0 flex-1 truncate text-sm font-medium">{step.title}</div>
            <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, controllerId, "removeStep", { stepId: step.id })} />
          </div>
          {step.description && <div className="text-xs text-muted-foreground">{step.description}</div>}
          <DndContext collisionDetection={closestCenter} onDragEnd={handleBlockDragEnd}>
            <SortableContext items={blockIds} strategy={verticalListSortingStrategy}>
              <div className="flex flex-col gap-1">
                {step.blocks.map((block) => (
                  <BlockCard key={block.id} block={block} stepId={step.id} controllerId={controllerId} onAction={onAction} />
                ))}
              </div>
            </SortableContext>
          </DndContext>
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Step

//#region Palette
function PaletteEntryRow({ entry, controllerId, onAction }: { readonly entry: PaletteEntryRecord; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const surfaceDrag = useUiDriverDragSurface();
  const { armed, arm } = useNativeDragArm();
  return (
    <div
      draggable={surfaceDrag || armed}
      onDragStart={(event) => {
        event.dataTransfer.setData(PALETTE_DRAG_MIME, entry.blockKind);
        event.dataTransfer.effectAllowed = "copy";
      }}
      className={cn("flex items-center gap-1 rounded border border-border p-single text-xs", surfaceDrag && "cursor-grab active:cursor-grabbing")}
      onClick={() => dispatchBlockListAction(onAction, controllerId, "addBlock", { kind: entry.blockKind })}
    >
      {!surfaceDrag ? <DragHandle labelId="ui.tree.drag.transfer" iconKind="move" onPointerDown={arm} onClick={(event) => event.stopPropagation()} /> : null}
      <Icon icon={entry.iconId} size="small" />
      {entry.label}
    </div>
  );
}

function PalettePanel({ palette, controllerId, onAction }: { readonly palette: readonly PaletteEntryRecord[]; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <div className="semio-palette flex shrink-0 flex-col gap-1 border-l border-border p-single">
      {palette.map((entry) => (
        <PaletteEntryRow key={entry.blockKind} entry={entry} controllerId={controllerId} onAction={onAction} />
      ))}
    </div>
  );
}
//#endregion Palette

//#region Component
export function BlockListHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.blockList;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.step");
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const steps = useMemo(() => {
    if (!scene) return [] as StepRecord[];
    try {
      return parseSceneJsonField<StepRecord[]>(scene.stepsJson);
    } catch {
      return [];
    }
  }, [scene]);
  const palette = useMemo(() => {
    if (!scene) return [] as PaletteEntryRecord[];
    try {
      return parseSceneJsonField<PaletteEntryRecord[]>(scene.paletteJson);
    } catch {
      return [];
    }
  }, [scene]);
  const stepIds = useMemo(() => steps.map((step) => step.id), [steps]);
  const stepsLabel = useLabel("ui.blockList.steps");
  const addStepLabel = useLabel("ui.blockList.addStep");
  const emptyLabel = useLabel("ui.host.emptyScene");

  //#region ContextMenu
  /** @emoji 🖱️ `BlockListScene` carries only `stepsJson`/`paletteJson` — no per-step pick/selection state reaches this
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
            menu: { id: "blockList" },
            surface: { surfaceId: node.surfaceId, kind: "blockList", hits: [], selection: [] },
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

  if (!scene) return <div className="semio-block-list-empty">{emptyLabel}</div>;

  function handleStepDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = steps.findIndex((step) => step.id === over.id);
    if (index === -1) return;
    dispatchBlockListAction(onAction, node.controllerId, "moveStep", { stepId: active.id, index });
  }

  return (
    <div className="semio-block-list-host flex h-full min-h-0 w-full" data-surface-id={node.surfaceId} onContextMenu={onContextMenu}>
      <div className="flex min-w-0 flex-1 flex-col gap-2 overflow-auto p-single">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">{stepsLabel}</span>
          <Button className="h-medium shrink-0 px-2" icon="plus" text={addStepLabel} type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, node.controllerId, "addStep", {})} />
        </div>
        <DndContext collisionDetection={closestCenter} onDragEnd={handleStepDragEnd}>
          <SortableContext items={stepIds} strategy={verticalListSortingStrategy}>
            <div className="flex flex-col gap-2">
              {steps.map((step) => (
                <StepCard key={step.id} step={step} palette={palette} controllerId={node.controllerId} onAction={onAction} />
              ))}
            </div>
          </SortableContext>
        </DndContext>
      </div>
      <PalettePanel palette={palette} controllerId={node.controllerId} onAction={onAction} />
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
//#endregion BlockListHost
//#endregion 🔖️BlockListHost
