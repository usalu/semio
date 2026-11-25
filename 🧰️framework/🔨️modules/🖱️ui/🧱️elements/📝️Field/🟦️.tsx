// #region 🧲️Header
// 💻️ framework/ui/elements/📝️Field/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { fieldAssociation } from "./🔗️association/🟦️.ts";
import { SharedBorderRow, SharedBorderRowCell } from "../🪢️SharedBorderRow/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🏷️Field
export interface FieldProps extends React.HTMLAttributes<HTMLDivElement> {
  /** 🪪️ The field ROW's own DOM id, and the stem of its control's — the label associates with
   * `${id}.control`, which is the id an authored `ui::field(...).try_child(control)` gives that child
   * (see `stepper_field` in puzzle 3d's settings panel). */
  readonly id?: string;
  /** 🎯️ The actual labelable child address; null marks a read-only presentation. */
  readonly controlId?: string | null;
  readonly label: React.ReactNode;
  readonly description?: React.ReactNode;
  readonly required?: boolean;
  readonly error?: React.ReactNode;
  readonly className?: string;
  readonly children: React.ReactNode;
}

/** 🏷️ Labelled form field: one vertical {@link SharedBorderRow} whose label, description and error are cells and whose control sits in its own nested row, so the control shares the field's hairlines instead of floating in a box of its own. */
export const Field: React.FC<FieldProps> = ({ id, controlId, label, description, required, error, className, children, ...props }) => {
  const association = fieldAssociation(id, controlId, Boolean(description), Boolean(error));
  const Label = association.target ? "label" : "span";
  return (
    <SharedBorderRow {...props} id={id} orientation="vertical" className={cn("min-w-0", className)} data-slot="field">
      <SharedBorderRowCell>
        <Label id={association.label} {...(association.target ? { htmlFor: association.target } : {})} className="text-sm font-medium text-foreground truncate" data-slot="field-label">
          {label}
          {required ? <span aria-hidden="true" className="text-destructive-foreground ms-half">*</span> : null}
        </Label>
      </SharedBorderRowCell>
      {description ? (
        <SharedBorderRowCell>
          <p id={association.description} className="text-xs text-muted-foreground" data-slot="field-description">
            {description}
          </p>
        </SharedBorderRowCell>
      ) : null}
      <SharedBorderRow orientation="vertical" data-slot="field-control" className="min-w-0">
        {children}
      </SharedBorderRow>
      {error ? (
        <SharedBorderRowCell tone="destructive">
          <p id={association.error} role="alert" className="text-xs text-destructive-foreground" data-slot="field-error">
            {error}
          </p>
        </SharedBorderRowCell>
      ) : null}
    </SharedBorderRow>
  );
};

// #endregion 🏷️Field
