#!/usr/bin/env node
import fsPromises from "node:fs/promises";

const exists = (x) =>
  fsPromises
    .stat(x)
    .then(() => true)
    .catch(() => false);

const files = [
  "index.darwin-x64.node",
  "index.win32-x64-msvc.node",
  "index.linux-x64-gnu.node",
];
let failed;
for (const file of files) {
  if (!(await exists(file))) {
    failed = true;
    console.error(
      `${file} does not exist. ` +
        "Make sure you have downloaded it from GitHub Actions artifacts!"
    );
  }
}

if (failed) {
  process.exit(1);
} else {
  console.log("All artifacts exist!");
}
