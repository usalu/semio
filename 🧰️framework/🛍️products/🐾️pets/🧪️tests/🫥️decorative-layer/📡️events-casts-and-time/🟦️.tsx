/** 📡️ The prop events, cast reporting, time scale and rotation tests. */
import { render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { castOf, type Menagerie } from "@semio-tech/pets";
import { PET_LAYER_Z_INDEX, PET_SHUN_SECONDS, PetLayer, petCast } from "@semio-tech/pets-react";
import { HOMELESS, MENAGERIE, layerOf, page, place, specimen, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** ⏱ Registers the changed props events, the on stage report, the time scale, the cast rotation and the unknown scene tests. */
export function registerEventsCastsAndTimeTests(): void {
  it("turns changed props into events of the same stage instead of rendering anew", () => {
    page();
    const { container, rerender } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    const alba = layer.querySelector('[data-pet="alba"]');
    const carla = layer.querySelector('[data-pet="carla"]');
    expect(alba).not.toBeNull();
    expect(carla).not.toBeNull();
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} zIndex={12} scale={0.5} />);
    vi.advanceTimersByTime(100);
    expect(layerOf(container)).toBe(layer);
    expect(layer.style.zIndex).toBe("12");
    expect(layer.querySelector('[data-pet="alba"]')).toBe(alba);
    expect(standing(layer).map((pet) => pet.scale)).toEqual([0.5, 0.5, 0.5]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} capacity={2} />);
    vi.advanceTimersByTime(20_000);
    expect(layer.style.zIndex).toBe(String(PET_LAYER_Z_INDEX));
    const fewer = standing(layer)
      .map((pet) => pet.species)
      .sort();
    expect(fewer).toHaveLength(2);
    expect(fewer[1]).toBe("carla");
    expect(["alba", "bruno"]).toContain(fewer[0]);
    expect(layer.querySelector('[data-pet="carla"]')).toBe(carla);
    rerender(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} />);
    vi.advanceTimersByTime(20_000);
    expect(standing(layer).map((pet) => pet.species)).toEqual(["dora"]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="somewhere-else" mode="calm" seed={7} />);
    vi.advanceTimersByTime(20_000);
    expect(standing(layer)).toEqual([]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    expect(standing(layer).map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={7} />);
    vi.advanceTimersByTime(1000);
    const frozen = layer.innerHTML;
    expect(vi.getTimerCount()).toBe(0);
    vi.advanceTimersByTime(60_000);
    expect(layer.innerHTML).toBe(frozen);
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="lively" seed={7} />);
    expect(vi.getTimerCount()).toBe(1);
  });
  it("tells its host who is on stage, in the order of the menagerie, only when that changes, and nobody once it is gone", () => {
    page();
    const heard: string[] = [];
    const listen = (species: readonly string[]): void => {
      heard.push(species.join(" "));
    };
    const { rerender, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} onCast={listen} />);
    expect(heard).toEqual([]);
    vi.advanceTimersByTime(3000);
    expect(heard).toEqual(["alba bruno carla"]);
    vi.advanceTimersByTime(30_000);
    expect(heard).toEqual(["alba bruno carla"]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} onCast={listen} />);
    vi.advanceTimersByTime(20_000);
    expect(heard.at(-1)).toBe("dora");
    for (let index = 1; index < heard.length; index++) expect(heard[index]).not.toBe(heard[index - 1]);
    const other: string[] = [];
    rerender(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} onCast={(species) => void other.push(species.join(" "))} />);
    vi.advanceTimersByTime(100);
    expect(other).toEqual(["dora"]);
    const told = heard.length;
    unmount();
    expect(other).toEqual(["dora", ""]);
    expect(heard).toHaveLength(told);
  });
  it("lets a host make the pets' time pass faster or slower, held between an eighth and eight times the wall clock", () => {
    page();
    const faded = (tempo: number | undefined, milliseconds: number): number => {
      const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} tempo={tempo} />);
      vi.advanceTimersByTime(milliseconds);
      const opacity = standing(layerOf(container))[0]!.opacity;
      unmount();
      return opacity;
    };
    const real = faded(undefined, 100);
    expect(real).toBeGreaterThan(0.2);
    expect(real).toBeLessThan(0.5);
    expect(faded(1, 100)).toBe(real);
    expect(faded(4, 100)).toBe(1);
    expect(faded(0.25, 100)).toBeLessThan(real);
    expect(Math.abs(faded(0.25, 400) - real)).toBeLessThanOrEqual(0.125);
    expect(faded(1000, 50)).toBe(faded(8, 50));
    expect(faded(0, 400)).toBe(faded(0.125, 400));
  });
  it("lets the rotation of a cast take turns every minute or two, never on a still or a quiet stage", () => {
    page();
    const company = (layer: Element): string[] =>
      standing(layer)
        .map((pet) => pet.species)
        .sort();
    const lively = render(<PetLayer menagerie={MENAGERIE} scene="parade" mode="calm" seed={7} capacity={2} />);
    const layer = layerOf(lively.container);
    vi.advanceTimersByTime(2000);
    const opening = company(layer);
    expect(opening).toHaveLength(2);
    expect(opening).toContain("alba");
    const turns = new Set<string>(opening);
    vi.advanceTimersByTime(55_000);
    expect(company(layer)).toEqual(opening);
    for (let second = 0; second < 200; second += 5) {
      vi.advanceTimersByTime(5000);
      const now = company(layer);
      expect(now).toContain("alba");
      expect(now.length).toBeLessThanOrEqual(3);
      for (const species of now) turns.add(species);
    }
    expect(turns.size).toBeGreaterThanOrEqual(3);
    lively.unmount();

    for (const [mode, quiet] of [["still", false], ["calm", true]] as const) {
      const resting = render(<PetLayer menagerie={MENAGERIE} scene="parade" mode={mode} quiet={quiet} seed={7} capacity={2} />);
      vi.advanceTimersByTime(2000);
      const cast = company(layerOf(resting.container));
      expect(cast, mode).toEqual(opening);
      for (let second = 0; second < 130; second += 10) {
        vi.advanceTimersByTime(10_000);
        expect(company(layerOf(resting.container)), mode).toEqual(cast);
      }
      resting.unmount();
    }
  });
  it("casts from the ground: a page that leaves room for slim pets only shows them within a survey, whatever the rotation would have cast", () => {
    const wide = { ...specimen("wide", "#111111"), size: { width: 90, height: 44 } };
    const slim = specimen("slim", "#222222");
    const menagerie: Menagerie = { ...MENAGERIE, id: "narrow-ground", species: [wide, slim], bonds: [], casts: [{ scene: "home", core: ["wide", "slim"], rotation: [] }] };
    const cast = petCast(menagerie, "home")!;
    const blind = [1, 2, 3, 4, 5, 6].filter((seed) => castOf(cast, 1, 0, seed)[0] === "wide");
    expect(blind.length, "seeds whose cast ignores the ground picks the wide pet that cannot stand").toBeGreaterThan(0);
    for (const seed of [1, 2, 3, 4, 5, 6]) {
      const { first, second } = page();
      place(first, { left: 100, top: 300, width: 60, height: 200 });
      place(second, { left: 2000, top: 420, width: 400, height: 200 });
      const floor = place(document.createElement("p"), { left: 0, top: 700, width: 1200, height: 100 });
      document.body.append(floor);
      const { container, unmount } = render(<PetLayer menagerie={menagerie} scene="home" mode="calm" seed={seed} capacity={1} />);
      vi.advanceTimersByTime(3000);
      expect(standing(layerOf(container)).map((pet) => pet.species), `seed ${seed}`).toEqual(["slim"]);
      unmount();
      document.body.replaceChildren();
    }
  });
  it("settles on the species that leave room for themselves when a page takes the room a pet stood on by its very presence", async () => {
    const long = { ...specimen("long", "#111111"), size: { width: 40, height: 44 } };
    const short = specimen("short", "#222222");
    const menagerie: Menagerie = { ...MENAGERIE, id: "reflow", species: [long, short], bonds: [], casts: [{ scene: "home", core: ["long", "short"], rotation: [] }] };
    const cast = petCast(menagerie, "home")!;
    const seed = [1, 2, 3, 4, 5, 6].find((candidate) => castOf(cast, 1, 0, candidate)[0] === "long")!;
    const { first, second } = page();
    place(first, { left: 2000, top: 300, width: 400, height: 200 });
    place(second, { left: 2000, top: 420, width: 400, height: 200 });
    const row = place(document.createElement("p"), { left: 0, top: 700, width: 1200, height: 100 });
    const heard: string[] = [];
    const listen = (species: readonly string[]): void => {
      heard.push(species.join(" "));
      if (species.includes("long")) document.body.append(row);
      else row.remove();
    };
    const { container } = render(<PetLayer menagerie={menagerie} scene="home" mode="calm" seed={seed} capacity={1} onCast={listen} />);
    await vi.advanceTimersByTimeAsync(20_000);
    expect(PET_SHUN_SECONDS).toBeGreaterThan(10);
    expect(standing(layerOf(container)).map((pet) => pet.species)).toEqual(["short"]);
    expect(document.body.contains(row)).toBe(false);
    const settled = heard.length;
    await vi.advanceTimersByTimeAsync(10_000);
    expect(heard.length, `the cast heard: ${heard.join(" | ")}`).toBe(settled);
    expect(settled).toBeLessThanOrEqual(6);
  });
  it("shows nobody for an unknown scoped scene even when a home cast exists", () => {
    page();
    expect(petCast(MENAGERIE, "garden")?.core).toEqual(["dora"]);
    expect(petCast(MENAGERIE, "nowhere")).toBeNull();
    expect(petCast(HOMELESS, "nowhere")).toBeNull();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="nowhere" mode="calm" seed={7} />);
    expect(layerOf(container).getAttribute("data-pet-scene")).toBe("nowhere");
    vi.advanceTimersByTime(5000);
    expect(layerOf(container).childElementCount).toBe(0);
    expect(vi.getTimerCount()).toBe(0);
  });
}
