// #region 🧲️Header

// 🥼️ .storybook/stories/ui/UIDialog.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { argControl, type ActionArgDef, type DialogDefinition } from "@semio-tech/framework";
import { UIDialog } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story.ts";
import { useState } from "react";
// #endregion 🔌️Adapters

// 🗨️#region 🗨️UIDialog
/** @emoji 🎛️ Minimal `renderField` — `UIDialog` is injected this renderer so `ui-react` never has to import from `framework/os/renderer` (see the prop's docstring on `UIDialogProps`). A real shell renders the full staged-arg control set; this story only needs text/number/toggle. */
function renderStoryField(def: ActionArgDef, value: unknown, onChange: (value: unknown) => void) {
  // 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6:
  // `def.control` is gone (derived, not stored) — `argControl(def)` mirrors Rust `ActionArgDef::control()`.
  const control = argControl(def);
  if (control.kind === "toggle") {
    return (
      <label className="flex items-center gap-single text-xs">
        <input type="checkbox" checked={Boolean(value)} onChange={(event) => onChange(event.target.checked)} />
        {def.label}
      </label>
    );
  }
  if (control.kind === "number" || control.kind === "slider") {
    return <input type="number" className="w-full border p-single text-xs" value={typeof value === "number" ? value : ""} min={control.min} max={control.max} onChange={(event) => onChange(Number(event.target.value))} />;
  }
  return <input type="text" className="w-full border p-single text-xs" value={typeof value === "string" ? value : ""} onChange={(event) => onChange(event.target.value)} />;
}

const addCapsuleDialog: DialogDefinition = {
  id: "dialog.story.add-capsule",
  title: "Add Capsule Instance",
  body: "Configure the new capsule piece and its placement in the design.",
  args: [
    { id: "quantity", label: "Quantity", control: { kind: "number", min: 1, max: 20 }, required: true, default: 1 },
    { id: "label", label: "Label", control: { kind: "text", placeholder: "Optional label" }, required: false },
    { id: "mirrored", label: "Mirrored", control: { kind: "toggle" }, required: false },
  ],
  submitAction: "action.add-capsule",
  submitLabel: "Add to Design",
  cancelLabel: "Cancel",
};

const confirmDialog: DialogDefinition = {
  id: "dialog.story.confirm-delete",
  title: "Delete Design?",
  body: "This cannot be undone.",
  args: [],
  submitAction: "action.delete-design",
  submitLabel: "Delete",
  cancelLabel: "Cancel",
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
