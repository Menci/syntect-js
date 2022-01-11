import type * as Binding from "./dist/syntect";

type ExportFunction<T> =
  T extends (...args: any[]) => any
  ? (...[t, v]: Parameters<T>) => Omit<ReturnType<T>, "free">
  : never;

type FunctionMembers<T> = (
  {
    [K in keyof T]: T[K] extends (...args: any[]) => any ? K : never
  }
)[keyof T];

type RemoveNonFunctionMembers<T> = Pick<T, FunctionMembers<T>>;

type Export<T> = {
  [K in keyof T]: T[K] extends (...args: any[]) => any ? ExportFunction<T[K]> : never;
};

declare const Syntect: Export<RemoveNonFunctionMembers<typeof Binding>>;
export = Syntect;
