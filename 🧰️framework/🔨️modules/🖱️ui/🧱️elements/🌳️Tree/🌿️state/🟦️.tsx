import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import * as React from "react";

// Hierarchical tree view with sections, items, and file trees.
// Consumers MUST wrap components in TreeStateProvider.

/**
 * 🌳️ TreeStateContextValue holds the data fields for a TreeStateContextValue record.
 **/
interface TreeStateContextValue {
  openStates: Readonly<Record<string, boolean>>;
  setOpenState: (id: string, open: boolean) => void;
  getOpenState: (id: string, defaultOpen: boolean) => boolean;
}

/**
 * 🌲️ TreeStateContext holds the data fields for a TreeStateContext record.
 **/
const TreeStateContext = reactHostPort.createContext<TreeStateContextValue | null>(null);

/**
 * 🪾️ Context provider managing tree expansion state — controlled via `openStates`/`onOpenStateChange` when a host
 * wants expansion to survive remounts (e.g. persisted across tab switches or reloads); uncontrolled (internal
 * state) otherwise. The map only ever holds explicit toggles — {@link useTreeOpenState}'s `getOpenState` falls
 * back to each item's own `defaultOpen` — so it already is the diff-from-default a host would persist.
 **/
export const TreeStateProvider: React.FC<{
  children: React.ReactNode;
  openStates?: Readonly<Record<string, boolean>>;
  onOpenStateChange?: (id: string, open: boolean) => void;
}> = ({ children, openStates: controlledOpenStates, onOpenStateChange }) => {
  const [internalOpenStates, setInternalOpenStates] = reactHostPort.useState<Record<string, boolean>>({});
  const openStates = controlledOpenStates ?? internalOpenStates;

  const setOpenState = (id: string, open: boolean) => {
    if (onOpenStateChange) {
      onOpenStateChange(id, open);
    } else {
      setInternalOpenStates((prev) => ({ ...prev, [id]: open }));
    }
  };

  const getOpenState = (id: string, defaultOpen: boolean) => {
    return openStates[id] !== undefined ? openStates[id] : defaultOpen;
  };

  return <TreeStateContext.Provider value={{ openStates, setOpenState, getOpenState }}>{children}</TreeStateContext.Provider>;
};

/**
 * 🪝️ Hook returning tree expansion state and toggle functions.
 **/
export const useTreeState = () => {
  const context = reactHostPort.useContext(TreeStateContext);
  if (!context) throw new Error("useTreeState must be used within TreeStateProvider");
  return context;
};

export const useTreeOpenState = (itemId: string, defaultOpen: boolean) => {
  const treeState = reactHostPort.useContext(TreeStateContext);
  const [fallbackOpen, setFallbackOpen] = reactHostPort.useState(defaultOpen);
  const open = treeState ? treeState.getOpenState(itemId, defaultOpen) : fallbackOpen;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      if (treeState) {
        treeState.setOpenState(itemId, value);
        return;
      }
      setFallbackOpen(value);
    },
    [itemId, treeState],
  );
  return { open, setOpen };
};

export const treeSectionElementMarker = Symbol.for("ui.tree.section");
export const treeSectionDoubleClickDelayMs = 300;

export type TreeComponentMarker = {
  [treeSectionElementMarker]?: boolean;
  displayName?: string;
};

const isTreeSectionElementType = (value: unknown): boolean => {
  if ((typeof value !== "function" && typeof value !== "object") || value === null) {
    return false;
  }
  return Boolean((value as TreeComponentMarker)[treeSectionElementMarker]);
};

export const hasNonEmptyChildren = (children: React.ReactNode): boolean => {
  if (!children) return false;
  const childArray = React.Children.toArray(children);
  return (
    childArray.length > 0 &&
    childArray.some((child) => {
      if (React.isValidElement(child)) return true;
      if (typeof child === "string" && child.trim().length > 0) return true;
      if (typeof child === "number") return true;
      return false;
    })
  );
};

const isIgnorableTreeChild = (child: React.ReactNode): boolean => child === null || child === undefined || typeof child === "boolean" || (typeof child === "string" && child.trim().length === 0);

export const assertNoNestedTreeSections = (children: React.ReactNode, ownerName: "TreeSection" | "TreeItem") => {
  const visitNestedChildren = (value: React.ReactNode) => {
    React.Children.forEach(value, (child) => {
      if (isIgnorableTreeChild(child)) {
        return;
      }
      if (!React.isValidElement(child)) {
        return;
      }
      const childProps = child.props as { children?: React.ReactNode };
      if (child.type === React.Fragment) {
        visitNestedChildren(childProps.children);
        return;
      }
      if (isTreeSectionElementType(child.type)) {
        throw new Error(`${ownerName} cannot contain a TreeSection. Only TreeItem elements can be nested.`);
      }
      visitNestedChildren(childProps.children);
    });
  };

  visitNestedChildren(children);
};
