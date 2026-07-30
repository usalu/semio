// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Dashboard landing page for the repo server.
// #endregion 🧲Header

export default function DashboardPage() {
  return (
    <main style={{ padding: "2rem", fontFamily: "system-ui" }}>
      <h1>compose repo</h1>
      <p>Monorepo management server</p>
      <nav>
        <ul>
          <li>
            <a href="/dashboard">Dashboard</a>
          </li>
          <li>
            <a href="/admin/developers">Admin: Developers</a>
          </li>
        </ul>
      </nav>
    </main>
  );
}
