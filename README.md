# Flashlight

Find usages of imported symbols in your codebase.

This package is especially helpful for finding imported symbols when the
symbol is a non-unique name or multiple import sources may export the
same name. In these situations, simple find/replace is not often enough and
syntax aware searching is required.

## Installation

You can install Flashlight by running the install script which will download
the [latest release](https://github.com/mskelton/flashlight/releases/latest).

```bash
curl -LSfs https://mskelton.dev/flashlight/install | sh
```

Or you can build from source.

```bash
git clone git@github.com:mskelton/flashlight.git
cd flashlight
cargo install --path .
```

## Commands

### `imports`

Searches for all imports for a given import source.

```bash
flashlight imports react
```

You can also search for a specific import specifier.

```bash
flashlight imports react useState
```

### `jsx-tags`

Searches for all JSX components with the given name.

```bash
flashlight jsx-tags Button
```

You can also search for components with a specific prop.

```bash
flashlight jsx-tags Button variant
```

### `unused-modules`

Searches for unused modules in your project.

> [!WARNING]
> This search is not perfect! It relies on best guess semantic analysis to
> determine if modules are imported, but dynamic imports or other such
> mechanisms can result in code being marked as unused that is actually in use.

```bash
flashlight unused-modules
```

## Flags

### Change working directory

By default, flashlight uses the current working directory to search. You can
change the working directory using the `--cwd` argument.

```bash
flashlight find-imports react --cwd ./packages/a
```

### Format

You can customize the output format based on your use case. The supported
formats are:

- `default` - The default console format
- `json` - Formats the output as JSON
- `quickfix` - Formats the output as a Vim quickfix list (alias `vi`)

```bash
flashlight find-imports --source react --format json
```
