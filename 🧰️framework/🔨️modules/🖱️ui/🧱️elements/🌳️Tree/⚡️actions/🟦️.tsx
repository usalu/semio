import type { ControlIcon } from "../../🔣️Icons/🟦️.tsx";
import { type UiLabel, uiDataLabel } from "../../🎗️UiLabel/🟦️.tsx";
import type { ContextMenuItem } from "../../🖱️ContextMenu/🎨️item-render/🟦️.tsx";
import * as React from "react";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { Action } from "../../⚡️ActionGroup/🟦️.tsx";
import { DisabledReasonHint } from "../../💡️ChromeControlHint/🟦️.tsx";

/** ⚡️ Layout of the section header action cluster. */
export const treeHeaderActionsClassName = "flex flex-shrink-0 items-center gap-single";

export type TreeActionPlacement = "row" | "menu";

/**
 * 🌳️ Configuration interface for an action button on a tree section.
 **/
export interface TreeSectionAction {
  kind?: "button";
  icon: ControlIcon;
  onClick: () => void;
  title?: UiLabel;
  text?: string;
  id?: string;
  /** 🚫️ A disabled action never runs; its explicit reason keeps it focusable for discovery. */
  disabled?: boolean;
  busy?: boolean;
  /** 💬️ Why a disabled action cannot run: its description (`aria-describedby`) and part of its tooltip. */
  reason?: string;
  /** 📍️ Row actions paint on the header; menu actions appear in the row context menu. */
  placement?: TreeActionPlacement;
}

/**
 * 🌲️ Configuration interface for a checkbox action on a tree header row.
 **/
export interface TreeCheckboxAction {
  kind: "checkbox";
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  title?: UiLabel;
  id?: string;
  disabled?: boolean;
  ariaLabel?: string;
}

export type TreeHeaderAction = TreeSectionAction | TreeCheckboxAction;

export type TreeDragRole = "sort" | "transfer";

/** 📮️ DOM mirror of a row's transfer payload — the first (and for every catalogue/palette row, the
 * only) MIME key and its bytes, exactly what a real `dragstart` would put on the `DataTransfer`. Same
 * principle as the world host's `data-instances-json`: a native HTML5 drag cannot be driven by synthetic
 * pointer moves, so without this the payload is unreadable outside a live OS drag. */
export function treeRowDragPayloadAttributes(dragData: Record<string, string> | undefined): { readonly "data-drag-mime"?: string; readonly "data-drag-payload"?: string } {
  const entry = Object.entries(dragData ?? {}).find(([, value]) => typeof value === "string" && value.length > 0);
  return entry ? { "data-drag-mime": entry[0], "data-drag-payload": entry[1] } : {};
}

/** 🫳️ Which drag handles a tree row exposes under the default driver. */
export function deriveTreeDragRoles(item: { readonly draggable?: boolean; readonly dragData?: Record<string, string>; readonly isDragHandle?: boolean }, paletteDragEnabled: boolean): readonly TreeDragRole[] {
  const roles: TreeDragRole[] = [];
  const hasTransfer = Boolean(item.dragData) || (paletteDragEnabled && (item.draggable || item.dragData));
  const hasSort = Boolean(item.isDragHandle) || Boolean(item.draggable && !item.dragData);
  if (hasSort) roles.push("sort");
  if (hasTransfer) roles.push("transfer");
  return roles;
}

function treeSectionActionPlacement(action: TreeSectionAction): TreeActionPlacement {
  return action.placement ?? "row";
}

function rowTreeHeaderActions(actions: readonly TreeHeaderAction[]): TreeHeaderAction[] {
  return actions.filter((action) => action.kind === "checkbox" || treeSectionActionPlacement(action) === "row");
}

function menuTreeHeaderActionsToContextItems(actions: readonly TreeHeaderAction[]): ContextMenuItem[] {
  return actions
    .filter((action): action is TreeSectionAction => action.kind !== "checkbox" && treeSectionActionPlacement(action) === "menu")
    .map((action, index) => ({
      id: action.id ?? `tree-row-action-${index}`,
      label: action.text !== undefined ? uiDataLabel(action.text) : (action.title ?? uiDataLabel("")),
      icon: action.icon,
      disabled: action.disabled || action.busy,
      onSelect: () => action.onClick(),
    }));
}

/** 🖱️ Merges menu-placement row actions into a host-built context menu. */
export function mergeTreeRowContextMenu(actions: readonly TreeHeaderAction[] | undefined, contextMenu: readonly ContextMenuItem[] | undefined): ContextMenuItem[] | undefined {
  const fromActions = menuTreeHeaderActionsToContextItems(actions ?? []);
  if (fromActions.length === 0) {
    return contextMenu?.length ? [...contextMenu] : undefined;
  }
  if (!contextMenu?.length) {
    return fromActions;
  }
  return [...contextMenu, { id: "tree-row-action-separator", separator: true }, ...fromActions];
}

type TreeDragHandleRenderProps = {
  readonly roles: readonly TreeDragRole[];
  readonly driverSurfaceDrag: boolean;
  readonly rowEmphasized: boolean;
  readonly armDrag?: () => void;
  readonly sortDndProps?: { readonly attributes?: object; readonly listeners?: Record<string, unknown> };
  readonly transferPointerDown?: React.PointerEventHandler<HTMLSpanElement>;
  readonly onSortHandleClick?: React.MouseEventHandler<HTMLSpanElement>;
};

export const renderTreeDragHandles = ({ roles, driverSurfaceDrag, rowEmphasized, armDrag, sortDndProps, transferPointerDown, onSortHandleClick }: TreeDragHandleRenderProps): React.ReactNode => {
  if (driverSurfaceDrag || roles.length === 0) {
    return null;
  }
  return (
    <>
      {roles.includes("sort") ? (
        <DragHandle
          labelId="ui.tree.drag.sort"
          iconKind="grip-vertical"
          emphasized={rowEmphasized}
          onPointerDown={armDrag ? () => armDrag() : undefined}
          attributes={sortDndProps?.attributes}
          listeners={sortDndProps?.listeners}
          onClick={onSortHandleClick}
        />
      ) : null}
      {roles.includes("transfer") ? (
        <DragHandle
          labelId="ui.tree.drag.transfer"
          iconKind="move"
          emphasized={rowEmphasized}
          onPointerDown={(event) => {
            armDrag?.();
            transferPointerDown?.(event);
          }}
        />
      ) : null}
    </>
  );
};

export interface TreeCheckboxProps {
  id?: string;
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  title?: UiLabel;
  disabled?: boolean;
  ariaLabel?: string;
  /** 🕰️ The value the state's AUTHORITY last published, when `checked` may still be an optimistic
   * draft — stamped as `data-published-value` so a reader outside React can tell a pending draft from a
   * landed change. See `WindowMeasureToggle`. */
  publishedValue?: string;
}

/**
 * ☑️ Compact tree-row checkbox used as a property control or header action.
 *
 * The wrapper only stops the click from reaching the enclosing tree row; it must never
 * `preventDefault()` it. Cancelling a checkbox's click runs the HTML "legacy-canceled activation
 * behavior", which reverts the checkedness the pre-click steps had already applied and suppresses the
 * native `change` event — so both a pointer click and the keyboard Space key left every window-option
 * toggle (grid visible/snap, LOD automatic, selectable kinds) visually stuck on its old value.
 *
 * @see https://html.spec.whatwg.org/multipage/input.html#checkbox-state-(type=checkbox)
 */
export const TreeCheckbox: React.FC<TreeCheckboxProps> = ({ id, checked, onCheckedChange, title, disabled, ariaLabel, publishedValue }) => (
  <label data-slot="tree-action-checkbox-wrapper" className="inline-flex h-medium min-w-tiny flex-shrink-0 cursor-pointer items-center justify-center" title={title} onClick={(event) => event.stopPropagation()}>
    <input
      data-slot="tree-action-checkbox"
      id={id}
      data-published-value={publishedValue}
      type="checkbox"
      className="m-0 size-tiny cursor-pointer accent-foreground"
      aria-label={ariaLabel ?? title ?? id}
      checked={checked}
      disabled={disabled}
      onChange={(event) => {
        event.stopPropagation();
        if (disabled) return;
        onCheckedChange(event.currentTarget.checked);
      }}
    />
  </label>
);

/** 🎯️ What a row's control exposes to `aria-describedby`: every focusable element and widget role inside it. */
const TREE_ROW_DESCRIBED_CONTROLS = 'input, select, textarea, button, [tabindex], [role="slider"], [role="spinbutton"], [role="combobox"], [role="switch"], [role="checkbox"]';

/** ℹ️ Names `descriptionId` in the `aria-describedby` of every focusable control inside `slot` — kept there across the
 * control's own re-renders (a refusal that sets or clears its own `aria-describedby`), so a property row's description is
 * always announced with its control, whatever element the control renders. */
export function useTreeRowDescribedControls(slot: React.RefObject<HTMLElement | null>, descriptionId: string | undefined): void {
  reactHostPort.useLayoutEffect(() => {
    const root = slot.current;
    if (!root || !descriptionId) return;
    const describe = () => {
      for (const element of root.querySelectorAll<HTMLElement>(TREE_ROW_DESCRIBED_CONTROLS)) {
        const ids = (element.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean);
        if (!ids.includes(descriptionId)) element.setAttribute("aria-describedby", [...ids, descriptionId].join(" "));
      }
    };
    describe();
    const observer = new MutationObserver(describe);
    observer.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-describedby"] });
    return () => observer.disconnect();
  }, [descriptionId, slot]);
}

/** 🎬️ A row action with an explicit disabled reason stays focusable, names that reason through `aria-describedby` and shows
 * it as visible text while hovered, focused or pressed ({@link DisabledReasonHint}); a disabled action without a reason is
 * unreachable (the shared corpus `♿️disabled-row-action`). A busy action — its own dispatch still pending — is never taken
 * out of the Tab order: it is `aria-disabled` and `aria-busy`, keeps the focus the press gave it, runs nothing, and still
 * names the reason its producer published (live fault F3: the native `disabled` attribute dropped focus and the reason).
 * The wgpu renderer paints and mirrors the same semantics (conformance case `💬️row-semantics`). */
function TreeRowActionButton({ action }: { readonly action: TreeSectionAction }): React.ReactElement {
  const reasonId = `${reactHostPort.useId()}-reason`;
  const reason = action.disabled ? action.reason : undefined;
  const disabled = action.disabled || action.busy;
  const button = (
    <Action
      onClick={(event) => {
        event.preventDefault();
        event.stopPropagation();
        if (!disabled) action.onClick();
      }}
      id={action.id}
      icon={action.icon}
      text={action.text ?? action.title}
      aria-disabled={disabled ? true : undefined}
      aria-busy={action.busy || undefined}
      disabled={!action.busy && action.disabled && !reason}
      aria-describedby={reason ? reasonId : undefined}
      {...(reason ? { title: undefined } : {})}
      className={disabled ? "cursor-not-allowed opacity-50" : undefined}
    />
  );
  return (
    <span>
      <DisabledReasonHint id={reasonId} reason={reason || undefined}>
        {button}
      </DisabledReasonHint>
    </span>
  );
}

const renderTreeHeaderAction = (action: TreeHeaderAction, key: React.Key) =>
  action.kind === "checkbox" ? (
    <TreeCheckbox key={key} id={action.id} checked={action.checked} onCheckedChange={action.onCheckedChange} title={action.title} disabled={action.disabled} ariaLabel={action.ariaLabel} />
  ) : (
    <TreeRowActionButton key={key} action={action} />
  );

export const renderTreeHeaderActions = (actions: TreeHeaderAction[]) => {
  const rowActions = rowTreeHeaderActions(actions);
  if (rowActions.length === 0) {
    return null;
  }
  return (
    <div data-slot="tree-header-actions" data-ui-reveal-region="" className={treeHeaderActionsClassName}>
      {rowActions.map((action, index) => renderTreeHeaderAction(action, action.id ?? index))}
    </div>
  );
};
