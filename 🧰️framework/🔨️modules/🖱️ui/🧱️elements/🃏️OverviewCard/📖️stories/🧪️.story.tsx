// #region 🧲️Header

// 🥼️ 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/📖️stories/🧪️.story.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { Icon, OverviewCard, OverviewCardAction, OverviewCardOpenChip } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../../🧪️tests/📚️storybook-types/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🃏️OverviewCard
const meta = {
  title: "🖱️ui⚛️react/OverviewCard",
  component: OverviewCard,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof OverviewCard>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📑️ A region with two real actions — the shape of a quiz card. */
export const Section: Story = {
  args: {
    as: "section",
    slot: "story-card",
    headingId: "story-card-heading",
    icon: <Icon icon="award" size="small" className="shrink-0 text-muted-foreground" />,
    title: "Heating",
    children: <p className="text-xs leading-normal text-foreground">U-values from single glazing to the passive-house roof.</p>,
    footerLeft: <OverviewCardAction>Last result</OverviewCardAction>,
    footerRight: <OverviewCardAction primary>Play again</OverviewCardAction>,
    className: "max-w-sm",
  },
};

/** 🔘️ A whole card that opens one app — the shape of the play and demonstrator overviews. */
export const Button: Story = {
  args: {
    as: "button",
    slot: "story-app-card",
    icon: <Icon icon="home" size="small" className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />,
    title: "Home",
    label: "Open Home: every app in one place",
    onClick: () => undefined,
    children: <span className="block truncate text-xs leading-normal text-muted-foreground">Every app in one place</span>,
    footerRight: <OverviewCardOpenChip slot="story-app-card">Open</OverviewCardOpenChip>,
    className: "max-w-xs",
  },
};
// #endregion 🃏️OverviewCard
