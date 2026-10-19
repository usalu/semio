/** 🧱️ The mesh border, reference transform, appearance and pose, volume transform and containment and entity flag suites of the infinite world r3f module. */

export async function registerMeshReferenceAndVolumeTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../🟦️.tsx").WorldR3fTestDependencies): Promise<void> {
  const { LineBasicMaterial, Mesh, Object3D, Vector3, WORLD_MESH_OUTLINE_USER_DATA_KEY, WORLD_REFERENCE_SELECTED_CONTENT_OPACITY, applyWorldMeshEdgeBorders, applyWorldReferenceTransform, applyWorldVolumeTransform, mountWorldReferencePose, patchWorldReferenceProps, resetWorldMeshBorderColorCache, sceneHostPort, tokenHex, worldEntityInspectable, worldEntityRenderMode, worldEntityRendered, worldEntitySelectable, worldMeshBorderColor, worldReferenceAppearance, worldVolumesContainAabb } = dependencies;
  const { describe, expect, it } = vitest;

  type WorldReferenceProps = import("../../../🟦️.tsx").WorldReferenceProps;
  type WorldVolumeProps = import("../../../🟦️.tsx").WorldVolumeProps;

  describe("worldMeshBorderColor", () => {
    it("returns an srgb-compatible color", () => {
      resetWorldMeshBorderColorCache();
      const color = worldMeshBorderColor();
      expect(color.length).toBeGreaterThan(0);
      expect(color).not.toMatch(/^oklab\(/iu);
    });
  });

  describe("applyWorldMeshEdgeBorders", () => {
    it("adds one outline child per mesh", () => {
      const { BoxGeometry } = sceneHostPort.three;
      const root = new Object3D();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1), new LineBasicMaterial());
      root.add(mesh);
      applyWorldMeshEdgeBorders(root, "#336699");
      expect(mesh.children).toHaveLength(1);
      expect(mesh.children[0]?.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]).toBe(true);
      applyWorldMeshEdgeBorders(root, "#336699");
      expect(mesh.children).toHaveLength(1);
    });
  });

  describe("applyWorldReferenceTransform", () => {
    it("applies the authored pose when async media mounts the group after the empty first render", () => {
      const reference: WorldReferenceProps = {
        id: "forest-reference",
        source: { url: "/forest.png", mediaKind: "image" },
        origin: [7, 0, 0.01],
        orientation: [0, 0, 0, 1],
        scale: [2, 3, 4],
      };
      expect(mountWorldReferencePose(null, reference)).toBeNull();
      const lateGroup = new Object3D();
      expect(mountWorldReferencePose(lateGroup as import("three").Group, reference)).toBe(lateGroup);
      lateGroup.updateMatrixWorld(true);
      expect(lateGroup.position.toArray()).toEqual([7, 0, 0.01]);
      expect(lateGroup.scale.toArray()).toEqual([2, 3, 4]);
      expect(new Vector3().setFromMatrixPosition(lateGroup.matrixWorld).toArray()).toEqual([7, 0, 0.01]);
    });

    it("writes gumball pose onto reference props", () => {
      const base: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/infinite-assets/✏️sketch/🖼️.png", mediaKind: "image" },
        origin: [0, 0, 0],
      };
      const next = applyWorldReferenceTransform(base, {
        position: [1, 2, 3],
        quaternion: [0, 0, 0, 1],
        scale: [2, 2, 2],
      });
      expect(next.origin).toEqual([1, 2, 3]);
      expect(next.scale).toEqual([2, 2, 2]);
    });
  });

  describe("worldReferenceAppearance", () => {
    it("returns hover and selection fills with halved content opacity when selected", () => {
      expect(worldReferenceAppearance({ asHover: false, showSelectedOutline: false }, 1)).toMatchObject({
        backgroundColor: null,
        contentOpacity: 1,
        outlineColor: null,
      });
      expect(worldReferenceAppearance({ asHover: true, showSelectedOutline: false }, 0.8)).toMatchObject({
        backgroundColor: expect.stringMatching(/^#/),
        contentOpacity: 0.8,
        outlineColor: expect.stringMatching(/^#/),
      });
      expect(worldReferenceAppearance({ asHover: true, showSelectedOutline: true }, 1)).toMatchObject({
        backgroundColor: tokenHex("primary"),
        contentOpacity: WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
        outlineColor: tokenHex("primary"),
      });
    });
  });

  describe("applyWorldVolumeTransform", () => {
    it("writes gumball pose onto volume props", () => {
      const base: WorldVolumeProps = { id: "vol-a", origin: [0, 0, 0] };
      const next = applyWorldVolumeTransform(base, {
        position: [1, 2, 3],
        quaternion: [0, 0, 0, 1],
        scale: [4, 6, 8],
      });
      expect(next.origin).toEqual([1, 2, 3]);
      expect(next.scale).toEqual([4, 6, 8]);
    });
  });

  describe("worldVolumesContainAabb", () => {
    it("returns true when no volumes are defined", () => {
      expect(worldVolumesContainAabb([], [0, 0, 0], [1, 1, 1])).toBe(true);
    });

    it("accepts an AABB fully inside a unit volume at the origin", () => {
      const volumes: WorldVolumeProps[] = [{ id: "v1", origin: [0, 0, 0], scale: [10, 10, 10] }];
      expect(worldVolumesContainAabb(volumes, [-2, -2, -2], [2, 2, 2])).toBe(true);
    });

    it("rejects an AABB that extends outside the volume", () => {
      const volumes: WorldVolumeProps[] = [{ id: "v1", origin: [0, 0, 0], scale: [4, 4, 4] }];
      expect(worldVolumesContainAabb(volumes, [-3, -3, -3], [3, 3, 3])).toBe(false);
    });
  });

  describe("worldReferencePose", () => {
    it("patchWorldReferenceProps updates origin rotation and scale", () => {
      const base: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/plan.png", mediaKind: "image" },
        origin: [0, 0, 0],
      };
      expect(patchWorldReferenceProps(base, "origin", [1, 2, 3])?.origin).toEqual([1, 2, 3]);
      const rotated = patchWorldReferenceProps(base, "rotation", [0, 90, 0]);
      expect(rotated?.orientation?.length).toBe(4);
      expect(patchWorldReferenceProps(base, "scaleUniform", 2)?.scale).toBe(2);
      expect(patchWorldReferenceProps(base, "widthWorld", 42)?.widthWorld).toBe(42);
    });
  });

  describe("worldEntityFlags", () => {
    it("treats hidden and locked entities as non-selectable", () => {
      expect(worldEntitySelectable(undefined)).toBe(true);
      expect(worldEntitySelectable({ hidden: true })).toBe(false);
      expect(worldEntitySelectable({ locked: true })).toBe(false);
    });

    it("excludes locked entities from canvas pick so a click equals background", () => {
      expect(worldEntitySelectable({ locked: true })).toBe(false);
      expect(worldEntityInspectable({ locked: true })).toBe(true);
    });

    it("treats hidden entities as non-inspectable and locked entities as inspectable", () => {
      expect(worldEntityInspectable(undefined)).toBe(true);
      expect(worldEntityInspectable({ hidden: true })).toBe(false);
      expect(worldEntityInspectable({ locked: true })).toBe(true);
    });

    it("reveals hidden entities on demand", () => {
      expect(worldEntityRendered({ hidden: true }, false)).toBe(false);
      expect(worldEntityRendered({ hidden: true }, true)).toBe(true);
      expect(worldEntityRendered({ locked: true }, false)).toBe(true);
    });

    it("resolves render mode for hidden reveal and locked dim", () => {
      expect(worldEntityRenderMode({ hidden: true }, { revealed: true, hovered: false })).toMatchObject({
        visible: true,
        asHover: true,
        dim: false,
        showSelectedOutline: false,
      });
      expect(worldEntityRenderMode({ locked: true }, { hovered: true, selected: true })).toMatchObject({
        visible: true,
        asHover: false,
        dim: true,
        showSelectedOutline: false,
      });
      expect(worldEntityRenderMode({ hidden: true, locked: true }, { revealed: true, hovered: true, selected: true })).toMatchObject({
        visible: true,
        asHover: true,
        dim: false,
        showSelectedOutline: false,
      });
    });
  });
}
