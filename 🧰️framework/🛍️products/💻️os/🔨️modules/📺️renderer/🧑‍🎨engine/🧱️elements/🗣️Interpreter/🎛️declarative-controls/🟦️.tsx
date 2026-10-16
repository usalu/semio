/** 🎛️ The declarative control boundary: control types, their disabled and activity rules, action dispatch and the slider and stepper controls.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { useEffect, useRef, type ReactElement } from "react";
import {
  Button,
  IconSelector,
  Input,
  Ring,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Stepper,
  Textarea,
  Toggle,
  classifyIconSelectorMode,
  loadingBorderElementClass,
  waitingBorderElementClass
} from "@semio-tech/ui-react";
import { type ActionDescriptor, type ContinuousGestureLane, continuousPressIdentity, createContinuousGestureLane } from "@semio-tech/framework";
import { uiAccessibilityValueV1 } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🖼️presentation/♿️accessibility/🟦️.ts";
import { interpLabel } from "../🧷️interpreter-context/🟦️.ts";
import { resolveControlIconNode } from "../⚡️action-dispatch/🟦️.tsx";

//#region 🔖️UiInterpreter
//#region DeclarativeControlBoundary
type DeclarativeControlBase = {
  readonly id?: string;
  readonly disabled?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly presence?: { readonly state?: string; readonly status?: string };
};

type DeclarativeUiControl =
  | (DeclarativeControlBase & { readonly type: "input"; readonly id: string; readonly inputKind: string; readonly value: string; readonly placeholder?: string; readonly commit?: string; readonly min?: number; readonly max?: number; readonly step?: number; readonly accept?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "select"; readonly id: string; readonly value: string; readonly items: readonly { readonly value: string; readonly label: string }[]; readonly placeholder?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "toggle"; readonly id: string; readonly iconId: string; readonly pressed: boolean; readonly text?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "keyValue"; readonly entries: readonly { readonly label: string; readonly value: string }[] })
  | (DeclarativeControlBase & { readonly type: "slider"; readonly id: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly unit?: string; readonly snaps?: readonly number[]; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "numberStepper"; readonly id: string; readonly value: number; readonly step: number; readonly uniform: boolean; readonly min?: number; readonly max?: number; readonly onAbsolute: ActionDescriptor; readonly onDelta: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "ring"; readonly id: string; readonly orbId: string; readonly t: number; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "iconSelect"; readonly id: string; readonly value: string; readonly uniform: boolean; readonly classifierKind: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "button"; readonly iconId: string; readonly label: string; readonly action: ActionDescriptor });

function declarativeControlDisabled(control: DeclarativeControlBase): boolean {
  return control.disabled === true || control.presence?.state === "disabled";
}

function declarativeControlActivityClass(control: DeclarativeControlBase): string | undefined {
  if (control.loading || control.presence?.status === "loading") return loadingBorderElementClass;
  if (control.waiting || control.presence?.status === "waiting") return waitingBorderElementClass;
  return undefined;
}

function dispatchDeclarativeControlAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({ ...descriptor, args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch } });
}

/** 🎛️ Renders the owned structural control payload retained by panel-tree composition. */
export function renderUiControl(control: DeclarativeUiControl, onAction: (action: ActionDescriptor) => void, path?: string): ReactElement {
  switch (control.type) {
    case "input": {
      const commitOnBlur = control.commit === "blur";
      const commitValue = (raw: string) => dispatchDeclarativeControlAction(onAction, control.onChange, { value: control.inputKind === "number" ? Number(raw) : raw });
      if (control.inputKind === "longText") {
        return <Textarea id={control.id} data-ui-path={path} className="min-h-[4.5rem] w-full min-w-0" value={control.value} placeholder={control.placeholder} onChange={commitOnBlur ? undefined : (event) => commitValue(event.target.value)} onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined} />;
      }
      const inputType = control.inputKind === "number" ? "number" : control.inputKind === "date" ? "date" : control.inputKind === "color" ? "color" : control.inputKind === "file" ? "file" : "text";
      return <Input id={control.id} data-ui-path={path} type={inputType} className="h-medium w-full min-w-0" value={control.inputKind === "file" ? undefined : control.value} placeholder={control.placeholder} min={control.min} max={control.max} step={control.step} accept={control.inputKind === "file" ? control.accept : undefined} onChange={commitOnBlur ? undefined : (event) => commitValue(control.inputKind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value)} onBlur={commitOnBlur ? (event) => commitValue(control.inputKind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value) : undefined} />;
    }
    case "select":
      return (
        <Select id={control.id} value={control.value || undefined} onValueChange={(value) => dispatchDeclarativeControlAction(onAction, control.onChange, { value })}>
          <SelectTrigger id={control.id} data-ui-path={path} className="h-medium w-full min-w-0" size="sm"><SelectValue placeholder={control.placeholder ?? interpLabel("ui.common.select")} /></SelectTrigger>
          <SelectContent>{control.items.map((item, index) => <SelectItem key={`${control.id}:${index}:${item.value}`} value={item.value}>{item.label}</SelectItem>)}</SelectContent>
        </Select>
      );
    case "toggle":
      return <Toggle id={control.id} pressed={control.pressed} text={control.text} icon={resolveControlIconNode(control.iconId)} onPressedChange={(pressed) => dispatchDeclarativeControlAction(onAction, control.onChange, { pressed })} />;
    case "keyValue":
      return <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs" data-ui-path={path}>{control.entries.map((entry, index) => <div key={`${entry.label}:${index}`} className="contents"><dt className="text-muted-foreground">{entry.label}</dt><dd className="tabular-nums">{entry.value}</dd></div>)}</dl>;
    case "slider":
      return <DeclarativeSliderControl control={control} onAction={onAction} path={path} />;
    case "numberStepper":
      return <DeclarativeStepperControl control={control} onAction={onAction} />;
    case "ring":
      return <Ring id={control.id} onOrbChange={(_orbId, _oldT, newT) => dispatchDeclarativeControlAction(onAction, control.onChange, { t: newT })} orbs={[{ disabled: declarativeControlDisabled(control), id: control.orbId, selected: true, t: control.t }]} />;
    case "iconSelect":
      return <IconSelector classifyIconSelectorMode={control.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined} id={control.id} onChange={(next) => dispatchDeclarativeControlAction(onAction, control.onChange, { value: next })} uniform={control.uniform} value={control.value} />;
    case "button": {
      const activityClass = declarativeControlActivityClass(control);
      return <Button id={control.id} data-ui-path={path} text={control.label} icon={resolveControlIconNode(control.iconId)} disabled={declarativeControlDisabled(control)} onClick={() => onAction(control.action)} className={activityClass} aria-busy={Boolean(activityClass) || undefined} />;
    }
  }
}
/** 🎚️ A declarative continuous control's press lane: ticks `{value, gesture, commit}`, the release, and the host
 * cancel `{gesture, abort}` — the same scrub protocol {@link useContinuousTriggerLane} speaks for interpreted nodes. */
function useDeclarativeLane(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, key: string): ContinuousGestureLane<number> {
  const bindingRef = useRef({ onAction, descriptor, key });
  bindingRef.current = { onAction, descriptor, key };
  const gestureRef = useRef<string | null>(null);
  const laneRef = useRef<ContinuousGestureLane<number> | null>(null);
  if (laneRef.current === null) {
    laneRef.current = createContinuousGestureLane<number>({
      send: (value, phase) => {
        if (gestureRef.current === null) gestureRef.current = continuousPressIdentity(bindingRef.current.key);
        const gesture = gestureRef.current;
        if (phase === "commit") gestureRef.current = null;
        dispatchDeclarativeControlAction(bindingRef.current.onAction, bindingRef.current.descriptor, { value, gesture, commit: phase === "commit" });
      },
      abort: (reason) => {
        const gesture = gestureRef.current;
        gestureRef.current = null;
        if (gesture !== null) dispatchDeclarativeControlAction(bindingRef.current.onAction, bindingRef.current.descriptor, { gesture, abort: reason });
      },
    });
  }
  const lane = laneRef.current;
  useEffect(() => () => lane.abort("retired"), [lane]);
  return lane;
}

function DeclarativeSliderControl({ control, onAction, path }: { readonly control: Extract<DeclarativeUiControl, { type: "slider" }>; readonly onAction: (action: ActionDescriptor) => void; readonly path?: string }) {
  const lane = useDeclarativeLane(onAction, control.onChange, control.id);
  const slider = (
    <Slider
      id={control.id}
      data-ui-path={path}
      className="w-full min-w-0"
      max={control.max}
      min={control.min}
      step={control.step}
      snapValues={control.snaps}
      value={[control.value]}
      aria-valuetext={uiAccessibilityValueV1({ type: "slider", value: control.value, min: control.min, max: control.max, step: control.step, unit: control.unit ?? null, snaps: [...(control.snaps ?? [])] }).valueText ?? undefined}
      onValueChange={(values) => lane.offer(values[0] ?? control.value)}
      onValueCommit={(values) => lane.commit(values[0] ?? control.value)}
      onPointerCancel={() => lane.abort("captureLost")}
      onBlur={() => lane.abort("blur")}
    />
  );
  if (!control.unit) return slider;
  return <div className="flex min-w-0 w-full items-center gap-single">{slider}<span className="text-muted-foreground shrink-0 text-xs tabular-nums">{control.value} {control.unit}</span></div>;
}

function DeclarativeStepperControl({ control, onAction }: { readonly control: Extract<DeclarativeUiControl, { type: "numberStepper" }>; readonly onAction: (action: ActionDescriptor) => void }) {
  const lane = useDeclarativeLane(onAction, control.onAbsolute, control.id);
  return <Stepper id={control.id} step={control.step} min={control.min} max={control.max} value={control.uniform ? control.value : undefined} mixed={!control.uniform} onChange={(value) => lane.offer(value)} onPointerUp={() => lane.commit()} onPointerCancel={() => lane.commit()} onDelta={(delta) => dispatchDeclarativeControlAction(onAction, control.onDelta, { delta })} />;
}
//#endregion DeclarativeControlBoundary
//#endregion 🔖️UiInterpreter
