// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🛡️AdminApp/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Tabs, TabsContent, TabsList, TabsTrigger } from "@semio-tech/ui-react";
import { ADMIN_LOCALES, useAdminLocale, useAdminT, type AdminI18nKey, type AdminLocale } from "../📚️I18n/🟦️.tsx";
import { AdminAccessGate, useAdminSession } from "../🔑️AdminSession/🟦️.tsx";
import { OverviewPage } from "../🏠️OverviewPage/🟦️.tsx";
import { SpacesPage } from "../🏛️SpacesPage/🟦️.tsx";
import { UsersPage } from "../🙋️UsersPage/🟦️.tsx";
import { ConnectionsPage } from "../🔴️ConnectionsPage/🟦️.tsx";
import { DocumentsPage } from "../📄️DocumentsPage/🟦️.tsx";
import { EventsPage } from "../📰️EventsPage/🟦️.tsx";
// #endregion 🔌️Adapters

const ADMIN_TABS = ["overview", "spaces", "users", "connections", "documents", "events"] as const;
type AdminTab = (typeof ADMIN_TABS)[number];

/** 🌐️ Plain locale toggle — deliberately not the shell's `UiDriver`/theme machinery (this app has no
 * shell chrome to theme, just a small self-contained locale pair). */
function LocaleSwitch(): React.ReactElement {
  const { locale, setLocale } = useAdminLocale();
  return (
    <Select id="admin-locale-switch-select" value={locale} onValueChange={(next) => setLocale(next as AdminLocale)}>
      <SelectTrigger id="admin-locale-switch" className="w-24">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {ADMIN_LOCALES.map((candidate) => (
          <SelectItem key={candidate} value={candidate}>
            {candidate.toUpperCase()}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

/** 🛡️ The whole admin SPA's tab shell — Overview/Spaces/Users/Connections/Documents/Events, gated
 * behind `AdminSessionProvider`'s auth probe (contract §C2's admin surface). Locale switch lives in
 * the header, next to the tabs, always visible regardless of auth state. */
export function AdminApp(): React.ReactElement {
  const t = useAdminT();
  const { status } = useAdminSession();
  const [tab, setTab] = React.useState<AdminTab>("overview");

  if (status !== "authorized") {
    return (
      <div className="flex h-full w-full flex-col">
        <header className="flex items-center justify-end border-b p-single">
          <LocaleSwitch />
        </header>
        <AdminAccessGate />
      </div>
    );
  }

  return (
    <div className="flex h-full w-full flex-col">
      <Tabs value={tab} onValueChange={(next) => setTab(next as AdminTab)} className="flex h-full w-full flex-col">
        <header className="flex items-center justify-between gap-single border-b p-single">
          <TabsList>
            {ADMIN_TABS.map((candidate) => (
              <TabsTrigger key={candidate} value={candidate} id={`admin-tab-${candidate}`}>
                {t(`admin.nav.${candidate}` as AdminI18nKey)}
              </TabsTrigger>
            ))}
          </TabsList>
          <LocaleSwitch />
        </header>
        <div className="min-h-0 flex-1 overflow-hidden">
          <TabsContent value="overview" className="h-full">
            <OverviewPage />
          </TabsContent>
          <TabsContent value="spaces" className="h-full">
            <SpacesPage />
          </TabsContent>
          <TabsContent value="users" className="h-full">
            <UsersPage />
          </TabsContent>
          <TabsContent value="connections" className="h-full">
            <ConnectionsPage />
          </TabsContent>
          <TabsContent value="documents" className="h-full">
            <DocumentsPage />
          </TabsContent>
          <TabsContent value="events" className="h-full">
            <EventsPage />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
}
