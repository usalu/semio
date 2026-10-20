import type { UiLabel } from "@semio-tech/ui/elements/UiLabel";
import * as React from "react";
import { InfoIcon, LightbulbIcon, TriangleAlertIcon, AlertCircleIcon, ChevronLeftIcon } from "@semio-tech/ui/elements/Icons";
import { useLabel } from "@semio-tech/ui/elements/Label";

// Read-only display wrappers for tooltips and callouts.
// Consumers MUST pass valid config objects.

// #region 📣️Aside
// Callout boxes for notes, tips, cautions, and dangers.
// Consumers MUST specify a valid kind prop.

/**
 * 🏷️ Props interface for the Aside callout component.
 **/
export interface AsideProps {
  kind?: "note" | "tip" | "caution" | "danger";
  title?: UiLabel;
  children: React.ReactNode;
}

/**
 * 🖼️ iconMap holds the data fields for a iconMap record.
 **/
const iconMap = {
  note: InfoIcon,
  tip: LightbulbIcon,
  caution: TriangleAlertIcon,
  danger: AlertCircleIcon,
};

/**
 * 🎨️ colorMap holds the data fields for a colorMap record.
 **/
const colorMap = {
  note: "border-info-border bg-info-bg text-info-foreground",
  tip: "border-success-border bg-success-bg text-success-foreground",
  caution: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

/**
 * 🔌️ Callout component rendering note, tip, caution, or danger boxes.
 **/
export const Aside: React.FC<AsideProps> = ({ kind = "note", title, children }) => {
  const Icon = iconMap[kind];
  const colorClass = colorMap[kind];

  return (
    <aside className={`my-small p-single border ${colorClass}`}>
      <div className="flex items-start gap-single">
        <Icon className="size-small mt-0.5 flex-shrink-0" />
        <div className="flex-1">
          {title && <div className="font-semibold mb-1">{title}</div>}
          <div>{children}</div>
        </div>
      </div>
    </aside>
  );
};
// #endregion 👥️PresenceBar

// #region 🎹️Spinner
// Animated loading spinner in small, medium, or large sizes.
// Consumers MUST choose an appropriate size for the context.

/**
 * ⏳️ Props interface for the Spinner component.
 **/
export interface SpinnerProps {
  size?: "small" | "medium" | "large";
  className?: string;
}

/**
 * ⌛️ Animated SVG loading spinner.
 **/
export const Spinner: React.FC<SpinnerProps> = ({ size = "medium", className = "" }) => {
  const sizeClass = size === "small" ? "size-small" : size === "large" ? "size-large" : "size-medium";
  return (
    <svg className={`animate-spin ${sizeClass} ${className}`} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
    </svg>
  );
};

// #endregion 🎹️Spinner

// #region 🔗️RouteLink
// Anchor that intercepts same-origin, protocol-less clicks for client-side navigation via history.pushState.
// Consumers MUST rely on the owned route-target parser; external/absolute hrefs always fall through to a plain anchor.

type OwnedRouteTarget = { kind: "internal"; href: string };

/** 🧭️ Parses the closed same-document route grammar without normalizing its path, query, or fragment. */
export function parseOwnedRouteTarget(href: unknown): OwnedRouteTarget | null {
  if (typeof href !== "string" || href.length === 0 || /[\s\u0000-\u001f\u007f\\]/u.test(href)) return null;
  if (/^[a-z][a-z0-9+.-]*:/i.test(href) || href.startsWith("//")) return null;
  try {
    if (new URL(href, "https://owned-route.invalid/current").origin !== "https://owned-route.invalid") return null;
  } catch {
    return null;
  }
  return { kind: "internal", href };
}

/** 🚦️ Performs the one owned browser-history command and publishes one matching navigation signal. */
export function navigateOwnedRoute(target: OwnedRouteTarget): { navigated: boolean } {
  if (typeof window === "undefined" || typeof window.history?.pushState !== "function" || typeof window.PopStateEvent !== "function") return { navigated: false };
  let event: PopStateEvent;
  try {
    event = new window.PopStateEvent("popstate");
    window.history.pushState(null, "", target.href);
  } catch {
    return { navigated: false };
  }
  window.dispatchEvent(event);
  return { navigated: true };
}

/** 🔗️ Anchor that delegates primary internal clicks to the owned navigation command while preserving native behavior for every other target or gesture. */
export function RouteLink({ href, target, download, onClick, ...props }: React.AnchorHTMLAttributes<HTMLAnchorElement>): React.ReactElement {
  const handleClick = (event: React.MouseEvent<HTMLAnchorElement>) => {
    onClick?.(event);
    if (event.defaultPrevented || download !== undefined || (target && target !== "_self")) return;
    if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    const route = parseOwnedRouteTarget(href);
    if (!route || !navigateOwnedRoute(route).navigated) return;
    event.preventDefault();
  };
  return <a href={href} target={target} download={download} onClick={handleClick} data-slot="route-link" {...props} />;
}

// #endregion 🔗️RouteLink

// 404-style placeholder with icon, title, and back navigation.
// Consumers MUST provide a title for the error.

/**
 * 🎛️ Props interface for the NotFound component.
 **/
export interface NotFoundProps {
  title: string;
  description?: string;
  parentPath?: string;
  parentLabel?: string;
  icon?: React.ReactNode;
}

/**
 * 👻️ Not-found placeholder page with navigation link.
 **/
export const NotFound: React.FC<NotFoundProps> = ({ title, description, parentPath, parentLabel, icon }) => {
  const goBackLabel = useLabel("ui.nav.back");
  const parentTarget = parseOwnedRouteTarget(parentPath);
  return (
    <div className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-muted-foreground">{icon || <AlertCircleIcon className="size-huge" />}</div>
      <h1 className="text-xl font-semibold">{title}</h1>
      {description && <p className="text-muted-foreground max-w-md">{description}</p>}
      {parentTarget && (
        <button type="button" onClick={() => navigateOwnedRoute(parentTarget)} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <ChevronLeftIcon className="size-small" />
          <span>{parentLabel || goBackLabel}</span>
        </button>
      )}
    </div>
  );
};
