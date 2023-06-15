// Using 'npm:test' for Node.js v16 compat
import test from "test";
import assert from "node:assert";
import * as index from "../index.js";

test("exports are all there", async () => {
  console.debug("index=%s", index);

  // https://docs.rs/rfd/latest/rfd/all.html
  const structs = [
    "AsyncFileDialog",
    "AsyncMessageDialog",
    "FileDialog",
    "FileHandle",
    "MessageDialog",
  ];
  const enums = ["MessageButtons", "MessageLevel"];
  for (const name of structs.concat(enums)) {
    await test(`exports ${name}`, () => {
      assert.notEqual(index[name], undefined);
    });
  }
});
