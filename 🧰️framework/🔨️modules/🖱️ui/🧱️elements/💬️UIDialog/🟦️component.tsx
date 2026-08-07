// #region 🧲️Header
// 💻️ framework/ui/elements/💬️UIDialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { effectiveActionArgs, missingRequiredArgs, type ActionArgDef, type DialogDefinition } from "@semio-tech/framework";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { veilClass } from "../🏷️ClassNames/🟦️component.tsx";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
import { Surface } from "../🌈️Surface/🟦️component.tsx";
import { useControlKeybinding, GLASS_OVERLAY_BOX_CLASS } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { Button } from "../🔘️Button/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🗨️Dialog
export type UIDialogProps = {
  readonly dialog: DialogDefinition;
  readonly seedArgs?: Readonly<Record<string, unknown>>;
  /** 🎛️ Injected staged-field renderer so ui-react never imports from framework/os/renderer. */
  readonly renderField: (def: ActionArgDef, value: unknown, onChange: (value: unknown) => void) => React.ReactElement;
  readonly onSubmit: (args: Record<string, unknown>) => void;
  readonly onCancel: () => void;
};

/** @emoji 🗨️ Modal form dialog: a full-screen glass veil plus a box styled identically to the
 * introduction info box (`GLASS_OVERLAY_BOX_CLASS`) presenting `dialog.args` as a staged form —
 * renders the declarative `DialogDefinition` contract. Submit dispatches the merged effective args;
 * cancel (Escape, veil click, or the Cancel button) all funnel through `onCancel`. */
export const UIDialog: React.FC<UIDialogProps> = ({ dialog, seedArgs, renderField, onSubmit, onCancel }) => {
  const cancelLabel = useLabel("ui.common.cancel");
  const [staged, setStaged] = reactHostPort.useState<Record<string, unknown>>({});
  const buffer = reactHostPort.useMemo(() => ({ ...seedArgs, ...staged }), [seedArgs, staged]);
  const effective = reactHostPort.useMemo(() => effectiveActionArgs(dialog.args, buffer), [dialog.args, buffer]);
  const missing = reactHostPort.useMemo(() => missingRequiredArgs(dialog.args, effective), [dialog.args, effective]);
  const canSubmit = missing.length === 0;

  const submit = reactHostPort.useCallback(() => {
    if (canSubmit) onSubmit(effective);
  }, [canSubmit, effective, onSubmit]);

  useControlKeybinding("ui.dialog.cancel", onCancel, { enableOnFormTags: true }, [onCancel]);
  useControlKeybinding("ui.dialog.submit", submit, { enableOnFormTags: true }, [submit]);

  return (
    <>
      <div data-level="dialog" className={cn(veilClass, "z-tutorial fixed inset-0 pointer-events-auto")} onClick={onCancel} />
      <Surface data-slot="dialog-box" level="dialog" fill="glass" className={cn(GLASS_OVERLAY_BOX_CLASS, "top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%]")}>
        <h3 className="mb-single text-sm font-medium">{dialog.title}</h3>
        {dialog.body && <p className="mb-double text-xs text-muted-foreground">{dialog.body}</p>}
        {dialog.args.length > 0 && (
          <div className="mb-double flex flex-col gap-single">
            {dialog.args.map((def) => (
              <div key={def.id} className="flex flex-col gap-tiny">
                <span className="text-xs text-muted-foreground">{def.label}</span>
                {renderField(def, effective[def.id], (value) => setStaged((prev) => ({ ...prev, [def.id]: value })))}
              </div>
            ))}
          </div>
        )}
        <div className="flex items-center justify-between gap-single">
          <Button id="ui.dialog.cancel" variant="ghost" icon="x" text={dialog.cancelLabel ?? cancelLabel} onClick={onCancel} />
          <Button id="ui.dialog.submit" icon="check" text={dialog.submitLabel} disabled={!canSubmit} onClick={submit} />
        </div>
      </Surface>
    </>
  );
};
// #endregion 🗨️Dialog
