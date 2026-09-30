// #region 🧲️Header
// 💻️ framework/ui/elements/💬️UIDialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { actionArgRequiresChoice, dialogChoiceArgs, effectiveActionArgs, invalidActionChoiceArgs, unresolvedActionArgs, unresolvedDialogChoiceArgs, type ActionArgDef, type DialogChoice, type DialogDefinition } from "@semio-tech/framework";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { keyboardEventMatchesOwnedHotkey, parseOwnedHotkeyChords, resolveControlKeybindingRaw, SHELL_KEYBINDINGS, useUiKeybindingsByControlId } from "../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { Dialog, DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogTitle } from "../💬️Dialog/🟦️.tsx";
import { detectShellLocale, GLASS_OVERLAY_BOX_CLASS, resolveUiLocalizedText, uiI18n, useShellScopeOptional, useUiTerminology } from "../../🎯️targets/⚛️react/🟦️";
import { Button } from "../🔘️Button/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🗨️Dialog
/** 🏷️ Semantic association supplied to every injected staged control, including composite fields. */
export interface UIDialogFieldBinding {
  readonly id: string;
  readonly labelledBy: string;
  readonly required: boolean;
}

export type UIDialogProps<Arg extends ActionArgDef = ActionArgDef> = {
  readonly dialog: Omit<DialogDefinition, "args"> & { readonly args: readonly Arg[] };
  readonly seedArgs?: Readonly<Record<string, unknown>>;
  readonly notice?: React.ReactNode;
  readonly choiceRevisions?: Readonly<Record<string, string>>;
  /** 🎛️ Injected staged-field renderer so ui-react never imports from framework/os/renderer. */
  readonly renderField: (def: Arg, value: unknown, onChange: (value: unknown) => void, field: UIDialogFieldBinding) => React.ReactElement;
  readonly onSubmit: (args: Record<string, unknown>) => void;
  /** 🔀️ One of `dialog.choices` taken with its dispatch args (`dialogChoiceArgs`: the seed context, the args it requires and its id). */
  readonly onChoose: (choice: DialogChoice, args: Record<string, unknown>) => void;
  readonly onCancel: () => void;
};

/** 🎨️ A choice's chrome: danger styling for a destructive one (never colour alone — its description rides
 * `aria-describedby`), the filled style for a primary tone, the outlined style otherwise. The class lands on the
 * button's group, so it reaches the button item through descendant selectors. */
function dialogChoiceChrome(choice: DialogChoice): { readonly variant: "default" | "outline"; readonly className?: string } {
  if (choice.destructive || choice.tone === "danger") return { variant: "outline", className: "text-destructive [&_[data-slot=button-group-item]]:text-destructive [&_[data-slot=button-group-item]]:!border-destructive/60 [&_[data-slot=button-group-item]:hover]:bg-destructive/10" };
  return { variant: choice.tone === "primary" ? "default" : "outline" };
}

/** 🗨️ Accessible staged form using the owned modal boundary and scoped configurable shortcuts. */
export function UIDialog<Arg extends ActionArgDef>({ dialog, seedArgs, notice, choiceRevisions, renderField, onSubmit, onChoose, onCancel }: UIDialogProps<Arg>): React.ReactElement {
  const cancelLabel = useLabel("ui.common.cancel");
  const shellScope = useShellScopeOptional();
  const [portalContainer, setPortalContainer] = React.useState<Element | null>(() => shellScope?.portalLayerRef.current ?? null);
  React.useLayoutEffect(() => { setPortalContainer(shellScope?.portalLayerRef.current ?? null); }, [shellScope]);
  const { terminology } = useUiTerminology();
  const locale = detectShellLocale(shellScope?.i18n.resolvedLanguage ?? uiI18n.resolvedLanguage);
  const text = (value: unknown) => resolveUiLocalizedText(value, terminology, locale);
  const body = text(dialog.body);
  const labelPrefix = React.useId();
  const contentRef = React.useRef<HTMLDivElement | null>(null);
  const bindings = useUiKeybindingsByControlId();
  const chords = React.useMemo(() => {
    const apple = typeof navigator !== "undefined" && /Mac|iPhone|iPad|iPod/.test(navigator.platform);
    return Object.fromEntries(["cancel", "submit"].map(action => {
      const id = `ui.dialog.${action}`;
      return [action, parseOwnedHotkeyChords(resolveControlKeybindingRaw(id, bindings) ?? SHELL_KEYBINDINGS[id] ?? "", apple)];
    }));
  }, [bindings]);
  const [staged, setStaged] = reactHostPort.useState<{ revisions: typeof choiceRevisions; args: Record<string, unknown> }>({ revisions: choiceRevisions, args: {} });
  const effective = reactHostPort.useMemo(() => {
    const args = effectiveActionArgs(dialog.args, staged.args, seedArgs);
    for (const def of dialog.args) if (actionArgRequiresChoice(def) && staged.revisions?.[def.id] !== choiceRevisions?.[def.id]) args[def.id] = undefined;
    return args;
  }, [dialog.args, staged, seedArgs, choiceRevisions]);
  React.useLayoutEffect(() => {
    const clear = new Set(invalidActionChoiceArgs(dialog.args, effective));
    for (const def of dialog.args) if (actionArgRequiresChoice(def) && staged.revisions?.[def.id] !== choiceRevisions?.[def.id]) clear.add(def.id);
    if (clear.size === 0) return;
    setStaged(prev => ({ revisions: choiceRevisions, args: { ...prev.args, ...Object.fromEntries([...clear].map(id => [id, undefined])) } }));
  }, [dialog.args, effective, staged.revisions, choiceRevisions, setStaged]);
  const canSubmit = unresolvedActionArgs(dialog.args, effective).length === 0;

  const submit = reactHostPort.useCallback(() => {
    if (canSubmit) onSubmit(effective);
  }, [canSubmit, effective, onSubmit]);
  const choices = dialog.choices ?? [];
  const choiceEnabled = (choice: DialogChoice) => unresolvedDialogChoiceArgs(choice, dialog.args, effective).length === 0;
  const choiceDescriptionId = (choice: DialogChoice) => `${labelPrefix}-choice-${choice.id}`;

  const onKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.defaultPrevented || event.nativeEvent.isComposing || !(event.target instanceof Element) || event.target.closest('[role="dialog"]') !== event.currentTarget) return;
    const cancel = chords.cancel?.some(chord => keyboardEventMatchesOwnedHotkey(event.nativeEvent, chord));
    const confirm = chords.submit?.some(chord => keyboardEventMatchesOwnedHotkey(event.nativeEvent, chord));
    if (!cancel && !confirm) return;
    if (!cancel && event.key === "Enter" && !event.ctrlKey && !event.metaKey && !event.altKey && !event.shiftKey && event.target.closest('button,select,textarea,[role="combobox"],[role="listbox"],[contenteditable]:not([contenteditable="false"])')) return;
    event.preventDefault();
    event.stopPropagation();
    if (cancel) onCancel();
    else submit();
  };

  if (shellScope && !portalContainer) return <></>;
  return (
    <Dialog open isolationRoot={shellScope?.rootRef.current ?? null} onOpenChange={open => { if (!open) onCancel(); }}>
      <DialogPortal container={portalContainer}>
      <DialogOverlay />
      <DialogContent ref={contentRef} showCloseButton={false} aria-describedby={body ? undefined : ""} onKeyDown={onKeyDown} onEscapeKeyDown={event => {
        const keyboard = event.originalEvent;
        if (keyboard && chords.cancel?.some(chord => keyboardEventMatchesOwnedHotkey(keyboard, chord))) return;
        event.preventDefault();
        keyboard?.preventDefault();
        keyboard?.stopPropagation();
        if (keyboard?.target instanceof Node && contentRef.current?.contains(keyboard.target) && chords.submit?.some(chord => keyboardEventMatchesOwnedHotkey(keyboard, chord))) submit();
      }} className={cn(GLASS_OVERLAY_BOX_CLASS, "block z-dialog sm:max-w-sm")}>
        <DialogTitle className="mb-single text-sm font-medium">{text(dialog.title)}</DialogTitle>
        {body && <DialogDescription className="mb-double text-xs text-muted-foreground">{body}</DialogDescription>}
        {notice}
        {dialog.args.length > 0 && (
          <div className="mb-double flex flex-col gap-single">
            {dialog.args.map((def, index) => (
              <div key={def.id} role="group" aria-labelledby={`${labelPrefix}-${index}`} className="flex flex-col gap-tiny">
                <span id={`${labelPrefix}-${index}`} className="text-xs text-muted-foreground">{text(def.label)}</span>
                {renderField(def, effective[def.id], (value) => setStaged((prev) => ({ ...prev, args: { ...prev.args, [def.id]: value } })), { id: def.id, labelledBy: `${labelPrefix}-${index}`, required: def.required ?? false })}
              </div>
            ))}
          </div>
        )}
        {choices.some((choice) => choice.description != null) && (
          <div className="mb-double flex flex-col gap-tiny">
            {choices.map((choice) => choice.description == null ? null : <p key={choice.id} id={choiceDescriptionId(choice)} data-dialog-choice-description={choice.id} className="text-xs text-muted-foreground">{text(choice.description)}</p>)}
          </div>
        )}
        <div className="flex flex-wrap items-center justify-between gap-single">
          <Button id="ui.dialog.cancel" variant="ghost" icon="x" text={dialog.cancelLabel ? text(dialog.cancelLabel) : cancelLabel} onClick={onCancel} />
          <div className="flex flex-wrap items-center justify-end gap-single">
            {choices.map((choice) => {
              const chrome = dialogChoiceChrome(choice);
              return <Button key={choice.id} id={`ui.dialog.choice.${choice.id}`} variant={chrome.variant} className={chrome.className} icon={choice.destructive ? "triangle-alert" : "arrow-right"} text={text(choice.label)} data-dialog-choice={choice.id} data-tone={choice.tone ?? "neutral"} data-destructive={choice.destructive ? "true" : undefined} aria-describedby={choice.description == null ? undefined : choiceDescriptionId(choice)} disabled={!choiceEnabled(choice)} onClick={() => { if (choiceEnabled(choice)) onChoose(choice, dialogChoiceArgs(choice, dialog.args, effective)); }} />;
            })}
            <Button id="ui.dialog.submit" icon="check" text={text(dialog.submitLabel)} disabled={!canSubmit} onClick={submit} />
          </div>
        </div>
      </DialogContent>
      </DialogPortal>
    </Dialog>
  );
}
// #endregion 🗨️Dialog
