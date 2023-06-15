#!/usr/bin/env node
import { $ } from "execa";
import fg from "fast-glob";
const exists = (x) =>
  fsPromises
    .stat(x)
    .then(() => true)
    .catch(() => false);

if (
  !(await fg("index.*.node")).length ||
  !(await exists("index.js")) ||
  !(await exists("index.d.ts"))
) {
  if (process.env.NODE_ENV === "production") {
    await $`napi build --platform --release`;
  } else {
    await $`napi build --platform`;
  }
}
