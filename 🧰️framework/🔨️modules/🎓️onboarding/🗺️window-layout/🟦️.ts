/** 🗺️ The hand-refined window layout tree an app names as `defaultLayout`/`namedLayouts` and a tutorial snapshots as
 * `layout`: readonly nodes, required `windowKindId`, optional sizes. Exported through `@semio-tech/framework`. */

export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";

export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
  readonly corner?: WindowStackCorner;
};

export type WindowLayoutStackNode = {
  readonly kind: "stack";
  readonly size?: number;
  readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
  readonly kind: "row" | "column";
  readonly size?: number;
  readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
  readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};
