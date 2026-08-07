// #region 🧲️Header
// 💻️ framework/ui/elements/🧭️PageNavigation/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { Button } from "../🔘️Button/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🪩️PageNavigation

/**
 * Configuration interface for a previous/next page link.
 **/
export interface PageNavigationLink {
  path: string;
  title: string;
  section?: string;
}
/**
 **/
export interface PageNavigationProps {
  prev?: PageNavigationLink;
  next?: PageNavigationLink;
}

/**
 * PageNavigation holds the data fields for a PageNavigation record.
 **/
const PageNavigation: React.FC<PageNavigationProps> = ({ prev, next }) => {
  const navigate = useNavigate();
  const { t } = useTranslation();

  if (!prev && !next) return null;

  return (
    <div className="flex items-center justify-between border-t pt-4 mt-8">
      {prev ? (
        <Button id="ui.docs.navigation.previous" onClick={() => navigate(`/${prev.path}`)} className="flex items-center gap-single" icon="chevron-left">
          <div className="text-start">
            <div className="text-xs text-muted-foreground">{t("pageNavigation.previous")}</div>
            <div className="font-medium">{prev.title}</div>
          </div>
        </Button>
      ) : (
        <div />
      )}
      {next ? (
        <Button id="ui.docs.navigation.next" onClick={() => navigate(`/${next.path}`)} className="flex items-center gap-single" icon="chevron-right">
          <div className="text-end">
            <div className="text-xs text-muted-foreground">{t("pageNavigation.next")}</div>
            <div className="font-medium">{next.title}</div>
          </div>
        </Button>
      ) : (
        <div />
      )}
    </div>
  );
};

export { PageNavigation };

// #endregion 🪩️PageNavigation
