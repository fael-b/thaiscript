# ThaiScript 🇹🇭

The best way to learn [Thai script](https://en.wikipedia.org/wiki/Thai_script).

Available for Windows, macOS, and Linux (Android and iOS versions when Tauri 2.0 is released).

Powered by [TypeScript](https://www.typescriptlang.org/), [React](https://react.dev/), and [Tauri](https://tauri.app/).

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/en/)
- [pnpm](https://pnpm.io/) (install it via `npm install -g pnpm`)
- [Rust](https://www.rust-lang.org/) (install it via [rustup](https://rustup.rs/))
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- [git](https://git-scm.com/) (optional)

### Installation

1. Clone the repository or download the source code. Don't run this command if you choose to download the source code.

```sh
git clone git@github.com:fael-b/thaiscript.git
```

2. Install the dependencies.

```sh
cd thaiscript
pnpm install
```

3. Run the app in development mode.

```sh
pnpm tauri dev
```

4. If everything went well, you can build the app to install it durably on your system.

```sh
pnpm tauri build
```

## Additional Compatibility Details

- Recommended Node.js version: **18 or higher**.
- Required Rust version: **1.76.0 or higher**.
- Having the Thai language pack installed on your system is **recommended** for the best experience, but _not required_.

## Problems / Suggestions

If you have any problems or suggestions, please open an [issue](https://github.com/fael-b/thaiscript/issues).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Hiragana Pro](https://play.google.com/store/apps/details?id=com.myapps.hiragana) and [Mochi](https://mochi.cards/) apps for inspiration.
- [thai-alphabet.com](https://thai-alphabet.com/) for the easy to read Thai script reference.
