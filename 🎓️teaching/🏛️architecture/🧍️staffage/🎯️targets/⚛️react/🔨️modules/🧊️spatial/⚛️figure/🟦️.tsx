/** ⚛️ `ModelFigure`: the React face of the 3D view. It mounts a {@link ModelStage} on a host element, keeps it in step with its props and disposes it with the component; all behaviour, texts and accessibility are the stage's, so the React figure and the custom element are one implementation.
 *
 * Fill the parent: the host takes the size its parent gives it (the stage's custom property `--staffage-model-min-height` is the floor).
 *
 * @see ../🖥️stage/🟦️.ts — the stage and its customisation properties
 */
import { useEffect, useRef, type ReactElement } from "react";
import type { Asset, DisplayRange, Environment, EvaluationQuery, ForceQuantity, Language, ThermalQuantity, View } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { ModelStage, type StageOptions, type StageState } from "../🖥️stage/🟦️.ts";
import type { CameraPresetName } from "../🟦️.ts";

/** ⚙️ The props of {@link ModelFigure}. */
export type ModelFigureProps = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly clock?: StaffageClock | null;
  readonly view: View;
  readonly scale: number;
  readonly language: Language;
  readonly preset: CameraPresetName;
  readonly onPreset?: (preset: CameraPresetName) => void;
  readonly onState?: (state: StageState) => void;
  readonly environment?: Environment;
  readonly thermalQuantity?: ThermalQuantity;
  readonly forceQuantity?: ForceQuantity;
  readonly range?: DisplayRange;
  readonly theme?: "light" | "dark";
  readonly background?: string | null;
  readonly toolbar?: boolean;
  readonly reducedMotion?: boolean;
  readonly backend?: StageOptions["backend"];
  readonly settleMs?: number;
  readonly className?: string;
};

const optionsOf = (props: ModelFigureProps, callbacks: Pick<StageOptions, "onPreset" | "onState">): StageOptions => {
  const { className: _className, onPreset: _onPreset, onState: _onState, ...rest } = props;
  return { ...rest, ...callbacks };
};

/** 🧊️ The interactive 3D model of an asset. */
export function ModelFigure(props: ModelFigureProps): ReactElement {
  const host = useRef<HTMLDivElement | null>(null);
  const stage = useRef<ModelStage | null>(null);
  const latest = useRef(props);
  latest.current = props;
  const callbacks: Pick<StageOptions, "onPreset" | "onState"> = { onPreset: (preset) => latest.current.onPreset?.(preset), onState: (state) => latest.current.onState?.(state) };
  useEffect(() => {
    if (host.current === null) return;
    const mounted = new ModelStage(host.current, optionsOf(latest.current, callbacks));
    stage.current = mounted;
    return () => {
      mounted.dispose();
      stage.current = null;
    };
  }, []);
  useEffect(() => stage.current?.update(optionsOf(props, callbacks)));
  return <div ref={host} className={props.className} data-model-figure="" data-asset={props.asset.id} style={{ inlineSize: "100%", blockSize: "100%", minBlockSize: 0 }} />;
}
