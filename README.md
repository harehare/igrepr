# igrepr - Interactive Grep Result

igrepr is a line-oriented search tool, allowing interactively select, filter and replace the results of grep.

![image](images/igr.gif)

## Installation

```bash
cargo install --git https://github.com/harehare/igrepr.git
```

## Features

- Open the selected line in any text editor (default is vim).
- One or more search, filter, or transform commands separated by a pipe (|) can be applied to the search results.

## Usage

igr [OPTIONS] [QUERY]

```bash
$ igr 'line.invert_match(let) | cli | tui | ignore_case(color) | upper_case()
```

## Options

```
Usage: igr [OPTIONS] [QUERY]

Arguments:
  [QUERY]

Options:
  -A, --after-context <AFTER_CONTEXT>
          Show lines before each match
  -B, --before-context <BEFORE_CONTEXT>
          Show lines before each match
  -C, --context <CONTEXT>
          Show lines before and after each match
      --custom-command <CUSTOM_COMMAND>
          Custom command used to open selected line. e.g.: --custom_command "code -g {file_path}:{line_no} [env: IGR_CUSTOM_COMMAND=]
      --context-separator <CONTEXT_SEPARATOR>
          The string used to separate [default: --]
  -c, --count
          Only print the count of individual match lines for each file
      --count-matches
          Only print the count of individual matches for each file
  -d, --disable-tui
          Disable tui
      --editor <EDITOR>
          Text editor used to open selected line [default: vim] [possible values: github, emacs, intellij, less, neovim, nano, vim, vscode]
  -., --hidden
          Search hidden files and directory
      --hide-help
          Hide Help
      --exclude-path <EXCLUDE_PATH>
          If specified, it excludes files or directories matching the given filename pattern from the search [env: IGR_EXCLUDE_PATH=]
      --max-depth <MAX_DEPTH>
          The maximum depth to recurse
      --no-git-ignore
          Don't respect .gitignore files
  -N, --no-file-name
          Never print the file path with the matched lines
      --no-line-no
          Never print the line number with the matched lines
      --no-color
          Not colored the output results
      --no-icon
          Not display icons
  -r, --replace
          Perform replacements if disable_tui is true
      --threads <THREADS>
          Number of grep worker threads to use
      --theme <THEME>
          Specify a theme [default: dark] [possible values: dark, light]
  -q, --quiet
          Do not output matched lines. instead, exit with status 0 when there is a match and with non-zero status when there isn’t
      --vimgrep
          Specifies whether all matched results are returned, including row and column numbers
  -p, --path <PATH>
          Searches for specified files and directories
  -h, --help
          Print help
  -V, --version
          Print version
```

## Keybindings

| Key        | Action            |
| ---------- | ----------------- |
| `Tab`      | Select command    |
| `Ctrl + c` | Quit              |
| `Ctrl + n` | Copy result       |
| `Ctrl + y` | Copy command      |
| `Ctrl + v` | Show file preview |
| `Ctrl + e` | Replace all       |

## Filter and Functions

| Command                 | Exapmle                         | Description                                                                                                                 |
| ----------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| camel_case              | camel_case()                    | Convert to a string with the separators denoted by having the next letter capitalised.                                      |
| constant                | constant()                      | Convert to an upper case, underscore separated string.                                                                      |
| contains                | contains()                      | Determines if the specified string is contains.                                                                             |
| delete                  | delete(index, index)            | Delete a string in the specified range.                                                                                     |
| ends_with               | ends_with(string)               | Determines if the string ends with a character from this string.                                                            |
| insert                  | insert(index, string)           | Inserts a string at the specified position.                                                                                 |
| invert_match            | invert_match(string)            | Select non-matching.                                                                                                        |
| invert_match_regex      | invert_match_regex(string)      | Select lines that do not match the regular expression.                                                                      |
| ignore_case             | ignore_case(string)             | Search without case sensitivity.                                                                                            |
| kebab_case              | kebab_case()                    | Convert to a lower case, dash separated string.                                                                             |
| line.contains           | line.contains(string)           | Determine if the line contains the specified string.                                                                        |
| line.match_regex        | line.match_regex(string)        | Searches for lines matching a regular expression                                                                            |
| line.starts_with        | line.starts_with(string)        | Searches for lines starting with a specified character.                                                                     |
| line.ends_with          | line.ends_with(string)          | Searches for lines ending with the specified character.                                                                     |
| line.invert_match       | line.invert_match(string)       | Select non-matching lines.                                                                                                  |
| line.invert_match_regex | line.invert_match_regex(string) | Select lines that do not match the regular expression.                                                                      |
| line.bytelength         | line.bytelength() > 10          | Filter by the number of bytes in a line.                                                                                    |
| line.length             | line.length() > 10              | Filter by the specified number of characters in a line.                                                                     |
| lower_case              | lower_case()                    | Convert to a string in lower case.                                                                                          |
| number                  | number() > 10                   | Search for numbers.                                                                                                         |
| regex                   | regex(regex_string)             | Search by regular expression.                                                                                               |
| replace                 | replace(string, replacement)    | Returns the new string replaced by the substitution.                                                                        |
| starts_with             | starts_with(string)             | Determines if the string starts with a character from this string.                                                          |
| snake_case              | snake_case()                    | Convert to a lower case, underscore separated.                                                                              |
| trim_end                | trim_end()                      | Removes whitespace from the end of this string.                                                                             |
| trim_start              | trim_start()                    | Removes whitespace from the start of this string.                                                                           |
| trim                    | trim()                          | Removes whitespace from both ends of this string.                                                                           |
| upper_case              | upper_case()                    | Convert to a string in upper case.                                                                                          |
| upper_camel_case        | upper_camel_case()              | Convert to a string with the separators denoted by having the next letter capitalised with the first character upper cased. |
| upper_kebab_case        | upper_kebab_case()              | Convert to a lower case, dash separated string with the first character upper cased.                                        |
| upper_snake_case        | upper_snake_case()              | Convert to a lower case, underscore separated with the first character upper cased.                                         |
| whole_word              | whole_word(string)              | Search by word.                                                                                                             |

## License

[MIT](http://opensource.org/licenses/MIT)
