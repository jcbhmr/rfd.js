import test from "node:test";
import assert from "node:assert";
import * as index from "../index.js";

// https://docs.rs/rfd/latest/rfd/all.html
const things = [
  "AsyncFileDialog",
  "AsyncMessageDialog",
  "FileDialog",
  "FileHandle",
  "MessageDialog",
  "MessageButtons",
  "MessageLevel",
];
for (const name of things) {
  test(`exports ${name}`, () => {
    assert(name in index);
  });
}
