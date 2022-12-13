# Notes

- `icu_segmenter` is incompatible with `icu@v1.0.0`.

## Collator

https://github.com/tc39/test262/blob/main/test/intl402/Collator/usage-de.js

- Missing support for the `usage` option, specifically the `search` value which requires locale data for
sensitivity level defaults.

https://github.com/tc39/test262/blob/main/test/intl402/Collator/numeric-and-caseFirst.js
https://github.com/tc39/test262/blob/main/test/intl402/Collator/missing-unicode-ext-value-defaults-to-true.js

- Need to discuss how to handle `DataRequest`s that fallback to the "und" locale if the requested
locale is not supported. Also, it would be good to check that the default locale is supported
by the data.

https://github.com/tc39/test262/blob/main/test/intl402/Collator/this-value-ignored.js

- Should be fixed after implementing the missing services.

## Array.prototype.toLocaleString

- Missing support for getting an "implementation-defined list-separator String value appropriate for
the host environment's current locale" (see https://tc39.es/ecma402/#sup-array.prototype.tolocalestring).
