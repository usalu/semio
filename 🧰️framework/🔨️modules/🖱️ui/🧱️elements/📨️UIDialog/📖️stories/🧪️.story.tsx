// #region 🧲️Header

// 🥼️ 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/📖️stories/🧪️.story.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { argControl, type ActionArgDef, type DialogDefinition } from "@semio-tech/framework";
import { Slider, Stepper, ToggleGroup, UIDialog, type UIDialogFieldBinding } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../../🧪️tests/📚️storybook-types/🟦️.ts";
import { useState } from "react";
import dialogChoicesFixture from "../../../../🛂️manifest/🧫️fixtures/🧫️dialog-choices/🔣️.json";
// #endregion 🔌️Adapters

// 🗨️#region 🗨️UIDialog
/** 🎛️ Story `renderField` — `UIDialog` is injected this renderer so `ui-react` never has to import from `framework/os/renderer` (see the prop's docstring on `UIDialogProps`). It mirrors the shell's `renderStagedArgControl` recipes for every control the stories stage: toggle, integer stepper, detented slider/dial read in display units, segmented choice, vector axes and reference chips. */
function renderStoryField(def: ActionArgDef, value: unknown, onChange: (value: unknown) => void, field: UIDialogFieldBinding) {
  const control = argControl(def);
  const text = (label: unknown) => (typeof label === "object" && label !== null ? ((label as { native?: { en?: string } }).native?.en ?? "") : String(label ?? ""));
  switch (control.kind) {
    case "toggle":
      return <input id={field.id} aria-labelledby={field.labelledBy} type="checkbox" checked={Boolean(value)} onChange={(event) => onChange(event.target.checked)} />;
    case "stepper":
      return <Stepper id={field.id} aria-labelledby={field.labelledBy} value={typeof value === "number" ? value : undefined} defaultValue={control.min ?? 0} min={control.min} max={control.max} step={control.step ?? 1} precision={control.precision} onChange={(next) => onChange(next)} />;
    case "slider":
    case "dial": {
      const numeric = typeof value === "number" ? value : control.min;
      const shown = (raw: number) => `${(raw * (control.displayFactor ?? 1)).toFixed(control.precision ?? 0)}${control.displayUnit ?? control.unit ? ` ${control.displayUnit ?? control.unit}` : ""}`;
      return <Slider id={field.id} aria-labelledby={field.labelledBy} aria-valuetext={shown(numeric)} min={control.min} max={control.max} step={control.step ?? 1} snapValues={control.snaps} formatDisplayValue={shown} value={[numeric]} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
    }
    case "segmented":
      return <ToggleGroup id={field.id} kind="single" aria-labelledby={field.labelledBy} value={typeof value === "string" ? value : ""} onValueChange={(next) => { if (next !== "") onChange(next); }} items={control.options.map((option) => ({ value: option.value, icon: "circle-dot", text: text(option.label) }))} />;
    case "number":
      return <input id={field.id} aria-labelledby={field.labelledBy} required={field.required} type="number" className="w-full border p-single text-xs" value={typeof value === "number" ? value : ""} min={control.min} max={control.max} onChange={(event) => onChange(Number(event.target.value))} />;
    case "vector": {
      const tuple = Array.isArray(value) ? (value as number[]) : Array.from({ length: control.dims }, () => 0);
      return (
        <div id={field.id} role="group" aria-labelledby={field.labelledBy} className="flex gap-single">
          {["x", "y", "z", "w"].slice(0, control.dims).map((axis, index) => (
            <div key={axis} className="flex items-center gap-tiny text-xs">
              <label htmlFor={`${field.id}.${axis}`}>{control.unit ? `${axis} (${control.unit})` : axis}</label>
              <input id={`${field.id}.${axis}`} type="number" step={control.step} className="w-full border p-single text-xs" value={tuple[index] ?? 0} onChange={(event) => onChange(tuple.map((entry, component) => (component === index ? Number(event.target.value) : entry)))} />
            </div>
          ))}
        </div>
      );
    }
    case "reference": {
      const ids = Array.isArray(value) ? (value as string[]) : typeof value === "string" && value !== "" ? [value] : [];
      return (
        <div id={field.id} role="group" aria-labelledby={field.labelledBy} className="flex flex-col gap-tiny text-xs">
          {ids.length === 0 ? <span>Nothing selected</span> : (
            <div role="toolbar" aria-labelledby={field.labelledBy} className="flex flex-wrap gap-tiny">
              {ids.map((id) => <button key={id} type="button" aria-label={`Remove ${id}`} className="rounded-sm border px-single" onClick={() => onChange(control.many ? ids.filter((other) => other !== id) : "")}>{id} ×</button>)}
            </div>
          )}
          <button type="button" className="self-start underline" onClick={() => onChange(control.many ? ["node-3", "node-4"] : "node-3")}>Use current selection</button>
        </div>
      );
    }
    default:
      return <input id={field.id} aria-labelledby={field.labelledBy} required={field.required} type="text" className="w-full border p-single text-xs" value={typeof value === "string" ? value : ""} onChange={(event) => onChange(event.target.value)} />;
  }
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

const editMutationDialog: DialogDefinition = {
  id: "dialog.story.edit-mutation",
  title: { native: { en: "Edit Rotate Selection", de: "Auswahl drehen bearbeiten" } },
  body: { native: { en: "The inputs of one mutation, as the history editor stages them.", de: "Die Eingaben einer Mutation, wie der Verlaufseditor sie vorbereitet." } },
  args: [
    { id: "steps", label: { native: { en: "Steps", de: "Schritte" } }, schema: { kind: "number", min: 1, max: 12, step: 1, integer: true }, presentation: { kind: "stepper" }, required: true, default: 4 },
    { id: "angle", label: { native: { en: "Angle", de: "Winkel" } }, schema: { kind: "number", min: 0, max: 6.2832, step: 0.01, integer: false, snaps: [0, 1.5708, 3.1416, 4.7124], precision: 0, displayUnit: "°", displayFactor: 57.29577951308232 }, presentation: { kind: "dial" }, required: true, default: 1.5708 },
    { id: "axis", label: { native: { en: "Axis", de: "Achse" } }, schema: { kind: "string", options: [{ value: "x", label: { native: { en: "X", de: "X" } } }, { value: "y", label: { native: { en: "Y", de: "Y" } } }] }, presentation: { kind: "segmented" }, required: true, default: "x" },
    { id: "pivot", label: { native: { en: "Pivot", de: "Drehpunkt" } }, schema: { kind: "vector", dims: 2, unit: "mm", step: 0.5 }, required: true, default: [0, 0] },
    { id: "targets", label: { native: { en: "Targets", de: "Ziele" } }, schema: { kind: "reference", kinds: ["node"], domain: "vortex", many: true, maxItems: 8 }, required: true, default: ["node-1", "node-2"] },
  ],
  submitAction: "action.edit-mutation",
  submitLabel: { native: { en: "Accept draft", de: "Entwurf übernehmen" } },
  cancelAction: "action.cancel-edit-mutation",
  cancelLabel: { native: { en: "Discard draft", de: "Entwurf verwerfen" } },
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
  return <UIDialog dialog={dialog} renderField={renderStoryField} onSubmit={(args) => setDismissed(`Submitted: ${JSON.stringify(args)}`)} onChoose={(choice, args) => setDismissed(`Chose ${choice.id}: ${JSON.stringify(args)}`)} onCancel={() => setDismissed("Cancelled")} />;
}

export const StagedForm: Story = {
  name: "Staged form (quantity stepper / label / toggle)",
  render: () => <UIDialogDemo dialog={addCapsuleDialog} />,
};

export const MutationInputs: Story = {
  name: "Mutation inputs (stepper / dial / segmented / vector / reference)",
  render: () => <UIDialogDemo dialog={editMutationDialog} />,
};

export const ConfirmOnly: Story = {
  name: "Message/confirm (no args)",
  render: () => <UIDialogDemo dialog={confirmDialog} />,
};

export const Choices: Story = {
  name: "Choices (new alternative / destructive overwrite)",
  render: () => <UIDialogDemo dialog={dialogChoicesFixture.dialog as DialogDefinition} />,
};
// #endregion 🗨️UIDialog
