import test from "node:test";
import assert from "node:assert";
import { AsyncFileDialog, FileHandle } from "../index.js";

if (!process.env.CI) {
  test("AsyncFileDialog demo", async () => {
    const handle = await new AsyncFileDialog()
      .addFilter("JavaScript", ["js", "mjs", "cjs", "jsx"])
      .addFilter("TypeScript", ["ts", "mts", "cts", "tsx"])
      .pickFile();

    if (handle == null) {
      console.warn("No file selected");
    } else {
      console.debug(handle);
      assert(handle instanceof FileHandle);
    }
  });
}
