import { FileDialog } from "../index.js";

const path = new FileDialog()
  .addFilter("JavaScript", ["js", "mjs", "cjs", "jsx"])
  .addFilter("TypeScript", ["ts", "mts", "cts", "tsx"])
  .pickFile();
console.debug("path=%s", path);
