/** 🗂️ The world layer stack context and its layer components.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { reactHostPort } from "@semio-tech/ui-react";
import { type ReactElement, type ReactNode } from "react";

// #region 🗂️LayerStack
const WorldLayerStackContext = reactHostPort.createContext(true);

export interface WorldLayerProps {
  readonly order: number;
  readonly name?: string;
  readonly children: ReactNode;
}

/** 🗂️ Ordered scene layer; renders in-place so React context (providers) stays valid. */
export function WorldLayer(props: WorldLayerProps): ReactElement {
  reactHostPort.useContext(WorldLayerStackContext);
  return (
    <group name={props.name} userData={{ worldLayerOrder: props.order }}>
      {props.children}
    </group>
  );
}

/** 🗂️ Root for {@link WorldLayer} children inside {@link WorldCanvas}. */
export function WorldLayerStack(props: { readonly children: ReactNode }): ReactElement {
  return <WorldLayerStackContext.Provider value={true}>{props.children}</WorldLayerStackContext.Provider>;
}
// #endregion 🗂️LayerStack
