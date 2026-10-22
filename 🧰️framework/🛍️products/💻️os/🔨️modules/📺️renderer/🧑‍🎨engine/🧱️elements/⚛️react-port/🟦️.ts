/** ⚛️ The React port of the renderer elements: the only module under `🧱️elements` that names `react` and `react-dom/client`. Every element host reaches React through the explicit re-exports below, so the React version and the lane's whole React surface are one reviewed list; tests mount hosts with `react` directly.
 *
 * 🧭️ Part of the React lane implementation (`@semio-tech/framework-renderer-react`). The lane contract that leaves the lane carries no React type — see {@link ../🗣️Interpreter/🧩️host-contract/🟦️.ts}.
 */

export { Component, createContext, createElement, Fragment, memo, Profiler, Suspense, useCallback, useContext, useEffect, useId, useLayoutEffect, useMemo, useReducer, useRef, useState, useSyncExternalStore } from "react";
export type { ClipboardEvent, ComponentProps, ComponentPropsWithoutRef, ComponentType, CSSProperties, Dispatch, DragEvent, ErrorInfo, FC, FormEvent, HTMLAttributes, KeyboardEvent, MouseEvent, MutableRefObject, PointerEvent, ReactElement, ReactNode, RefObject, SetStateAction, WheelEvent } from "react";
export { createRoot } from "react-dom/client";
export type { Root } from "react-dom/client";
