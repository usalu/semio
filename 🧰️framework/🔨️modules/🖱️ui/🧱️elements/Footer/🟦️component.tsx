// #region 🧲️Header
// 💻️ framework/ui/elements/Footer/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🫀️core/ClassNames/🟦️component.tsx";
import { type NavbarItem } from "../Navbar/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { shellFloorPaints, shellFloorFillClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { useSurface, SurfaceScope } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🎮️Footer
// Bottom navigation bar, symmetric to Navbar — normal document flow, border on top.
// Consumers MUST provide NavbarItem entries.

/**
 * Props interface for the Footer component.
 **/
export interface FooterProps {
  items: NavbarItem[];
  className?: string;
}

/** @emoji 🪟️ Footer mirrors {@link Navbar} exactly (normal flow, centered-item overlay) but anchored to the bottom edge with the border on top instead of the bottom. */
const Footer: React.FC<FooterProps> = ({ items, className = "" }) => {
  const parent = useSurface();
  const paints = shellFloorPaints(parent);
  const bgClass = shellFloorFillClass(parent);
  const normalItems = items.filter((item) => !item.centered);
  const centeredItems = items.filter((item) => item.centered);
  const body = (
    <>
      <div className="p-single flex gap-single items-center min-w-0 h-full">
        {normalItems.map((item, index) => (
          <div key={item.key ?? index} className={cn("h-medium flex shrink-0 items-center min-w-0", item.className)}>
            {item.content}
          </div>
        ))}
      </div>
      {centeredItems.map((item, index) => (
        <div key={item.key ?? index} className="pointer-events-none absolute inset-0 flex items-center justify-center">
          <div className={cn("pointer-events-auto h-medium flex items-center", item.className)}>{item.content}</div>
        </div>
      ))}
    </>
  );
  return (
    <footer id="ui.footer" data-slot="footer" data-level="base" data-ui-reveal-region="footer" data-elevation-root="" className={cn("relative h-large z-navbar", bgClass, className)}>
      {paints ? <SurfaceScope level="base" fill="surface">{body}</SurfaceScope> : body}
    </footer>
  );
};

export { Footer };

// #endregion 🎮️Footer
