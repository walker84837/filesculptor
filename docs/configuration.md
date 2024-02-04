# JSON Configuration

This part of the documentation provides instructions for configuring JSON to
specify text replacements during file normalisation. The configuration file
allows you to define a set of changes to be made to the content of the input
file.

## Table of Contents

  - [Configuration Structure](#configuration-structure)
      - [Changes](#changes)
  - [Replacing Unicode Characters](#replacing-unicode-characters)

## Configuration structure

The JSON configuration file consists of a single object. It contains a "changes"
field that specifies the replacements to be performed.

### Changes

The "changes" field is a dictionary, where each key represents the original
string, while the corresponding value represents the replacement string.

Here's an example that replaces every asterisk with a dash and every single
quotation mark with a double quotation mark:

``` json
{
  "changes": {
    "*": "-",
    "'": "\"",
  }
}
```

## Replacing Unicode characters

To replace Unicode characters in your text, define the *escaped* sequence of
each Unicode character as a key in the "Changes" field, and assign the desired
replacement string as the corresponding value.

For example, if you wanted to replace `\u{201C}` with a double quote (") and
`\u{201D}` with another double quote ("), your configuration would look like
this

``` json
{
  "changes": {
    "\u{201C}": "\"",
    "\u{201D}": "\""
  }
}
```

Please note that this application is currently only compatible with UTF-8
encoded files.

This documentation is licensed under the GNU Free Documentation License (GFDL),
and all examples are licensed under the GNU General Public License, version 3
(GPLv3).
