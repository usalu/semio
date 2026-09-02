// #region 🧲️Header
// 💻️ framework/ui/modules/class-name-composition/slot.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "./🟦️.ts";
// #endregion 🔌️Adapters

// #region 🪆️SingleChildSlot
type SlotChildProps = Record<string, unknown> & {
  className?: string;
  ref?: React.Ref<HTMLElement>;
  style?: React.CSSProperties;
};

type SlotEvent = { defaultPrevented?: boolean };
type SlotHandler = (...args: unknown[]) => unknown;

/** 🪆️ Props accepted by the owned exactly-one-child composition boundary. */
type SlotProps = React.HTMLAttributes<HTMLElement> & {
  children: React.ReactElement<SlotChildProps>;
};

const setRef = <T,>(ref: React.Ref<T> | undefined, value: T | null): void => {
  if (typeof ref === "function") ref(value);
  else if (ref) ref.current = value;
};

const composeRefs = <T,>(childRef: React.Ref<T> | undefined, wrapperRef: React.Ref<T> | undefined): React.RefCallback<T> => {
  return (value) => {
    setRef(childRef, value);
    setRef(wrapperRef, value);
  };
};

const composeHandlers = (childHandler: SlotHandler, wrapperHandler: SlotHandler): SlotHandler => {
  return (...args) => {
    childHandler(...args);
    if (!(args[0] as SlotEvent | undefined)?.defaultPrevented) wrapperHandler(...args);
  };
};

const mergeSlotProps = (wrapperProps: SlotChildProps, childProps: SlotChildProps): SlotChildProps => {
  const merged: SlotChildProps = { ...wrapperProps, ...childProps };
  const names = new Set([...Object.keys(wrapperProps), ...Object.keys(childProps)]);
  for (const name of names) {
    const childHandler = childProps[name];
    const wrapperHandler = wrapperProps[name];
    if (/^on[A-Z]/.test(name) && typeof childHandler === "function" && typeof wrapperHandler === "function") {
      merged[name] = composeHandlers(childHandler as SlotHandler, wrapperHandler as SlotHandler);
    }
  }
  if (wrapperProps.className || childProps.className) merged.className = cn(wrapperProps.className, childProps.className);
  if (wrapperProps.style || childProps.style) merged.style = { ...wrapperProps.style, ...childProps.style };
  return merged;
};

/** 🪆️ Clones exactly one element while preserving child precedence and composing wrapper behavior. */
const Slot = React.forwardRef<HTMLElement, SlotProps>(function Slot({ children, ...wrapperProps }, forwardedRef) {
  if (React.Children.count(children) !== 1 || !React.isValidElement<SlotChildProps>(children)) {
    throw new Error("Slot requires exactly one valid React element child.");
  }
  const childProps = children.props;
  const mergedProps = mergeSlotProps(wrapperProps as SlotChildProps, childProps);
  mergedProps.ref = composeRefs(childProps.ref, forwardedRef);
  return React.cloneElement(children, mergedProps);
});

export { Slot };
export type { SlotProps };
// #endregion 🪆️SingleChildSlot
