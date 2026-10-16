/** 🧱️ Node rendering and component dispatch: the per-node subscribing view, the component switch and the views that recurse into child nodes (container, tree, table).
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  useSyncExternalStore,
  type ReactElement,
  type ReactNode,
  type MouseEvent as ReactMouseEvent
} from "react";
import { rowActionAdmissionKeyV1 } from "../🎬️row-action-admission/🟦️.ts";
import {
  Field,
  Section,
  Tree,
  borderElementClass,
  borderNormalTopClass,
  catalogueTreeDragController,
  cn,
  treeRowHeightPx,
  treeWindowDomAttributes,
  treeWindowPathOf,
  treeWindowSpacerRows,
  useLabel,
  type TreeDataActivationContext,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController
} from "@semio-tech/ui-react";
import {
  UiDocumentStore,
  rowActivationBinding,
  useUiDocumentRevision,
  useUiNode,
  type UiDocumentState
} from "../../📃️UiDocumentStore/🟦️.tsx";
import { type Component, type UiNodeId, type UiNodeRecord } from "@semio-tech/framework";
import { wireLabel } from "../🌉️scene-host-contract/🟦️.ts";
import { leftoverItemSelectedV1, leftoverSelectionOverlayV1, subscribeLeftoverSelectionV1 } from "../🗺️element-host-registry/🟦️.ts";
import { type UiPresenceOverlayValue, usePresenceOverlayEntry, useUiPresenceOverlay } from "../👥️presence-overlay/🟦️.ts";
import { treeSectionsCatalogueDragMime } from "../🌲️tree-panel-boundary/🟦️.ts";
import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { accessibilityAriaProps, layoutSpecStyle, styleSpecDataAttributes } from "../📐️layout-accessibility/🟦️.tsx";
import { dispatchTrigger, resolveControlIconNode } from "../⚡️action-dispatch/🟦️.tsx";
import {
  activityBorderClass,
  ButtonView,
  ExtensionView,
  ImageView,
  interpretUiNodeBusyShell,
  KeyValueListView,
  ProgressView,
  TextView,
  UnknownComponentView
} from "../🔤️display-views/🟦️.tsx";
import {
  IconSelectView,
  InputView,
  NumberStepperView,
  RingView,
  SelectView,
  SliderView,
  ToggleView
} from "../✍️input-views/🟦️.tsx";
import {
  collectTreeItemControls,
  collectTreeItems,
  TREE_ROW_TONE_CLASSES,
  treeRowStandsForControl,
  treeStatusSection,
  type TreeWalkNode
} from "../🌳️tree-walk/🟦️.ts";
import { nodeDomId, uiChildReactKeys, uiNodeDomId, uiSiblingReactKeys } from "../🪪️stable-dom-ids/🟦️.ts";
import {
  dispatchTreePick,
  registerTreeWalkRow,
  TABLE_COLUMN_WINDOW_WIDTH_PX,
  treeDomIdFromOpenStateIdV1,
  type TreeWalkContextV1,
  useTreeWindowContext,
  useTreeWindowObserver
} from "../🪟️tree-windows/🟦️.ts";
import { type RowActionAdmissionControllerV1, rowTreeActions, TableRowActionButton, useRowActionAdmissionV1 } from "../🧾️row-actions/🟦️.tsx";
import { SurfaceView } from "../🖼️surface-views/🟦️.tsx";
import {
  tableWindowColumnTemplate,
  tableWindowNextColumnV1,
  tableWindowNextRowV1,
  tableWindowScrollLeftForColumnV1,
  tableWindowScrollTopForRowV1,
  tableWindowViewportCapRowsV1
} from "../📊️table-window/🟦️.ts";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
function ContainerView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "container" }>;
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  const presence = usePresenceOverlayEntry(record.key);
  const style = layoutSpecStyle(record.layout);
  const dataAttrs = styleSpecDataAttributes(record.style);
  const childIds = record.children ?? [];
  const childReactKeys = uiChildReactKeys(store.getState(), childIds);
  const children = childIds.map((childId, index) => <UiNodeView key={childReactKeys[index]} store={store} id={childId} context={context} />);
  const role = component.role === "form" ? "form" : component.role === "toolbar" ? "toolbar" : undefined;
  const activateBinding = (record.bindings ?? []).find((binding) => binding.trigger === "activate");

  // 🪪️ A section and a field carry the SAME stable DOM id every other interpreted node gets. Both
  // wrappers accept `id` and both dropped it here, so an app that authored `ui::section(...).try_id(…)`
  // / `ui::field(...).try_id(…)` — puzzle 3d's whole Settings panel does — rendered a tree whose only
  // addressable node was the innermost control, with no row or section to reach from a keybinding, an
  // introduction anchor, a scripted driver or assistive technology (ticket 26/09/02 wave B12).
  if (component.role === "section" || component.role === "group") {
    return (
      <Section id={nodeDomId(store, record, context.domScope)} title={component.label ? wireLabel(component.label) : undefined} className={cn(presence.selected && "ring-primary ring-1")}>
        {describedBy}
        {children}
      </Section>
    );
  }
  if (component.role === "field") {
    return (
      <Field id={nodeDomId(store, record, context.domScope)} label={component.label ? wireLabel(component.label) : ""} description={component.description ?? undefined} required={component.required ?? undefined} error={component.error ?? undefined}>
        {describedBy}
        {children}
      </Field>
    );
  }
  return (
    <div
      style={style}
      {...dataAttrs}
      {...aria}
      role={activateBinding ? "button" : role}
      id={nodeDomId(store, record, context.domScope)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      data-activity={record.activity}
      className={cn(activityBorderClass(record), activateBinding && cn(borderElementClass, "border cursor-pointer rounded-md"), presence.selected && "ring-primary ring-1", presence.hovered && "outline-primary/50 outline-1")}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      onClick={activateBinding ? (event) => { event.stopPropagation(); dispatchTrigger(context, record, "activate"); } : undefined}
    >
      {describedBy}
      {children}
    </div>
  );
}

//#region Tree
/** 🌲️ One tree row's inline controls, reconciled on {@link uiSiblingReactKeys} like every other
 * sibling run — a row's control renumbers with the body exactly as its owning surface does. */
function renderTreeItemControls(store: UiDocumentStore, controls: readonly UiNodeRecord[], context: UiInterpreterContext): readonly ReactElement[] {
  const keys = uiSiblingReactKeys(controls);
  return controls.map((child, index) => <UiNodeView key={keys[index]} store={store} id={child.id} context={context} />);
}

export function treeItemToTreeData(store: UiDocumentStore, state: UiDocumentState, node: TreeWalkNode, context: UiInterpreterContext, overlay: UiPresenceOverlayValue, leftoverIds?: readonly string[], walk?: TreeWalkContextV1, parentWindowPath?: string, rowAdmission?: RowActionAdmissionControllerV1): TreeDataItem {
  const { record, props } = node;
  // 🪟️ A WINDOW is identified by its path; an unwindowed row has no window identity and keeps its authored
  // key, which is the only thing the host ever holds for it (a fold).
  const windowPath = props.window ? treeWindowPathOf(parentWindowPath, record.key) : undefined;
  // 🔑️ …and the window's ROW id is the path too, because `🌳️Tree` keys its expansion map by row id and the
  // authored key is legitimately shared across a body (`📐️cad`'s one `object.id` under four pane sections):
  // two rows under one id would fold together. Identical to the key for a top-level container, so nothing
  // flat changes; `walk.pickTargets` still resolves this id back to the bare `record.key`, so picks do not
  // notice (📓️f2-sdk-body-node-ledger.md §10).
  const domId = uiNodeDomId(state.surface, windowPath ?? record.key, record.id);
  registerTreeWalkRow(walk, windowPath ?? record.key, domId);
  const presence = overlay.byKey.get(record.key) ?? {};
  const activateBinding = rowActivationBinding(record);
  const hoverBinding = (record.bindings ?? []).find((b) => b.trigger === "hoverPreview");
  const childItems = collectTreeItems(state, record.children ?? []);
  const controlRecords = collectTreeItemControls(state, record.children ?? []);
  const activatableControl = activateBinding ? undefined : controlRecords.find((child) => !child.disabled && (child.bindings ?? []).some((b) => b.trigger === "activate") && treeRowStandsForControl(child, props.label));
  const granularity = typeof props.granularity === "string" && props.granularity.length > 0 ? props.granularity : undefined;
  if (granularity && walk?.pick) walk.pickTargets?.set(domId, { key: record.key, granularity });
  const pickClick = !activateBinding && granularity && walk?.pick ? (event: ReactMouseEvent, activation: TreeDataActivationContext) => dispatchTreePick(context, walk, record.key, granularity, event, activation) : undefined;
  return {
    id: domId,
    window: props.window ?? undefined,
    windowKey: record.key || undefined,
    windowPath,
    label: props.label,
    className: record.style.tone === undefined ? undefined : TREE_ROW_TONE_CLASSES[record.style.tone],
    description: [props.description, ...(presence.notes ?? [])].filter((line): line is string => typeof line === "string" && line !== "").join(" · ") || undefined,
    icon: props.icon ? resolveControlIconNode(props.icon, 12) : undefined,
    defaultOpen: props.defaultOpen ?? undefined,
    isSelected: props.selected === true || Boolean(presence.selected) || leftoverItemSelectedV1(record.key, leftoverIds),
    isHighlighted: presence.hovered === true || presence.previewed === true ? true : undefined,
    loading: record.activity === "loading",
    waiting: record.activity === "waiting",
    isHidden: props.dimmed ?? undefined,
    draggable: props.draggable ?? undefined,
    dragData: props.dragData ? (Object.fromEntries(Object.entries(props.dragData).filter((entry): entry is [string, string] => entry[1] !== undefined)) as Record<string, string>) : undefined,
    control: controlRecords.length > 0 && controlRecords.length !== (activatableControl ? 1 : 0) ? <>{renderTreeItemControls(store, controlRecords.filter((child) => child !== activatableControl), { ...context, domScope: domId })}</> : undefined,
    items: childItems.length > 0 ? childItems.map((child) => treeItemToTreeData(store, state, child, context, overlay, leftoverIds, walk, windowPath ?? parentWindowPath, rowAdmission)) : undefined,
    onClick: activateBinding ? () => dispatchTrigger(context, record, "activate") : (pickClick ?? (activatableControl ? () => dispatchTrigger(context, activatableControl, "activate") : undefined)),
    onPointerEnter: hoverBinding ? () => dispatchTrigger(context, record, "hoverPreview") : undefined,
    actions: rowTreeActions(record, props, context, rowAdmission),
  };
}

function TreeView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const revision = useUiDocumentRevision(store);
  const rowAdmission = useRowActionAdmissionV1(context);
  const overlay = useUiPresenceOverlay();
  const leftover = useSyncExternalStore(subscribeLeftoverSelectionV1, leftoverSelectionOverlayV1, leftoverSelectionOverlayV1);
  const leftoverIds = leftover?.ids;
  const windows = useTreeWindowContext();
  const rootRef = useRef<HTMLDivElement | null>(null);
  const walked = useMemo((): { readonly sections: TreeDataSection[]; readonly walk: TreeWalkContextV1 } => {
    void revision;
    const state = store.getState();
    const rootActivate = (record.bindings ?? []).find((b) => b.trigger === "activate");
    const walk: TreeWalkContextV1 = {
      openStates: windows?.openStates,
      domOpenStates: {},
      keysByDomId: new Map<string, string>(),
      pick: rootActivate ? { record, binding: rootActivate } : undefined,
      pickTargets: new Map<string, { readonly key: string; readonly granularity: string }>(),
    };
    const sectionRecords = (record.children ?? []).map((id) => state.nodes.get(id)).filter((r): r is UiNodeRecord => !!r && r.component.type === "treeSection");
    const sections = sectionRecords.map((sectionRecord) => {
      const sectionProps = sectionRecord.component as Extract<Component, { type: "treeSection" }>;
      const items = collectTreeItems(state, sectionRecord.children ?? []);
      const domId = uiNodeDomId(state.surface, sectionRecord.key, sectionRecord.id);
      // 🪟️ A section is top-level, so its window path IS its key — a flat body is unchanged by path identity.
      const windowPath = sectionProps.window ? treeWindowPathOf(undefined, sectionRecord.key) : undefined;
      registerTreeWalkRow(walk, windowPath ?? sectionRecord.key, domId);
      return {
        id: domId,
        window: sectionProps.window ?? undefined,
        windowKey: sectionRecord.key || undefined,
        windowPath,
        label: sectionProps.label ?? "",
        defaultOpen: sectionProps.defaultOpen ?? undefined,
        loading: sectionRecord.activity === "loading",
        waiting: sectionRecord.activity === "waiting",
        items: items.map((item) => treeItemToTreeData(store, state, item, context, overlay, leftoverIds, walk, windowPath, rowAdmission)),
      };
    });
    return { sections, walk };
  }, [store, record, revision, context, overlay, leftoverIds, windows, rowAdmission]);
  const sections = walked.sections;
  // 🔑️ `TreeSection`/`TreeItem` each route a fold through the provider TWICE — once from the data view
  // that owns the controlled `open` prop and once from the row component's own `useTreeOpenState`, both
  // under the same state id. The host's map is idempotent (`createTreeWindowSchedulerV1.setOpen` returns
  // on an unchanged value), so the duplicate costs nothing and is deliberately not filtered here.
  const handleOpenStateChange = useCallback((stateId: string, open: boolean) => windows?.setOpen(walked.walk.keysByDomId?.get(treeDomIdFromOpenStateIdV1(stateId)) ?? stateId, open), [windows, walked]);
  useTreeWindowObserver(rootRef, windows, revision, store);
  const dragController: TreeDragAndDropController | undefined = useMemo(() => {
    const dropBinding = (record.bindings ?? []).find((b) => b.trigger === "drop");
    const catalogueMime = treeSectionsCatalogueDragMime(sections);
    const catalogue = catalogueMime ? catalogueTreeDragController(catalogueMime) : undefined;
    if (!dropBinding && !catalogue) return undefined;
    return {
      ...(catalogue ?? {}),
      ...(dropBinding ? { handleDrop: () => dispatchTrigger(context, record, "drop")?.then(() => undefined) } : {}),
    };
  }, [record, context, sections]);
  const ownsScroll = record.layout.kind === "scroll";
  return (
    <div ref={rootRef} data-ui-node-id={record.id} data-ui-node-key={record.key} className={ownsScroll ? "min-h-0 min-w-0 flex-1" : "contents"} style={ownsScroll ? layoutSpecStyle(record.layout) : undefined}>
      <Tree
        className={ownsScroll ? "min-h-0 min-w-0" : "min-h-0 min-w-0 flex-1 overflow-auto"}
        presentation={(record.component as Extract<Component, { type: "tree" }>).presentation}
        sections={sections.length > 0 ? sections : [treeStatusSection(store, record)]}
        selectionMode="single"
        showLines
        dragAndDropController={dragController}
        sortableSections={false}
        {...(windows ? { openStates: walked.walk.domOpenStates, onOpenStateChange: handleOpenStateChange } : {})}
      />
    </div>
  );
}
//#endregion Tree
//#endregion ComponentRenderers

//#region 📊️TableWindow
/** 📊️ One `Component::Table`: a keyboard- and screen-reader-accessible grid whose rows are the host's
 * WINDOW of a logically `window.total`-long row list. It speaks the tree windows' own streaming protocol —
 * the same `data-tree-window-*` stamps, the same spacers, the same {@link useTreeWindowObserver} — so the
 * body's one scheduler asks the guest for exactly the rows the viewport shows, on the same node ledger.
 *
 * ⌨️ One tab stop (the active row). Up/Down, PageUp/PageDown and Home/End move it over the whole logical
 * extent, scrolling rows the host has not streamed yet into view and focusing them the moment they arrive;
 * Enter/Space fire the row's own activation; Right/Left walk the row's actions; Escape returns to the row. */
function TableView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const revision = useUiDocumentRevision(store);
  const rowAdmission = useRowActionAdmissionV1(context);
  const windows = useTreeWindowContext();
  const rootRef = useRef<HTMLDivElement | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const focusRowRef = useRef<number | null>(null);
  const focusCellRef = useRef<{ readonly row: number; readonly column: number } | null>(null);
  const component = record.component as Extract<Component, { type: "table" }>;
  const rows = useMemo(() => {
    void revision;
    const state = store.getState();
    return (record.children ?? []).map((id) => state.nodes.get(id)).filter((row): row is UiNodeRecord => !!row && row.component.type === "tableRow");
  }, [store, record, revision]);
  const window = component.window ?? undefined;
  const total = window ? Math.max(rows.length, Math.floor(window.total)) : rows.length;
  const { leading, trailing } = treeWindowSpacerRows(window ?? { total, offset: 0, rowExtent: "standard" }, rows.length);
  const columnWindow = component.columnWindow ?? undefined;
  const columnTotal = columnWindow ? Math.max(component.columns.length, Math.floor(columnWindow.total)) : component.columns.length;
  const { leading: columnLeading, trailing: columnTrailing } = treeWindowSpacerRows(columnWindow ?? { total: columnTotal, offset: 0, rowExtent: "standard" }, component.columns.length);
  const rowPx = treeRowHeightPx;
  const actionColumns = rows.reduce((widest, row) => Math.max(widest, ((row.component as Extract<Component, { type: "tableRow" }>).rowActions ?? []).length), 0);
  const hasActions = actionColumns > 0;
  const template = tableWindowColumnTemplate(component.columns.length, actionColumns, columnLeading, columnTrailing);
  const [active, setActive] = useState(0);
  const activeRow = active >= leading && active < leading + rows.length ? active : leading;
  const range = useLabel("ui.host.tableRowRange", { from: rows.length > 0 ? leading + 1 : 0, to: leading + rows.length, total });
  const [servableRows, setServableRows] = useState<number | null>(null);
  const viewportCapRows = tableWindowViewportCapRowsV1(null, servableRows, total);
  const rowRange = component.rowLabel ? `${component.rowLabel}: ${rows.length > 0 ? leading + 1 : 0}–${leading + rows.length} / ${total}` : range;
  const columnRange = `${component.columnLabel ? `${component.columnLabel}: ` : ""}${component.columns.length > 0 ? columnLeading + 1 : 0}–${columnLeading + component.columns.length} / ${columnTotal}`;
  useTreeWindowObserver(rootRef, windows, revision, store, (memory) => {
    const learned = memory.get(record.key)?.capacity ?? null;
    if (learned !== null) setServableRows((current) => (current === learned ? current : learned));
  });
  useEffect(() => {
    const wanted = focusRowRef.current;
    if (wanted === null || wanted < leading || wanted >= leading + rows.length) return;
    const element = rootRef.current?.querySelector<HTMLElement>(`[data-table-row-index="${wanted}"]`);
    if (!element) return;
    focusRowRef.current = null;
    element.focus({ preventScroll: true });
  }, [leading, rows]);
  useEffect(() => {
    const wanted = focusCellRef.current;
    if (!wanted || wanted.column < columnLeading || wanted.column >= columnLeading + component.columns.length) return;
    const row = rootRef.current?.querySelector<HTMLElement>(`[data-table-row-index="${wanted.row}"]`);
    const cell = row?.querySelector<HTMLElement>(`[data-table-column-index="${wanted.column}"]`);
    const target = cell?.querySelector<HTMLElement>("input, textarea, select, button, [contenteditable='true'], [tabindex]") ?? cell;
    if (!target) return;
    focusCellRef.current = null;
    target.focus({ preventScroll: true });
  }, [columnLeading, component.columns.length, rows]);
  const moveTo = (index: number) => {
    setActive(index);
    focusRowRef.current = index;
    const scroller = scrollRef.current;
    if (scroller) scroller.scrollTop = tableWindowScrollTopForRowV1(index, rowPx, scroller.scrollTop, scroller.clientHeight - rowPx);
    const element = rootRef.current?.querySelector<HTMLElement>(`[data-table-row-index="${index}"]`);
    if (element) {
      focusRowRef.current = null;
      element.focus({ preventScroll: true });
    }
  };
  const activateRow = (row: UiNodeRecord) => {
    if (rowActivationBinding(row)) {
      void dispatchTrigger(context, row, "activate");
    }
  };
  const moveCell = (row: number, column: number) => {
    focusCellRef.current = { row, column };
    const scroller = scrollRef.current;
    if (scroller) scroller.scrollLeft = tableWindowScrollLeftForColumnV1(column, TABLE_COLUMN_WINDOW_WIDTH_PX, scroller.scrollLeft, scroller.clientWidth);
    const cell = rootRef.current?.querySelector<HTMLElement>(`[data-table-row-index="${row}"] [data-table-column-index="${column}"]`);
    const target = cell?.querySelector<HTMLElement>("input, textarea, select, button, [contenteditable='true'], [tabindex]") ?? cell;
    if (target) {
      focusCellRef.current = null;
      target.focus({ preventScroll: true });
    }
  };
  const onRowKeyDown = (event: import("react").KeyboardEvent<HTMLDivElement>, row: UiNodeRecord, index: number) => {
    if (event.target !== event.currentTarget) {
      const cell = (event.target as HTMLElement).closest<HTMLElement>("[data-table-column-index]");
      if (cell && (event.ctrlKey || event.metaKey)) {
        const current = Number(cell.getAttribute("data-table-column-index"));
        const next = tableWindowNextColumnV1(event.key, current, columnTotal);
        if (next !== null) {
          event.preventDefault();
          moveCell(index, next);
          return;
        }
      }
      const buttons = Array.from(event.currentTarget.querySelectorAll<HTMLElement>("[data-table-row-action]"));
      const position = buttons.indexOf(event.target as HTMLElement);
      if (event.key === "ArrowRight" && position >= 0 && position < buttons.length - 1) buttons[position + 1]!.focus();
      else if (event.key === "ArrowLeft" && position > 0) buttons[position - 1]!.focus();
      else if (event.key === "Escape" || (event.key === "ArrowLeft" && position === 0)) event.currentTarget.focus();
      else return;
      event.preventDefault();
      return;
    }
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      activateRow(row);
      return;
    }
    if (event.key === "ArrowRight") {
      const first = event.currentTarget.querySelector<HTMLElement>("[data-table-column-index] input, [data-table-column-index] textarea, [data-table-column-index] select, [data-table-column-index] button, [data-table-column-index] [contenteditable='true'], [data-table-column-index][tabindex]") ?? event.currentTarget.querySelector<HTMLElement>("[data-table-row-action]");
      if (first) {
        event.preventDefault();
        first.focus();
      }
      return;
    }
    const page = Math.max(1, Math.floor(((scrollRef.current?.clientHeight ?? 2 * rowPx) - rowPx) / rowPx));
    const next = tableWindowNextRowV1(event.key, index, total, page);
    if (next === null) return;
    event.preventDefault();
    moveTo(next);
  };
  const cellClass = "flex min-w-0 items-center truncate px-single";
  return (
    <div
      ref={rootRef}
      id={nodeDomId(store, record, context.domScope)}
      data-ui-node-id={record.id}
      data-ui-node-key={record.key}
      role="grid"
      aria-label={component.label}
      aria-rowcount={total + 1}
      aria-colcount={columnTotal + (hasActions ? 1 : 0)}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      className={cn("flex min-h-0 min-w-0 flex-1 flex-col", activityBorderClass(record))}
      style={layoutSpecStyle(record.layout)}
    >
      <div ref={scrollRef} role="rowgroup" data-slot="table-window-scroll" data-table-viewport-cap={viewportCapRows ?? undefined} className="min-h-0 min-w-0 flex-1 overflow-auto" style={{ scrollPaddingTop: rowPx, maxHeight: viewportCapRows === null ? undefined : (viewportCapRows + 1) * rowPx }}>
        <div
          {...(treeWindowDomAttributes(columnWindow, component.columns.length, `${record.key}.columns`) ?? {})}
          data-tree-window-axis={columnWindow ? "column" : undefined}
          className="min-w-0"
          style={{ width: columnWindow ? columnTotal * TABLE_COLUMN_WINDOW_WIDTH_PX : undefined }}
        >
          <div role="row" aria-rowindex={1} className={cn("sticky top-0 z-10 grid min-w-0 bg-background text-2xs font-semibold uppercase tracking-wide text-muted-foreground", borderNormalTopClass)} style={{ gridTemplateColumns: template, height: rowPx }}>
            {columnLeading > 0 ? <div aria-hidden="true" data-table-column-spacer="leading" /> : null}
            {component.columns.map((column, position) => {
              const logicalColumn = columnLeading + position;
              return (
                <div key={logicalColumn} role="columnheader" aria-colindex={logicalColumn + 1} data-tree-window-column={logicalColumn} data-table-column-index={logicalColumn} className={cellClass}>
                  {column}
                </div>
              );
            })}
            {hasActions ? (
              <div role="columnheader" aria-colindex={columnTotal + 1} className={cellClass}>
                {component.actionsLabel ?? ""}
              </div>
            ) : null}
            {columnTrailing > 0 ? <div aria-hidden="true" data-table-column-spacer="trailing" /> : null}
          </div>
          <div {...(treeWindowDomAttributes(window, rows.length, record.key) ?? {})} className="min-w-0">
            {leading > 0 ? <div aria-hidden="true" data-tree-window-spacer="leading" style={{ height: leading * rowPx }} /> : null}
            {rows.map((row, position) => {
              const index = leading + position;
              const props = row.component as Extract<Component, { type: "tableRow" }>;
              const cellNodes = row.children ?? [];
              const name = props.cells[0] ?? row.key;
              const rowDomId = nodeDomId(store, row, context.domScope);
              const cellContext: UiInterpreterContext = { ...context, domScope: rowDomId };
              return (
                <div
                  key={row.key}
                  id={rowDomId}
                  data-ui-node-id={row.id}
                  data-ui-node-key={row.key}
                  data-tree-window-row={index}
                  data-table-row-index={index}
                  role="row"
                  aria-rowindex={index + 2}
                  tabIndex={index === activeRow ? 0 : -1}
                  onFocus={(event) => {
                    if (event.target === event.currentTarget) setActive(index);
                  }}
                  onClick={(event) => {
                    if (event.target === event.currentTarget) {
                      moveTo(index);
                      return;
                    }
                    if ((event.target as HTMLElement).closest("input, textarea, select, button, [contenteditable='true'], [role='button'], [role='textbox']")) return;
                    moveTo(index);
                  }}
                  onDoubleClick={(event) => {
                    if (!(event.target as HTMLElement).closest("input, textarea, select, button, [contenteditable='true'], [role='button'], [role='textbox']")) activateRow(row);
                  }}
                  onKeyDown={(event) => onRowKeyDown(event, row, index)}
                  className="grid min-w-0 cursor-default border-b border-border/40 text-xs outline-none hover:bg-muted/40 focus-visible:bg-muted/60 focus-visible:ring-1 focus-visible:ring-primary"
                  style={{ gridTemplateColumns: template, minHeight: rowPx, height: cellNodes.length > 0 ? undefined : rowPx }}
                >
                  {columnLeading > 0 ? <div aria-hidden="true" data-table-column-spacer="leading" /> : null}
                  {component.columns.map((column, cell) => {
                    const logicalColumn = columnLeading + cell;
                    return (
                      <div key={logicalColumn} role="gridcell" aria-colindex={logicalColumn + 1} data-table-column-index={logicalColumn} className={cellClass} title={cellNodes[cell] ? undefined : (props.cells[cell] ?? "")}>
                        {cellNodes[cell] ? <UiNodeView store={store} id={cellNodes[cell]!} context={cellContext} /> : (props.cells[cell] ?? "")}
                      </div>
                    );
                  })}
                  {hasActions ? (
                    <div role="gridcell" aria-colindex={columnTotal + 1} className="flex min-w-0 items-center gap-single px-single">
                      {(props.rowActions ?? []).map((action, actionIndex) => (
                        <TableRowActionButton key={actionIndex} action={action} name={name} busy={props.target ? rowAdmission.isPending(props.target, action) : false} unavailable={!rowAdmission.active} refused={rowAdmission.refusedBy(action)} onRun={() => rowAdmission.dispatch(row.id, row.key, actionIndex, rowActionAdmissionKeyV1(props.target!, action))} />
                      ))}
                    </div>
                  ) : null}
                  {columnTrailing > 0 ? <div aria-hidden="true" data-table-column-spacer="trailing" /> : null}
                </div>
              );
            })}
            {trailing > 0 ? <div aria-hidden="true" data-tree-window-spacer="trailing" style={{ height: trailing * rowPx }} /> : null}
          </div>
        </div>
      </div>
      <div role="status" aria-live="polite" className="shrink-0 px-single text-2xs text-muted-foreground">
        {rowRange} · {columnRange}
      </div>
    </div>
  );
}
//#endregion 📊️TableWindow

/** 🌳️ Renders one record's component, recursing into children via {@link UiNodeView} — never reads a
 * child's own record directly, only its id, so a child's change never re-renders this switch. */
function renderComponent(store: UiDocumentStore, record: UiNodeRecord, context: UiInterpreterContext): ReactNode {
  const busyShell = interpretUiNodeBusyShell(record);
  if (busyShell) return busyShell;
  switch (record.component.type) {
    case "container":
      return <ContainerView store={store} record={record} context={context} />;
    case "text":
      return <TextView record={record} />;
    case "button":
      return <ButtonView record={record} context={context} />;
    case "separator":
      return <hr className={cn("border-0", borderNormalTopClass)} data-ui-node-id={record.id} data-ui-node-key={record.key} />;
    case "input":
      return <InputView record={record} context={context} />;
    case "select":
      return <SelectView record={record} context={context} />;
    case "toggle":
      return <ToggleView record={record} context={context} />;
    case "keyValueList":
      return <KeyValueListView record={record} />;
    case "slider":
      return <SliderView record={record} context={context} />;
    case "numberStepper":
      return <NumberStepperView record={record} context={context} />;
    case "ring":
      return <RingView record={record} context={context} />;
    case "iconSelect":
      return <IconSelectView record={record} context={context} />;
    case "progress":
      return <ProgressView record={record} />;
    case "tree":
      return <TreeView store={store} record={record} context={context} />;
    case "treeSection":
    case "treeItem":
      // 🌲️ Reached only when a section/item is rendered OUTSIDE a `tree` parent (malformed document) —
      // `TreeView` walks these directly, never through `UiNodeView`, in the well-formed case.
      return <ContainerView store={store} record={record} context={context} />;
    case "image":
      return <ImageView record={record} />;
    case "surface":
      return <SurfaceView record={record} context={context} />;
    case "extension":
      return <ExtensionView record={record} />;
    case "table":
      return <TableView store={store} record={record} context={context} />;
    default:
      return <UnknownComponentView record={record} />;
  }
}

/** 🌳️ The atomic per-node subscribing unit — reads exactly `id`'s own record via `useUiNode`, so it
 * re-renders when (and only when) THAT record changes. */
export function UiNodeView({ store, id, context }: { readonly store: UiDocumentStore; readonly id: UiNodeId; readonly context: UiInterpreterContext }): ReactElement | null {
  const record = useUiNode(store, id);
  if (!record) return null;
  return <>{renderComponent(store, record, context)}</>;
}
//#endregion 🔖️UiInterpreter
