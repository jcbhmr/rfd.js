import { AsyncFileDialog } from "../index.js";

const handle = await new AsyncFileDialog()
  .addFilter("JavaScript", ["js", "mjs", "cjs", "jsx"])
  .addFilter("TypeScript", ["ts", "mts", "cts", "tsx"])
  .pickFile();
console.debug("handle=%s", handle);

const fileName = handle.fileName();
console.debug("fileName=%s", fileName);

const path = handle.path();
console.debug("path=%s", path);

const read = await handle.read();
console.debug("read=%s", read);

const stringified = `${handle}`;
console.debug("stringified=%s", stringified);
