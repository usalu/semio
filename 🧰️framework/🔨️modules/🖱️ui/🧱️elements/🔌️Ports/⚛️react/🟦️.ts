import * as React from "react";

/** 🔌️ Shared React adapter contract, independent of scene and diagram hosts. */
export interface ReactHostPort {
  readonly createElement: typeof React.createElement;
  readonly useState: typeof React.useState;
  readonly useEffect: typeof React.useEffect;
  readonly useMemo: typeof React.useMemo;
  readonly useCallback: typeof React.useCallback;
  readonly useRef: typeof React.useRef;
  readonly useContext: typeof React.useContext;
  readonly useLayoutEffect: typeof React.useLayoutEffect;
  readonly useSyncExternalStore: typeof React.useSyncExternalStore;
  readonly useId: typeof React.useId;
  readonly useImperativeHandle: typeof React.useImperativeHandle;
  readonly memo: typeof React.memo;
  readonly forwardRef: typeof React.forwardRef;
  readonly lazy: typeof React.lazy;
  readonly Suspense: typeof React.Suspense;
  readonly createContext: typeof React.createContext;
}

/** ⚛️ Default host port — inject a test double via {@link setReactHostPort} before render. */
export let reactHostPort: ReactHostPort = {
  createElement: React.createElement,
  useState: React.useState,
  useEffect: React.useEffect,
  useMemo: React.useMemo,
  useCallback: React.useCallback,
  useRef: React.useRef,
  useContext: React.useContext,
  useLayoutEffect: React.useLayoutEffect,
  useSyncExternalStore: React.useSyncExternalStore,
  useId: React.useId,
  useImperativeHandle: React.useImperativeHandle,
  memo: React.memo,
  forwardRef: React.forwardRef,
  lazy: React.lazy,
  Suspense: React.Suspense,
  createContext: React.createContext,
};

/** 🪢️ ESM importers cannot assign an imported `export let` binding directly, so this is the only
 * way to swap {@link reactHostPort} from outside this module (the barrel's `configureHostPorts` calls
 * this instead of a direct assignment). Returns the previously-installed port. */
export function setReactHostPort(port: ReactHostPort): ReactHostPort {
  const previous = reactHostPort;
  reactHostPort = port;
  return previous;
}

