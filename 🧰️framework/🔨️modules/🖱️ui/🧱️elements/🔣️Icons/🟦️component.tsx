// #region 🧲️Header
// 💻️ framework/ui/elements/🔣️Icons/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralMap } from "@semio-tech/framework";
import * as React from "react";
import { domSizePx, activeUiTheme, subscribeActiveUiTheme, STYLING_COMPACT_ROOT_PX, type UiTheme } from "@semio-tech/ui-styling";
import {
  ICONS,
  isIconName,
  resolveCatalogIconSvgFromTheme,
  shortcodeCatalogKey,
  shortcodeEmoji,
  type IconName,
} from "@semio-tech/assets";
import {
  isMetabolismIconName,
  METABOLISM_ICONS,
  resolveMetabolismIconSvgFromTheme,
  type MetabolismIconName,
} from "@semio-tech/assets";
import { uiSpacingLen } from "../🪵️Tree/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
export type { IconName };
// #endregion 🔌️Adapters

// #region 🔖️Icon
/** @emoji 📐️ Named size tokens for {@link Icon}. */
export type IconSizeToken = "tiny" | "small" | "base" | "large";

const ICON_SIZE_PX: Record<IconSizeToken, number> = {
  tiny: domSizePx("iconTinyUiSpacing"),
  small: domSizePx("iconSmallUiSpacing"),
  base: domSizePx("iconBaseUiSpacing"),
  large: domSizePx("iconLargeUiSpacing"),
};

/** @emoji 📐️ Resolves {@link Icon} `size` to pixel dimensions. */
export function resolveIconSizePx(size?: number | IconSizeToken): number {
  if (size === undefined) return ICON_SIZE_PX.base;
  if (typeof size === "number") return size;
  return ICON_SIZE_PX[size];
}

// #region 🖼️IconCodec

/** @emoji 🖼️ Canonical structured icon payload shared across canvases and UI chrome. */
export type Icon =
  | { readonly kind: "url"; readonly url: string }
  | { readonly kind: "shortcode"; readonly code: string }
  | { readonly kind: "data"; readonly data: string }
  | { readonly kind: "emoji"; readonly emoji: string }
  | { readonly kind: "typst"; readonly src: string }
  | { readonly kind: "text"; readonly text: string }
  | { readonly kind: "svg"; readonly svg: string }
  | { readonly kind: "catalog"; readonly key: IconName }
  | { readonly kind: "themed"; readonly key: MetabolismIconName }
  | { readonly kind: "node"; readonly node: React.ReactNode };

/** @emoji 🎛️ Shared icon editor tab buckets aligned with {@link Icon}. */
export type IconSelectorMode = "url" | "shortcode" | "data" | "emoji" | "math" | "text" | "vector";

function isRasterDataUrlPayloadForIcon(s: string): boolean {
  const u = s.trim().toLowerCase();
  return u.startsWith("data:image/png;base64,") || u.startsWith("data:image/jpeg;base64,") || u.startsWith("data:image/jpg;base64,") || u.startsWith("data:image/webp;base64,") || u.startsWith("data:image/gif;base64,");
}

function isSvgDataUrlPayloadForIcon(s: string): boolean {
  return s.trim().toLowerCase().startsWith("data:image/svg+xml");
}

function looksLikeShortcodeToken(t: string): boolean {
  return t.length >= 3 && t.startsWith(":") && t.endsWith(":") && /^:[\w+-]+:$/.test(t);
}

function looksLikeAsciiCatalogishStemForIcon(s: string): boolean {
  const t = s.trim();
  if (t === "" || !/^[\w.-]+$/.test(t)) {
    return false;
  }
  return /[._-]/.test(t) || t.length > 48;
}

function looksLikeBareUrlForIcon(s: string): boolean {
  const lower = s.trim().toLowerCase();
  return lower.startsWith("http://") || lower.startsWith("https://");
}

function looksLikeBareEmojiForIcon(s: string): boolean {
  return /\p{Extended_Pictographic}/u.test(s.trim());
}

/** @emoji 🔤️ Decodes a canonical icon string into a structured {@link Icon}. */
export function decodeIcon(encoded: string): Icon | undefined {
  const t = encoded.trim();
  if (t === "") {
    return undefined;
  }
  if (t.startsWith("url:")) {
    const url = t.slice("url:".length).trim();
    return url === "" ? undefined : { kind: "url", url };
  }
  if (looksLikeBareUrlForIcon(t)) {
    return { kind: "url", url: t };
  }
  if (looksLikeShortcodeToken(t)) {
    return { kind: "shortcode", code: t.slice(1, -1) };
  }
  if (t.startsWith("typst:")) {
    const src = t.slice("typst:".length).trim();
    return src === "" ? undefined : { kind: "typst", src };
  }
  if (t.startsWith("$")) {
    return { kind: "typst", src: t };
  }
  if (t.startsWith("emoji:")) {
    const emoji = t.slice("emoji:".length).trim();
    return emoji === "" ? undefined : { kind: "emoji", emoji };
  }
  if (t.startsWith("text:")) {
    const text = t.slice("text:".length).trim();
    return text === "" ? undefined : { kind: "text", text };
  }
  if (isRasterDataUrlPayloadForIcon(t) || isSvgDataUrlPayloadForIcon(t) || t.toLowerCase().startsWith("data:")) {
    return { kind: "data", data: t };
  }
  const lower = t.toLowerCase();
  if (lower.startsWith("<?xml") || lower.includes("<svg")) {
    return { kind: "svg", svg: t };
  }
  if (isMetabolismIconName(t)) {
    return { kind: "themed", key: t };
  }
  if (isIconName(t)) {
    return { kind: "catalog", key: t };
  }
  if (looksLikeAsciiCatalogishStemForIcon(t)) {
    return undefined;
  }
  if (looksLikeBareEmojiForIcon(t)) {
    return { kind: "emoji", emoji: t };
  }
  if ([...t].length <= 16) {
    return { kind: "text", text: t };
  }
  return undefined;
}

/** @emoji 🔤️ Encodes a structured {@link Icon} into the canonical wire string. */
export function encodeIcon(icon: Icon): string {
  switch (icon.kind) {
    case "url":
      return `url:${icon.url.trim()}`;
    case "shortcode":
      return `:${icon.code.trim()}:`;
    case "data":
      return icon.data.trim();
    case "emoji":
      return `emoji:${icon.emoji.trim()}`;
    case "typst":
      return icon.src.trim().startsWith("$") ? icon.src.trim() : `typst:${icon.src.trim()}`;
    case "text":
      return `text:${icon.text.trim()}`;
    case "svg":
      return icon.svg.trim();
    case "catalog":
      return icon.key;
    case "themed":
      return icon.key;
    case "node":
      return "";
  }
}

/** @emoji 🧭️ Picks an {@link IconSelectorMode} tab for a stored icon string. */
export function classifyIconSelectorMode(raw: string): IconSelectorMode {
  const icon = decodeIcon(raw);
  if (!icon) {
    return "math";
  }
  switch (icon.kind) {
    case "url":
      return "url";
    case "shortcode":
      return "shortcode";
    case "data":
      return "data";
    case "emoji":
      return "emoji";
    case "typst":
      return "math";
    case "text":
      return "text";
    case "svg":
    case "catalog":
    case "themed":
      return "vector";
    case "node":
      return "vector";
  }
}

function resolveShortcodeIcon(code: string): Icon {
  const key = code.trim();
  const lower = key.toLowerCase();
  const emoji = shortcodeEmoji(lower);
  if (emoji) {
    return { kind: "emoji", emoji };
  }
  const catalog = shortcodeCatalogKey(key);
  if (catalog) {
    return { kind: "catalog", key: catalog };
  }
  return { kind: "shortcode", code: key };
}

const iconUrlDataCache = ephemeralMap<string, string>("framework.modules.ui.elements.Icons.component.tsx.iconUrlDataCache");

async function fetchIconUrlAsDataUrl(url: string): Promise<string | undefined> {
  const cached = iconUrlDataCache.get(url);
  if (cached) {
    return cached;
  }
  try {
    const res = await fetch(url);
    if (!res.ok) {
      return undefined;
    }
    const blob = await res.blob();
    const data = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(typeof reader.result === "string" ? reader.result : "");
      reader.onerror = () => reject(reader.error);
      reader.readAsDataURL(blob);
    });
    const trimmed = data.trim();
    if (trimmed !== "") {
      iconUrlDataCache.set(url, trimmed);
    }
    return trimmed || undefined;
  } catch {
    return undefined;
  }
}

/** @emoji 🌐️ Prefetches `url:`/`http(s)` icons in board JSON to inline `data:` payloads before WASM sync. */
export async function resolveIconUrlsInBoardJson(json: string): Promise<string> {
  let root: unknown;
  try {
    root = JSON.parse(json);
  } catch {
    return json;
  }
  if (!root || typeof root !== "object") {
    return json;
  }
  const record = root as Record<string, unknown>;
  const pending: Promise<void>[] = [];
  const visit = (iconKind: unknown, apply: (next: string) => void) => {
    if (typeof iconKind !== "string" || iconKind.trim() === "") {
      return;
    }
    const icon = decodeIcon(iconKind);
    if (!icon || icon.kind !== "url") {
      return;
    }
    const cached = iconUrlDataCache.get(icon.url);
    if (cached) {
      apply(cached);
      return;
    }
    pending.push(
      fetchIconUrlAsDataUrl(icon.url).then((data) => {
        if (data) {
          apply(data);
        }
      }),
    );
  };
  for (const bucket of ["nodes", "handles"] as const) {
    const rows = record[bucket];
    if (!Array.isArray(rows)) {
      continue;
    }
    for (const row of rows) {
      if (!row || typeof row !== "object") {
        continue;
      }
      const obj = row as Record<string, unknown>;
      visit(obj.iconKind, (next) => {
        obj.iconKind = next;
      });
    }
  }
  if (pending.length === 0) {
    return json;
  }
  await Promise.all(pending);
  return JSON.stringify(root);
}

/** @emoji 🖼️ Icon payload: canonical union, vendored catalog name, or legacy shorthand. */
export type IconSource = Icon | IconName | { readonly name: IconName } | { readonly svg: string } | { readonly url: string } | { readonly node: React.ReactNode };

/** @emoji 🎛️ Required icon slot for chrome controls (buttons, toggles, actions). */
export type ControlIcon = IconSource | React.ReactElement;

function isIconSource(value: ControlIcon): value is IconSource {
  if (typeof value === "string") return true;
  if (typeof value === "object" && value !== null && !React.isValidElement(value)) {
    return "kind" in value || "name" in value || "svg" in value || "url" in value || "node" in value;
  }
  return false;
}

function useThemeIcons(): UiTheme["icons"] {
  return React.useSyncExternalStore(
    subscribeActiveUiTheme,
    () => activeUiTheme().icons,
    () => activeUiTheme().icons,
  );
}

/** @emoji 🖼️ Resolves catalog icon SVG for the active theme. */
export function resolveCatalogIconSvg(name: IconName, icons: UiTheme["icons"] = activeUiTheme().icons): string {
  return resolveCatalogIconSvgFromTheme(name, icons);
}

/** @emoji 🖼️ Resolves metabolism icon SVG for the active theme. */
export function resolveMetabolismIconSvg(name: MetabolismIconName, icons: UiTheme["icons"] = activeUiTheme().icons): string {
  return resolveMetabolismIconSvgFromTheme(name, icons);
}

const CATALOG_ICON_ALIASES: Partial<Record<string, IconName>> = {
  trash: "trash-2",
  menu: "list",
  "circle-off": "eye-off",
  "square-pen": "pencil",
  trees: "list-tree",
  gauge: "sliders-horizontal",
  "building-2": "building",
  compass: "focus",
  microscope: "search",
  "drafting-compass": "cad-shape",
  "search-check": "search",
  "file-chart-column": "bar-chart-3",
  blocks: "component",
  "flask-conical": "cylinder",
  blend: "combine",
  "user-plus": "user",
  "clipboard-paste": "clipboard",
  "clipboard-copy": "copy",
  "git-branch-plus": "git-branch",
  "git-compare": "git-branch",
  history: "clock",
  house: "home",
  "list-filter": "list",
  "notebook-pen": "book-open",
  pointer: "mouse-pointer",
  "scan-eye": "eye",
  "square-dashed-mouse-pointer": "mouse-pointer-2",
  upload: "hard-drive",
  video: "camera",
  "folder-tree": "folder",
  "layout-panel-left": "panel-left",
  "refresh-cw": "rotate-cw",
  "text-cursor-input": "typography",
  terminal: "code",
  zap: "sparkles",
  calculator: "hash",
};

function coerceIconSource(source: IconSource): Icon {
  if (typeof source === "string") {
    const key = source.trim();
    const alias = CATALOG_ICON_ALIASES[key];
    if (alias) {
      return { kind: "catalog", key: alias };
    }
    if ((ICONS as Record<string, string>)[key]) {
      return { kind: "catalog", key };
    }
    const emoji = shortcodeEmoji(key.toLowerCase());
    if (emoji) {
      return { kind: "emoji", emoji };
    }
    const catalog = shortcodeCatalogKey(key);
    if (catalog) {
      return { kind: "catalog", key: catalog };
    }
    const decoded = decodeIcon(key);
    if (decoded) {
      return decoded;
    }
    return { kind: "text", text: key };
  }
  if ("kind" in source) {
    return source;
  }
  if ("node" in source) {
    return { kind: "node", node: source.node };
  }
  if ("svg" in source) {
    return { kind: "svg", svg: source.svg };
  }
  if ("url" in source) {
    return { kind: "url", url: source.url };
  }
  return { kind: "catalog", key: source.name };
}

/** @emoji 🎛️ Renders a control icon or a visible missing-icon placeholder. */
export function renderControlIcon(icon: ControlIcon | undefined | null | false, size: number | IconSizeToken = "small"): React.ReactNode {
  if (icon === undefined || icon === null || icon === false) {
    return <span data-missing-icon data-icon-kind="missing" className="inline-flex size-small shrink-0 rounded-sm bg-destructive/30" aria-hidden />;
  }
  if (isIconSource(icon)) return <Icon icon={icon} size={size} />;
  return icon;
}

export interface IconProps {
  icon: IconSource;
  size?: number | IconSizeToken;
  className?: string;
  title?: UiLabel;
}

/** @emoji 🖼️ Raw vendored SVG markup for an icon name, or `undefined` when the name is not a vendored {@link IconName}. */
export function iconSvgMarkup(name: IconName): string {
  return resolveCatalogIconSvg(name);
}

const ICON_MASK_CACHE = ephemeralMap<string, string>("framework.modules.ui.elements.Icons.component.tsx.ICON_MASK_CACHE");

/** @emoji 🩻️ Alpha-mask image for an icon's own resolved SVG — lets CSS paint gradients (e.g. the celebrate conic) through the glyph instead of behind it. `currentColor` is baked to opaque black because a mask image renders in its own context and only its alpha channel is read. */
export function iconMaskImage(svgMarkup: string): string {
  const cached = ICON_MASK_CACHE.get(svgMarkup);
  if (cached) return cached;
  const forMask = svgMarkup.replace(/currentColor/gi, "#000");
  const url = `url("data:image/svg+xml,${encodeURIComponent(forMask)}")`;
  ICON_MASK_CACHE.set(svgMarkup, url);
  return url;
}

const ICON_SIZE_CLASS: Record<IconSizeToken, string> = {
  tiny: "size-tiny",
  small: "size-small",
  base: "size-workbench",
  large: "size-xl",
};

function iconBoxStyle(size: number | IconSizeToken): React.CSSProperties | undefined {
  if (typeof size !== "number") {
    return undefined;
  }
  return { width: uiSpacingLen(size / (STYLING_COMPACT_ROOT_PX * 0.2)), height: uiSpacingLen(size / (STYLING_COMPACT_ROOT_PX * 0.2)) };
}

function iconBoxClassName(size: number | IconSizeToken, className?: string): string {
  return cn(typeof size === "string" ? ICON_SIZE_CLASS[size] : undefined, className);
}

/** @emoji 🖼️ Renders canonical icons without depending on an external icon library. */
export function Icon({ icon, size = "base", className, title }: IconProps): React.ReactElement {
  const themeIcons = useThemeIcons();
  const boxStyle = iconBoxStyle(size);
  const boxClass = iconBoxClassName(size, className);
  let normalized = coerceIconSource(icon);
  if (normalized.kind === "shortcode") {
    normalized = resolveShortcodeIcon(normalized.code);
  }
  if (normalized.kind === "node") {
    return (
      <span data-icon-kind="node" className={cn("inline-flex shrink-0 items-center justify-center", boxClass)} style={boxStyle} title={title}>
        {normalized.node}
      </span>
    );
  }
  if (normalized.kind === "url" || normalized.kind === "data") {
    const src = normalized.kind === "url" ? normalized.url : normalized.data;
    return <img src={src} alt="" data-icon-kind="image" className={cn("shrink-0 object-contain", boxClass)} style={boxStyle} title={title} />;
  }
  if (normalized.kind === "emoji") {
    return (
      <span
        data-icon-kind="emoji"
        className={cn("inline-flex shrink-0 items-center justify-center text-base leading-none", boxClass)}
        style={{ ...boxStyle, fontFamily: "'Noto Color Emoji','Segoe UI Emoji',sans-serif" }}
        title={title}
        aria-hidden={title ? undefined : true}
      >
        {normalized.emoji}
      </span>
    );
  }
  if (normalized.kind === "text" || normalized.kind === "typst") {
    const label = normalized.kind === "text" ? normalized.text : normalized.src;
    return (
      <span data-icon-kind={normalized.kind} className={cn("inline-flex shrink-0 items-center justify-center font-mono text-xs", boxClass)} style={boxStyle} title={title}>
        {label}
      </span>
    );
  }
  const svgMarkup =
    normalized.kind === "svg" ? normalized.svg : normalized.kind === "catalog" ? resolveCatalogIconSvgFromTheme(normalized.key, themeIcons) : normalized.kind === "themed" ? resolveMetabolismIconSvgFromTheme(normalized.key, themeIcons) : undefined;
  if (!svgMarkup) {
    return (
      <span data-icon-kind={normalized.kind === "shortcode" ? "shortcode" : "missing"} className={cn("inline-flex shrink-0 items-center justify-center font-mono text-2xs text-muted-foreground", boxClass)} style={boxStyle} title={title}>
        {normalized.kind === "catalog" || normalized.kind === "themed" ? normalized.key.slice(0, 3) : "?"}
      </span>
    );
  }
  return (
    <span
      className={cn("inline-flex shrink-0 items-center justify-center [&>svg]:size-full", boxClass)}
      style={{ ...boxStyle, ["--icon-mask" as string]: iconMaskImage(svgMarkup) }}
      title={title}
      data-icon={normalized.kind === "catalog" ? normalized.key : normalized.kind === "themed" ? normalized.key : undefined}
      data-icon-kind={normalized.kind === "catalog" ? "catalog" : normalized.kind === "themed" ? "themed" : "svg"}
      dangerouslySetInnerHTML={{ __html: svgMarkup }}
      aria-hidden={title ? undefined : true}
    />
  );
}

// #endregion 🖼️IconCodec

/** @emoji 🔗️ Binds a built-in {@link IconName} for APIs expecting `ComponentType<{ size?: number }>`. */
export function createIconComponent(name: IconName): React.ComponentType<{ size?: number; className?: string }> {
  return function BoundIcon({ size = 16, className }: { size?: number; className?: string }) {
    return <Icon icon={name} size={size} className={className} />;
  };
}

export const AddIcon = createIconComponent("plus");
export const AlertCircleIcon = createIconComponent("alert-circle");
export const ArrowLeftIcon = createIconComponent("arrow-left");
export const AwardIcon = createIconComponent("award");
export const BookIcon = createIconComponent("book-open");
export const BoxIcon = createIconComponent("box");
export const CameraIcon = createIconComponent("camera");
export const ChatIcon = createIconComponent("message-circle");
export const CheckIcon = createIconComponent("check");
export const CheckIconAlt = CheckIcon;
export const ChevronDownIcon = createIconComponent("chevron-down");
export const ChevronDownIconAlt = ChevronDownIcon;
export const ChevronLeftIcon = createIconComponent("chevron-left");
export const ChevronRightIcon = createIconComponent("chevron-right");
export const ChevronUpIcon = createIconComponent("chevron-up");
export const ChevronsUpDownIcon = createIconComponent("chevrons-up-down");
export const CircleDotIcon = createIconComponent("circle-dot");
export const CloseIcon = createIconComponent("x");
export const CloseIconAlt = CloseIcon;
export const CodeIcon = createIconComponent("code");
export const ComponentIcon = createIconComponent("component");
export const ConnectionIcon = createIconComponent("network");
export const ConnectorIcon = createIconComponent("crosshair");
export const CopyIcon = createIconComponent("copy");
export const DetailsIcon = createIconComponent("info");
export const DiagramIcon = createIconComponent("grid-3x3");
export const DisconnectIcon = createIconComponent("link-2-off");
export const DocumentIcon = createIconComponent("file-text");
export const ExternalLinkIcon = createIconComponent("external-link");
export const FileArchiveIcon = createIconComponent("file-archive");
export const FileCodeIcon = createIconComponent("file-code");
export const FileImageIcon = createIconComponent("file-image");
export const FileJsonIcon = createIconComponent("file-json");
export const FileSpreadsheetIcon = createIconComponent("file-spreadsheet");
export const FileTypeIcon = createIconComponent("file-type");
export const FileVideoIcon = createIconComponent("file-video");
export const FilterIcon = createIconComponent("filter");
export const FindInViewIcon = createIconComponent("text-search");
export const FocusIcon = createIconComponent("focus");
export const FolderIcon = createIconComponent("folder");
export const FolderOpenIcon = createIconComponent("folder-open");
export const GlobeIcon = createIconComponent("globe");
export const GripVerticalIcon = createIconComponent("grip-vertical");
export const HandIcon = createIconComponent("hand");
export const HashIcon = createIconComponent("hash");
export const HomeIcon = createIconComponent("home");
export const HudIcon = createIconComponent("hud-overlay");
export const HudPanelIcon = createIconComponent("panel-top");
export const InfoIcon = createIconComponent("info");
export const IntersectIcon = createIconComponent("combine");
export const LandmarkIcon = createIconComponent("landmark");
export const LassoIcon = createIconComponent("lasso");
export const LayoutIcon = createIconComponent("layout");
export const LayoutGridIcon = createIconComponent("layout-grid");
export const LeftSidePanelIcon = createIconComponent("panel-left");
export const LightbulbIcon = createIconComponent("lightbulb");
export const LinkIcon = createIconComponent("link");
export const LoaderIcon = createIconComponent("loader-2");
export const LocalKitIcon = createIconComponent("hard-drive");
export const Maximize2Icon = createIconComponent("maximize-2");
export const MessageCircle = createIconComponent("message-circle");
export const MessageSquareIcon = createIconComponent("message-square");
export const Minimize2Icon = createIconComponent("minimize-2");
export const MonitorIcon = createIconComponent("monitor");
export const MoonIcon = createIconComponent("moon");
export const MoreHorizontalIcon = createIconComponent("more-horizontal");
export const MousePointerIcon = createIconComponent("mouse-pointer-2");
export const MoveIcon = createIconComponent("move");
export const NavigateBackIcon = createIconComponent("arrow-left");
export const NavigateForwardIcon = createIconComponent("arrow-right");
export const NavigateUpIcon = createIconComponent("arrow-up");
export const PanelRightIcon = createIconComponent("panel-right");
export const PauseIcon = createIconComponent("pause");
export const PieceIcon = createIconComponent("puzzle");
export const PlayIcon = createIconComponent("play");
export const PlugIcon = createIconComponent("plug");
export const PlusIcon = createIconComponent("plus");
export const PortIcon = createIconComponent("plug");
export const Puzzle2dIconFileImportIcon = createIconComponent("image-plus");
export const Puzzle2dIconMathGlyphIcon = createIconComponent("sigma");
export const Puzzle2dIconRasterGlyphIcon = createIconComponent("image");
export const RecordIcon = createIconComponent("circle");
export const RemoteKitIcon = createIconComponent("cloud");
export const RemoveIcon = createIconComponent("minus");
export const ResetIcon = createIconComponent("rotate-ccw");
export const RightSidePanelIcon = createIconComponent("panel-right");
export const SceneIcon = createIconComponent("scene-3d");
export const SearchIcon = createIconComponent("search");
export const SelectUtilityIcon = createIconComponent("mouse-pointer-2");
export const Settings2Icon = createIconComponent("settings-2");
export const SettingsIcon = createIconComponent("settings");
export const SkipBackIcon = createIconComponent("skip-back");
export const SkipForwardIcon = createIconComponent("skip-forward");
export const SmartphoneIcon = createIconComponent("smartphone");
export const SortAscendingIcon = createIconComponent("arrow-up");
export const SortDescendingIcon = createIconComponent("arrow-down");
export const StatsIcon = createIconComponent("bar-chart-3");
export const StopIcon = createIconComponent("square");
export const SunIcon = createIconComponent("sun");
export const TabletIcon = createIconComponent("tablet");
export const TableViewIcon = createIconComponent("table-2");
export const TemporaryKitIcon = createIconComponent("clock");
export const TriangleAlertIcon = createIconComponent("triangle-alert");
export const TutorialIcon = createIconComponent("graduation-cap");
export const TypeIcon = createIconComponent("typography");
export const UserIcon = createIconComponent("user");
export const UsersIcon = createIconComponent("users");
export const UtilitiesIcon = createIconComponent("wrench");
export const UtilityBarIcon = createIconComponent("hammer");
export const WorkbenchIcon = createIconComponent("workbench");

interface CursorProps {
  color: string;
  x?: number;
  y?: number;
}

/**
 **/
const Cursor: React.FC<CursorProps> = ({ color, x = 0, y = 0 }) => {
  return (
    <svg
      style={{
        position: "absolute",
        left: 0,
        top: 0,
        transform: `translateX(${x}px) translateY(${y}px)`,
      }}
      width="24"
      height="36"
      viewBox="0 0 24 36"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M5.65376 12.3673H5.46026L5.31717 12.4976L0.500002 16.8829L0.500002 1.19841L11.7841 12.3673H5.65376Z" fill={color} />
    </svg>
  );
};

export { Cursor };
// #endregion 🔖️Icon
