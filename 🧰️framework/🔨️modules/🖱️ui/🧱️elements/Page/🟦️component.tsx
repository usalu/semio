// #region 🧲️Header
// 💻️ framework/ui/elements/Page/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf file
// without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
import { type UiLabel, reactHostPort, getElementById, Scrollable } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🌈️Page
// Full-page content wrapper with frontmatter and footer.
// Consumers MUST provide frontmatter and children.

/**
 * Frontmatter metadata interface for a documentation page.
 **/
export interface PageFrontmatter {
  title?: UiLabel;
  description?: string;
  icon?: string;
  sidebar?: boolean;
  order?: number;
  concepts?: string[];
}

/**
 * Props interface for the Page component.
 **/
export interface PageProps {
  frontmatter?: PageFrontmatter;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  footer?: React.ReactNode;
  children: React.ReactNode;
}

/**
 * Full-page wrapper with frontmatter header and footer.
 **/
export const Page: React.FC<PageProps> = ({ frontmatter, focusedItemId, onFocusComplete, footer, children }) => {
  const scrollAreaRef = reactHostPort.useRef<HTMLDivElement>(null);

  reactHostPort.useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const element = getElementById(focusedItemId);
      if (element) {
        element.scrollIntoView({ behavior: "smooth", block: "center" });
        if (onFocusComplete) {
          setTimeout(() => onFocusComplete(), 600);
        }
      }
    }
  }, [focusedItemId, onFocusComplete]);
  return (
    <Scrollable ref={scrollAreaRef} className="h-full w-full">
      <div className="prose prose-sm max-w-none dark:prose-invert p-medium">
        {frontmatter?.title && <h1>{frontmatter.title}</h1>}
        {frontmatter?.description && <p className="text-muted-foreground">{frontmatter.description}</p>}
        {children}
        {footer}
      </div>
    </Scrollable>
  );
};
// #endregion 🌈️Page
