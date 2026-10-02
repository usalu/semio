// #region 🧲️Header

// 🥼️ 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/📖️stories/🧪️.story.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { Icon, LayeredOverview, OverviewCard, OverviewCardAction, type IconName, type LayeredCardState, type LayeredPane } from "@semio-tech/ui-react";
import type { ComponentProps } from "react";
import type { Meta, StoryObj } from "../../../🧪️tests/📚️storybook-types/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🥞️LayeredOverview
const SECTIONS: readonly (readonly [string, string, IconName, string])[] = [
  ["learner", "Learner", "user", "Points, rank and every run so far."],
  ["physics", "Physical Understanding", "award", "Power or energy, from a tea light to the Sun."],
  ["intro", "How it works", "info", "Classify, sort and match; the best run counts."],
  ["heating", "Heating", "sun", "U-values and heating loads across six decades."],
  ["board", "Leaderboard", "list-ordered", "Every learner, sortable by quiz."],
  ["cooling", "Cooling", "clock", "Air change rates and cooling loads."],
  ["badges", "Badges", "award", "Seven badges, three earned."],
  ["demand", "Energy Demand", "workflow", "Standards by their profiles."],
  ["prefs", "Preferences", "settings", "Language, theme and text size."],
];

const PANES: readonly LayeredPane[] = SECTIONS.map(([id, label, icon, text], index) => ({
  id,
  label,
  icon,
  render: () => (
    <div className="flex h-full w-full flex-col gap-double bg-background p-large text-foreground" style={{ backgroundImage: `linear-gradient(135deg, hsl(${index * 40} 60% 50% / 0.25), transparent 60%)` }}>
      <h1 className="text-3xl font-semibold">{label}</h1>
      <p className="max-w-2xl text-lg">{text}</p>
    </div>
  ),
}));

const CELLS = Object.fromEntries(SECTIONS.map(([id], index) => [id, { column: index % 3, row: Math.floor(index / 3) }]));

const LABELS = { grid: "Quizzes", overview: "Overview", waiting: (pane: LayeredPane) => `${pane.label} is waiting to start`, failed: (pane: LayeredPane) => `${pane.label} could not be loaded.` };

function card(pane: LayeredPane, state: LayeredCardState) {
  return (
    <OverviewCard
      as="section"
      slot="story-layered-card"
      headingId={`story-layered-${pane.id}`}
      headingLevel={3}
      className={state.mode === "list" ? "pointer-events-auto w-full max-w-sm" : "pointer-events-auto w-full max-w-xs"}
      icon={pane.icon ? <Icon icon={pane.icon} size="small" className="shrink-0 text-muted-foreground" /> : null}
      title={pane.label}
      footerRight={
        <OverviewCardAction primary onClick={state.open}>
          Open
        </OverviewCardAction>
      }
    >
      <p className="text-xs leading-normal text-muted-foreground">{SECTIONS.find(([id]) => id === pane.id)?.[3]}</p>
    </OverviewCard>
  );
}

const meta = {
  title: "🖱️ui⚛️react/LayeredOverview",
  component: LayeredOverview,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: { panes: PANES, cells: CELLS, renderCard: card, labels: LABELS, routing: "none", overlayClassName: "grid grid-cols-3 grid-rows-3 place-items-center gap-double p-double", lifecycle: { budget: 9, warmStartMs: 300, warmIntervalMs: 150 } },
  render: (props: ComponentProps<typeof LayeredOverview>) => (
    <div className="relative h-[36rem] w-full">
      <LayeredOverview {...props} />
    </div>
  ),
} satisfies Meta<typeof LayeredOverview>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🥞️ Nine pages behind one glass; hover or focus a card to see its page clear, open it with its button. */
export const Strip: Story = {};

/** 📱️ Touch phones: one snap section per page, each under its own glass. */
export const List: Story = { args: { mode: "list" } };

/** 🔓️ A page opened full size, with the Overview button. */
export const Opened: Story = { args: { openedId: "board" } };

/** 🐢️ Reduced motion: reveals snap instead of gliding and the pointer never pans. */
export const ReducedMotion: Story = { args: { reducedMotion: "always" } };
// #endregion 🥞️LayeredOverview
