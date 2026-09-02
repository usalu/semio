// #region 🧲️Header
// 💻️ framework/ui/elements/📻️TableAvatar/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 📻️TableAvatar
type AvatarImageStatus = "loading" | "loaded" | "error";

/** 📻️ Owned native avatar props. */
export interface TableAvatarProps extends Omit<React.HTMLAttributes<HTMLSpanElement>, "children"> {
  icon?: string | React.ReactNode;
  name?: string;
  isSelected?: boolean;
  isHovered?: boolean;
  fallbackStyle?: React.CSSProperties;
}

/** 📻️ Renders a native image with an immediate fallback until the current source loads. */
export const TableAvatar = reactHostPort.forwardRef<HTMLSpanElement, TableAvatarProps>(function TableAvatar({ id, icon, name, className, isSelected, isHovered, style, fallbackStyle, ...props }, ref) {
  const normalizedName = (name ?? "").trim();
  const initials = normalizedName
    ? normalizedName
        .split(" ")
        .slice(0, 2)
        .map((word: string) => word.charAt(0))
        .join("")
        .toUpperCase()
        .substring(0, 2)
    : "";
  const isImageIcon = typeof icon === "string" && icon.length > 0;
  const isReactIcon = icon && !isImageIcon;
  const imageSource = isImageIcon ? icon : undefined;
  const [imageState, setImageState] = reactHostPort.useState<{ source: string; status: AvatarImageStatus }>(() => ({ source: imageSource ?? "", status: "loading" }));
  const imageLoaded = Boolean(imageSource && imageState.source === imageSource && imageState.status === "loaded");
  const isSizeClass = className && (className.includes("size-") || className.includes("w-") || className.includes("h-"));
  const isFullSize = className && className.includes("size-full");
  const hasExplicitSize = style && (style.width || style.height);

  return (
    <span
      ref={ref}
      data-slot="avatar"
      id={id}
      style={style}
      className={cn(
        "relative flex overflow-hidden rounded-full",
        !hasExplicitSize && "shrink-0",
        !isFullSize && "border",
        !isSizeClass && !hasExplicitSize && "size-small",
        className,
        isSelected && "ring-1 ring-[color:var(--active-base)]",
        isHovered && "ring-1 ring-[color:var(--hover-base)]",
      )}
      {...props}
    >
      {imageSource ? (
        <img
          key={imageSource}
          data-slot="avatar-image"
          className="aspect-square size-full"
          src={imageSource}
          alt={normalizedName}
          hidden={!imageLoaded}
          onLoad={() => setImageState({ source: imageSource, status: "loaded" })}
          onError={() => setImageState({ source: imageSource, status: "error" })}
        />
      ) : null}
      <span
        data-slot="avatar-fallback"
        role={normalizedName ? "img" : undefined}
        aria-label={normalizedName || undefined}
        hidden={imageLoaded}
        style={fallbackStyle}
        className={cn("bg-muted flex size-full items-center justify-center rounded-full text-xs", isSelected ? "bg-[color:var(--active-base)] text-[color:var(--active-foreground)]" : isHovered ? "bg-[color:var(--hover-base)]" : "")}
      >
        {isReactIcon ? icon : initials}
      </span>
    </span>
  );
});
TableAvatar.displayName = "TableAvatar";
// #endregion 📻️TableAvatar
