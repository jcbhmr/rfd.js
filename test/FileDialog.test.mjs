import test from "node:test";
import assert from "node:assert";
import { FileDialog } from "../index.js";

if (!process.env.CI) {
  test("FileDialog demo", () => {
    const path = new FileDialog()
      .addFilter("JavaScript", ["js", "mjs", "cjs", "jsx"])
      .addFilter("TypeScript", ["ts", "mts", "cts", "tsx"])
      .pickFile();

    if (path == null) {
      console.warn("No file selected");
    } else {
      console.debug(path);
      assert(typeof path === "string");
    }
  });
}
