# JSON Configuration

This guide provides instructions on configuring the JSON file for specifying text replacements during the file normalization process. The configuration file allows you to define a series of modifications to be applied to the content of the input file.

## Table of Contents

  - [Configuration Structure](#configuration-structure)
    - [Changes](#changes)
  - [Replacing Unicode Characters](#replacing-unicode-characters)
  - [Regex Replacements](#regex-replacements)

## Configuration Structure

The JSON configuration file is a single object containing a "changes" field that specifies the replacements to be made.

### Changes

The "changes" field is a dictionary where each key represents the original string, and the corresponding value specifies the replacement. Replacements can be either direct string substitutions or regex-based.

**Example of direct replacements:**

```json
{
    "changes": {
      "*": "-",
      "'": "\""
    }
}
```

### Replacing Unicode Characters

To replace Unicode characters, define the escape sequence for each Unicode character as a key in the "changes" field and assign the desired replacement string as the corresponding value.

**Example:**

To replace `'\u{201C}'` with a double quote `"` and `'\u{201D}'` with another double quote `"`, your configuration would look like this:

```json
{
    "changes": {
        "\\u{201C}": "\"",
        "\\u{201D}": "\""
    }
}
```

### Regex Replacements

You can also specify replacements using regular expressions. For regex-based replacements, use the following structure where the key is the original string and the value is an object containing the `pattern` and `replacement`.

**Example:**

To replace sequences of one or more dots with a single dot, and to replace `'` with `’`:

```json
{
    "changes": {
        "'": "’",
        "\"": "”",
        "!": ".",
        "example": {
            "pattern": "[.]+",
            "replacement": "."
        }
    }
}
```

In this example:
- `'` is replaced with `’`
- `"` is replaced with `”`
- `!` is replaced with `.`
- `example` is a key with regex pattern `[.]+` that matches one or more dots and replaces them with a single dot.

---

This documentation is licensed under the [GNU Free Documentation License (GFDL)](LICENSE.md), and all examples are licensed under the [GNU General Public License, version 3 (GPLv3)](../LICENSE.md).
