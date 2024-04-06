# Flashlight

CLI tool to help you analyze code usage in your project.

Flashlight allows you to perform searches that would be difficult with normal
string or regex searching such as:

- Find all imports from `react` that imported `useMemo`
- Find all `<Button>` tags with the attribute `type="primary"`

Regex searches can work for this at times, but it becomes very complex when
imports or tags span multiple lines. Flashlight provides a much easier way to
perform these searches.

## Installation

You can install Flashlight by running the install script which will download
the [latest release](https://github.com/mskelton/flashlight/releases/latest).

```bash
curl -LSfs https://go.mskelton.dev/flashlight/install | sh
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

### `tags`

Searches for all JSX elements with the given name.

```bash
flashlight tags Button
```

You can also search for elements with a specific attribute or attribute/value
pair.

```bash
flashlight tags Button variant
flashlight tags Button variant=primary
```

## Flags

### Change working directory

By default, flashlight uses the current working directory to search. You can
change the working directory using the `--cwd` argument.

```bash
flashlight --cwd ./packages/a imports react
```

### Format

You can customize the output format based on your use case. The supported
formats are:

- `default` - The default console format
- `json` - Formats the output as JSON
- `quickfix` - Formats the output as a Vim quickfix list (alias `vi`)

```bash
flashlight --format json imports react
```
