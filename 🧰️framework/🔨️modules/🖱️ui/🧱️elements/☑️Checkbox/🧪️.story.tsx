// #region 🔌️Adapters
import { Checkbox, type CheckboxState } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story";
// #endregion 🔌️Adapters

// #region ☑️CheckboxMatrix
const meta = {
  title: "🖱️ui⚛️react/Checkbox",
  component: Checkbox,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof Checkbox>;

export default meta;
type Story = StoryObj<typeof meta>;

const states: readonly CheckboxState[] = [true, false, "indeterminate"];

export const StateMatrix: Story = {
  render: () => (
    <div className="grid grid-cols-2 gap-double text-xs">
      {states.flatMap((state) =>
        [false, true].map((disabled) => (
          <label key={`${state}-${disabled}`} className="flex items-center gap-single">
            <Checkbox checked={state} disabled={disabled} onChange={() => undefined} />
            {String(state)} · {disabled ? "disabled" : "enabled"}
          </label>
        )),
      )}
    </div>
  ),
};
// #endregion ☑️CheckboxMatrix
