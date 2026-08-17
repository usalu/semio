// #region Header
// framework/ui/elements/💡️ChromeControlHint/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

// #region 🔌️Adapters
import * as React from "react";
import { useControlAccessibleLabel, useControlTooltipText } from "../🏷️Label/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 💡️ChromeControlHint
/** @emoji 🏷️ Native title/aria-label for chrome controls (avoids Radix tooltip `setTrigger` ref update loops). */
export function ChromeControlHint({ id, text, always = false, children }: { readonly id?: string; readonly text?: string; readonly always?: boolean; readonly children: React.ReactElement }): React.ReactElement {
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text, { always });
  if (!React.isValidElement(children)) return children;
  if (!accessibleLabel && !tooltipText) return children;
  const childProps = children.props as { readonly title?: string; readonly "aria-label"?: string };
  return React.cloneElement(children, {
    title: childProps.title ?? tooltipText,
    "aria-label": childProps["aria-label"] ?? accessibleLabel,
  } as Record<string, unknown>);
}
// #endregion 💡️ChromeControlHint
