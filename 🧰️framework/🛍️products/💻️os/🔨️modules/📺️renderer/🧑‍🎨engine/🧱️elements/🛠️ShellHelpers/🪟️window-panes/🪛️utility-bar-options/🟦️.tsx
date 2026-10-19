/** 🪛️ Window panes, utility bar options: the options of the selection utility and the utility bar node. */
import { type ActionDescriptor, type MergeMode, SET_ACTIVE_UTILITY_ACTION_ID, type UtilityNode } from "@semio-tech/framework";
import { Icon, RibbonDivider, ToggleGroup, useLabel, useShellScope } from "@semio-tech/ui-react";
import { type ReactNode, useState } from "react";
import { groupUtilityNodesByCategory, UTILITY_CATEGORIES, UtilityTree } from "../../../🎛️UtilityTree/🟦️.tsx";

//#region ShellHelpers
/** 🔢️ `generation` is the window's utility register generation this picker rendered — stamped as `expectedGeneration` on its `setActiveUtility` press (`🎯️input-ledger` L3); absent, the press is an unconditional write (a legacy call site). */
export function SelectionUtilityOptions({ activeUtilityId, windowId, onAction, generation }: { readonly activeUtilityId: string | undefined; readonly windowId: string; readonly onAction: (action: ActionDescriptor) => void; readonly generation?: number }) {
  const methodLabel = useLabel("ui.selection.method");
  const modeLabel = useLabel("ui.selection.mode");
  const rectangleLabel = useLabel("ui.selection.rectangle");
  const lassoLabel = useLabel("ui.selection.lasso");
  const selectiveLabel = useLabel("ui.selection.selective");
  const additiveLabel = useLabel("ui.selection.additive");
  const subtractiveLabel = useLabel("ui.selection.subtractive");
  const invertiveLabel = useLabel("ui.selection.invertive");
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";
  const selectionStore = useShellScope().selection;

  const [selectionMode, setSelectionMode] = useState<MergeMode>(() => selectionStore.get());

  const handleModeChange = (mode: MergeMode) => {
    selectionStore.set(mode);
    setSelectionMode(mode);
  };

  const handleMethodChange = (method: "rectangle" | "lasso") => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee", ...(generation === undefined ? {} : { expectedGeneration: generation }) },
    });
  };

  return (
    <div className="flex w-full min-w-0 max-w-full flex-wrap items-start gap-double">
      <div className="flex min-w-0 max-w-full flex-wrap items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{methodLabel}</span>
        <ToggleGroup
          kind="single"
          semantics="radio"
          aria-label={methodLabel}
          className="h-auto max-w-full flex-wrap"
          value={selectionMethod}
          onValueChange={(val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          }}
          items={[
            { value: "rectangle", icon: <Icon icon="square-dashed" size="small" />, text: rectangleLabel },
            { value: "lasso", icon: <Icon icon="lasso" size="small" />, text: lassoLabel },
          ]}
        />
      </div>
      <RibbonDivider />
      <div className="flex min-w-0 max-w-full flex-wrap items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{modeLabel}</span>
        <ToggleGroup
          kind="single"
          semantics="radio"
          aria-label={modeLabel}
          className="h-auto max-w-full flex-wrap"
          value={selectionMode}
          onValueChange={(val) => {
            if (val === "replace" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          }}
          items={[
            { value: "replace", icon: "mouse-pointer", text: selectiveLabel },
            { value: "additive", icon: "plus", text: additiveLabel },
            { value: "subtractive", icon: "minus", text: subtractiveLabel },
            { value: "invertive", icon: { kind: "emoji", emoji: "🔄" }, text: invertiveLabel },
          ]}
        />
      </div>
    </div>
  );
}

/** 🎓️ Whether a utility node tree has a node (leaf or group) with the given id anywhere in it — used
 * to decide if this window's utility bar is the one an introduction step's `Utility` anchor targets. */
export function utilityNodeTreeContainsId(nodes: readonly UtilityNode[], targetId: string): boolean {
  return nodes.some((node) => node.id === targetId || (node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId)));
}

export function utilityBarNode(utilities: readonly UtilityNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void, revealUtilityId?: string | null, utilityOptions?: ReactNode): ReactNode {
  if (!utilities?.length && !utilityOptions) return undefined;
  const categories = groupUtilityNodesByCategory(utilities ?? [], UTILITY_CATEGORIES);
  if (!categories.length && !utilityOptions) return undefined;
  const grouped: UtilityNode[] = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "utilities" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return <UtilityTree id={`ui.utilities.${windowId}`} utilities={grouped} onAction={onAction} direction="up" revealUtilityId={revealUtilityId} utilityOptions={utilityOptions} renderSelectionOptions={({ activeUtilityId, windowId: selectionWindowId, generation }) => <SelectionUtilityOptions activeUtilityId={activeUtilityId} windowId={selectionWindowId} onAction={onAction} generation={generation} />} />;
}
//#endregion ShellHelpers
