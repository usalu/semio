// #region 🧲️Header
// 💻️ framework/ui/elements/🎴️Card/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
import { Icon, type IconSource } from "../🔣️Icons/🟦️component.tsx";
import { ContextMenu, type ContextMenuItem } from "../🖱️ContextMenu/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🎬️Card
// Card container and grid layout for content blocks.
/**
 * Props interface for the Card component.
 *
 **/
export interface CardProps {
  title: string;
  icon?: string | IconSource;
  children: React.ReactNode;
  className?: string;
  contextMenu?: ContextMenuItem[];
}

/**
 * Content card with title, icon, and children.
 **/
export const Card: React.FC<CardProps> = ({ title, icon, children, className = "", contextMenu }) => {
  const contextMenuTitle = useLabel("ui.common.actions");
  return (
    <ContextMenu items={contextMenu} title={contextMenuTitle}>
      <div className={`border p-single ${className}`}>
        <div className="flex items-start gap-tiny mb-single">
          {icon && typeof icon !== "string" && <Icon icon={icon} size="small" className="flex-shrink-0 mt-0.5" />}
          {typeof icon === "string" && <span className="text-xl flex-shrink-0">{icon}</span>}
          <h3 className="font-semibold text-base">{title}</h3>
        </div>
        <div className="text-sm">{children}</div>
      </div>
    </ContextMenu>
  );
};

/**
 * Props interface for the CardGrid component.
 **/
export interface CardGridProps {
  stagger?: boolean;
  className?: string;
  children: React.ReactNode;
}

/** 📐️ Lays out children in a responsive card grid (1-2 columns). */
export const CardGrid: React.FC<CardGridProps> = ({ stagger = false, children, className = "" }) => {
  return <div className={`grid grid-cols-1 md:grid-cols-2 gap-medium my-medium ${className}`}>{children}</div>;
};

// #endregion 🎬️Card
