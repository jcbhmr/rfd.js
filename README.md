# Rusty File Dialogs for JavaScript

📂 Lets you open native file picker and message boxes in JavaScript!

<div align="center">

![](https://github.com/jcbhmr/rfd.node/assets/61068799/dd5a6ead-6d0f-4009-a53a-63f2b899804e)

</div>

👨‍💻 Uses your OS' native API \
🐖 Piggy-backs on the [Rusty File Dialogs] project \
📂 Lets you open file picker dialogs \
🔔 Utility alert boxes too!

## Installation

![npm](https://img.shields.io/static/v1?style=for-the-badge&message=npm&color=CB3837&logo=npm&logoColor=FFFFFF&label=)
![Yarn](https://img.shields.io/static/v1?style=for-the-badge&message=Yarn&color=2C8EBB&logo=Yarn&logoColor=FFFFFF&label=)
![pnpm](https://img.shields.io/static/v1?style=for-the-badge&message=pnpm&color=222222&logo=pnpm&logoColor=F69220&label=)

You can install this package using npm, [pnpm], [Yarn], or your favorite
npm-compatible package manager!

```sh
npm install @bindrs/rfd
```

If you're using Deno, you can use the new `npm:` specifiers to import this
package straight from the npm registry. It's not possible to use an npm CDN
because this package uses a `.node` compiled binary.

```js
import {} from "npm:@bindrs/rfd";
```

🛑 This package doesn't work in the browser. It's meant to be used with
[Node.js], [Deno], or [Bun] which all support `.node` native addons.

## Usage

![Node.js](https://img.shields.io/static/v1?style=for-the-badge&message=Node.js&color=339933&logo=Node.js&logoColor=FFFFFF&label=)
![Deno](https://img.shields.io/static/v1?style=for-the-badge&message=Deno&color=000000&logo=Deno&logoColor=FFFFFF&label=)
![Bun](https://img.shields.io/static/v1?style=for-the-badge&message=Bun&color=000000&logo=Bun&logoColor=FFFFFF&label=)

```js
import { homedir } from "node:os";
import { AsyncFileDialog } from "@bindrs/rfd";

const fileHandle = await new AsyncFileDialog()
  .addFilter("Images", ["png", "jpg", "jpeg"])
  .addFilter("All Files", ["*"])
  .setTitle("Pick an image")
  .setDirectory(homedir())
  .pickFile();
const path = fileHandle.path();
console.log(`You chose to open ${path}!`);
//=> You chose to open /home/jcbhmr/cool.png!
```

You can also open message boxes:

```js
import { AsyncMessageDialog } from "@bindrs/rfd";

const answer = await new AsyncMessageDialog()
  .setTitle("Hello!")
  .setDescription("Click OK if you're feeling good 😊")
  .show();
if (answer) {
  console.log("You clicked OK!");
  //=> You clicked OK!
}
```

📚 Check out the [TypeDoc website] for more extensive API documentation. You can
also refer to the original [rfd crate documentation] if you want to learn more
about the features and limitations inherited from the Rust API.

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![Node.js](https://img.shields.io/static/v1?style=for-the-badge&message=Node.js&color=339933&logo=Node.js&logoColor=FFFFFF&label=)

This project uses [NAPI-RS] to bridge the gap between Rust and JavaScript. Other
options like [Neon] also work, but this seemed the simplest. To get started,
you'll need the [Rust toolchain] installed in addition to the usual Node.js
stuff. You'll also need the GTK-3 or other platform-specific libraries installed
for your desktop GUI.

To quickly get setup on Ubuntu, you can run the below commands. If you're on
Windows, you don't need any extra GUI libraries; those are already all included
with the Windows native toolkits!

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install libgtk-3-dev
```

We use a testing REPL instead of more concrete tests. This is because any
concrete test we write _won't work in CI_ because there's no GUI in GitHub
Actions! 😭 It's better to just tinker around with it in a REPL.

```sh
npm run test:repl
```

[Rusty File Dialogs]: https://github.com/PolyMeilex/rfd#readme
[Node.js]: https://nodejs.org/
[Deno]: https://deno.land/
[Bun]: https://bun.sh/
[pnpm]: https://pnpm.io/
[Yarn]: https://yarnpkg.com/
[TypeDoc website]: https://bindrs.github.io/rfd.js/
[rfd crate documentation]: https://docs.rs/rfd
[NAPI-RS]: https://napi.rs/
[Neon]: https://neon-bindings.com/
[Rust toolchain]: https://rustup.rs/
