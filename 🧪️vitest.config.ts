// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Root Vitest configuration for the monorepo test runner.

// #endregion 🧲️Header

// #region 🗄️Configuration
// Root Vitest configuration aggregating all workspace test projects.
// Configuration MUST reference all workspace vite config files that define tests.

// #region 🔌️Adapters
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

export default defineConfig({
  test: {
    projects: [
      "./compose/client/lib/js/⚙️vite.config.ts",
      "./compose/client/lib/sketchpad/js/🧪️vitest.config.ts",
      "./compose/dev/algorithm/js/🧪️vitest.config.ts",
      "./🧰️framework/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔨️modules/◻2d/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔨️modules/🧊️3d/⚡️implementations/🟦️typescript/📐️brep/🧪️vitest.config.ts",
      "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🟦️typescript/🖼️canvas/🎨️react-renderer/🧪️vitest.config.ts",
      "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🟦️typescript/🌍️world/🎨️r3f/🧪️vitest.config.ts",
      "./🧰️framework/🔨️modules/🧮️math/⚡️implementations/🟦️typescript/🕸️graph/🗣️dsl/🫀️core/🟦️typescript/🧪️vitest.config.ts",
      "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/⚡️implementations/🟦️typescript/🧑️‍🎨️engine/⚛️react/🧪️vitest.config.ts",
      "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/⚡️implementations/🦀️rust/🧑️‍🎨️engine/🧊️wgpu/🧪️vitest.config.ts",
      "./🧰️framework/🔨️modules/🖱️ui/⚡️implementations/🟦️typescript/⚛️react/🧪️vitest.config.ts",
      "./🧰️framework/🔨️modules/🖱️ui/⚡️implementations/🦀️rust/🎨️styling/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🔨️modules/📐️brepjs/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🔨️modules/🎰️stately/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🔨️modules/🔍️query/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🔨️modules/📺️renderer/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
      "./✏️s/🔌️plugins/📐️cad/🔨️modules/🏃️runtime/⚡️implementations/🟦️typescript/🧪️vitest.config.ts",
    ],
  },
});

// #endregion 🗄️Configuration
