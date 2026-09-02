// #region 🧲️Header
// 💻️ framework/ui/elements/💬️UIDialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { effectiveActionArgs, missingRequiredArgs, type ActionArgDef, type DialogDefinition } from "@semio-tech/framework";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { veilClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { useControlKeybinding } from "../../🔨️modules/⌨️control-keybinding-context/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { Surface } from "../🌈️Surface/🟦️.tsx";
import { detectShellLocale, GLASS_OVERLAY_BOX_CLASS, resolveUiLocalizedText, uiI18n, useShellScopeOptional, useUiTerminology } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx";
import { Button } from "../🔘️Button/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🗨️Dialog
export type UIDialogProps<Arg extends ActionArgDef = ActionArgDef> = {
  readonly dialog: Omit<DialogDefinition, "args"> & { readonly args: readonly Arg[] };
  readonly seedArgs?: Readonly<Record<string, unknown>>;
  /** 🎛️ Injected staged-field renderer so ui-react never imports from framework/os/renderer. */
  readonly renderField: (def: Arg, value: unknown, onChange: (value: unknown) => void) => React.ReactElement;
  readonly onSubmit: (args: Record<string, unknown>) => void;
  readonly onCancel: () => void;
};

/** @emoji 🗨️ Modal form dialog: a full-screen glass veil plus a box styled identically to the
 * introduction info box (`GLASS_OVERLAY_BOX_CLASS`) presenting `dialog.args` as a staged form —
 * renders the declarative `DialogDefinition` contract. Submit dispatches the merged effective args;
 * cancel (Escape, veil click, or the Cancel button) all funnel through `onCancel`. */
export function UIDialog<Arg extends ActionArgDef>({ dialog, seedArgs, renderField, onSubmit, onCancel }: UIDialogProps<Arg>): React.ReactElement {
  const cancelLabel = useLabel("ui.common.cancel");
  const shellScope = useShellScopeOptional();
  const { terminology } = useUiTerminology();
  const locale = detectShellLocale(shellScope?.i18n.resolvedLanguage ?? uiI18n.resolvedLanguage);
  const text = (value: unknown) => resolveUiLocalizedText(value, terminology, locale);
  const body = text(dialog.body);
  const [staged, setStaged] = reactHostPort.useState<Record<string, unknown>>({});
  const effective = reactHostPort.useMemo(() => effectiveActionArgs(dialog.args, staged, seedArgs), [dialog.args, staged, seedArgs]);
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
        <h3 className="mb-single text-sm font-medium">{text(dialog.title)}</h3>
        {body && <p className="mb-double text-xs text-muted-foreground">{body}</p>}
        {dialog.args.length > 0 && (
          <div className="mb-double flex flex-col gap-single">
            {dialog.args.map((def) => (
              <div key={def.id} className="flex flex-col gap-tiny">
                <span className="text-xs text-muted-foreground">{text(def.label)}</span>
                {renderField(def, effective[def.id], (value) => setStaged((prev) => ({ ...prev, [def.id]: value })))}
              </div>
            ))}
          </div>
        )}
        <div className="flex items-center justify-between gap-single">
          <Button id="ui.dialog.cancel" variant="ghost" icon="x" text={dialog.cancelLabel ? text(dialog.cancelLabel) : cancelLabel} onClick={onCancel} />
          <Button id="ui.dialog.submit" icon="check" text={text(dialog.submitLabel)} disabled={!canSubmit} onClick={submit} />
        </div>
      </Surface>
    </>
  );
}
// #endregion 🗨️Dialog
