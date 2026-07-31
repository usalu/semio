// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Next.js configuration for the repo server app.
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { NextConfig } from "next";
// #endregion 🔌️Adapters

const nextConfig: NextConfig = {
  output: "standalone",
  serverExternalPackages: ["pg", "pg-boss"],
};

export default nextConfig;
