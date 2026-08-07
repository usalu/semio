// #region 🧲️Header
// 💻️ framework/ui/elements/📝Field/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🏷️Field
export interface FieldProps {
  readonly id?: string;
  readonly label: React.ReactNode;
  readonly description?: React.ReactNode;
  readonly required?: boolean;
  readonly error?: React.ReactNode;
  readonly className?: string;
  readonly children: React.ReactNode;
}

/** @emoji 🏷️ Labelled form field wrapper with description and validation message. */
export const Field: React.FC<FieldProps> = ({ id, label, description, required, error, className, children }) => {
  return (
    <div className={cn("flex flex-col gap-single min-w-0", className)} data-slot="field">
      <div className="flex items-baseline gap-single min-w-0">
        <label htmlFor={id} className="text-sm font-medium text-foreground truncate" data-slot="field-label">
          {label}
          {required ? <span className="text-destructive ms-half">*</span> : null}
        </label>
      </div>
      {description ? (
        <p className="text-xs text-muted-foreground" data-slot="field-description">
          {description}
        </p>
      ) : null}
      <div data-slot="field-control" className="min-w-0">
        {children}
      </div>
      {error ? (
        <p className="text-xs text-destructive" data-slot="field-error">
          {error}
        </p>
      ) : null}
    </div>
  );
};

// #endregion 🏷️Field
