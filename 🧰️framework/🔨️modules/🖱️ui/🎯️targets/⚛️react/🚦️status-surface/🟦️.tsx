import * as React from "react";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { BoxIcon, AlertCircleIcon } from "@semio-tech/ui/elements/Icons";
import type { UiLabel } from "@semio-tech/ui/elements/UiLabel";
import { useLabel } from "@semio-tech/ui/elements/Label";
import { SurfaceScope, surfaceLevelAbove, useLevel } from "@semio-tech/ui/elements/Surface";
import { glassClass } from "@semio-tech/ui/modules/surface-presentation";
import { SharedBorderRow, SharedBorderRowCell } from "@semio-tech/ui/elements/SharedBorderRow";

// #region 🚦️StatusSurface
// Tone-based status-surface kit: badges, status chips, stat tiles, and empty/error placeholders.
// Consumers MUST use StatusTone (never hardcoded palette colors) for tone-driven styling.

/** 🚦️ Semantic tone shared by {@link Badge}, {@link StatusChip}, and {@link StatCard} deltas. */
export type StatusTone = "neutral" | "info" | "success" | "warning" | "danger";

/**
 * 🎶️ STATUS_TONE_INK_CLASS holds the semantic ink classes for a StatusTone.
 **/
const STATUS_TONE_INK_CLASS: Record<StatusTone, string> = {
  neutral: "text-muted-foreground",
  info: "text-info-ink",
  success: "text-success-ink",
  warning: "text-warning-ink",
  danger: "text-destructive-foreground",
};

/**
 * 🎼️ STATUS_TONE_DOT_CLASS holds the solid dot background class for a StatusTone.
 **/
const STATUS_TONE_DOT_CLASS: Record<StatusTone, string> = {
  neutral: "bg-muted-foreground",
  info: "bg-info-border",
  success: "bg-success-border",
  warning: "bg-warning-border",
  danger: "bg-destructive-border",
};

/**
 * 🎖️ Props interface for the Badge component.
 **/
export interface BadgeProps {
  id?: string;
  tone?: StatusTone;
  text: string;
  icon?: React.ReactNode;
}

/** 🏷️ Inline status badge with semantic ink on glass above the owner. */
export function Badge({ id, tone = "neutral", text, icon }: BadgeProps): React.ReactElement {
  const level = surfaceLevelAbove(useLevel());
  return (
    <span id={id} data-slot="badge" data-glass-chip="" data-level={level} className={cn("inline-flex items-center gap-single px-double py-0.5 text-xs font-medium", glassClass, STATUS_TONE_INK_CLASS[tone])}>
      <SurfaceScope level={level} fill="glass">
        {icon ? <span className="inline-flex shrink-0 items-center justify-center size-tiny [&_svg]:size-full">{icon}</span> : null}
        <span className="truncate">{text}</span>
      </SurfaceScope>
    </span>
  );
}

/**
 * 🏅️ Props interface for the StatusChip component.
 **/
export interface StatusChipProps {
  id?: string;
  status: "ok" | "busy" | "error" | "offline";
  label: string;
}

/**
 * 🔰️ STATUS_CHIP_TONE maps a StatusChip status to its StatusTone.
 **/
const STATUS_CHIP_TONE: Record<StatusChipProps["status"], StatusTone> = {
  ok: "success",
  busy: "warning",
  error: "danger",
  offline: "neutral",
};

/** 🟢️ Inline dot-and-label indicator for live status (connection, task, presence). */
export function StatusChip({ id, status, label }: StatusChipProps): React.ReactElement {
  const tone = STATUS_CHIP_TONE[status];
  const level = surfaceLevelAbove(useLevel());
  return (
    <span id={id} data-slot="status-chip" data-status={status} data-glass-chip="" data-level={level} className={cn("inline-flex items-center gap-half text-xs text-muted-foreground", glassClass)}>
      <SurfaceScope level={level} fill="glass">
        <span className={cn("size-tiny shrink-0", STATUS_TONE_DOT_CLASS[tone])} aria-hidden />
        <span className="truncate">{label}</span>
      </SurfaceScope>
    </span>
  );
}

/**
 * 🎛️ Props interface for the StatCard component.
 **/
export interface StatCardProps {
  id?: string;
  label: string;
  value: string;
  delta?: { value: string; tone: StatusTone };
  icon?: React.ReactNode;
}

/** 📊️ Labelled metric tile with an optional tone-colored delta badge. */
export function StatCard({ id, label, value, delta, icon }: StatCardProps): React.ReactElement {
  return (
    <SharedBorderRow id={id} data-slot="stat-card" orientation="vertical">
      <SharedBorderRowCell className="flex flex-col items-stretch gap-single p-single">
        <div className="flex items-center gap-single text-muted-foreground">
          {icon ? <span className="inline-flex shrink-0 items-center justify-center size-small">{icon}</span> : null}
          <span className="truncate text-xs">{label}</span>
        </div>
        <div className="flex items-baseline gap-single">
          <span className="text-xl font-semibold text-foreground">{value}</span>
          {delta ? <Badge tone={delta.tone} text={delta.value} /> : null}
        </div>
      </SharedBorderRowCell>
    </SharedBorderRow>
  );
}

/**
 * 🫥️ Props interface for the EmptyState component.
 **/
export interface EmptyStateProps {
  id?: string;
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: { label: string; onClick: () => void };
}

/** 📭️ Centered placeholder for lists or panels with no content yet. */
export function EmptyState({ id, icon, title, description, action }: EmptyStateProps): React.ReactElement {
  return (
    <div id={id} data-slot="empty-state" className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-muted-foreground">{icon || <BoxIcon className="size-huge" />}</div>
      <h2 className="text-xl font-semibold">{title}</h2>
      {description && <p className="text-muted-foreground max-w-md">{description}</p>}
      {action && (
        <button type="button" onClick={action.onClick} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <span>{action.label}</span>
        </button>
      )}
    </div>
  );
}

/**
 * 💥️ Props interface for the ErrorView component.
 **/
export interface ErrorViewProps {
  id?: string;
  title?: UiLabel;
  message: string;
  onRetry?: () => void;
}

/** 🚨️ Centered error placeholder with an optional retry action. */
export function ErrorView({ id, title, message, onRetry }: ErrorViewProps): React.ReactElement {
  const somethingWentWrongLabel = useLabel("ui.common.somethingWentWrong");
  const retryLabel = useLabel("ui.common.retry");
  return (
    <div id={id} data-slot="error-view" className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-destructive-foreground">
        <AlertCircleIcon className="size-huge" />
      </div>
      <h2 className="text-xl font-semibold">{title || somethingWentWrongLabel}</h2>
      <p className="text-muted-foreground max-w-md">{message}</p>
      {onRetry && (
        <button type="button" onClick={onRetry} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <span>{retryLabel}</span>
        </button>
      )}
    </div>
  );
}

// #endregion 🚦️StatusSurface

/** 🧭️ Compile-time checklist of chrome components that must accept the shared {@link UiElementStateProps} axes. */
export const UI_ELEMENT_REGISTRY = ["Window", "Panel", "Canvas", "Button", "Slider", "TreeItem", "Action"] as const;
