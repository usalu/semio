// #region 🧲️Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the VS Code extension bundling.

// #endregion 🧲️Header

// #region 🗄️Configuration
// Vite build configuration for the VS Code extension.
// Configuration MUST output a CJS bundle targeting Node 18.

// #region 🔌️Adapters
import path from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vite";
import { semioViteProductionBuild } from "../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
// #endregion 🔌️Adapters

/**
 * Absolute file path of the current module.
 * Path MUST be derived from import.meta.url.
 **/
const __filename = fileURLToPath(import.meta.url);
/**
 * Absolute directory path of the current module.
 * Path MUST be derived from __filename.
 **/
const __dirname = path.dirname(__filename);

// Vite configuration for building the VS Code extension as a CJS bundle.
// Export MUST call defineConfig with lib entry, rollup externals, and resolve aliases.
export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, "🟦️extension.ts"),
      formats: ["cjs"],
      fileName: () => "extension",
    },
    rollupOptions: {
      external: ["vscode"],
      output: {
        entryFileNames: "extension.js",
        format: "cjs",
        sourcemap: false,
      },
    },
    outDir: "out",
    emptyOutDir: true,
    ...semioViteProductionBuild({ target: "node18" }),
    ssr: true,
  },
  ssr: {
    noExternal: true,
  },
  resolve: {
    alias: {
      "@semio-tech/assets": path.resolve(__dirname, "../../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/📦️index.ts"),
    },
  },
});
// #endregion 🗄️Configuration
