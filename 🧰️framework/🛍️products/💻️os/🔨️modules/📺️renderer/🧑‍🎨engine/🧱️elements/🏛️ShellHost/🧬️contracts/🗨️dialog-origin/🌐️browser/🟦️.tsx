import React from "react";
import type { ActionArgDef } from "@semio-tech/framework";
import { UIDialog, type UIDialogProps } from "@semio-tech/ui-react";
import { type ShellDialogV1, type ShellDialogOriginV1 } from "../🟦️.ts";

export type OwnedShellDialogProps<Arg extends ActionArgDef = ActionArgDef> = Readonly<{
  owner: ShellDialogV1;
  dialog: UIDialogProps<Arg>["dialog"];
  renderField: UIDialogProps<Arg>["renderField"];
  isCurrent: (origin: ShellDialogOriginV1) => boolean;
  close: (openingId: number) => boolean;
  dispatch: (actionId: string, origin: ShellDialogOriginV1, args?: Record<string, unknown>) => void;
}>;

/** 📨️ A new opening remounts staged fields; late callbacks can only consume their exact opening. */
export function OwnedShellDialog<Arg extends ActionArgDef>({ owner, dialog, renderField, isCurrent, close, dispatch }: OwnedShellDialogProps<Arg>): React.ReactElement | null {
  if (!isCurrent(owner.origin)) return null;
  const settle = (actionId: string | undefined, args?: Record<string, unknown>): void => {
    const current = isCurrent(owner.origin);
    if (!close(owner.openingId) || !current || actionId === undefined) return;
    dispatch(actionId, owner.origin, args);
  };
  return <UIDialog<Arg> key={owner.openingId} dialog={dialog} seedArgs={owner.seedArgs} renderField={renderField}
    onSubmit={(args) => settle(dialog.submitAction, args)} onCancel={() => settle(dialog.cancelAction)} />;
}
