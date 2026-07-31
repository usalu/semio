// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// PostCSS configuration for the JavaScript workspace with Tailwind.

// #endregion 🧲Header

// #region 🔌Adapters
// PostCSS program configuration for the js library.
// Configuration MUST use the @tailwindcss/postcss program.

import { Config } from "postcss-load-config";
// #endregion 🔌Adapters

// #region 🗄️Configuration
/**
 * PostCSS configuration with the Tailwind CSS PostCSS program.
 * Config MUST include the @tailwindcss/postcss program.
 **/
const config: Config = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

// Default export of the PostCSS configuration.
// Export MUST be the config object.
export default config;
// #endregion 🗄️Configuration
