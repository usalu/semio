//#region 📚️StorySchema

type StoryComponentProperties<TComponent> = TComponent extends (properties: infer TProperties, ...arguments_: infer _TArguments) => unknown ? TProperties : Record<string, unknown>;

type StoryMetaComponent<TMeta> = TMeta extends { readonly component: infer TComponent } ? TComponent : TMeta;

/** 🗂️ Owned component-fixture metadata shared by the internal scene browser and Storybook exporter. */
export type Meta<TComponent = unknown> = {
  readonly title?: string;
  readonly component?: TComponent;
  readonly parameters?: unknown;
  readonly args?: Partial<StoryComponentProperties<TComponent>>;
  readonly argTypes?: unknown;
  readonly tags?: readonly string[];
  readonly [field: string]: unknown;
};

/** 🎭️ Owned component-fixture definition with typed component properties and browser play context. */
export type StoryObj<TMeta = unknown> = {
  readonly name?: string;
  readonly args?: Partial<StoryComponentProperties<StoryMetaComponent<TMeta>>>;
  readonly render?: (properties: StoryComponentProperties<StoryMetaComponent<TMeta>>) => unknown;
  readonly play?: (context: { readonly canvasElement: HTMLElement }) => void | Promise<void>;
  readonly parameters?: unknown;
  readonly [field: string]: unknown;
};

//#endregion 📚️StorySchema
