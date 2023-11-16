# JSON Configuration

This guide offers instructions on how to configure the JSON file for specifying text replacements during the file normalization process. The configuration file enables the definition of a series of modifications to be implemented in the input file's content.

## Table of Contents

- [Configuration Structure](#configuration-structure)
  - [Changes](#changes)
- [Replacing Unicode Characters](#replacing-unicode-characters)

## Configuration Structure

The JSON configuration file comprises a solitary object containing a "changes" field that specifies the replacements to be executed.

### Changes

The "changes" field is a dictionary, where each key represents the original string, while the corresponding value represents the replacement string.

Here's an example which replaces every asterisk with a dash, and replaces every single quote with a double quote

```json
{
  "changes": {
    "*": "-",
    "'": "\"",
  }
}
```

## Replacing Unicode Characters

To replace Unicode characters in your text, define the escape sequence of each Unicode character as a key in the "changes" field, and assign the desired replacement string as the corresponding value.

If you want to replace `'\u{201C}'` with a double quote `"`, and `'\u{201D}'` with another double quote `"`, your configuration would look like this:

```json
{
  "changes": {
    "\\u{201C}": "\"",
    "\\u{201D}": "\""
  }
}
```

This documentation is licensed under the GNU Free Documentation License (GFDL), and all examples are licensed under the GNU General Public License, version 3 (GPLv3).