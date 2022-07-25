# This is a guide for the localization format presented in this folder's files

## Filename format

Each filename must be 10 characters long, in a **"`language`-`COUNTRY`.json"** format, where `language` is the file's language according to `ISO 3166-1 alpha-2` and `COUNTRY` is the country according to `ISO 639-1` of a localized or the official version of `language` (Links to `ISO 3166-1 alpha-2` and `ISO 639-1` codes: [`ISO 3166-1 alpha-2`](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) [`ISO 639-1`](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements)).

## File contents

Each file contains a JSON object. The exact structure of the object can be seen in `template.json` as a JSON object or as a Rust struct in `src/locales.rs::structures::Localization`. All the contents of the JSON file are either child objects or strings. The displayed language is a field on the top of the object named `lang_title`

## String contents

All strings must contain the translated version of `en-US.json` in the language referred in the `lang_title` field. However, some strings might contain {} or {1} and {2}. These are formatted strings

### Formatted strings

Formatted strings are strings that aren't displayed in the console output as is. They are formatted before being displayed to the user.

There currently exist only two (2) types of formatted strings:

1) Strings that are formatted only once (for example, a string that displays the secret number). They have one `{}`. Let's call them `f1` strings
2) Strings that are formatted twice (for example, the range within we can find secret number). They have one `{1}` and one `{2}`. Let's call the `f2` strings

Both `f1` and `f2` strings have a comment on the same line describing what each format argument displays to the user (format arguments are the curly brackets if you didn't notice)

## Contributing to the translation

In order to contribute to the translation, make sure that you have understood how exactly the locales of this program work. To achieve this, read this README multiple times. Once you think you are ready, fork the repo, copy `template.json` and give it a name according to [Filename format](#filename-format). Once you are done filling the fields, make a pull request.
