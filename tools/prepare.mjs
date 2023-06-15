#!/usr/bin/env node
import { $ } from "execa";
import fg from "fast-glob";
import fsPromises from "node:fs/promises";

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
  console.log("Build artifacts don't exist. Building locally...")
  if (process.env.NODE_ENV === "production") {
    await $`napi build --platform --release`;
  } else {
    await $`napi build --platform`;
  }
} else {
  console.log("Build artifact already exists! Skipping...")
}
