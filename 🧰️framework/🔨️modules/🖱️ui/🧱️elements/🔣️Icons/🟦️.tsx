// #region 🧲️Header
// 💻️ framework/ui/elements/🔣️Icons/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { domSizePx, activeUiTheme, subscribeActiveUiTheme, STYLING_COMPACT_ROOT_PX, uiSpacingLen, type UiTheme } from "@semio-tech/ui-styling";
import { ephemeralMap } from "@semio-tech/framework-transient";
import {
  ICONS,
  isIconName,
  resolveCatalogIconSvgFromTheme,
  shortcodeCatalogKey,
  shortcodeEmoji,
  type IconName,
} from "@semio-tech/assets";
import { type MetabolismIconName } from "@semio-tech/assets";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
export type { IconName };
// #endregion 🔌️Adapters

// #region 🌱️ThemedIconCatalog
/** 🌱️ The second, themed icon catalog (the metabolism icons) that {@link Icon} resolves next to the built-in one. It is
 * installed by the host rather than imported here, so a page that renders only built-in icons does not ship it; the
 * React target barrel installs `@semio-tech/assets`' metabolism catalog when it loads. */
export interface ThemedIconCatalog {
  readonly has: (key: string) => key is MetabolismIconName;
  readonly svg: (name: MetabolismIconName, icons: UiTheme["icons"] | undefined) => string;
}

let themedIconCatalog: ThemedIconCatalog | undefined;

/** 🔌️ Installs (or with `undefined` removes) the {@link ThemedIconCatalog}; returns the one installed before. */
export function setThemedIconCatalog(catalog: ThemedIconCatalog | undefined): ThemedIconCatalog | undefined {
  const previous = themedIconCatalog;
  themedIconCatalog = catalog;
  return previous;
}

function isThemedIconName(key: string): key is MetabolismIconName {
  return themedIconCatalog?.has(key) ?? false;
}

function themedIconSvg(name: MetabolismIconName, icons: UiTheme["icons"] | undefined): string | undefined {
  return themedIconCatalog?.svg(name, icons);
}
// #endregion 🌱️ThemedIconCatalog

// #region 🔖️Icon
/** 📐️ Named size tokens for {@link Icon}. */
export type IconSizeToken = "tiny" | "small" | "base" | "large";

const ICON_SIZE_PX: Record<IconSizeToken, number> = {
  tiny: domSizePx("iconTinyUiSpacing"),
  small: domSizePx("iconSmallUiSpacing"),
  base: domSizePx("iconBaseUiSpacing"),
  large: domSizePx("iconLargeUiSpacing"),
};

/** 📐️ Resolves {@link Icon} `size` to pixel dimensions. */
export function resolveIconSizePx(size?: number | IconSizeToken): number {
  if (size === undefined) return ICON_SIZE_PX.base;
  if (typeof size === "number") return size;
  return ICON_SIZE_PX[size];
}

// #region 🖼️IconCodec

/** 🖼️ Canonical structured icon payload shared across canvases and UI chrome. */
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

/** 🎛️ Shared icon editor tab buckets aligned with {@link Icon}. */
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

/** 🔤️ Decodes a canonical icon string into a structured {@link Icon}. */
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
  if (isThemedIconName(t)) {
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

/** 🔤️ Encodes a structured {@link Icon} into the canonical wire string. */
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

/** 🧭️ Picks an {@link IconSelectorMode} tab for a stored icon string. */
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

/** 🖼️ Icon payload: canonical union, vendored catalog name, or legacy shorthand. */
export type IconSource = Icon | IconName | { readonly name: IconName } | { readonly svg: string } | { readonly url: string } | { readonly node: React.ReactNode };

/** 🎛️ Required icon slot for chrome controls (buttons, toggles, actions). */
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

/** 🖼️ Resolves catalog icon SVG for the active theme. */
export function resolveCatalogIconSvg(name: IconName, icons: UiTheme["icons"] = activeUiTheme().icons): string {
  return resolveCatalogIconSvgFromTheme(name, icons);
}

/** 🖼️ Resolves metabolism icon SVG for the active theme through the installed {@link ThemedIconCatalog} (empty without one). */
export function resolveMetabolismIconSvg(name: MetabolismIconName, icons: UiTheme["icons"] = activeUiTheme().icons): string {
  return themedIconSvg(name, icons) ?? "";
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
  ellipsis: "more-horizontal",
};

function catalogIconName(key: string): IconName | undefined {
  return CATALOG_ICON_ALIASES[key] ?? (isIconName(key) ? key : undefined);
}

function coerceIconSource(source: IconSource): Icon {
  if (typeof source === "string") {
    const key = source.trim();
    const named = catalogIconName(key);
    if (named !== undefined) return { kind: "catalog", key: named };
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

/** 🎛️ Renders a control icon or a visible missing-icon placeholder. */
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

/** 🗂️ Props for a named catalog glyph without a rich icon source decoder. */
export interface CatalogIconProps extends Omit<IconProps, "icon"> {
  readonly icon: IconName;
}

/** 🖼️ Raw vendored SVG markup for an icon name, or `undefined` when the name is not a vendored {@link IconName}. */
export function iconSvgMarkup(name: IconName): string {
  return resolveCatalogIconSvg(name);
}

const ICON_MASK_CACHE = ephemeralMap<string, string>("framework.modules.ui.elements.Icons.component.tsx.ICON_MASK_CACHE");

/** 🩻️ Alpha-mask image for an icon's own resolved SVG — lets CSS paint gradients (e.g. the celebrate conic) through the glyph instead of behind it. `currentColor` is baked to opaque black because a mask image renders in its own context and only its alpha channel is read. */
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

/** 🖌️ Renders a named catalog glyph synchronously, keeping shortcode and rich source codecs outside slim chrome. */
export function CatalogIcon({ icon, size = "base", className, title }: CatalogIconProps): React.ReactElement {
  const themeIcons = useThemeIcons();
  const name = catalogIconName(icon)!;
  const svgMarkup = resolveCatalogIconSvgFromTheme(name, themeIcons);
  const boxStyle = iconBoxStyle(size);
  return <span className={cn("inline-flex shrink-0 items-center justify-center [&>svg]:size-full", iconBoxClassName(size, className))} style={{ ...boxStyle, ["--icon-mask" as string]: iconMaskImage(svgMarkup) }} title={title} data-icon={name} data-icon-kind="catalog" dangerouslySetInnerHTML={{ __html: svgMarkup }} aria-hidden={title ? undefined : true} />;
}

/** 🖼️ Renders canonical icons without depending on an external icon library. */
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
    normalized.kind === "svg" ? normalized.svg : normalized.kind === "catalog" ? resolveCatalogIconSvgFromTheme(normalized.key, themeIcons) : normalized.kind === "themed" ? themedIconSvg(normalized.key, themeIcons) : undefined;
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

/** 🔗️ Binds a built-in {@link IconName} for APIs expecting `ComponentType<{ size?: number }>`. */
export function createIconComponent(name: IconName): React.ComponentType<{ size?: number; className?: string }> {
  return function BoundIcon({ size = 16, className }: { size?: number; className?: string }) {
    return <CatalogIcon icon={name} size={size} className={className} />;
  };
}

export const AddIcon = /* @__PURE__ */ createIconComponent("plus");
export const AlertCircleIcon = /* @__PURE__ */ createIconComponent("alert-circle");
export const ArrowLeftIcon = /* @__PURE__ */ createIconComponent("arrow-left");
export const AwardIcon = /* @__PURE__ */ createIconComponent("award");
export const BookIcon = /* @__PURE__ */ createIconComponent("book-open");
export const BoxIcon = /* @__PURE__ */ createIconComponent("box");
export const CameraIcon = /* @__PURE__ */ createIconComponent("camera");
export const ChatIcon = /* @__PURE__ */ createIconComponent("message-circle");
export const CheckIcon = /* @__PURE__ */ createIconComponent("check");
export const CheckIconAlt = CheckIcon;
export const ChevronDownIcon = /* @__PURE__ */ createIconComponent("chevron-down");
export const ChevronDownIconAlt = ChevronDownIcon;
export const ChevronLeftIcon = /* @__PURE__ */ createIconComponent("chevron-left");
export const ChevronRightIcon = /* @__PURE__ */ createIconComponent("chevron-right");
export const ChevronUpIcon = /* @__PURE__ */ createIconComponent("chevron-up");
export const ChevronsUpDownIcon = /* @__PURE__ */ createIconComponent("chevrons-up-down");
export const CircleDotIcon = /* @__PURE__ */ createIconComponent("circle-dot");
export const CloseIcon = /* @__PURE__ */ createIconComponent("x");
export const CloseIconAlt = CloseIcon;
export const CodeIcon = /* @__PURE__ */ createIconComponent("code");
export const ComponentIcon = /* @__PURE__ */ createIconComponent("component");
export const ConnectionIcon = /* @__PURE__ */ createIconComponent("network");
export const ConnectorIcon = /* @__PURE__ */ createIconComponent("crosshair");
export const CopyIcon = /* @__PURE__ */ createIconComponent("copy");
export const DetailsIcon = /* @__PURE__ */ createIconComponent("info");
export const DiagramIcon = /* @__PURE__ */ createIconComponent("grid-3x3");
export const DisconnectIcon = /* @__PURE__ */ createIconComponent("link-2-off");
export const DocumentIcon = /* @__PURE__ */ createIconComponent("file-text");
export const ExternalLinkIcon = /* @__PURE__ */ createIconComponent("external-link");
export const FileArchiveIcon = /* @__PURE__ */ createIconComponent("file-archive");
export const FileCodeIcon = /* @__PURE__ */ createIconComponent("file-code");
export const FileImageIcon = /* @__PURE__ */ createIconComponent("file-image");
export const FileJsonIcon = /* @__PURE__ */ createIconComponent("file-json");
export const FileSpreadsheetIcon = /* @__PURE__ */ createIconComponent("file-spreadsheet");
export const FileTypeIcon = /* @__PURE__ */ createIconComponent("file-type");
export const FileVideoIcon = /* @__PURE__ */ createIconComponent("file-video");
export const FilterIcon = /* @__PURE__ */ createIconComponent("filter");
export const FindInViewIcon = /* @__PURE__ */ createIconComponent("text-search");
export const FocusIcon = /* @__PURE__ */ createIconComponent("focus");
export const FolderIcon = /* @__PURE__ */ createIconComponent("folder");
export const FolderOpenIcon = /* @__PURE__ */ createIconComponent("folder-open");
export const GlobeIcon = /* @__PURE__ */ createIconComponent("globe");
export const GripVerticalIcon = /* @__PURE__ */ createIconComponent("grip-vertical");
export const HandIcon = /* @__PURE__ */ createIconComponent("hand");
export const HashIcon = /* @__PURE__ */ createIconComponent("hash");
export const HomeIcon = /* @__PURE__ */ createIconComponent("home");
export const HudIcon = /* @__PURE__ */ createIconComponent("hud-overlay");
export const HudPanelIcon = /* @__PURE__ */ createIconComponent("panel-top");
export const InfoIcon = /* @__PURE__ */ createIconComponent("info");
export const IntersectIcon = /* @__PURE__ */ createIconComponent("combine");
export const LandmarkIcon = /* @__PURE__ */ createIconComponent("landmark");
export const LassoIcon = /* @__PURE__ */ createIconComponent("lasso");
export const LayoutIcon = /* @__PURE__ */ createIconComponent("layout");
export const LayoutGridIcon = /* @__PURE__ */ createIconComponent("layout-grid");
export const LeftSidePanelIcon = /* @__PURE__ */ createIconComponent("panel-left");
export const LightbulbIcon = /* @__PURE__ */ createIconComponent("lightbulb");
export const LinkIcon = /* @__PURE__ */ createIconComponent("link");
export const LoaderIcon = /* @__PURE__ */ createIconComponent("loader-2");
export const LocalKitIcon = /* @__PURE__ */ createIconComponent("hard-drive");
export const Maximize2Icon = /* @__PURE__ */ createIconComponent("maximize-2");
export const MessageCircle = /* @__PURE__ */ createIconComponent("message-circle");
export const MessageSquareIcon = /* @__PURE__ */ createIconComponent("message-square");
export const Minimize2Icon = /* @__PURE__ */ createIconComponent("minimize-2");
export const MonitorIcon = /* @__PURE__ */ createIconComponent("monitor");
export const MoonIcon = /* @__PURE__ */ createIconComponent("moon");
export const MoreHorizontalIcon = /* @__PURE__ */ createIconComponent("more-horizontal");
export const MousePointerIcon = /* @__PURE__ */ createIconComponent("mouse-pointer-2");
export const MoveIcon = /* @__PURE__ */ createIconComponent("move");
export const NavigateBackIcon = /* @__PURE__ */ createIconComponent("arrow-left");
export const NavigateForwardIcon = /* @__PURE__ */ createIconComponent("arrow-right");
export const NavigateUpIcon = /* @__PURE__ */ createIconComponent("arrow-up");
export const PanelRightIcon = /* @__PURE__ */ createIconComponent("panel-right");
export const PauseIcon = /* @__PURE__ */ createIconComponent("pause");
export const PieceIcon = /* @__PURE__ */ createIconComponent("puzzle");
export const PlayIcon = /* @__PURE__ */ createIconComponent("play");
export const PlugIcon = /* @__PURE__ */ createIconComponent("plug");
export const PlusIcon = /* @__PURE__ */ createIconComponent("plus");
export const PortIcon = /* @__PURE__ */ createIconComponent("plug");
export const Puzzle2dIconFileImportIcon = /* @__PURE__ */ createIconComponent("image-plus");
export const Puzzle2dIconMathGlyphIcon = /* @__PURE__ */ createIconComponent("sigma");
export const Puzzle2dIconRasterGlyphIcon = /* @__PURE__ */ createIconComponent("image");
export const RecordIcon = /* @__PURE__ */ createIconComponent("circle");
export const RemoteKitIcon = /* @__PURE__ */ createIconComponent("cloud");
export const RemoveIcon = /* @__PURE__ */ createIconComponent("minus");
export const ResetIcon = /* @__PURE__ */ createIconComponent("rotate-ccw");
export const RightSidePanelIcon = /* @__PURE__ */ createIconComponent("panel-right");
export const SceneIcon = /* @__PURE__ */ createIconComponent("scene-3d");
export const SearchIcon = /* @__PURE__ */ createIconComponent("search");
export const SelectUtilityIcon = /* @__PURE__ */ createIconComponent("mouse-pointer-2");
export const Settings2Icon = /* @__PURE__ */ createIconComponent("settings-2");
export const SettingsIcon = /* @__PURE__ */ createIconComponent("settings");
export const SkipBackIcon = /* @__PURE__ */ createIconComponent("skip-back");
export const SkipForwardIcon = /* @__PURE__ */ createIconComponent("skip-forward");
export const SmartphoneIcon = /* @__PURE__ */ createIconComponent("smartphone");
export const SortAscendingIcon = /* @__PURE__ */ createIconComponent("arrow-up");
export const SortDescendingIcon = /* @__PURE__ */ createIconComponent("arrow-down");
export const StatsIcon = /* @__PURE__ */ createIconComponent("bar-chart-3");
export const StopIcon = /* @__PURE__ */ createIconComponent("square");
export const SunIcon = /* @__PURE__ */ createIconComponent("sun");
export const TabletIcon = /* @__PURE__ */ createIconComponent("tablet");
export const TableViewIcon = /* @__PURE__ */ createIconComponent("table-2");
export const TemporaryKitIcon = /* @__PURE__ */ createIconComponent("clock");
export const TriangleAlertIcon = /* @__PURE__ */ createIconComponent("triangle-alert");
export const TutorialIcon = /* @__PURE__ */ createIconComponent("graduation-cap");
export const TypeIcon = /* @__PURE__ */ createIconComponent("typography");
export const UserIcon = /* @__PURE__ */ createIconComponent("user");
export const UsersIcon = /* @__PURE__ */ createIconComponent("users");
export const UtilitiesIcon = /* @__PURE__ */ createIconComponent("wrench");
export const UtilityBarIcon = /* @__PURE__ */ createIconComponent("hammer");
export const WorkbenchIcon = /* @__PURE__ */ createIconComponent("workbench");

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
