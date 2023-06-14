# JavaScript native file dialogs

📂 Rust File Dialog bindings for JavaScript

<div align="center">

![](https://user-images.githubusercontent.com/61068799/245672430-faad9fd3-e295-4fb0-b41a-1cc2e0b83c74.png)

</div>

👨‍💻 Uses your OS' native API \
🐖 Piggy-backs on the [Rust File Dialogs] project \
📂 Lets you open file picker dialogs \
🔔 Utility alert boxes too!

## Installation

```sh
npm install @jcbhmr/rfd
```

This package doesn't work in the browser. It's meant to be used with [Node.js].

## Usage

```js
import os from "node:os";
import { AsyncFileDialog } from "@jcbhmr/rfd";

const path = `${await new AsyncFileDialog()
  .addFilter("Images", ["png", "jpg", "jpeg"])
  .addFilter("All Files", ["*"])
  .setTitle("Pick an image")
  .setDirectory(os.homedir())
  .pickFile()}`;
console.info("You chose to open %s!", path);
//=> You chose to open /home/jcbhmr/cool.png!
```

## Development

```sh
npm test
```

```sh
npx napi build --platform
```
