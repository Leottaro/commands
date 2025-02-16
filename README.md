# Rust Commands

This project contains several Rust command-line utilities for managing directories and files.

## Commands

### `rmd`

The `rmd` command deletes the current directory and all of its contents.

#### Usage

```sh
rmd
```

#### Description

-   Prompts the user for confirmation before deleting the current directory.
-   If the user confirms, changes the working directory to the parent directory and deletes the current directory.

### `mkd`

The `mkd` command creates one or more directories and changes the working directory to the first created directory.

#### Usage

```sh
mkd <dir1> [dir2] ...
```

#### Description

-   Creates the specified directories (can create nested directories like `foo/bar`).
-   Changes the working directory to the first created directory.
-   Displays a help message if no arguments are provided or if the `--help` option is used.

### `comp`

The `comp` command compresses specified directories into `.tar.gz` archives, excluding files and directories specified in `.gitignore`.

#### Usage

```sh
comp <path1> [path2] ... [OPTIONS]
```

#### Options

-   `--compression=[0-9]`: Adjust the compression level (default: 6).
-   `-v`, `--verbose`: Output detailed information during the compression process.

#### Description

-   Compresses the specified directories into `.tar.gz` archives.
-   Excludes files and directories specified in `.gitignore`.
-   Displays a help message if no arguments are provided or if the `--help` option is used.

## License

This project is licensed under the GNU GPL-3.0 License.
