// #region 🧲️Header
/** 🛡️ `os-hub-admin` entry — mounts `AdminApp` under the explicit locale and protected relay session
 * providers. Authenticated development is served only by `os-hub:dev-secure-admin`; Vite is a
 * static, authority-free UI iteration surface. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createRoot } from "react-dom/client";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { AdminLocaleProvider } from "../../🧱️elements/📚️I18n/🟦️.tsx";
import { AdminSessionProvider } from "../../🧱️elements/🔑️AdminSession/🟦️.tsx";
import { AdminApp } from "../../🧱️elements/🛡️AdminApp/🟦️.tsx";
import "./🎨️.css";
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
