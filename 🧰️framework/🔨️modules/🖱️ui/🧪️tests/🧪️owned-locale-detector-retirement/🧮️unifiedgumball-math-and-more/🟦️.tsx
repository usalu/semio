import type * as Three from "three";
import * as ui from "@semio-tech/ui-react/source";
import { OrthographicCamera } from "@react-three/drei/core/OrthographicCamera.js";
import { PerspectiveCamera } from "@react-three/drei/core/PerspectiveCamera.js";
import { ICONS, assertUniqueIconConceptAssignments, shortcodeCatalogKey, shortcodeEmoji } from "@semio-tech/assets";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerUnifiedgumballMathAndMoreTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { DEFAULT_GUMBALL_CONFIG, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP, GUMBALL_DEFAULT_SHIFT_SCALE_SNAP, GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_SIZE, GUMBALL_PREVIEW_DISK_RADIUS, GUMBALL_PREVIEW_MIN_EXTENT, GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_RING_RADIUS, ICONS, Icon, OrthographicCamera, PerspectiveCamera, THREE, assertUniqueIconConceptAssignments, classifyIconSelectorMode, decodeIcon, encodeIcon, gumballApplyHandleVisualMaterial, gumballAxisRotateAngle, gumballAxisScaleFactor, gumballConfigVisible, gumballEffectiveSnapValue, gumballHandleAllowedByPlane, gumballHandleEnabled, gumballHandleKindToTransformMode, gumballHandleRaycast, gumballHandleVisualState, gumballKindFromRaycastObject, gumballPlaneScaleCorner, gumballPlaneScaleFactors, gumballPointerConsumesCanvasEventRef, gumballPreviewWorldExtent, gumballProjectRayOntoAxis, gumballRayAxisParameter, gumballRayFromNdc, gumballRayPlanePoint, gumballRaycastOwnedAtClientPoint, gumballResolveDragSnaps, gumballResolveHandleVisual, gumballScaleAxisOffset, gumballScalePlaneAxisIndices, gumballSnapScalar, iconSvgMarkup, parseUiTheme, renderToStaticMarkup, resolveCatalogIconSvg, resolveGumballConfig, resolveGumballVisualPalette, semioTheme, setActiveUiTheme, shortcodeCatalogKey, shortcodeEmoji } = dependencies;
  type Camera = any;
  type GumballVec3 = any;
  type IconName = any;
  const { describe, expect, it } = vitest;

  describe("UnifiedGumball math", () => {
    it("computes axis translate, plane hit, rotate angle, scale factor, and snapping", () => {
      expect(gumballRayAxisParameter([0, 0, 0], [0, 0, 1], [1, 2, 3], [1, 0, 0])).toBeCloseTo(1, 5);
      const eye: readonly [number, number, number] = [0, 0, 1];
      const axis: readonly [number, number, number] = [1, 0, 0];
      const pivot: readonly [number, number, number] = [0, 0, 0];
      const norm = (v: readonly [number, number, number]): readonly [number, number, number] => {
        const l = Math.hypot(v[0], v[1], v[2]);
        return [v[0] / l, v[1] / l, v[2] / l];
      };
      const leftParam = gumballProjectRayOntoAxis([0, 0, 10], norm([-0.2, 0, -1]), pivot, axis, eye);
      const rightParam = gumballProjectRayOntoAxis([0, 0, 10], norm([0.2, 0, -1]), pivot, axis, eye);
      expect(leftParam).not.toBeNull();
      expect(rightParam).not.toBeNull();
      expect(rightParam! > leftParam!).toBe(true);
      expect(gumballRayPlanePoint([0, 0, 0], [0, 0, 1], [0, 0, 0], [0, 0, 1])).toEqual([0, 0, 0]);
      expect(gumballAxisRotateAngle([1, 0, 0], [0, 1, 0], [0, 0, 1])).toBeCloseTo(Math.PI / 2, 5);
      expect(gumballAxisScaleFactor(2, 4)).toBe(2);
      expect(gumballSnapScalar(1.05, 0.5)).toBe(1);
      expect(gumballEffectiveSnapValue(undefined, false, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBeUndefined();
      expect(gumballEffectiveSnapValue(undefined, true, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBe(GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP);
      expect(gumballEffectiveSnapValue(0.25, true, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBe(0.25);
      const shiftRotate = gumballResolveDragSnaps({}, true);
      expect(shiftRotate.rotationSnap).toBe(GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP);
      expect(shiftRotate.scaleSnap).toBe(GUMBALL_DEFAULT_SHIFT_SCALE_SNAP);
      expect(gumballResolveDragSnaps({}, false).rotationSnap).toBeUndefined();
      expect(gumballSnapScalar(Math.PI / 3, Math.PI / 4)).toBeCloseTo(Math.PI / 4, 6);
      expect(gumballSnapScalar(Math.PI / 6, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBeCloseTo(Math.PI / 6, 6);
      expect(gumballHandleKindToTransformMode("moveYZ")).toBe("translate");
      expect(gumballHandleKindToTransformMode("rotateX")).toBe("rotate");
      expect(gumballHandleKindToTransformMode("scaleUniform")).toBe("scale");
      expect(gumballConfigVisible(DEFAULT_GUMBALL_CONFIG)).toBe(true);
      expect(gumballConfigVisible({ moveAxes: false, movePlanes: false, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false })).toBe(false);
      expect(gumballHandleAllowedByPlane("moveZ", undefined)).toBe(true);
      expect(gumballHandleAllowedByPlane("moveZ", "xy")).toBe(false);
      expect(gumballHandleAllowedByPlane("moveX", "xy")).toBe(true);
      expect(gumballHandleAllowedByPlane("rotateZ", "xy")).toBe(true);
      expect(gumballHandleAllowedByPlane("rotateX", "xy")).toBe(false);
      expect(gumballHandleAllowedByPlane("moveYZ", "yz")).toBe(true);
      expect(gumballHandleAllowedByPlane("rotateY", "xz")).toBe(true);
      const xyMove = resolveGumballConfig({ moveAxes: true, movePlanes: true, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false, plane: "xy" });
      expect(gumballHandleEnabled("moveX", xyMove)).toBe(true);
      expect(gumballHandleEnabled("moveZ", xyMove)).toBe(false);
      expect(gumballHandleEnabled("moveXY", xyMove)).toBe(true);
      expect(gumballHandleEnabled("moveYZ", xyMove)).toBe(false);
      expect(gumballHandleEnabled("rotateZ", xyMove)).toBe(true);
      expect(gumballHandleEnabled("rotateX", xyMove)).toBe(false);
      expect(gumballConfigVisible(xyMove)).toBe(true);
      expect(resolveGumballConfig({ plane: "yz" }).plane).toBe("yz");
      expect(gumballHandleKindToTransformMode("scaleXY")).toBe("scale");
      expect(gumballHandleVisualState("moveX", "moveY", null)).toBe("dimmed");
      expect(gumballHandleVisualState("moveX", "moveX", null)).toBe("hover");
      expect(gumballHandleVisualState("moveX", null, "moveX")).toBe("active");
      expect(gumballHandleVisualState("moveY", "moveX", "moveX")).toBe("dimmed");
      const palette = resolveGumballVisualPalette();
      expect(palette.axisX).toBe("#ff344f");
      expect(palette.axisY).toBe("#34d1bf");
      expect(palette.axisZ).toBe("#fa9500");
      expect(gumballResolveHandleVisual("#ff0000", "hover", palette).color).toBe("#ff0000");
      expect(gumballResolveHandleVisual("#ff0000", "hover", palette).opacity).toBe(palette.hoverOpacity);
      expect(gumballResolveHandleVisual("#ff0000", "active", palette).color).toBe("#ff0000");
      expect(gumballResolveHandleVisual("#ff0000", "active", palette).opacity).toBe(palette.activeOpacity);
      expect(gumballResolveHandleVisual("#ff0000", "idle", palette).color).toBe("#ff0000");
      expect(gumballResolveHandleVisual(palette.axisX, "hover", palette).color).toBe(palette.axisX);
      expect(gumballResolveHandleVisual(palette.axisY, "hover", palette).color).toBe(palette.axisY);
      expect(gumballResolveHandleVisual(palette.axisZ, "active", palette).color).toBe(palette.axisZ);
      const tintRoot = new THREE.Group();
      const visibleMesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshBasicMaterial({ color: "#000000" }));
      const pickMesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshBasicMaterial({ color: "#000000" }));
      pickMesh.visible = false;
      pickMesh.userData = { gumballHandlePick: true };
      tintRoot.add(visibleMesh, pickMesh);
      const tintMaterial = new THREE.MeshBasicMaterial({ color: palette.axisX, transparent: true, opacity: palette.hoverOpacity });
      gumballApplyHandleVisualMaterial(tintRoot, tintMaterial);
      expect(visibleMesh.material).toBe(tintMaterial);
      expect(pickMesh.material).not.toBe(tintMaterial);
      tintMaterial.dispose();
      visibleMesh.geometry.dispose();
      pickMesh.geometry.dispose();
      (visibleMesh.material as Three.Material).dispose();
      (pickMesh.material as Three.Material).dispose();
      expect(gumballScaleAxisOffset()).toBeGreaterThan(GUMBALL_RING_RADIUS);
      const perspective = new THREE.PerspectiveCamera(50, 1.6, 0.1, 1000);
      perspective.position.set(0, 0, 20);
      perspective.updateProjectionMatrix();
      const previewPivot = new THREE.Vector3(0, 0, 0);
      expect(gumballPreviewWorldExtent(perspective, previewPivot)).toBeGreaterThan(GUMBALL_PREVIEW_MIN_EXTENT);
      const ortho = new THREE.OrthographicCamera(-10, 10, 10, -10, 0.1, 1000);
      ortho.position.set(0, 0, 20);
      ortho.updateProjectionMatrix();
      expect(gumballPreviewWorldExtent(ortho, previewPivot)).toBeGreaterThan(9);
      expect(GUMBALL_PREVIEW_DISK_RADIUS).toBe(GUMBALL_RING_RADIUS);
      expect(GUMBALL_PREVIEW_DISK_RADIUS).toBeLessThan(GUMBALL_PREVIEW_RING_RADIUS);
      expect(gumballScalePlaneAxisIndices("scaleXY")).toEqual([0, 1]);
      expect(gumballScalePlaneAxisIndices("scaleYZ")).toEqual([1, 2]);
      expect(gumballScalePlaneAxisIndices("scaleXZ")).toEqual([0, 2]);
      expect(gumballPlaneScaleCorner("xy")[0]).toBeGreaterThan(GUMBALL_PLANE_OFFSET + GUMBALL_PLANE_SIZE * 0.5);
      expect(gumballPlaneScaleFactors([1, 2], [2, 4])).toEqual([2, 2]);
      expect(gumballPlaneScaleFactors([3, 4], [6, 8], true)).toEqual([2, 2]);
      expect(gumballPlaneScaleFactors([1, 0], [2, 0], true)).toEqual([2, 2]);
      const gumballMesh = new THREE.Mesh();
      gumballMesh.userData = { gumballHandleKind: "moveX" };
      const gumballChild = new THREE.Mesh();
      gumballMesh.add(gumballChild);
      expect(gumballKindFromRaycastObject(gumballMesh)).toBe("moveX");
      expect(gumballKindFromRaycastObject(gumballChild)).toBe("moveX");
      expect(gumballKindFromRaycastObject(null)).toBeNull();
      gumballPointerConsumesCanvasEventRef.current = true;
      gumballPointerConsumesCanvasEventRef.current = false;
      const gumballRoot = new THREE.Group();
      const gumballHandle = new THREE.Mesh(new THREE.BoxGeometry(2, 2, 2), new THREE.MeshBasicMaterial());
      gumballHandle.userData = { gumballHandleKind: "moveX" };
      gumballHandle.raycast = gumballHandleRaycast;
      gumballRoot.add(gumballHandle);
      const ownedCamera = new THREE.OrthographicCamera(-2, 2, 2, -2, 0.1, 100);
      ownedCamera.position.set(0, 0, 5);
      ownedCamera.lookAt(0, 0, 0);
      ownedCamera.updateMatrixWorld(true);
      const ownedCanvas = { getBoundingClientRect: () => ({ left: 0, top: 0, width: 200, height: 200, right: 200, bottom: 200, x: 0, y: 0, toJSON: () => ({}) }) } as HTMLElement;
      expect(gumballRaycastOwnedAtClientPoint(ownedCamera, ownedCanvas, 100, 100, gumballRoot)?.kind).toBe("moveX");
      expect(gumballRaycastOwnedAtClientPoint(ownedCamera, ownedCanvas, 10, 10, gumballRoot)).toBeNull();
      gumballHandle.geometry.dispose();
      (gumballHandle.material as Three.Material).dispose();
      const biasedPickMesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshBasicMaterial());
      biasedPickMesh.raycast = gumballHandleRaycast;
      const raycaster = new THREE.Raycaster();
      raycaster.set(new THREE.Vector3(0, 0, 10), new THREE.Vector3(0, 0, -1));
      const biased: Three.Intersection[] = [];
      biasedPickMesh.raycast(raycaster, biased);
      const plain: Three.Intersection[] = [];
      THREE.Mesh.prototype.raycast.call(biasedPickMesh, raycaster, plain);
      expect(biased.length).toBeGreaterThan(0);
      expect(plain.length).toBeGreaterThan(0);
      expect(biased[0]!.distance).toBeLessThan(plain[0]!.distance);
      biasedPickMesh.geometry.dispose();
      (biasedPickMesh.material as Three.Material).dispose();
    });

    it("builds parallel orthographic rays via isOrthographicCamera duck-typing (not instanceof)", () => {
      const ortho = new THREE.OrthographicCamera(-100, 100, 50, -50, 0.1, 1000);
      ortho.position.set(0, 0, 100);
      ortho.up.set(0, 1, 0);
      ortho.lookAt(0, 0, 0);
      ortho.updateMatrixWorld(true);
      ortho.updateProjectionMatrix();

      const center = gumballRayFromNdc(0, 0, ortho);
      const right = gumballRayFromNdc(0.5, 0, ortho);
      expect(center.dir[0]).toBeCloseTo(right.dir[0], 5);
      expect(center.dir[1]).toBeCloseTo(right.dir[1], 5);
      expect(center.dir[2]).toBeCloseTo(right.dir[2], 5);
      expect(Math.abs(right.origin[0] - center.origin[0])).toBeCloseTo(50, 5);

      const planeHit = (ray: { origin: GumballVec3; dir: GumballVec3 }): GumballVec3 => {
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, [0, 0, 0], [0, 0, 1]);
        expect(hit).not.toBeNull();
        return hit!;
      };
      const centerHit = planeHit(center);
      const rightHit = planeHit(right);
      expect(rightHit[0] - centerHit[0]).toBeCloseTo(50, 5);
      expect(rightHit[1] - centerHit[1]).toBeCloseTo(0, 5);

      // 🦆️ Duplicate-three failure mode: plain object with the flag but not `instanceof OrthographicCamera`.
      const duck = {
        isOrthographicCamera: true,
        position: ortho.position,
        matrixWorld: ortho.matrixWorld,
        projectionMatrix: ortho.projectionMatrix,
        projectionMatrixInverse: ortho.projectionMatrixInverse,
        left: ortho.left,
        right: ortho.right,
        top: ortho.top,
        bottom: ortho.bottom,
        zoom: ortho.zoom,
      } as unknown as Three.Camera;
      expect(duck instanceof THREE.OrthographicCamera).toBe(false);
      const duckCenter = gumballRayFromNdc(0, 0, duck);
      const duckRight = gumballRayFromNdc(0.5, 0, duck);
      expect(duckCenter.dir[0]).toBeCloseTo(duckRight.dir[0], 5);
      expect(duckCenter.dir[2]).toBeCloseTo(duckRight.dir[2], 5);
      expect(Math.abs(duckRight.origin[0] - duckCenter.origin[0])).toBeCloseTo(50, 5);
      expect(gumballPreviewWorldExtent(duck, new THREE.Vector3(0, 0, 0))).toBeGreaterThan(50);
    });
  });

  describe("iconSvgMarkup", () => {
    it("returns vendored svg markup for a known icon", () => {
      expect(iconSvgMarkup("component")).toContain("<svg");
    });
  });

  describe("iconCodec", () => {
    it("round-trips canonical icon strings", () => {
      const samples = ["url:https://example.com/icon.png", ":smile:", "data:image/png;base64,iVBORw0KGgo=", "emoji:☺️", "typst:$x^2$", "text:Hi", "pen-tool"];
      for (const sample of samples) {
        const icon = decodeIcon(sample);
        expect(icon).toBeTruthy();
        const encoded = encodeIcon(icon!);
        expect(decodeIcon(encoded)).toEqual(icon);
      }
    });

    it("decodes metabolism stems as themed and catalog ids as catalog", () => {
      expect(decodeIcon("capsule_J")).toEqual({ kind: "themed", key: "capsule_J" });
      expect(decodeIcon("pen-tool")).toEqual({ kind: "catalog", key: "pen-tool" });
      expect(decodeIcon("definitely-not-a-known-icon-stem")).toBeUndefined();
    });

    it("classifies selector modes for all kinds", () => {
      expect(classifyIconSelectorMode("url:https://x.test/a.png")).toBe("url");
      expect(classifyIconSelectorMode(":plus:")).toBe("shortcode");
      expect(shortcodeEmoji("grinning")).toBeTruthy();
      expect(shortcodeCatalogKey("plus")).toBe("plus");
      expect(classifyIconSelectorMode("text:Hi")).toBe("text");
      expect(classifyIconSelectorMode("capsule_J")).toBe("vector");
    });

    it("resolveCatalogIconSvg applies theme variants without changing icon id", () => {
      const themed = parseUiTheme({
        ...semioTheme(),
        icons: { variants: { search: "<svg data-theme-variant></svg>" } },
      });
      setActiveUiTheme(themed);
      expect(resolveCatalogIconSvg("search")).toContain("data-theme-variant");
      setActiveUiTheme(semioTheme());
    });
  });

  describe("icon concept assignments", () => {
    it("keeps every canonical concept on a unique catalog icon", () => {
      expect(() => assertUniqueIconConceptAssignments()).not.toThrow();
    });
  });

  describe("icon hover animations", () => {
    const FULL_TURN_KEYFRAMES = new Set(["icon-rotate-ccw", "icon-rotate-cw", "icon-loader-2", "icon-flip-horizontal", "icon-flip-vertical", "icon-settings", "icon-globe"]);
    const NON_CATALOG_KINDS = ["emoji", "text", "typst", "image", "svg", "node", "shortcode", "missing"];
    // Icons whose SVG has cleanly separable moving parts get a mechanism-accurate per-part animation
    // instead (--icon-animation: none plus one or more --icon-part-<N>-anim assignments); see 🔧️IconPartAnim.
    // Every icon below — and every whole-icon icon outside this set — has its own uniquely-named,
    // individually-tuned keyframes; nothing is shared across icons (see the specificity/exclusivity tests).
    // Physical-correctness pass moved hammer/box/move/arrow-up/arrow-down/arrow-left/arrow-right/undo/
    // undo-2/redo/redo-2 OUT of this set: each is one rigid object (hammer head+handle, arrow head+shaft,
    // the whole "move" cross, an undo/redo hook+curve), so it now animates as a single whole-icon rigid
    // motion instead of independently-moving parts. Remaining whole-icon icons either have a single fused
    // SVG path (no separable part exists without redrawing the vendored asset — e.g.
    // folder/folder-open/message-square/message-circle, handled with a best-effort single-rigid-body
    // motion instead) or their rigid motion already is the mechanically correct animation (globe, search,
    // settings, rotate-cw/ccw, flip-horizontal/vertical, text-cursor).
    const PART_MECHANISM_ICONS = new Set([
      "alert-circle",
      "align-left",
      "app-window",
      "arrow-right-left",
      "award",
      "bar-chart-3",
      "bell",
      "book-open",
      "calendar-days",
      "camera",
      "folder",
      "folder-open",
      "check-circle-2",
      "chevrons-up-down",
      "circle-dot",
      "clipboard",
      "clipboard-list",
      "clock",
      "code",
      "combine",
      "component",
      "copy",
      "crosshair",
      "cylinder",
      "download",
      "edit-3",
      "eraser",
      "external-link",
      "eye",
      "eye-off",
      "file",
      "file-archive",
      "file-code",
      "file-image",
      "file-json",
      "file-spreadsheet",
      "file-text",
      "file-type",
      "file-video",
      "focus",
      "git-branch",
      "git-commit",
      "git-merge",
      "graduation-cap",
      "grid-3x3",
      "grip-vertical",
      "hand",
      "hard-drive",
      "hash",
      "home",
      "image",
      "image-plus",
      "image-up",
      "info",
      "landmark",
      "lasso",
      "layers",
      "layout",
      "layout-grid",
      "library",
      "lightbulb",
      "link",
      "link-2-off",
      "list",
      "list-ordered",
      "list-tree",
      "lock",
      "lock-open",
      "magnet",
      "maximize-2",
      "minimize-2",
      "monitor",
      "more-horizontal",
      "mouse-pointer",
      "move-3d",
      "network",
      "paint-bucket",
      "paintbrush",
      "panel-left",
      "panel-right",
      "panel-top",
      "pause",
      "pen-tool",
      "pipette",
      "plug",
      "plus",
      "save",
      "scaling",
      "scissors",
      "settings-2",
      "shapes",
      "skip-back",
      "skip-forward",
      "smartphone",
      "smile",
      "sparkles",
      "square-arrow-down-left",
      "square-arrow-down-right",
      "square-arrow-up-left",
      "square-arrow-up-right",
      "square-dashed",
      "sun",
      "table-2",
      "tablet",
      "tags",
      "text-search",
      "trash-2",
      "triangle-alert",
      "unlink",
      "user",
      "users",
      "x",
      "zoom-in",
      "zoom-out",
    ]);

    async function readUiCss(): Promise<string> {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      return readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
    }

    it("gives every vendored icon id and non-catalog kind a hover keyframes block and animation assignment", async () => {
      const css = await readUiCss();
      for (const name of Object.keys(ICONS as Record<string, string>)) {
        if (PART_MECHANISM_ICONS.has(name)) {
          const ruleMatch = css.match(new RegExp(`\\[data-icon="${name}"\\] \\{([^}]*)\\}`));
          expect(ruleMatch, `missing [data-icon="${name}"] rule`).toBeTruthy();
          const rule = ruleMatch![1];
          expect(rule).toMatch(/--icon-animation:\s*none;/);
          const partAnims = [...rule.matchAll(/--icon-part-(\d+)-anim:\s*([\w-]+);/g)];
          expect(partAnims.length, `${name} should assign at least one --icon-part-N-anim`).toBeGreaterThan(0);
          for (const [, , kfName] of partAnims) {
            expect(css, `missing @keyframes ${kfName} referenced by ${name}`).toContain(`@keyframes ${kfName} {`);
          }
          continue;
        }
        // Whole-icon animations are named icon-<id> or, when a descriptive suffix reads better
        // (e.g. icon-folder-flap), icon-<id>-<suffix> — same convention as per-part icons.
        const ruleMatch = css.match(new RegExp(`\\[data-icon="${name}"\\] \\{([^}]*)\\}`));
        expect(ruleMatch, `missing [data-icon="${name}"] rule`).toBeTruthy();
        const animMatch = ruleMatch![1].match(/--icon-animation:\s*(icon-[\w-]+);/);
        expect(animMatch, `${name} has no --icon-animation assignment`).toBeTruthy();
        const animName = animMatch![1];
        expect(animName === `icon-${name}` || animName.startsWith(`icon-${name}-`), `${name} -> ${animName} does not match icon-${name}(-suffix)?`).toBe(true);
        expect(css, `missing @keyframes ${animName}`).toContain(`@keyframes ${animName} {`);
      }
      for (const kind of NON_CATALOG_KINDS) {
        expect(css).toContain(`@keyframes icon-kind-${kind} {`);
        expect(css).toMatch(new RegExp(`\\[data-icon-kind="${kind}"\\][^\\n]*--icon-animation:\\s*icon-kind-${kind};`));
      }
    });

    it("keeps every icon keyframes block closed (identical 0% and 100% frames), except declared full-turn spins", async () => {
      const css = await readUiCss();
      const blockRe = /@keyframes\s+(icon-[\w-]+)\s*\{([\s\S]*?)\n\}/g;
      let match: RegExpExecArray | null;
      let checked = 0;
      while ((match = blockRe.exec(css))) {
        const [, name, body] = match;
        checked++;
        if (FULL_TURN_KEYFRAMES.has(name)) {
          expect(body).toMatch(/0%\s*\{\s*transform:\s*(?:rotate|rotateX|rotateY)\(0deg\);/);
          expect(body).toMatch(/100%\s*\{\s*transform:\s*(?:rotate|rotateX|rotateY)\(-?360deg\);/);
          continue;
        }
        expect(body).toMatch(/0%,\s*\n\s*100%\s*\{/);
      }
      // Every icon has its own dedicated keyframes now, so this count roughly tracks icon count; kept as a
      // loose sanity floor (not exact) against an empty/broken scan rather than a precise assertion.
      expect(checked).toBeGreaterThan(100);
    });

    // Keyed by the required animation-name prefix ("icon-<id>" for data-icon, "icon-kind-<kind>" for
    // data-icon-kind), not the raw selector value, so specificity/exclusivity checks apply the right rule.
    function collectIconRules(css: string): Map<string, string> {
      const rules = new Map<string, string>();
      for (const m of css.matchAll(/\[data-icon="([a-z0-9-]+)"\] \{ ([^}]*)\}/g)) {
        rules.set(`icon-${m[1]}`, m[2]);
      }
      for (const m of css.matchAll(/\[data-icon-kind="([a-z0-9-]+)"\] \{ ([^}]*)\}/g)) {
        rules.set(`icon-kind-${m[1]}`, m[2]);
      }
      return rules;
    }

    function collectAnimNames(ruleBody: string): string[] {
      return [...ruleBody.matchAll(/--icon-(?:part-\d+-)?anim(?:ation)?:\s*([\w-]+);/g)].map((m) => m[1]).filter((n) => n !== "none");
    }

    it("names every icon's animation after that exact icon — no borrowed/generic names", async () => {
      const css = await readUiCss();
      const rules = collectIconRules(css);
      const violations: string[] = [];
      for (const [prefix, body] of rules) {
        for (const name of collectAnimNames(body)) {
          if (name !== prefix && !name.startsWith(`${prefix}-`)) {
            violations.push(`${prefix} -> ${name}`);
          }
        }
      }
      expect(violations, `every animation name must start with icon-<id>: ${violations.join(", ")}`).toEqual([]);
    });

    it("never reuses one icon's animation for a different icon (fully handcrafted, not shared)", async () => {
      const css = await readUiCss();
      const rules = collectIconRules(css);
      const owners = new Map<string, Set<string>>();
      for (const [id, body] of rules) {
        for (const name of collectAnimNames(body)) {
          if (!owners.has(name)) owners.set(name, new Set());
          owners.get(name)!.add(id);
        }
      }
      const shared = [...owners.entries()].filter(([, ids]) => ids.size > 1);
      expect(shared, `these animations are referenced by more than one icon: ${shared.map(([n, ids]) => `${n} -> [${[...ids].join(", ")}]`).join("; ")}`).toEqual([]);
    });

    it("has no orphaned keyframes — every defined icon-* animation is used by exactly one icon rule", async () => {
      const css = await readUiCss();
      const rules = collectIconRules(css);
      const referenced = new Set<string>();
      for (const [, body] of rules) {
        for (const name of collectAnimNames(body)) referenced.add(name);
      }
      const defined = [...css.matchAll(/^@keyframes (icon-[a-z0-9-]+)/gm)].map((m) => m[1]).filter((n) => !n.startsWith("icon-kind-"));
      const dead = defined.filter((n) => !referenced.has(n));
      expect(dead, `dead keyframes with no icon referencing them: ${dead.join(", ")}`).toEqual([]);
    });
  });

  describe("Icon hover animation attributes", () => {
    it("stamps data-icon and data-icon-kind for catalog icons", () => {
      const markup = renderToStaticMarkup(<Icon icon="bell" />);
      expect(markup).toMatch(/data-icon="bell"/);
      expect(markup).toMatch(/data-icon-kind="catalog"/);
      expect(markup).toMatch(/--icon-mask:url\(&quot;data:image\/svg\+xml,/);
    });

    it("stamps data-icon-kind for every non-catalog icon kind", () => {
      expect(renderToStaticMarkup(<Icon icon={{ kind: "emoji", emoji: "🙂️" }} />)).toMatch(/data-icon-kind="emoji"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "emoji", emoji: "🙂️" }} />)).not.toMatch(/--icon-mask:/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "text", text: "Hi" }} />)).toMatch(/data-icon-kind="text"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "text", text: "Hi" }} />)).not.toMatch(/--icon-mask:/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "typst", src: "$x^2$" }} />)).toMatch(/data-icon-kind="typst"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "url", url: "https://example.com/a.png" }} />)).toMatch(/data-icon-kind="image"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "svg", svg: "<svg></svg>" }} />)).toMatch(/data-icon-kind="svg"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "node", node: <span>x</span> }} />)).toMatch(/data-icon-kind="node"/);
      expect(renderToStaticMarkup(<Icon icon={"definitely-not-a-vendored-icon" as IconName} />)).toMatch(/data-icon-kind="text"/);
    });
  });
}
