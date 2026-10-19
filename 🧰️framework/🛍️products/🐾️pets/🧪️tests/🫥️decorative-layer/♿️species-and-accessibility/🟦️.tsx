/** ♿️ The shiny species, menagerie admission and accessibility tests. */
import { getRoles, isInaccessible, render, within } from "@testing-library/react";
import { roles } from "aria-query";
import { expect, it, vi } from "vitest";
import { menagerieIssues, type Menagerie } from "@semio-tech/pets";
import { PET_LAYER_Z_INDEX, PetLayer } from "@semio-tech/pets-react";
import { HOMELESS, MENAGERIE, layerOf, page, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** 🦋 Registers the shiny species update, menagerie admission, hidden element, assistive technology and keyboard exposure tests. */
export function registerSpeciesAndAccessibilityTests(): void {
  it("updates shiny species on existing motionless actors without changing their positions", () => {
    page();
    const { container, rerender } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={7} shiny={["alba"]} />);
    vi.advanceTimersByTime(1000);
    const layer = layerOf(container);
    const alba = layer.querySelector('[data-pet="alba"]')!;
    expect(alba.hasAttribute("data-pet-shiny")).toBe(true);
    expect(layer.querySelector('[data-pet="bruno"]')?.hasAttribute("data-pet-shiny")).toBe(false);
    const position = alba.getAttribute("style");
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={7} shiny={["bruno"]} />);
    vi.advanceTimersByTime(100);
    expect(layer.querySelector('[data-pet="alba"]')).toBe(alba);
    expect(alba.hasAttribute("data-pet-shiny")).toBe(false);
    expect(alba.getAttribute("style")).toBe(position);
    expect(layer.querySelector('[data-pet="bruno"]')?.hasAttribute("data-pet-shiny")).toBe(true);
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={7} />);
    vi.advanceTimersByTime(100);
    expect(layer.querySelectorAll("[data-pet-shiny]")).toHaveLength(0);
  });
  it("is built on a menagerie the core accepts", () => {
    expect(menagerieIssues(MENAGERIE)).toEqual([]);
    expect(menagerieIssues(HOMELESS)).toEqual([]);
  });
  it("renders one static element hidden from assistive technology and lets a cast appear in it", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    const layer = layerOf(container);
    expect(layer.tagName).toBe("DIV");
    expect(layer.getAttribute("aria-hidden")).toBe("true");
    expect(layer.getAttributeNames().sort()).toEqual(["aria-hidden", "class", "data-pet-scene", "style"]);
    expect(layer.className).toBe("pet-layer");
    expect(layer.style.zIndex).toBe(String(PET_LAYER_Z_INDEX));
    expect(layer.childElementCount).toBe(0);
    vi.advanceTimersByTime(3000);
    const pets = standing(layer);
    expect(pets.map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    expect(layer.childElementCount).toBe(3);
    for (const pet of pets) {
      expect(Math.abs(pet.flip), pet.species).toBe(1);
      expect(pet.scale, pet.species).toBe(1);
      expect(pet.opacity, pet.species).toBeGreaterThan(0);
    }
    expect(layerOf(container)).toBe(layer);
  });
  it("exposes nothing to assistive technology and nothing to the keyboard", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    const nodes = [layer, ...layer.querySelectorAll("*")];
    expect(nodes.length).toBeGreaterThan(20);
    expect(getRoles(layer)).toEqual({});
    expect(Object.values(getRoles(container)).flat()).toEqual([container]);
    for (const node of nodes) expect(isInaccessible(node), node.nodeName).toBe(true);
    const known = [...roles.keys()].filter((role) => roles.get(role)?.abstract !== true);
    expect(known.length).toBeGreaterThan(60);
    for (const role of known) expect(within(container).queryAllByRole(role), role).toEqual([]);
    for (const node of nodes) {
      expect((node as HTMLElement | SVGElement).tabIndex, node.nodeName).toBe(-1);
      for (const name of ["tabindex", "role", "id", "title", "aria-label", "aria-labelledby", "aria-live", "href", "contenteditable"]) expect(node.hasAttribute(name), `${node.nodeName} ${name}`).toBe(false);
      expect(node.matches("a, button, input, select, textarea, summary, details, iframe, object, embed, audio, video, title, desc, style, script, text, foreignObject"), node.nodeName).toBe(false);
    }
    for (const pet of layer.querySelectorAll("svg")) expect(pet.getAttribute("focusable")).toBe("false");
    expect(layer.textContent).toBe("");
  });
  it("never takes pointer events without its stylesheet either: a running show says so on the layer itself", () => {
    page();
    expect(document.head.querySelector("style")).toBeNull();
    const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(layer.style.pointerEvents).toBe("none");
    expect(layer.querySelectorAll("svg.pet").length).toBe(3);
    for (const node of layer.querySelectorAll("*")) expect(getComputedStyle(node).pointerEvents, node.nodeName).toBe("none");
    unmount();
    expect(layer.getAttribute("style") ?? "").toBe("");
  });
  it("shows nothing of a menagerie the core rejects, so nothing the schema forbids reaches the page", () => {
    page();
    const forged: Menagerie = { ...MENAGERIE, species: MENAGERIE.species.map((species, index) => (index === 0 ? { ...species, palette: { ...species.palette, body: "url(https://example.org/pixel)" } } : species)) };
    expect(menagerieIssues(forged).map((issue) => issue.code)).toEqual(["out-of-range"]);
    const { container } = render(<PetLayer menagerie={forged} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(5000);
    const layer = layerOf(container);
    expect(layer.childElementCount).toBe(0);
    expect(layer.getAttributeNames().sort()).toEqual(["aria-hidden", "class", "data-pet-scene"]);
    expect(vi.getTimerCount()).toBe(0);
  });
}
