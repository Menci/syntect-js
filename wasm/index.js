import * as Binding from "./dist/syntect.js";

function listGetters(instance) {
  return Object.entries(
    Object.getOwnPropertyDescriptors(
      Reflect.getPrototypeOf(instance)
    )
  )
  .filter(e => typeof e[1].get === "function" && e[0] !== "__proto__")
  .map(e => e[0]);
}

function createExport(name) {
  return function () {
    const result = Binding[name].apply(null, arguments);
    if (typeof result === "object" && result.free) {
      const copiedResult = listGetters(result).reduce((o, key) => Object.assign(o, { [key]: result[key] }), {});
      result.free();
      return copiedResult;
    }

    return result;
  }
}

export const getCSS = createExport("getCSS");
export const highlight = createExport("highlight");
export default { getCSS, highlight }
