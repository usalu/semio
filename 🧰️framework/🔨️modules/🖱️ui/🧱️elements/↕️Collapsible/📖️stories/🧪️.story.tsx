// #region 🧲️Header

// 🥼️ 🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/📖️stories/🧪️.story.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Button, Collapsible, CollapsibleContent, CollapsibleTrigger, Section, SharedBorderRow, SharedBorderRowCell, uiDataLabel } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../../🧪️tests/📚️storybook-types/🟦️.ts";
import { useState } from "react";

// 🔷️#region 🖥️Collapsible
const Box = createIconComponent("box");
const ChevronDown = createIconComponent("chevron-down");

const meta = {
  title: "🖱️ui⚛️react/Collapsible",
  component: Collapsible,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Collapsible>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(true);
    return (
      <Collapsible open={isOpen} onOpenChange={setIsOpen} className="w-96">
        <SharedBorderRow>
          <SharedBorderRowCell className="flex-1 gap-double">
            <Box className="size-tiny" />
            <h4 className="text-sm font-semibold">Capsule Variants (3)</h4>
          </SharedBorderRowCell>
          <CollapsibleTrigger asChild>
            <Button variant="ghost" icon={<ChevronDown className={`size-small transition-transform ${isOpen ? "rotate-180" : ""}`} />}>
              <span className="sr-only">Toggle</span>
            </Button>
          </CollapsibleTrigger>
        </SharedBorderRow>
        <SharedBorderRow orientation="vertical" edges={["right", "bottom", "left"]}>
          <SharedBorderRowCell className="justify-between text-sm">
            <span>Capsule J</span>
            <span className="text-xs text-muted-foreground">2.5m × 4.0m</span>
          </SharedBorderRowCell>
        </SharedBorderRow>
        <CollapsibleContent asChild>
          <SharedBorderRow orientation="vertical" edges={["right", "bottom", "left"]}>
            <SharedBorderRowCell className="text-sm">Capsule L</SharedBorderRowCell>
            <SharedBorderRowCell className="text-sm">Capsule P</SharedBorderRowCell>
          </SharedBorderRow>
        </CollapsibleContent>
      </Collapsible>
    );
  },
};

// #endregion 🖥️Collapsible

// 📑️#region 🖲️Section
export const SectionDefault: Story = {
  render: () => (
    <div className="w-96">
      <Section title={uiDataLabel("Module Types")} id="module-types">
        <p className="text-sm">Capsule J, Capsule K, Base, Tambour A, Capital</p>
      </Section>
      <Section title={uiDataLabel("Design Properties")} id="design-properties">
        <p className="text-sm">Volume: 25.0 m³, Area: 10.0 m², Connections: 4</p>
      </Section>
    </div>
  ),
};
// #endregion 🖲️Section

// 🔷️#region 🪬️Steps
export const StepsDefault: Story = {
  render: () => (
    <div className="w-96">
      <SharedBorderRow as="ol" orientation="vertical">
        <SharedBorderRowCell as="li" className="flex-col items-start">
          <h3 className="font-semibold">Step 1: Create a Kit</h3>
          <p className="text-sm text-muted-foreground">Initialize a new kit with types and qualities.</p>
        </SharedBorderRowCell>
        <SharedBorderRowCell as="li" className="flex-col items-start">
          <h3 className="font-semibold">Step 2: Add Types</h3>
          <p className="text-sm text-muted-foreground">Define the building blocks of your design.</p>
        </SharedBorderRowCell>
        <SharedBorderRowCell as="li" className="flex-col items-start">
          <h3 className="font-semibold">Step 3: Create a Design</h3>
          <p className="text-sm text-muted-foreground">Compose types into a complete design.</p>
        </SharedBorderRowCell>
      </SharedBorderRow>
    </div>
  ),
};
// #endregion 🪬️Steps
