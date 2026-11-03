// #region 🔌️Adapters
import * as React from "react";
import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { TableAvatar } from "../../🟦️.tsx";
import spacing from "../🧫️fixtures/📐️spacing/🔣️.json";
import { calc } from "@csstools/css-calc";
import styling from "../../../../🎨️styling/🔣️.json";
// #endregion 🔌️Adapters

afterEach(cleanup);

// #region 📻️TableAvatarMatrix
describe("TableAvatar", () => {
  it("uses the neutral sizing contract with positive border-interior clearance", () => {
    const { container } = render(<TableAvatar name="Ada Lovelace" />);
    expect(container.querySelector('[data-slot="avatar"]')?.classList.contains(spacing.sizeClass)).toBe(true);
    const singlePx = Number.parseFloat(styling.spacing.compact) * styling.metrics.dom.rootRemPx;
    expect(singlePx).toBe(spacing.gapPx);
    expect(Number.parseFloat(calc(`calc(1.5 * ${styling.metrics.chrome.controlHeightSmallUiSpacing} * ${singlePx}px)`))).toBe(spacing.outerPx);
    expect(styling.metrics.typography.textXsLineHeightPx).toBe(spacing.lineBoxPx);
    for (const vector of spacing.cases) {
      const interior = spacing.outerPx - 2 * vector.borderPx;
      expect(interior).toBe(vector.interiorPx);
      expect(Number.parseFloat(calc(`calc(${spacing.outerPx}px - 2 * ${vector.borderPx}px)`))).toBe(interior);
      expect((interior - spacing.lineBoxPx) / 2).toBe(vector.clearancePx);
      expect((interior - spacing.iconPx) / 2).toBeGreaterThan(0);
    }
  });

  it.each([{}, { isSelected: true }, { isHovered: true }])("keeps fallback typography invariant for %j", (state) => {
    const { container } = render(<TableAvatar name="Ada Lovelace" {...state} />);
    expect(container.querySelector('[data-slot="avatar-fallback"]')?.classList.contains(spacing.textClass)).toBe(true);
  });

  it("contains images and raw SVG icons within the avatar interior", () => {
    const { container, rerender } = render(<TableAvatar name="Ada Lovelace" icon="/ada.png" />);
    expect(container.querySelector('[data-slot="avatar-image"]')?.classList.contains("object-cover")).toBe(true);
    expect(container.querySelector('[data-slot="avatar-image"]')?.classList.contains("p-px")).toBe(true);
    rerender(<TableAvatar name="Ada Lovelace" icon={<svg width="100" height="100" />} />);
    const icon = container.querySelector('[data-slot="avatar-icon"]');
    expect(icon?.classList.contains(spacing.iconClass)).toBe(true);
    expect(icon?.classList.contains("[&_svg]:size-full")).toBe(true);
    expect(icon?.classList.contains("[&>*]:size-full")).toBe(true);
    rerender(<TableAvatar icon={<span><svg width="100" height="100" /></span>} />);
    expect(container.querySelector('[data-slot="avatar-icon"] > span > svg')).not.toBeNull();
  });

  it("shows a semantic fallback until the current image loads and restores it on error", () => {
    const { container, rerender } = render(<TableAvatar name="Ada Lovelace" icon="/ada.png" />);
    const image = container.querySelector('[data-slot="avatar-image"]') as HTMLImageElement;
    const fallback = container.querySelector('[data-slot="avatar-fallback"]') as HTMLSpanElement;

    expect(image.alt).toBe("Ada Lovelace");
    expect(image.hidden).toBe(true);
    expect(fallback.hidden).toBe(false);
    expect(fallback.textContent).toBe("AL");
    expect(fallback.getAttribute("aria-label")).toBe("Ada Lovelace");

    fireEvent.load(image);
    expect(image.hidden).toBe(false);
    expect(fallback.hidden).toBe(true);

    rerender(<TableAvatar name="Ada Lovelace" icon="/ada-new.png" />);
    const newImage = container.querySelector('[data-slot="avatar-image"]') as HTMLImageElement;
    expect(newImage.hidden).toBe(true);
    expect(fallback.hidden).toBe(false);

    fireEvent.error(newImage);
    expect(newImage.hidden).toBe(true);
    expect(fallback.hidden).toBe(false);
  });

  it("forwards the root ref, class and style while retaining selected and hovered rings", () => {
    const ref = React.createRef<HTMLSpanElement>();
    const { getByTestId } = render(
      <>
        <TableAvatar ref={ref} data-testid="selected" name="Selected Person" className="custom-avatar" style={{ width: 24 }} fallbackStyle={{ color: "red" }} isSelected />
        <TableAvatar data-testid="hovered" name="Hovered Person" isHovered />
      </>,
    );
    const selected = getByTestId("selected");
    const hovered = getByTestId("hovered");

    expect(ref.current).toBe(selected);
    expect(selected.className).toContain("custom-avatar");
    expect(selected.className).toContain("ring-[color:var(--active-base)]");
    expect(selected.style.width).toBe("24px");
    expect(selected.querySelector<HTMLElement>('[data-slot="avatar-fallback"]')?.style.color).toBe("red");
    expect(hovered.className).toContain("ring-[color:var(--hover-base)]");
  });
});
// #endregion 📻️TableAvatarMatrix
