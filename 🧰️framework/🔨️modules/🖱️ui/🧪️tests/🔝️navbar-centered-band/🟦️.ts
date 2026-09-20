/** 🛟️ The centered-navbar-band law: a `centered` navbar (or footer) item is centered on the bar, and never
 * across the chrome the bar's own flow row carries.
 *
 * The defect this pins: the centered playground cluster is absolutely positioned over the whole bar, so
 * it is invisible to the flow row's layout. When the `top-right` `PanelChromeTabBar` grew a `Tool runs`
 * tab the rail's left edge moved from ~1039 px to 947 px, and at 1280 px the navbar `Viewer` role button
 * (923…1016) ended up UNDER it — `document.elementFromPoint` at the button's own centre resolved to
 * `framework.panel.inspection`, so every click on the role switch reached the panel rail instead and the
 * shell never left the editor role (`📓️role-switch-regression-2026-09-14.md`).
 *
 * 🧫️ Read from `🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json`.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🔝️navbar-centered-band/🔣️.json" with { type: "json" };
import { navbarCenteredLeftV1, navbarFlowChildOccupiesV1, navbarFreeBandV1 } from "../../🧱️elements/🔝️Navbar/🟦️.tsx";

describe("🛟️ navbar centered band", () => {
  it("declares the contract the bar answers to", () => {
    expect(fixture.contract.centeredItemNeverCrossesOccupiedChrome).toBe(true);
    expect(fixture.contract.fullyOccupiedBarKeepsWholeWidth).toBe(true);
    expect(fixture.contract.emptyFlowChildIsFreeRoom).toBe(true);
    expect(fixture.contract.itemWiderThanBandStartsAtBandLeft).toBe(true);
    expect(fixture.contract.topMiddleBelongsToCenteredCluster).toBe(true);
    expect(fixture.contract.footerTrailingOrder).toEqual(["presence", "hubConnection", "bottomRight"]);
    expect(fixture.contract.mobileNavbarOmitsDesktopCenterControls).toBe(true);
    expect(fixture.contract.mobileFooterKeepsHubConnection).toBe(true);
    expect(fixture.contract.dockCapDepthUsesControlAndPadding).toBe(true);
  });

  for (const scenario of fixture.occupancy) {
    it(scenario.name, () => {
      expect(navbarFlowChildOccupiesV1({ childElementCount: scenario.childElementCount, textContent: scenario.textContent })).toBe(scenario.occupies);
    });
  }

  for (const scenario of fixture.bands) {
    it(scenario.name, () => {
      expect(navbarFreeBandV1(scenario.width, scenario.occupied)).toEqual(scenario.band);
    });
  }

  for (const scenario of fixture.placements) {
    it(scenario.name, () => {
      expect(navbarCenteredLeftV1(scenario.width, scenario.band, scenario.contentWidth)).toBe(scenario.left);
    });
  }

  it("never lets a centered item cross the band it was placed in", () => {
    for (const scenario of fixture.placements) {
      const left = navbarCenteredLeftV1(scenario.width, scenario.band, scenario.contentWidth);
      const width = Math.min(scenario.contentWidth, scenario.band.right - scenario.band.left);
      expect(left).toBeGreaterThanOrEqual(scenario.band.left);
      expect(left + width).toBeLessThanOrEqual(scenario.band.right);
    }
  });

  it("is a fixed point: placing an item that its own max-width already clamped answers the same left", () => {
    for (const scenario of fixture.placements) {
      const bandWidth = scenario.band.right - scenario.band.left;
      const clamped = Math.min(scenario.contentWidth, bandWidth);
      expect(navbarCenteredLeftV1(scenario.width, scenario.band, clamped)).toBe(navbarCenteredLeftV1(scenario.width, scenario.band, Math.min(clamped, bandWidth)));
    }
  });

  it("answers the room the 1280 px playground navbar actually leaves, and keeps the role group inside it", () => {
    const width = 1280;
    const band = navbarFreeBandV1(width, [
      { left: 0, right: 173 },
      { left: 947, right: 1185 },
    ]);
    expect(band).toEqual({ left: 173, right: 947 });
    const left = navbarCenteredLeftV1(width, band, 753);
    expect(left + 753).toBeLessThanOrEqual(947);
  });

  it("derives dock cap depth from its padded control while navbar height stays independently customizable", () => {
    for (const scenario of fixture.dockCapMetrics) {
      const controlHeight = scenario.spacing * scenario.controlHeightUiSpacing;
      const padding = scenario.spacing * scenario.paddingUiSpacing;
      expect(controlHeight).toBeCloseTo(scenario.controlHeight);
      expect(controlHeight + padding * 2).toBeCloseTo(scenario.capDepth);
      expect(scenario.spacing * scenario.navbarHeightUiSpacing).toBeCloseTo(scenario.navbarHeight);
    }
    expect(fixture.dockCapMetrics[1]!.capDepth).not.toBe(fixture.dockCapMetrics[1]!.navbarHeight);
  });
});
