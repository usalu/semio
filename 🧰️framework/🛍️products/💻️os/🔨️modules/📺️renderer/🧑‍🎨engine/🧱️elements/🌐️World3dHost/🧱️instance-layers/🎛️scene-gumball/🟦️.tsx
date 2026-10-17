/** 🎛️ World3d instance layers, scene gumball: the transform gumball a selection carries in the three.js scene. */
import { useEffect, useRef, useState } from "react";
import { UnifiedGumball, useUiDriver, type GumballConfig, type GumballHandleKind, type GumballPose, Object3D } from "@semio-tech/ui-react";
import { useUiCanvasHovered } from "../../🧮️gumball-math/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
export function SceneGumball({
  target,
  config,
  active,
  onDraggingChanged,
  onDragStart,
  onDrag,
  onDragEnd,
}: {
  readonly target?: readonly [number, number, number];
  readonly config: GumballConfig;
  readonly active: boolean;
  readonly onDraggingChanged: (dragging: boolean) => void;
  readonly onDragStart?: (kind: GumballHandleKind, before: GumballPose) => void;
  readonly onDrag?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}) {
  const pivotRef = useRef<Object3D>(new Object3D());
  const draggingRef = useRef(false);
  const [ready, setReady] = useState(false);
  const driver = useUiDriver();
  const hovered = useUiCanvasHovered(driver.gumball === "hover");
  useEffect(() => {
    if (!target || draggingRef.current) return;
    pivotRef.current.position.set(target[0], target[1], target[2]);
    pivotRef.current.quaternion.set(0, 0, 0, 1);
    pivotRef.current.scale.set(1, 1, 1);
    pivotRef.current.updateMatrixWorld(true);
    setReady(true);
  }, [target]);
  if (!active || !target || !ready || !hovered) return null;
  return (
    <>
      <primitive object={pivotRef.current} />
      <UnifiedGumball
        target={pivotRef.current}
        config={config}
        onDraggingChanged={(dragging) => {
          draggingRef.current = dragging;
          onDraggingChanged(dragging);
        }}
        onDragStart={onDragStart}
        onDrag={onDrag}
        onDragEnd={(kind, before, after) => {
          onDragEnd(kind, before, after);
          pivotRef.current.position.set(after.position[0], after.position[1], after.position[2]);
          pivotRef.current.quaternion.set(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]);
          pivotRef.current.scale.set(after.scale[0], after.scale[1], after.scale[2]);
          pivotRef.current.updateMatrixWorld(true);
          draggingRef.current = false;
        }}
      />
    </>
  );
}
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
