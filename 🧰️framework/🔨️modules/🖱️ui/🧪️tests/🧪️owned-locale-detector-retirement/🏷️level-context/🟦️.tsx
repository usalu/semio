import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerLevelContextTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Canvas, Footer, LEVELS, Layout, LevelProvider, Mode, Navbar, Popover, PopoverContent, PopoverTrigger, React, Surface, WindowChrome, borderNormalClass, cn, glassClass, renderToStaticMarkup, shellFloorFillClass, shellFloorPaints, surfaceClass, uiDataLabel, useLevel, useSurface, windowChromeTitleChipClass } = dependencies;
  const { describe, expect, it, vi } = vitest;
  const { render, screen } = await import("@testing-library/react");

  describe("Level Context", () => {
    it("LEVELS is ordered base..menu", () => {
      expect(LEVELS).toEqual(["base", "window", "pane", "panel", "dialog", "menu"]);
    });

    it("useLevel resolves the nearest LevelProvider when nested, defaulting to base with none", () => {
      const ProbeLevel: React.FC = () => {
        const level = useLevel();
        return <span data-testid="probe-level" data-level={level} />;
      };
      const { container, rerender } = render(<ProbeLevel />);
      expect(container.querySelector('[data-testid="probe-level"]')?.getAttribute("data-level")).toBe("base");

      rerender(
        <LevelProvider level="window">
          <LevelProvider level="dialog">
            <ProbeLevel />
          </LevelProvider>
        </LevelProvider>,
      );
      expect(container.querySelector('[data-testid="probe-level"]')?.getAttribute("data-level")).toBe("dialog");
    });

    it("a transient (Popover) always resolves menu level on its own portaled content root, regardless of the ambient LevelProvider", () => {
      render(
        <LevelProvider level="dialog">
          <Popover open>
            <PopoverTrigger>Open</PopoverTrigger>
            <PopoverContent>Body</PopoverContent>
          </Popover>
        </LevelProvider>,
      );
      const content = document.querySelector('[data-slot="popover-content"]');
      expect(content?.getAttribute("data-level")).toBe("menu");
      expect(content?.className).toContain("z-menu");
      expect(content?.className).toContain("ui-glass");
      expect(content?.className).not.toContain("ui-glass-panel");
      expect(content?.className).not.toContain("ui-glass-menu");
    });

    it("WindowChrome with level=dialog stamps its level, keeps payload transparent, and paints chip glass", () => {
      const { container } = render(<WindowChrome level="dialog" stackSlot="level-dialog-stack" titleChips={<span>Title</span>} body={<div data-testid="dialog-body">Body</div>} />);
      const stack = container.querySelector('[data-slot="level-dialog-stack"]') as HTMLElement;
      expect(stack.getAttribute("data-level")).toBe("dialog");
      const body = container.querySelector('[data-slot="window-chrome-body"]') as HTMLElement;
      expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
      expect(body.className).not.toContain("ui-glass");
      const chip = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
      expect(chip.className).toContain("ui-glass");
      expect(screen.getByTestId("dialog-body")).toBeTruthy();
    });

    // 🪜️ D1 regression: `ui-surface`/`ui-glass`/`ui-veil` are the only per-level fills — cn() must
    // merge them like any other bg-color utility (mutually exclusive, last-in-cn wins, both
    // directions), never silently keeping a stray `bg-transparent`/`bg-active-base` alongside one.
    it("cn() treats ui-surface/ui-glass/ui-veil as bg-color-group utilities: last one wins, both directions", () => {
      expect(cn("bg-transparent", glassClass)).toBe(glassClass);
      expect(cn(glassClass, "bg-transparent")).toBe("bg-transparent");
      expect(cn(surfaceClass, "bg-active-base")).toBe("bg-active-base");
      expect(cn("bg-active-base", surfaceClass)).toBe(surfaceClass);
      expect(cn(surfaceClass, glassClass)).toBe(glassClass);
      // modifier'd utilities (hover:/data-[...]:) are a distinct conflict key — never dropped by a fill
      expect(cn(surfaceClass, "hover:bg-hover-interactive-fill")).toBe(`${surfaceClass} hover:bg-hover-interactive-fill`);
    });

    // 🪟️ D2 regression: the introduction/context-menu title chip must never re-paint an opaque fill
    // or rectangular border over the already-painted chip-cap cell it sits inside.
    it("windowChromeTitleChipClass never carries an opaque fill or piecewise border of its own", () => {
      expect(windowChromeTitleChipClass).not.toContain("ui-surface");
      expect(windowChromeTitleChipClass).toContain("bg-transparent");
      expect(windowChromeTitleChipClass).toContain("border-0");
      expect(windowChromeTitleChipClass).not.toMatch(/(?:^|\s)border(?:\s|$)/);
      expect(windowChromeTitleChipClass).not.toContain(borderNormalClass);
    });

    it("useSurface resolves the nearest Surface/SurfaceScope, or null outside any", () => {
      const ProbeSurface: React.FC = () => {
        const surface = useSurface();
        return <span data-testid="probe-surface" data-level={surface?.level ?? "none"} data-fill={surface?.fill ?? "none"} />;
      };
      const { container: outside } = render(<ProbeSurface />);
      expect(outside.querySelector('[data-testid="probe-surface"]')?.getAttribute("data-level")).toBe("none");

      const { container: inside } = render(
        <Surface level="panel" fill="glass">
          <ProbeSurface />
        </Surface>,
      );
      const probe = inside.querySelector('[data-testid="probe-surface"]');
      expect(probe?.getAttribute("data-level")).toBe("panel");
      expect(probe?.getAttribute("data-fill")).toBe("glass");
    });

    it("Surface warns (dev-only) when nested inside an ancestor Surface painting the same level, never when levels differ or fill is none", () => {
      const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => undefined);
      render(
        <Surface level="panel" fill="glass">
          <Surface level="panel" fill="surface">
            <span />
          </Surface>
        </Surface>,
      );
      expect(warnSpy).toHaveBeenCalledTimes(1);
      warnSpy.mockClear();

      render(
        <Surface level="panel" fill="glass">
          <Surface level="pane" fill="glass">
            <span />
          </Surface>
        </Surface>,
      );
      expect(warnSpy).not.toHaveBeenCalled();

      render(
        <Surface level="panel" fill="glass">
          <Surface level="panel" fill="none">
            <span />
          </Surface>
        </Surface>,
      );
      expect(warnSpy).not.toHaveBeenCalled();
      warnSpy.mockRestore();
    });

    it("shellFloorPaints/shellFloorFillClass defer opaque fill only on an already-painted base floor", () => {
      expect(shellFloorPaints(null)).toBe(true);
      expect(shellFloorFillClass(null)).toBe(surfaceClass);
      expect(shellFloorPaints({ level: "base", fill: "surface" })).toBe(false);
      expect(shellFloorFillClass({ level: "base", fill: "surface" })).toBe("bg-transparent");
      expect(shellFloorPaints({ level: "base", fill: "none" })).toBe(true);
      expect(shellFloorPaints({ level: "window", fill: "surface" })).toBe(true);
    });

    it("Layout paints one continuous base floor; navbar, footer, canvas, and mode-body stay transparent over it", () => {
      const { container } = render(
        <Layout
          navbar={<Navbar items={[{ key: "n", content: "Nav" }]} showFullscreenToggle={false} />}
          footer={<Footer items={[{ key: "f", content: "Foot" }]} />}
          canvas={<Mode windows={[{ id: "w", title: uiDataLabel("W"), iconId: "app-window", children: <div>Body</div> }]} activeWindowId="w" onActiveWindowChange={() => {}} />}
        />,
      );
      const layout = container.querySelector('[data-slot="layout"]');
      expect(layout?.getAttribute("data-level")).toBe("base");
      expect(layout?.className).toContain("ui-surface");
      const navbar = container.querySelector('[data-slot="navbar"]');
      expect(navbar?.getAttribute("data-level")).toBe("base");
      expect(navbar?.className).toContain("bg-transparent");
      expect(navbar?.className).not.toContain("ui-surface");
      const footer = container.querySelector('[data-slot="footer"]');
      expect(footer?.getAttribute("data-level")).toBe("base");
      expect(footer?.className).toContain("bg-transparent");
      expect(footer?.className).not.toContain("ui-surface");
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.getAttribute("data-level")).toBe("base");
      expect(modeBody?.className).toContain("bg-transparent");
      expect(modeBody?.className).not.toContain("ui-surface");
    });

    it("Navbar, Footer, Canvas, and Mode-body paint ui-surface at base when used standalone outside Layout", () => {
      expect(renderToStaticMarkup(<Navbar items={[{ key: "n", content: "Nav" }]} showFullscreenToggle={false} />)).toContain("ui-surface");
      expect(renderToStaticMarkup(<Footer items={[{ key: "f", content: "Foot" }]} />)).toContain("ui-surface");
      expect(renderToStaticMarkup(<Canvas>c</Canvas>)).toContain("ui-surface");
      const { container } = render(<Mode windows={[{ id: "w", title: uiDataLabel("W"), iconId: "app-window", children: <div>Body</div> }]} activeWindowId="w" onActiveWindowChange={() => {}} />);
      expect(container.querySelector('[data-slot="mode-body"]')?.className).toContain("ui-surface");
    });
  });
}
