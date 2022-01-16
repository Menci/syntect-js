# Syntect for Node.js and WASM

[Syntect (Syntax highlighter in Rust)](https://github.com/trishume/syntect) for Node.js and WebAssembly.

Syntect highlights code with TextMate's `.tmTheme` theme. And the highlighting result is independent of the theme (using CSS classes, unlike [Shiki](https://github.com/shikijs/shiki), which uses inline styles) so you can switch themes in your website by only switching CSS.


```bash
# Install the meta-package
yarn add syntect       # Auto choose --
                       # Native binding for Node.js
                       # WebAssembly for browser

# Or Install manually
yarn add @syntect/node # Node.js native binding
yarn add @syntect/wasm # WebAssembly
```

# Usage

**Note:** See [WASM Notes](#wasm-notes) if you encountered any error using the WASM version.

```typescript
import { getCSS, highlight } from "syntect" /* or "@syntect/node" / "@syntect/wasm" */;

// Generate CSS code from .tmTheme file
const result = getCSS(
  // The text content of .tmTheme file
  tmThemeText,
  // The prefix of CSS class name,
  "hl-"
);
console.log(result.css);
// Only needed once.
// You could save the CSS file for your website.
// See also [CLI]

// Highlight code
const highlightResult = highlight(
  // The code to highlight
  "#include <cstdio>",
  // The language name or file extension. Use "plain" or "plaintext" for plain text
  "cpp",
  // The prefix of CSS class name,
  "hl-"
);
console.log(result.html);
```

# CLI

There's a cli utility for `getCSS` function. You should install the Node.js version (`syntect` or `@syntect/node`) to use it.

```bash
# Install with `yarn add syntect` or `yarn add @syntect/node`

# yarn syntect-css <prefix>
cat my-theme.tmTheme | yarn syntect-css hl-

# e.g. the tomorrow theme
curl https://raw.githubusercontent.com/chriskempson/textmate-tomorrow-theme/master/Tomorrow.tmTheme | yarn syntect-css hl-
```

# WASM Notes

The WASM version has all code in ES Modules.

To use the WASM version with Node.js, please add `--experimental-wasm-modules` to your Node.js command line options to enable Node.js to load `.wasm` modules. For older version of Node.js you'll also need `--experimental-modules` to enable basic ES Modules support.

To use the WASM version in browser, you may need to configure your bundler:

* *Webpack 5*: enable `experiments.asyncWebAssembly`.
* *Vite*: not currently supported but related works are in progress.
