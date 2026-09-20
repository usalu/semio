// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Table/component.tsx
/** @emoji 📊️ `Table` — the tabular data scene host: column/row/selection/sort parsing, stepper and
 * row-action-button cell renderers, row drag source wiring, row drop targets, and the per-row
 * context menu (program-supplied items merged with row-scoped button-placement actions). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useMemo, useState } from "react";
import { Button, ContextMenuController, Icon, Input, Table, uiDataLabel, useLabel, useShellScopeOptional, type ContextMenuItem, type IconName, type TableColumn } from "@semio-tech/ui-react";
import { type ActionDescriptor, type ComponentSceneHostProps } from "@semio-tech/framework";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../🗣️Interpreter/🟦️.tsx";
import { WindowInstanceIdContext } from "../🌐️World3dHost/🟦️.tsx";
import { useMapContextMenuSpecs } from "../🏛️ShellHost/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️TableHost
//#region TableHost
//#region Types
type TableColumnRecord = { readonly id: string; readonly label: string; readonly sortable?: boolean };
type TableCellButton = { readonly iconId: IconName; readonly label?: string; readonly action: ActionDescriptor; readonly placement?: "row" | "menu" };
type TableCellRecord =
  | { readonly kind: "text"; readonly value: string }
  | { readonly kind: "number"; readonly value: number }
  | { readonly kind: "stepper"; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly action: ActionDescriptor }
  | { readonly kind: "buttons"; readonly buttons: readonly TableCellButton[] };
type TableRowRecord = Record<string, unknown> & { readonly id?: string; readonly _drag?: Record<string, unknown> };

/** 📐️ How many steps one PageUp/PageDown moves a stepper cell. */
const TABLE_STEPPER_PAGE = 10;
//#endregion Types

//#region Helpers
function isTableCellRecord(value: unknown): value is TableCellRecord {
  return typeof value === "object" && value !== null && "kind" in value;
}

function dispatchCellAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function tableRowMenuPlacementItems(row: TableRowRecord, onAction: (action: ActionDescriptor) => void): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  for (const value of Object.values(row)) {
    if (!isTableCellRecord(value) || value.kind !== "buttons") continue;
    for (const [index, button] of value.buttons.entries()) {
      if ((button.placement ?? "row") !== "menu") continue;
      items.push({
        id: `table-row-action-${button.iconId}-${index}`,
        label: button.label === undefined ? undefined : uiDataLabel(button.label),
        icon: button.iconId,
        onSelect: () => dispatchCellAction(onAction, button.action, {}),
      });
    }
  }
  return items;
}

/** ⌨️ How far one key moves a stepper cell: arrows step once, Page keys step ten, Home/End jump to a bound. */
export function tableStepperKeyDelta(key: string, cell: { readonly value: number; readonly min: number; readonly max: number; readonly step: number }): number {
  switch (key) {
    case "ArrowUp":
    case "ArrowRight":
      return cell.step;
    case "ArrowDown":
    case "ArrowLeft":
      return -cell.step;
    case "PageUp":
      return cell.step * TABLE_STEPPER_PAGE;
    case "PageDown":
      return -cell.step * TABLE_STEPPER_PAGE;
    case "Home":
      return cell.min - cell.value;
    case "End":
      return cell.max - cell.value;
    default:
      return 0;
  }
}

/** 🧮️ The delta actually dispatched: never past `min`/`max`, and `0` when the cell already sits on that bound. */
export function tableStepperClampedDelta(delta: number, cell: { readonly value: number; readonly min: number; readonly max: number }): number {
  return Math.min(cell.max, Math.max(cell.min, cell.value + delta)) - cell.value;
}

/**
 * 🪜️ One `TableCell::Stepper` cell: decrement · value · increment, driving the SAME contract the
 * wgpu table widget drives — the cell's own `ActionDescriptor` with `{ delta }` merged into its args,
 * suppressed at the bounds. The readout carries `role="spinbutton"` with the live `aria-value*`
 * triple, so the whole control is one keyboard stop (arrows, PageUp/PageDown, Home/End). The
 * buttons keep their `button-group`/`button-group-item` slots, which is what the touch stylesheet
 * grows to `--layout-touch-min` at phone width.
 */
function TableStepperCell({ cell, id, columnLabel, onAction }: { readonly cell: Extract<TableCellRecord, { kind: "stepper" }>; readonly id: string; readonly columnLabel: string | undefined; readonly onAction: (action: ActionDescriptor) => void }): React.ReactElement {
  const decrementLabel = useLabel("ui.tableStepper.decrement");
  const incrementLabel = useLabel("ui.tableStepper.increment");
  const valueLabel = useLabel("ui.tableStepper.value");
  const readoutLabel = columnLabel?.trim() ? columnLabel : valueLabel;
  const step = (delta: number): void => {
    const clamped = tableStepperClampedDelta(delta, cell);
    if (clamped === 0) return;
    dispatchCellAction(onAction, cell.action, { delta: clamped });
  };
  return (
    <div className="flex min-w-0 items-center gap-1" data-slot="table-stepper" data-stepper-for={id} onClick={(event) => event.stopPropagation()}>
      <Button
        aria-label={`${decrementLabel} ${readoutLabel}`}
        className="h-medium shrink-0 px-2"
        data-stepper-control="decrement"
        data-stepper-for={id}
        disabled={cell.value <= cell.min}
        icon="minus"
        onClick={() => dispatchCellAction(onAction, cell.action, { delta: -cell.step })}
        tabIndex={-1}
        title={decrementLabel}
        type="button"
        variant="outline"
      />
      <Input
        aria-label={readoutLabel}
        aria-valuemax={cell.max}
        aria-valuemin={cell.min}
        aria-valuenow={cell.value}
        aria-valuetext={String(cell.value)}
        className="h-medium w-14 min-w-0 text-center font-mono text-xs"
        data-stepper-control="value"
        id={id}
        onKeyDown={(event) => {
          const delta = tableStepperKeyDelta(event.key, cell);
          if (delta === 0) return;
          event.preventDefault();
          step(delta);
        }}
        readOnly
        role="spinbutton"
        value={String(cell.value)}
      />
      <Button
        aria-label={`${incrementLabel} ${readoutLabel}`}
        className="h-medium shrink-0 px-2"
        data-stepper-control="increment"
        data-stepper-for={id}
        disabled={cell.value >= cell.max}
        icon="plus"
        onClick={() => dispatchCellAction(onAction, cell.action, { delta: cell.step })}
        tabIndex={-1}
        title={incrementLabel}
        type="button"
        variant="outline"
      />
    </div>
  );
}

function renderTableCell(cell: TableCellRecord, id: string, columnLabel: string | undefined, onAction: (action: ActionDescriptor) => void): React.ReactNode {
  switch (cell.kind) {
    case "text":
      return cell.value;
    case "number":
      return String(cell.value);
    case "stepper":
      return <TableStepperCell cell={cell} columnLabel={columnLabel} id={id} onAction={onAction} />;
    case "buttons":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          {cell.buttons
            .filter((button) => (button.placement ?? "row") === "row")
            .map((button, index) => (
            <Button icon={button.iconId} key={index} className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, button.action, {})} title={button.label} type="button" variant="outline">
              <Icon icon={button.iconId} size="small" />
            </Button>
          ))}
        </div>
      );
  }
}
//#endregion Helpers

//#region Component
export function TableHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.table;
  // 🐚️ Optional — this host is also unit-tested standalone, outside any `ShellScopeProvider`.
  const shellScope = useShellScopeOptional();
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.row");
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const columns = useMemo(() => {
    if (!scene) return [] as TableColumnRecord[];
    try {
      return parseSceneJsonField<TableColumnRecord[]>(scene.columnsJson);
    } catch {
      return [];
    }
  }, [scene]);
  const rows = useMemo(() => {
    if (!scene) return [] as TableRowRecord[];
    try {
      return parseSceneJsonField<TableRowRecord[]>(scene.rowsJson);
    } catch {
      return [];
    }
  }, [scene]);
  const rowIds = useMemo(() => new Map(rows.map((row, index) => [row, String(row.id ?? row.pluginId ?? index)])), [rows]);
  const getRowId = useCallback((row: TableRowRecord): string => {
    const id = rowIds.get(row);
    if (id === undefined) throw new Error("table row is not part of the captured scene");
    return id;
  }, [rowIds]);
  const selectedRows = useMemo(() => {
    if (!scene?.selectionJson) return undefined;
    try {
      const parsed = parseSceneJsonField<{ readonly selectedIds?: readonly string[] }>(scene.selectionJson);
      return new Set(parsed.selectedIds ?? []);
    } catch {
      return undefined;
    }
  }, [scene]);
  const sort = useMemo(() => {
    if (!scene?.sortJson) return undefined;
    try {
      return parseSceneJsonField<{ readonly columnId?: string; readonly direction?: "asc" | "desc" }>(scene.sortJson);
    } catch {
      return undefined;
    }
  }, [scene]);
  const tableColumns = useMemo<TableColumn<TableRowRecord>[]>(
    () =>
      columns.map((column) => ({
        id: column.id,
        header: column.label,
        sortable: column.sortable,
        accessor: (row) => {
          const value = row[column.id];
          if (isTableCellRecord(value)) return renderTableCell(value, `${node.surfaceId}.${getRowId(row)}.${column.id}`, column.label, onAction);
          return String(value ?? "");
        },
      })),
    [columns, getRowId, node.surfaceId, onAction],
  );

  if (!scene) return <div className="semio-table-empty">{emptySceneLabel}</div>;

  const rowDragMime = scene.rowDragMime;
  const dropAction = scene.dropActionJson ? parseSceneJsonField<ActionDescriptor>(scene.dropActionJson) : undefined;

  return (
    <div
      className="semio-table-host h-full min-h-0 w-full"
      data-surface-id={node.surfaceId}
      onDragOver={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.dataTransfer.dropEffect = "copy";
            }
          : undefined
      }
      onDrop={
        dropAction
          ? (event) => {
              event.preventDefault();
              const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
              if (!encoded?.trim()) return;
              try {
                dispatchCellAction(onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
              } catch {
                return;
              }
            }
          : undefined
      }
    >
      <Table
        className="h-full w-full"
        columns={tableColumns}
        data={rows}
        getRowId={getRowId}
        selectedRows={selectedRows}
        sortColumn={sort?.columnId}
        sortDirection={sort?.direction}
        onSort={(columnId, direction) =>
          onAction({
            controllerId: node.controllerId,
            action: "sortTable",
            args: { surfaceId: node.surfaceId, columnId, direction },
          })
        }
        rowDragProps={
          rowDragMime
            ? (row) =>
                row._drag
                  ? {
                      draggable: true,
                      onDragStart: (event) => {
                        event.dataTransfer.setData(rowDragMime, JSON.stringify(row._drag));
                        event.dataTransfer.effectAllowed = "copy";
                      },
                    }
                  : {}
            : undefined
        }
        onRowClick={(row) =>
          scene.domainId && scene.domainGranularityId
            ? onAction({
                controllerId: node.controllerId,
                action: "interactionSelect",
                args: { domainId: scene.domainId, targets: JSON.stringify([{ granularity: scene.domainGranularityId, id: getRowId(row) }]), merge: "replace", method: "pick" },
              })
            : onAction({
                controllerId: node.controllerId,
                action: "selectRow",
                args: { surfaceId: node.surfaceId, row },
              })
        }
        onRowContextMenu={(row, index, event) => {
          if (!requestContextMenu) return;
          event.preventDefault();
          event.stopPropagation();
          const rowId = String(row.id ?? row.pluginId ?? index);
          void (async () => {
            const menu = await openSurfaceContextMenu(
              requestContextMenu,
              {
                menu: { id: "table", args: null },
                surface: {
                  surfaceId: node.surfaceId,
                  kind: "table",
                  hits: [{ domain: "row", id: rowId }],
                  selection: selectedRows && selectedRows.size > 0 ? [{ domain: "row", ids: [...selectedRows] }] : [],
                },
                windowInstanceId: windowInstanceId ?? undefined,
                point: { x: event.clientX, y: event.clientY },
              },
              mapContextMenu,
              shellContextMenuFallback,
            );
            const menuActions = tableRowMenuPlacementItems(row, onAction);
            setContextMenu({
              x: event.clientX,
              y: event.clientY,
              ...menu,
              items: menuActions.length ? [...menu.items, ...(menu.items.length ? [{ id: "table-row-action-separator", separator: true } as ContextMenuItem] : []), ...menuActions] : menu.items,
            });
          })();
        }}
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
//#endregion Component
//#endregion TableHost
//#endregion 🔖️TableHost
