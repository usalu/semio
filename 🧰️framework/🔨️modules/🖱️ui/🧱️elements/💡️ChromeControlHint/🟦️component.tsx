// #region Header
// framework/ui/elements/💡️ChromeControlHint/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

// #region 🔌️Adapters
import * as React from "react";
import { useControlAccessibleLabel } from "../🏷️Label/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 💡️ChromeControlHint
/** @emoji 🏷️ Native title/aria-label for chrome controls (avoids Radix tooltip `setTrigger` ref update loops). */
export function ChromeControlHint({ id, text, children }: { readonly id?: string; readonly text?: string; readonly children: React.ReactElement }): React.ReactElement {
  const label = useControlAccessibleLabel(id, text);
  if (!label || !React.isValidElement(children)) return children;
  const childProps = children.props as { readonly title?: string; readonly "aria-label"?: string };
  return React.cloneElement(children, {
    title: childProps.title ?? label,
    "aria-label": childProps["aria-label"] ?? label,
  } as Record<string, unknown>);
}
// #endregion 💡️ChromeControlHint
