/** 📱️ Shell hooks: what a plugin may decide from the device, the keyboard and what it announces; the TypeScript twin of `🛰️.proto`. */
export type ShellHookCause = "start" | "resize" | "inputModality" | "selection";

export type ShellSheetState = "collapsed" | "half" | "full";

export interface ShellDeviceSignal {
  readonly widthPx: number;
  readonly touch: boolean;
}

export interface ShellPlatformPrefs {
  readonly forcedColors: boolean;
  readonly prefersContrastMore: boolean;
  readonly prefersDark: boolean;
  readonly prefersReducedMotion: boolean;
}

export interface ShellLayoutRequest {
  readonly appId: string;
  readonly signal: ShellDeviceSignal;
  readonly cause: ShellHookCause;
  readonly prefs?: string;
  readonly hasSelection: boolean;
  readonly lang: string;
}

export interface ShellBottomSheet {
  readonly label: string;
  readonly state: ShellSheetState;
  readonly height: number;
  readonly minHandlePx: number;
}

export interface ShellKeyEvent {
  readonly scope: string;
  readonly key: string;
  readonly ctrl: boolean;
  readonly alt: boolean;
  readonly shift: boolean;
  readonly meta: boolean;
}

export type ShellKeyOutcome = { readonly type: "ignored" } | { readonly type: "consumed" } | { readonly type: "action"; readonly action: string } | { readonly type: "select"; readonly domainId: string; readonly targets: readonly ShellSelectTarget[]; readonly merge: string } | { readonly type: "verb"; readonly action: string; readonly args: Readonly<Record<string, string>> };

export type ShellTouchGesture = "longPress" | "tap" | "doubleTap";

export interface ShellTouchProfile {
  readonly oneFingerOrbit: boolean;
  readonly longPress: boolean;
  readonly hitPx: number;
}

export interface ShellTouchEvent {
  readonly scope: string;
  readonly gesture: ShellTouchGesture;
  readonly x: number;
  readonly y: number;
  readonly widthPx: number;
}

export interface ShellSelectTarget {
  readonly granularity: string;
  readonly id: string;
}

export interface ShellAccessibleNode {
  readonly key: string;
  readonly role: string;
  readonly name: string;
  readonly description?: string;
  readonly roleDescription?: string;
  readonly valueText?: string;
  readonly position?: number;
  readonly setSize?: number;
  readonly selected?: boolean;
  readonly depth: number;
}
