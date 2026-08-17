// #region 🧲️Header
/** @emoji 🛡️ `os-hub-admin` entry — mounts `AdminApp` under `AdminLocaleProvider`/`AdminSessionProvider`.
 * Served by the hub itself at `/admin` in production (see `📦️bin.rs`'s `🔖️AdminPage` region); in dev,
 * `⚙️vite.config.ts` proxies `/directory`, `/admin/api`, `/auth`, `/spaces` to the hub so this page
 * behaves identically at both the `8790` dev port and the hub's own `/admin`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createRoot } from "react-dom/client";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { AdminLocaleProvider } from "../../🧱️elements/📚️I18n/🟦️component.tsx";
import { AdminSessionProvider } from "../../🧱️elements/🔑️AdminSession/🟦️component.tsx";
import { AdminApp } from "../../🧱️elements/🛡️AdminApp/🟦️component.tsx";
import "./🎨️globals.css";
// #endregion 🔌️Adapters

bootstrapElementsSurfaceChromeDocument();

const container = document.getElementById("root");
if (!container) throw new Error("os-hub-admin: #root not found");

createRoot(container).render(
  <AdminLocaleProvider>
    <AdminSessionProvider baseUrl={window.location.origin}>
      <AdminApp />
    </AdminSessionProvider>
  </AdminLocaleProvider>,
);
