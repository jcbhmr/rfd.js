import test from "node:test";
import assert from "node:assert";
import * as index from "../index.js";

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
  test(`exports ${name}`, () => {
    assert.notEqual(index[name], undefined);
  });
}
