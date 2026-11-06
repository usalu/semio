/** 💾️ Create, edit and reopen a 2D node through the real shell and board. */
import { join } from "node:path";
import { mkdirSync, readFileSync } from "node:fs";
import { OUT, stamp, currentLocale, mod, page, renderer, expectedReloads, setExpectedReloads, setProbeReloadAt } from "@semio-tech/framework/time-travel/testing/state";
import { attachFolder } from "@semio-tech/framework/time-travel/testing/history-session";
import { closePanels, waitForBoot } from "@semio-tech/framework/time-travel/testing/chrome";
import { prepareChord } from "@semio-tech/framework/time-travel/testing/band";
import { DESKTOP, onFreshPages, waitUntil } from "@semio-tech/framework/time-travel/testing/page";
import { verdict, note } from "@semio-tech/framework/time-travel/testing/report";
import { cameraOf, dragBy, frameBoard, movedIds, offsetOf, paneBox, pickCloneSource, positions, selectionIds, toScreen, vitals } from "../🎲️board/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { schema: string; addedNodes: number; minMovement: number };

/** 🧩️ A duplicate is created through the keyboard, dragged, saved to a local folder and reopened. */
export const step24 = async () => {
  verdict("persistence-journey-fixture", fixture.schema === "s.puzzle2d.persistence-journey.v1", { schema: fixture.schema });
  if (renderer !== "react") {
    note("persistence-journey-local-folder-unavailable", { renderer, reading: "The browser wgpu shell has no local folder transport; this journey runs on React." });
    return;
  }
  const folder = join(OUT, `puzzle2d-create-${stamp}-${currentLocale}`);
  mkdirSync(folder, { recursive: true });
  await closePanels();
  const attached = await attachFolder(folder);
  verdict("persistence-journey-folder-attached", attached.typed === folder && attached.attachButton, { attached });
  if (attached.typed !== folder || !attached.attachButton) return;
  await closePanels();
  await frameBoard(1);
  const spot = await pickCloneSource([]);
  if (!spot) {
    verdict("persistence-journey-source", false, {});
    return;
  }
  const before = await positions();
  await page.mouse.click(spot.row.at.x, spot.row.at.y);
  const selected = await waitUntil(vitals, (v) => selectionIds(v).includes(spot.row.id), 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`);
  const created = await waitUntil(positions, (p) => Object.keys(p).length === Object.keys(before).length + fixture.addedNodes, 30000);
  const id = Object.keys(created.value).find((key) => !(key in before));
  verdict("persistence-journey-created-node", selected.ok && created.ok && Boolean(id), { id });
  if (!id) return;
  await prepareChord();
  await page.keyboard.press("Escape");
  await waitUntil(vitals, (v) => !selectionIds(v).includes(id), 8000);
  const camera = cameraOf(await vitals());
  const center = toScreen(created.value[id], camera, await paneBox());
  const grab = { x: center.x + 8 * camera.zoom * Math.SQRT1_2, y: center.y + 8 * camera.zoom * Math.SQRT1_2 };
  await dragBy(grab, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const edited = await waitUntil(positions, (p) => offsetOf(created.value, p, id)?.some((v) => Math.abs(v) > fixture.minMovement) === true, 20000);
  verdict("persistence-journey-edited-node", edited.ok && movedIds(created.value, edited.value).every((key) => key === id), { offset: offsetOf(created.value, edited.value, id) });
  if (!edited.ok) return;
  let saved = false;
  await onFreshPages(async (open) => {
    const peer = await open(DESKTOP, "puzzle2d-persistence-readback");
    if (!peer) {
      verdict("persistence-journey-archive-readback", false, { booted: false });
      return;
    }
    await closePanels();
    const attached = await attachFolder(folder);
    const readback = await waitUntil(positions, (p) => movedIds(edited.value, p, 1e-6).length === 0, 60000, 1000);
    saved = attached.typed === folder && attached.attachButton && readback.ok;
    verdict("persistence-journey-archive-readback", saved, { attached, drift: movedIds(edited.value, readback.value) });
  });
  if (!saved) return;
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  const booted = await waitForBoot("puzzle2d-persistence", 60);
  const reattached = await attachFolder(folder);
  const restored = await waitUntil(positions, (p) => movedIds(edited.value, p, 1e-6).length === 0, 60000, 1000);
  verdict("persistence-journey-reopened-edit", booted && restored.ok, { reattached, drift: movedIds(edited.value, restored.value) });
  await closePanels();
  await prepareChord();
  await page.keyboard.press(`${mod}+z`);
  const undone = await waitUntil(positions, (p) => movedIds(created.value, p, 1e-6).length === 0, 30000);
  verdict("persistence-journey-undo-after-reopen", undone.ok, { drift: movedIds(created.value, undone.value) });
  await prepareChord();
  await page.keyboard.press(`${mod}+Shift+z`);
  const redone = await waitUntil(positions, (p) => movedIds(edited.value, p, 1e-6).length === 0, 30000);
  verdict("persistence-journey-redo-after-reopen", redone.ok, { drift: movedIds(edited.value, redone.value) });
};
