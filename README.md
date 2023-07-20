# JavaScript native file dialogs

📂 [Rusty File Dialogs] bindings for JavaScript

<div align="center">

![](https://github.com/jcbhmr/rfd.node/assets/61068799/dd5a6ead-6d0f-4009-a53a-63f2b899804e)

</div>

👨‍💻 Uses your OS' native API \
🐖 Piggy-backs on the [Rusty File Dialogs] project \
📂 Lets you open file picker dialogs \
🔔 Utility alert boxes too!

## Installation

```sh
npm install @bindrs/rfd
```

This package doesn't work in the browser. It's meant to be used with [Node.js].

## Usage

```js
import { homedir } from "node:os";
import { AsyncFileDialog } from "@bindrs/rfd";

const fileHandle = await new AsyncFileDialog()
  .addFilter("Images", ["png", "jpg", "jpeg"])
  .addFilter("All Files", ["*"])
  .setTitle("Pick an image")
  .setDirectory(homedir())
  .pickFile();
console.info(`You chose to open ${fileHandle}!`);
//=> You chose to open /home/jcbhmr/cool.png!
```

## Development

```sh
npm test
```

```sh
npm run build:debug
```

[Rusty File Dialogs]: https://github.com/PolyMeilex/rfd#readme
[Node.js]: https://nodejs.org/
