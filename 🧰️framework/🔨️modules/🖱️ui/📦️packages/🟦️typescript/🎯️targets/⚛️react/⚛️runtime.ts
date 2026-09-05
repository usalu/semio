// #region 🧲️Header
/** @emoji ⚛️ Owned React runtime boundary for applications that consume the UI framework. */
// #endregion 🧲️Header

// #region 🔌️Implementation
import { Component, useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
// #endregion 🔌️Implementation

// #region 🧬️Contracts
/** @emoji 🧩 Structurally owned UI element contract. */
export interface UiElement {
  readonly type: unknown;
  readonly props: unknown;
  readonly key: string | null;
}

/** @emoji 🧶 Renderable UI value accepted by the owned runtime. */
export type UiNode = UiElement | string | number | bigint | boolean | null | undefined | readonly UiNode[];

/** @emoji 🧷 Mutable reference returned by the owned hook boundary. */
export interface UiRef<Value> {
  current: Value;
}

/** @emoji 🔁 State update accepted by the owned hook boundary. */
export type UiStateUpdate<Value> = Value | ((previous: Value) => Value);

/** @emoji 🎭 Consumer-facing component shape. */
export type UiComponent<Props> = (props: Props) => UiElement | null;

/** @emoji 🛟 Error-boundary lifecycle supplied without exposing the React class hierarchy. */
export interface UiErrorBoundaryDefinition<Props, State> {
  readonly initialState: State;
  readonly deriveState: (error: Error) => State;
  readonly didCatch?: (props: Props, error: Error) => void;
  readonly render: (props: Props, state: State) => UiNode;
}

/** @emoji 🌱 Mounted root controlled through an owned interface. */
export interface UiRoot {
  render(node: UiNode): void;
  unmount(): void;
}
// #endregion 🧬️Contracts

// #region 🪝️Hooks
/** @emoji 🎛️ Owns state-hook signatures at the framework boundary. */
export function useUiState<Value>(initial: Value | (() => Value)): readonly [Value, (update: UiStateUpdate<Value>) => void] {
  return useState(initial);
}

/** @emoji 🧠 Owns memo-hook signatures at the framework boundary. */
export function useUiMemo<Value>(factory: () => Value, dependencies: readonly unknown[]): Value {
  return useMemo(factory, dependencies);
}

/** @emoji 🔗 Owns callback-hook signatures at the framework boundary. */
export function useUiCallback<Arguments extends readonly unknown[], Result>(callback: (...args: Arguments) => Result, dependencies: readonly unknown[]): (...args: Arguments) => Result {
  return useCallback(callback, dependencies);
}

/** @emoji 🧭 Owns effect-hook signatures at the framework boundary. */
export function useUiEffect(effect: () => void | (() => void), dependencies?: readonly unknown[]): void {
  useEffect(effect, dependencies);
}

/** @emoji 📍 Owns reference-hook signatures at the framework boundary. */
export function useUiRef<Value>(initial: Value): UiRef<Value> {
  return useRef(initial);
}
// #endregion 🪝️Hooks

// #region 🛟️ErrorBoundary
/** @emoji 🧯 Creates a render-error boundary without leaking its implementation class. */
export function createUiErrorBoundary<Props extends object, State>(definition: UiErrorBoundaryDefinition<Props, State>): UiComponent<Props> {
  class OwnedErrorBoundary extends Component<Props, State> {
    readonly state = definition.initialState;

    static getDerivedStateFromError(error: Error): State {
      return definition.deriveState(error);
    }

    override componentDidCatch(error: Error): void {
      definition.didCatch?.(this.props, error);
    }

    override render(): ReactNode {
      return definition.render(this.props, this.state) as ReactNode;
    }
  }

  return OwnedErrorBoundary as unknown as UiComponent<Props>;
}
// #endregion 🛟️ErrorBoundary

// #region 🌱️Root
/** @emoji 🌳 Mounts one UI tree through the owned root contract. */
export function mountUiRoot(container: Element, node: UiNode): UiRoot {
  const root = createRoot(container);
  root.render(node as ReactNode);
  return {
    render(next) {
      root.render(next as ReactNode);
    },
    unmount() {
      root.unmount();
    },
  };
}
// #endregion 🌱️Root
