// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Layout.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Canvas, Panel, PanelChromeTabBar, Footer, HorizontalWindows, Layout, Navbar, singleTreeLeaf, Window, fundedByZukunftBauFooterItem, navbarFillItem } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState, type ComponentType } from "react";

// 🔷️#region 🪨️Layout
const Home = createIconComponent("home");
const Info = createIconComponent("info");
const Layers = createIconComponent("layers");
const Settings = createIconComponent("settings");
const User = createIconComponent("user");

const meta = {
  title: "🖱️ui⚛️react/Layout",
  component: Layout,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Layout>;

export default meta;

type Story = StoryObj<typeof meta>;

// Interior content of a `<Window>` — the window's own root already fills `ui-surface` at
// level="window" (WindowChrome contract), so this stays bg-transparent (one ui-surface/ui-glass
// per level root; interior containers stay transparent).
const ExampleContent = ({ title }: { title: string }) => (
  <div className="flex items-center justify-center h-full bg-transparent">
    <h2 className="text-2xl font-bold">{title}</h2>
  </div>
);

const layoutPanelLeafTab = (id: string, icon: ComponentType<{ size?: number }>, name: string, order: number, content: string) =>
  singleTreeLeaf({ id, icon, name, order, tree: { sections: [{ id: `${id}.section`, label: "", items: [{ id: `${id}.item`, label: "", control: <div className="p-2">{content}</div> }] }] } });

// Root-level "Workbench" branch with two leaf children — exercises the depth split between a chrome-hosted
// bar (root row only) and the floating panel it opens (depth ≥ 1 rows), the same shape os-shell's own
// Document/Catalogue tabs use.
const topLeftTabs = [
  {
    kind: "branch" as const,
    id: "workbench",
    icon: Layers,
    name: "Workbench",
    children: [
      singleTreeLeaf({
        id: "explorer",
        icon: Layers,
        name: "Explorer",
        order: 0,
        tree: { sections: [{ id: "explorer.section", label: "", items: [{ id: "explorer.item", label: "", control: <div className="p-double">Explorer content</div> }] }] },
      }),
      singleTreeLeaf({ id: "search", icon: Info, name: "Search", order: 1, tree: { sections: [{ id: "search.section", label: "", items: [{ id: "search.item", label: "", control: <div className="p-double">Search content</div> }] }] } }),
    ],
  },
];

const bottomRightTabs = [layoutPanelLeafTab("settings", Settings, "Settings", 0, "Settings panel content")];

export const Default: Story = {
  args: { canvas: null },
  render: () => {
    const [topLeftSize, setTopLeftSize] = useState(250);
    const [topMiddleSize, setTopMiddleSize] = useState(360);
    const [topRightSize, setTopRightSize] = useState(300);
    const [topLeftVisible, setTopLeftVisible] = useState(true);
    const [topLeftPath, setTopLeftPath] = useState<readonly string[]>(["workbench", "explorer"]);
    const [bottomRightVisible, setBottomRightVisible] = useState(false);
    const [bottomRightPath, setBottomRightPath] = useState<readonly string[]>(["settings"]);

    return (
      <Layout
        navbar={
          <Navbar
            items={[
              { content: <Home size={20} />, key: "home" },
              { content: <span className="font-bold">Application</span>, key: "title" },
              { content: <input type="text" placeholder="Search..." className="px-2 py-1 bg-panel border rounded w-full" />, key: "search", className: "flex-1" },
              {
                key: "topLeftPanelTabs",
                content: (
                  <PanelChromeTabBar anchor="top-left" tabs={topLeftTabs} visible={topLeftVisible} onVisibleChange={setTopLeftVisible} activeTabPath={topLeftPath} onActiveTabPathChange={setTopLeftPath} />
                ),
              },
              { content: <User size={20} />, key: "user" },
            ]}
          />
        }
        footer={
          <Footer
            items={[
              { content: "Ready", key: "status" },
              { content: "Ln 1, Col 1", key: "cursor" },
              { content: "UTF-8", key: "selection" },
              navbarFillItem("footerTrailingFill"),
              fundedByZukunftBauFooterItem(),
              { key: "fundedByGap", className: "w-huge", content: null },
              {
                key: "bottomRightPanelTabs",
                content: (
                  <PanelChromeTabBar
                    anchor="bottom-right"
                    tabs={bottomRightTabs}
                    visible={bottomRightVisible}
                    onVisibleChange={setBottomRightVisible}
                    activeTabPath={bottomRightPath}
                    onActiveTabPathChange={setBottomRightPath}
                  />
                ),
              },
            ]}
          />
        }
        panels={{
          "bottom-right": {
            visible: bottomRightVisible,
            onVisibleChange: setBottomRightVisible,
            activeTabPath: bottomRightPath,
            onActiveTabPathChange: setBottomRightPath,
            tabBarHost: "chrome",
            size: 300,
            onSizeChange: () => {},
            tabs: bottomRightTabs,
          },
          "top-left": {
            visible: topLeftVisible,
            onVisibleChange: setTopLeftVisible,
            activeTabPath: topLeftPath,
            onActiveTabPathChange: setTopLeftPath,
            tabBarHost: "chrome",
            size: topLeftSize,
            onSizeChange: setTopLeftSize,
            tabs: topLeftTabs,
          },
          "top-middle": {
            visible: true,
            size: topMiddleSize,
            onSizeChange: setTopMiddleSize,
            tabs: [
              singleTreeLeaf({
                id: "console",
                icon: Info,
                name: "Console",
                order: 0,
                tree: { sections: [{ id: "console.section", label: "", items: [{ id: "console.item", label: "", control: <div className="p-double font-mono text-xs">Console output...</div> }] }] },
              }),
            ],
          },
          "top-right": {
            visible: true,
            size: topRightSize,
            onSizeChange: setTopRightSize,
            tabs: [
              singleTreeLeaf({
                id: "properties",
                icon: Settings,
                name: "Properties",
                order: 0,
                tree: { sections: [{ id: "properties.section", label: "", items: [{ id: "properties.item", label: "", control: <div className="p-double">Properties content</div> }] }] },
              }),
            ],
          },
        }}
        canvas={
          <Canvas>
            <HorizontalWindows>
              <Window id="main" defaultSize={50}>
                <ExampleContent title="Main Window" />
              </Window>
              <Window id="side" defaultSize={50}>
                <ExampleContent title="Side Window" />
              </Window>
            </HorizontalWindows>
          </Canvas>
        }
      />
    );
  },
};

// 📱️ Mobile: the merged panel fills the space between navbar and footer while open; the canvas stays
// mounted underneath (hidden, not unmounted) so toggling the panel doesn't replug the 3D world.
export const Mobile: Story = {
  args: { canvas: null },
  render: () => {
    const [visible, setVisible] = useState(true);
    const [activeTabPath, setActiveTabPath] = useState<readonly string[]>(["workbench", "explorer"]);
    const mobileTabs = [...topLeftTabs, ...bottomRightTabs];

    return (
      <div className="h-[812px] w-[375px] border mx-auto">
        <Layout
          mobile
          mobilePanel={{ visible, tabs: mobileTabs, activeTabPath, onActiveTabPathChange: setActiveTabPath }}
          navbar={
            <Navbar
              items={[
                { content: <Home size={20} />, key: "home" },
                { content: <span className="font-bold">Application</span>, key: "title" },
                navbarFillItem("mobileNavbarFill"),
                { key: "mobilePanelToggle", content: <button onClick={() => setVisible((current) => !current)}>Panel</button> },
              ]}
              showFullscreenToggle={false}
            />
          }
          footer={<Footer items={[{ content: "Ready", key: "status" }]} />}
          canvas={
            <Canvas>
              <HorizontalWindows>
                <Window id="main" defaultSize={100}>
                  <ExampleContent title="Main Window" />
                </Window>
              </HorizontalWindows>
            </Canvas>
          }
        />
      </div>
    );
  },
};

// #endregion 🪨️Layout

// 💻️#region 🧭️Panel
export const PanelDefault: Story = {
  args: { canvas: null },
  render: () => {
    const [size, setSize] = useState(300);
    return (
      <div className="relative h-[400px] w-[600px] border ui-surface" data-level="base">
        <Panel
          anchor="top-left"
          size={size}
          onSizeChange={setSize}
          tabs={[layoutPanelLeafTab("types", Layers, "Types", 0, "Types panel content"), layoutPanelLeafTab("settings", Settings, "Settings", 1, "Settings panel content"), layoutPanelLeafTab("info", Info, "Info", 2, "Info panel content")]}
        />
      </div>
    );
  },
};
// #endregion 🧭️Panel
