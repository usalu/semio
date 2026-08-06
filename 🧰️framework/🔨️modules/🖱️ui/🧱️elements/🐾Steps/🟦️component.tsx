// #region 🧲️Header
// 💻️ framework/ui/elements/🐾Steps/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// #endregion 🔌️Adapters

// #region 🪬️Steps
// Ordered step list container for tutorial or wizard flows.
// Consumers MUST provide step children in order.

/**
 * Props interface for the Steps component.
 **/
export interface StepsProps {
  children: React.ReactNode;
  className?: string;
}

/**
 * Ordered step list container rendering numbered children.
 **/
export const Steps: React.FC<StepsProps> = ({ children, className = "" }) => {
  return <ol className={`flex flex-col gap-medium ${className}`}>{children}</ol>;
};

// #endregion 🪬️Steps
