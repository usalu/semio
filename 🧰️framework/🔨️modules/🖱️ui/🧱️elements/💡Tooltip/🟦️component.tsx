// #region 🧲️Header
// 💻️ framework/ui/elements/💡Tooltip/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import { Link } from "react-router";
import { useTranslation } from "react-i18next";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { type UiLabel } from "../🫀️core/🏷️UiLabel/🟦️component.tsx";
import { glassClass } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { SurfaceScope } from "../🫀️core/🌈️Surface/🟦️component.tsx";
import { useFlow } from "../🫀️core/🧭️Flow/🟦️component.tsx";
import { useLabel, useIdLabel } from "../🫀️core/🏷️Label/🟦️component.tsx";
import { useUiDriverTooltips, useUiDriver, useControlHotkey, useControlHotkeyTooltipVisible, resolveControlLabelId, isInternalChromeControlId, humanizeControlId, BookIcon, TutorialIcon, type UiTranslationKey, type UiRegisteredTranslationKey } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🎙️Tooltip
// Tooltip components with driver-adaptive content (see UiDriverTooltips).
// Consumers MUST supply a driver via UiDriverProvider or the stored default.

/**
 * Configuration for enhanced tooltip with label, paths, and hotkey.
 **/
export interface TooltipConfig {
  labelKey: UiTranslationKey | UiRegisteredTranslationKey;
  manualPath?: string;
  tutorialPath?: string;
  hotkey?: string;
}

/**
 * Data interface for description-based tooltip content.
 **/
export interface DescriptionTooltipData {
  label?: UiLabel;
  description?: string;
  descriptionBeginner?: string;
  manual?: string;
  tutorial?: string;
  hotkey?: string;
}

/**
 * TooltipProvider holds the data fields for a TooltipProvider record.
 **/
function TooltipProvider({ delayDuration = 400, ...props }: React.ComponentProps<typeof TooltipPrimitive.Provider>) {
  return <TooltipPrimitive.Provider data-slot="tooltip-provider" delayDuration={delayDuration} {...props} />;
}

/**
 * Tooltip holds the data fields for a Tooltip record.
 **/
function Tooltip({ ...props }: React.ComponentProps<typeof TooltipPrimitive.Root>) {
  return (
    <TooltipProvider>
      <TooltipPrimitive.Root data-slot="tooltip" {...props} />
    </TooltipProvider>
  );
}

/**
 * TooltipTrigger holds the data fields for a TooltipTrigger record.
 **/
function TooltipTrigger({ className, asChild, ...props }: React.ComponentProps<typeof TooltipPrimitive.Trigger>) {
  return <TooltipPrimitive.Trigger data-slot="tooltip-trigger" asChild={asChild} className={cn(className)} {...props} />;
}

/**
 * TooltipContent holds the data fields for a TooltipContent record.
 **/
function TooltipContent({ className, sideOffset = 8, children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Content>) {
  const flow = useFlow();
  return (
    <TooltipPrimitive.Portal>
      <TooltipPrimitive.Content
        data-slot="tooltip-content"
        data-level="menu"
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        sideOffset={sideOffset}
        className={cn(
          "border border-accent-foreground text-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-menu origin-(--radix-tooltip-content-transform-origin) p-single text-xs text-balance w-max max-w-fit",
          glassClass,
          className,
        )}
        {...props}
      >
        <SurfaceScope level="menu" fill="glass">
          {children}
        </SurfaceScope>
      </TooltipPrimitive.Content>
    </TooltipPrimitive.Portal>
  );
}

/**
 * EnhancedTooltipContentProps holds the data fields for a EnhancedTooltipContentProps record.
 **/
interface EnhancedTooltipContentProps {
  config: TooltipConfig;
}

/** EnhancedTooltipContent holds the data fields for a EnhancedTooltipContent record.
 **/
/**
 **/
function EnhancedTooltipContent({ config }: EnhancedTooltipContentProps) {
  const { t } = useTranslation();
  const tooltips = useUiDriverTooltips();
  const driver = useUiDriver();

  const { labelKey, manualPath, tutorialPath, hotkey: configHotkey } = config;
  const showManual = tooltips === "full";
  const showTutorial = tooltips === "full";
  const allowInline = driver.labels === "full";
  const registryHotkey = useControlHotkey(labelKey);
  const hotkey = configHotkey ?? registryHotkey;
  const showHotkey = hotkey && useControlHotkeyTooltipVisible(allowInline);

  const label = useLabel(labelKey);
  const manualLabel = useLabel("tooltip.manual");
  const tutorialLabel = useLabel("tooltip.tutorial");

  if (tooltips === "none") return null;

  const fullManualPath = manualPath ? `/doc/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/doc/tutorial/${tutorialPath}` : undefined;

  const handleHotkeyClick = () => {
    if (labelKey) {
      window.dispatchEvent(
        new CustomEvent("navigate-to-hotkey", {
          detail: { path: labelKey },
        }),
      );
    }
  };

  return (
    <div className="flex flex-col gap-single">
      <span>{label}</span>
      {(showManual && fullManualPath) || (showTutorial && fullTutorialPath) || showHotkey ? (
        <div className="grid w-full grid-cols-3 items-center border-t border-accent-foreground pt-single gap-single">
          {showManual && fullManualPath ? (
            <Link to={fullManualPath} className="flex items-center gap-single cursor-pointer text-element transition-colors p-single hover:bg-hover-interactive-fill hover:text-emphasized">
              <BookIcon className="size-tiny" />
              <span>{manualLabel}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {showTutorial && fullTutorialPath ? (
            <Link to={fullTutorialPath} className="flex items-center gap-single cursor-pointer text-element transition-colors p-single hover:bg-hover-interactive-fill hover:text-emphasized">
              <TutorialIcon className="size-tiny" />
              <span className="block text-center">{tutorialLabel}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {showHotkey ? (
            <kbd onClick={handleHotkeyClick} className="border border-accent-foreground text-muted-foreground p-single text-2xs font-mono justify-self-end cursor-pointer">
              {hotkey}
            </kbd>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

/**
 * DescriptionTooltipContentProps holds the data fields for a DescriptionTooltipContentProps record.
 **/
interface DescriptionTooltipContentProps {
  id: string;
}

/**
 * DescriptionTooltipContent holds the data fields for a DescriptionTooltipContent record.
 **/
function DescriptionTooltipContent({ id }: DescriptionTooltipContentProps) {
  const { t } = useTranslation();
  const driver = useUiDriver();
  const labelId = resolveControlLabelId(id);
  const manualLabel = useLabel("tooltip.manual");
  const tutorialLabel = useLabel("tooltip.tutorial");
  const localized = useIdLabel(labelId);
  const allowInline = driver.labels === "full";
  const hotkey = useControlHotkey(id);
  const showHotkey = Boolean(hotkey) && useControlHotkeyTooltipVisible(allowInline);

  if (driver.tooltips === "none") return null;
  if (isInternalChromeControlId(id)) return null;

  const value = t(labelId as any) as any;
  const manualPath = typeof value === "object" && value?.manual ? value.manual : undefined;
  const tutorialPath = typeof value === "object" && value?.tutorial ? value.tutorial : undefined;
  const label =
    localized ??
    (typeof value === "string" && value !== labelId
      ? value
      : typeof value === "object" && value?.label
        ? typeof value.label === "string"
          ? value.label
          : typeof value.label === "object"
            ? driver.labelTier === "beginner" && value.label.beginner !== undefined
              ? String(value.label.beginner)
              : value.label.normal !== undefined
                ? String(value.label.normal)
                : value.label.beginner !== undefined
                  ? String(value.label.beginner)
                  : undefined
            : undefined
        : labelId.startsWith("ui.")
          ? humanizeControlId(labelId)
          : undefined);

  const showManual = driver.tooltips === "full" && manualPath;
  const showTutorial = driver.tooltips === "full" && tutorialPath;

  const fullManualPath = manualPath ? `/doc/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/doc/tutorial/${tutorialPath}` : undefined;

  const hasLinks = showManual || showTutorial || showHotkey;

  const handleHotkeyClick = () => {
    window.dispatchEvent(
      new CustomEvent("navigate-to-hotkey", {
        detail: { path: id },
      }),
    );
  };

  return (
    <div className="flex flex-col gap-single">
      {label && <span className="text-sm">{label}</span>}
      {hasLinks ? (
        <div className="flex w-full items-center border-t border-accent-foreground pt-single gap-single">
          {showManual && fullManualPath && (
            <Link to={fullManualPath} className="flex items-center gap-single cursor-pointer text-element transition-colors p-single hover:bg-hover-interactive-fill hover:text-emphasized">
              <BookIcon className="size-3" />
              <span>{manualLabel}</span>
            </Link>
          )}
          {showTutorial && fullTutorialPath && (
            <Link to={fullTutorialPath} className="flex items-center gap-single cursor-pointer text-element transition-colors p-single hover:bg-hover-interactive-fill hover:text-emphasized">
              <TutorialIcon className="size-3" />
              <span className="block text-center">{tutorialLabel}</span>
            </Link>
          )}
          {showHotkey && hotkey ? (
            <kbd onClick={handleHotkeyClick} className="border border-accent-foreground text-muted-foreground p-single text-2xs font-mono ms-auto cursor-pointer">
              {hotkey}
            </kbd>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

export { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger, EnhancedTooltipContent, DescriptionTooltipContent };

// #endregion 🎙️Tooltip
