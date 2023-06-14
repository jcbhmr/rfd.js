import { AsyncFileDialog } from "../index.js";

const handle = await new AsyncFileDialog()
  .addFilter("JavaScript", ["js", "mjs", "cjs", "jsx"])
  .addFilter("TypeScript", ["ts", "mts", "cts", "tsx"])
  .pickFile();
console.debug("handle=%s", handle);
