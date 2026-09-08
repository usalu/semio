// #region 🧲️Header

// 🥼️ .storybook/stories/ui/UIDialog.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { argControl, type ActionArgDef, type DialogDefinition } from "@semio-tech/framework";
import { UIDialog, type UIDialogFieldBinding } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../../../🧪️tests/📚️storybook-types/🟦️.ts";
import { useState } from "react";
// #endregion 🔌️Adapters

// 🗨️#region 🗨️UIDialog
/** @emoji 🎛️ Minimal `renderField` — `UIDialog` is injected this renderer so `ui-react` never has to import from `framework/os/renderer` (see the prop's docstring on `UIDialogProps`). A real shell renders the full staged-arg control set; this story only needs text/number/toggle. */
function renderStoryField(def: ActionArgDef, value: unknown, onChange: (value: unknown) => void, field: UIDialogFieldBinding) {
  const control = argControl(def);
  if (control.kind === "toggle") {
    return <input id={field.id} aria-labelledby={field.labelledBy} type="checkbox" checked={Boolean(value)} onChange={(event) => onChange(event.target.checked)} />;
  }
  if (control.kind === "number" || control.kind === "slider") {
    return <input id={field.id} aria-labelledby={field.labelledBy} required={field.required} type="number" className="w-full border p-single text-xs" value={typeof value === "number" ? value : ""} min={control.min} max={control.max} onChange={(event) => onChange(Number(event.target.value))} />;
  }
  return <input id={field.id} aria-labelledby={field.labelledBy} required={field.required} type="text" className="w-full border p-single text-xs" value={typeof value === "string" ? value : ""} onChange={(event) => onChange(event.target.value)} />;
}

const addCapsuleDialog: DialogDefinition = {
  id: "dialog.story.add-capsule",
  title: { native: { en: "Add Capsule Instance", de: "Kapselinstanz hinzufügen" } },
  body: { native: { en: "Configure the new capsule piece and its placement in the design.", de: "Das neue Kapselstück und seine Platzierung im Entwurf konfigurieren." } },
  args: [
    { id: "quantity", label: { native: { en: "Quantity", de: "Anzahl" } }, schema: { kind: "number", min: 1, max: 20, step: 1, integer: true }, required: true, default: 1 },
    { id: "label", label: { native: { en: "Label", de: "Bezeichnung" } }, schema: { kind: "string", options: [] }, required: false },
    { id: "mirrored", label: { native: { en: "Mirrored", de: "Gespiegelt" } }, schema: { kind: "boolean" }, required: false },
  ],
  submitAction: "action.add-capsule",
  submitLabel: { native: { en: "Add to Design", de: "Zum Entwurf hinzufügen" } },
  cancelAction: "action.cancel-add-capsule",
  cancelLabel: { native: { en: "Cancel", de: "Abbrechen" } },
};

const confirmDialog: DialogDefinition = {
  id: "dialog.story.confirm-delete",
  title: { native: { en: "Delete Design?", de: "Entwurf löschen?" } },
  body: { native: { en: "This cannot be undone.", de: "Dies kann nicht rückgängig gemacht werden." } },
  args: [],
  submitAction: "action.delete-design",
  submitLabel: { native: { en: "Delete", de: "Löschen" } },
  cancelAction: "action.cancel-delete-design",
  cancelLabel: { native: { en: "Cancel", de: "Abbrechen" } },
};

const meta = {
  title: "🖱️ui⚛️react/UIDialog",
  component: UIDialog,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof UIDialog>;

export default meta;

type Story = StoryObj<typeof meta>;

function UIDialogDemo({ dialog }: { readonly dialog: DialogDefinition }) {
  const [dismissed, setDismissed] = useState<string | null>(null);
  if (dismissed) return <div className="p-double text-xs text-muted-foreground">{dismissed}</div>;
  return <UIDialog dialog={dialog} renderField={renderStoryField} onSubmit={(args) => setDismissed(`Submitted: ${JSON.stringify(args)}`)} onCancel={() => setDismissed("Cancelled")} />;
}

export const StagedForm: Story = {
  name: "Staged form (quantity / label / toggle)",
  render: () => <UIDialogDemo dialog={addCapsuleDialog} />,
};

export const ConfirmOnly: Story = {
  name: "Message/confirm (no args)",
  render: () => <UIDialogDemo dialog={confirmDialog} />,
};
// #endregion 🗨️UIDialog
