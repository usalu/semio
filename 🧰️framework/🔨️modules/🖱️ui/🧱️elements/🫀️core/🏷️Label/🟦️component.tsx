// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🏷️Label/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header


// #region 🔌️Adapters
import * as React from "react";
import { useTranslation } from "react-i18next";
import i18next from "i18next";
import { sizeVar } from "@semio-tech/ui-styling";
import { reactHostPort } from "../🔌Ports/🟦️component.tsx";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { TreeContext, TreeRowAlignmentContext, TreeAlignedRow, PropertyValueColumnContext, detailPanelIndentPx, detailPanelIndentLen, detailPanelPropertyInlineGapPx, detailPanelPropertyStackedToInlineHysteresisPx, detailPanelPropertyRowClassName, detailPanelPropertyControlClassName, detailPanelHeaderLineCenterPx, treeItemLabelStyle, treeHeaderRowClassName, treeInspectorInnerRowClassName, treeHeaderMainClassName } from "../../🪵Tree/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { type UiTranslationKey, type UiRegisteredTranslationKey, type UiTranslateFn, activeUiDriver, useUiDriver, isInternalChromeControlId, resolveControlLabelId, panelKindFromPanelToggleControlId, humanizeEngagementStepId, humanizeControlId } from "../../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🏷️Label
/** @emoji 🪁️ Typed {@link useTranslation} bound to {@link UiTranslationKey} and registered product bundles. */
export function useUiTranslation(): { readonly t: UiTranslateFn; readonly i18n: typeof i18next } {
  const { t, i18n } = useTranslation();
  return { t: t as UiTranslateFn, i18n };
}
/**
 * React hook that resolves a localized label by i18n key and driver label tier. Strict: `id` must be a
 * real key from the domain-neutral chrome schema or a product's {@link registerUiTranslationBundles}
 * bundle — both are guaranteed complete for every {@link UiLocale}, so a defined `id` always yields a
 * `string`. For ids that may or may not be a registered key (e.g. resolved from an arbitrary DOM/control
 * id), use {@link useIdLabel} instead — it is the only place a lookup is allowed to come up empty.
 **/
export function useLabel(id: UiTranslationKey | UiRegisteredTranslationKey, options?: Record<string, unknown>): UiLabel;
export function useLabel(id: UiTranslationKey | UiRegisteredTranslationKey | undefined, options?: Record<string, unknown>): UiLabel | undefined;
export function useLabel(id: UiTranslationKey | UiRegisteredTranslationKey | undefined, options?: Record<string, unknown>): UiLabel | undefined {
  const { t } = useUiTranslation();
  const labelTier = activeUiDriver().labelTier;
  if (!id) return undefined;
  const value = t(id as UiTranslationKey, options);

  if (typeof value === "string") return value as UiLabel;

  if (value && typeof value === "object" && "label" in value) {
    const label = value.label;

    if (typeof label === "string") {
      return label as UiLabel;
    }

    if (label && typeof label === "object") {
      if (labelTier === "beginner" && "beginner" in label && label.beginner !== undefined) {
        return String(label.beginner) as UiLabel;
      }
      if ("normal" in label && label.normal !== undefined) {
        return String(label.normal) as UiLabel;
      }
      if ("beginner" in label && label.beginner !== undefined) {
        return String(label.beginner) as UiLabel;
      }
    }
  }

  return undefined;
}
/**
 * @emoji 🏷️ Resolves a label only when `id` happens to be a registered translation key — the deliberate
 * dynamic port for generic components whose `id` is not a key contract (e.g. resolved from an arbitrary
 * DOM/control id via `resolveControlLabelId`). Chrome call sites with a known key must use the strict
 * {@link useLabel} instead. Checks existence first, so it never echoes an unresolved id back as if it
 * were a translation — unlike i18next's default missing-key behavior.
 **/
export function useIdLabel(id: string | undefined): UiLabel | undefined {
  const { t, i18n } = useUiTranslation();
  const labelTier = activeUiDriver().labelTier;
  if (!id || !i18n.exists(id)) return undefined;
  const value = t(id as UiTranslationKey);

  if (typeof value === "string") return value as UiLabel;

  if (value && typeof value === "object" && "label" in value) {
    const label = value.label;

    if (typeof label === "string") {
      return label as UiLabel;
    }

    if (label && typeof label === "object") {
      if (labelTier === "beginner" && "beginner" in label && label.beginner !== undefined) {
        return String(label.beginner) as UiLabel;
      }
      if ("normal" in label && label.normal !== undefined) {
        return String(label.normal) as UiLabel;
      }
      if ("beginner" in label && label.beginner !== undefined) {
        return String(label.beginner) as UiLabel;
      }
    }
  }

  return undefined;
}
/**
 * Resolves a localized string from a raw translation value and driver label tier.
 * Pure function (non-hook) variant of useLabel for use outside React render context.
 * Handles: string, {label: string}, {label: {normal, beginner}}, {normal, beginner}.
 **/
export function resolveTranslationLabel(value: unknown): string | undefined {
  const labelTier = activeUiDriver().labelTier;

  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object") {
    const obj = value as Record<string, unknown>;

    if ("label" in obj) {
      const label = obj.label;

      if (typeof label === "string") {
        return label;
      }

      if (label && typeof label === "object") {
        const labelObj = label as Record<string, unknown>;
        if (labelTier === "beginner" && "beginner" in labelObj && labelObj.beginner !== undefined) {
          return String(labelObj.beginner);
        }
        if ("normal" in labelObj && labelObj.normal !== undefined) {
          return String(labelObj.normal);
        }
        if ("beginner" in labelObj && labelObj.beginner !== undefined) {
          return String(labelObj.beginner);
        }
      }
    }

    if ("normal" in obj || "beginner" in obj) {
      if (labelTier === "beginner" && "beginner" in obj && obj.beginner !== undefined) {
        return String(obj.beginner);
      }
      if ("normal" in obj && obj.normal !== undefined) {
        return String(obj.normal);
      }
      if ("beginner" in obj && obj.beginner !== undefined) {
        return String(obj.beginner);
      }
    }
  }

  return undefined;
}
/** @emoji 🏷️ Resolves the user-facing caption for a control (i18n, explicit text, or `ui.*` fallback). */
export function useControlAccessibleLabel(id: string | undefined, text?: string): string | undefined {
  if (text !== undefined && text !== "") return text;
  if (!id || isInternalChromeControlId(id)) return undefined;
  const labelId = resolveControlLabelId(id);
  const localized = useIdLabel(labelId);
  if (localized) return localized;
  const panelKind = panelKindFromPanelToggleControlId(id);
  if (panelKind) {
    const fromUiPanel = useIdLabel(`ui.panelToggle.${panelKind}`);
    if (fromUiPanel) return fromUiPanel;
    return humanizeEngagementStepId(panelKind);
  }
  if (labelId.startsWith("ui.")) return humanizeControlId(labelId);
  return undefined;
}
/** @emoji 🏷️ Resolves inline icon+label caption for buttons/toggles; omitted when the driver hides labels. */
export function useControlInlineText(id: string | undefined, text?: string): string | undefined {
  const driver = useUiDriver();
  const accessibleLabel = useControlAccessibleLabel(id, text);
  return driver.labels === "icons" ? undefined : accessibleLabel;
}
// Foundational internal components like Label.
// Consumers MUST use these as building blocks for inputs.

/**
 * LabelProps holds the data fields for a LabelProps record.
 **/
interface LabelProps {
  id?: string;
  rowId?: string;
  label?: React.ReactNode;
  labelElementId?: string;
  className?: string;
  /**
   * Property rows use the label/value grid; tree group headers mirror TreeItem header geometry
   * (gutter, tree-label slot, trailing control) so collection rows do not drift into the value column.
   */
  labelLayoutKind?: "property" | "treeGroupHeader";
  children?: React.ReactNode;
}
// [🏘️compose📚️js🗃️sketchpad💻️elements🔖️basecomponents🪨️label](repo://p/u/compose/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Base%20Components/d/i/Label)
export function Label({ id, rowId, label, labelElementId, className, children, labelLayoutKind = "property" }: LabelProps) {
  const localizedLabel = useIdLabel(id);
  const resolvedLabel = label ?? localizedLabel;
  const fallbackLabel = reactHostPort.useMemo(() => {
    if (!id) return "";
    const trailingToken = id.split(".").pop() ?? id;
    const normalizedToken = trailingToken.replace(/[-_]+/g, " ").trim();
    if (!normalizedToken) return id;
    return normalizedToken
      .split(/\s+/)
      .map((word) => (word.length > 0 ? `${word[0].toUpperCase()}${word.slice(1)}` : word))
      .join(" ");
  }, [id]);
  const displayLabel = resolvedLabel ?? fallbackLabel;
  const controlHint = useControlAccessibleLabel(id);
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const isInsideTreeRow = reactHostPort.useContext(TreeRowAlignmentContext);
  const treePropertyRowOffsetPx = detailPanelIndentPx(level, indentMultiplier);
  const propertyRowRef = reactHostPort.useRef<HTMLDivElement>(null);
  const propertyLabelRef = reactHostPort.useRef<HTMLDivElement>(null);
  const propertyControlRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [propertyRowStacked, setPropertyRowStacked] = reactHostPort.useState(false);
  const propertyRowStackedRef = reactHostPort.useRef(propertyRowStacked);
  propertyRowStackedRef.current = propertyRowStacked;

  reactHostPort.useEffect(() => {
    const rowElement = propertyRowRef.current;
    const labelElement = propertyLabelRef.current;
    const controlElement = propertyControlRef.current;
    if (!rowElement || !labelElement || !controlElement) {
      return;
    }

    let animationFrame = 0;
    const resolvePropertyLayout = () => {
      animationFrame = 0;
      const rowWidthPx = rowElement.clientWidth;
      const labelWidthPx = Math.ceil(labelElement.scrollWidth);
      const controlMinWidthPx = Math.ceil(controlElement.scrollWidth);
      const minimumInlineWidthPx = labelWidthPx + controlMinWidthPx + detailPanelPropertyInlineGapPx;
      const labelRect = labelElement.getBoundingClientRect();
      const controlRect = controlElement.getBoundingClientRect();
      const overlaps = labelRect.right + detailPanelPropertyInlineGapPx > controlRect.left;
      const stacked = propertyRowStackedRef.current;
      const shouldStack = stacked ? overlaps || minimumInlineWidthPx > rowWidthPx - detailPanelPropertyStackedToInlineHysteresisPx : overlaps || minimumInlineWidthPx > rowWidthPx;
      setPropertyRowStacked((current) => (current === shouldStack ? current : shouldStack));
    };

    const scheduleResolvePropertyLayout = () => {
      if (animationFrame !== 0) {
        cancelAnimationFrame(animationFrame);
      }
      animationFrame = requestAnimationFrame(resolvePropertyLayout);
    };

    const observer = new ResizeObserver(() => scheduleResolvePropertyLayout());
    observer.observe(rowElement);
    observer.observe(labelElement);
    observer.observe(controlElement);
    scheduleResolvePropertyLayout();

    return () => {
      observer.disconnect();
      if (animationFrame !== 0) {
        cancelAnimationFrame(animationFrame);
      }
    };
  }, [id, label, level, treePropertyRowOffsetPx]);

  if (labelLayoutKind === "treeGroupHeader") {
    const treeGroupHeaderLabel = (
      <span data-slot="tree-label" id={labelElementId} title={controlHint} className="flex min-w-0 flex-1 items-center text-xs font-normal text-start truncate h-medium" style={treeItemLabelStyle}>
        {displayLabel}
      </span>
    );

    const treeGroupHeaderInner = (
      <div id={rowId} data-slot="tree-group-header-row" className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName, className)}>
        <div className={cn(treeHeaderMainClassName, "min-h-medium items-center")}>
          {treeGroupHeaderLabel}
          <div data-slot="tree-group-header-control" className="ms-auto flex min-w-0 shrink-0 items-center justify-end">
            {children}
          </div>
        </div>
      </div>
    );

    if (!isTree) {
      return <TreeRowAlignmentContext.Provider value={false}>{treeGroupHeaderInner}</TreeRowAlignmentContext.Provider>;
    }

    if (isInsideTreeRow) {
      return <TreeRowAlignmentContext.Provider value={false}>{treeGroupHeaderInner}</TreeRowAlignmentContext.Provider>;
    }

    return (
      <TreeRowAlignmentContext.Provider value={false}>
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0">
          {treeGroupHeaderInner}
        </TreeAlignedRow>
      </TreeRowAlignmentContext.Provider>
    );
  }

  const propertyLabelElement = isTree ? (
    <div ref={propertyLabelRef} data-slot="property-label-tree" className="min-w-0" style={{ paddingInlineStart: detailPanelIndentLen(level, indentMultiplier) }}>
      <div className="inline-flex min-w-0 h-medium">
        <span data-slot="property-label" id={labelElementId} title={controlHint} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-start truncate cursor-pointer transition-colors h-medium ps-single">
          {resolvedLabel}
        </span>
      </div>
    </div>
  ) : (
    <div ref={propertyLabelRef} data-slot="property-label-inline" className="min-w-0">
      <span
        data-slot="property-label"
        id={labelElementId}
        title={controlHint}
        className="inline-flex items-center text-xs font-medium flex-shrink-0 text-start truncate cursor-pointer transition-colors text-element hover:bg-hover-interactive-fill hover:text-emphasized h-medium"
      >
        {resolvedLabel}
      </span>
    </div>
  );

  const propertyRowElement = (
    <div
      ref={propertyRowRef}
      id={rowId}
      data-dim
      data-slot="property-row"
      data-property-layout={propertyRowStacked ? "stacked" : "inline"}
      style={{
        ...(isTree ? { marginInlineStart: `calc(-1 * ${detailPanelIndentLen(level, indentMultiplier)})`, width: level > 0 ? `calc(100% + ${detailPanelIndentLen(level, indentMultiplier)})` : "100%" } : {}),
        gridTemplateColumns: propertyRowStacked ? "minmax(0, 1fr)" : `${sizeVar("layoutLabel")} minmax(0, 1fr)`,
        rowGap: propertyRowStacked ? sizeVar("spacingSingle") : "0",
      }}
      className={cn(detailPanelPropertyRowClassName, !isTree && "w-full", className)}
    >
      {propertyLabelElement}
      <div ref={propertyControlRef} data-slot="property-control" className={detailPanelPropertyControlClassName} style={propertyRowStacked ? { paddingInlineStart: `calc(${sizeVar("layoutLabel")} + ${sizeVar("spacingDouble")})` } : undefined}>
        <PropertyValueColumnContext.Provider value={true}>{children}</PropertyValueColumnContext.Provider>
      </div>
    </div>
  );

  if (isTree) {
    if (isInsideTreeRow) {
      return propertyRowElement;
    }
    return (
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0} anchorOffsetPx={detailPanelHeaderLineCenterPx}>
        {propertyRowElement}
      </TreeAlignedRow>
    );
  }

  return propertyRowElement;
}
// #endregion 🏷️Label
