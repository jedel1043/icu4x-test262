# Notes

- Switched from `AnyProvider` to `BufferProvider`, because `icu_testdata@v1.0.0` for the any provider
has a fallback bug (already fixed).

- `icu_segmenter` is incompatible with `icu@v1.0.0`.

## ListFormat

https://github.com/tc39/test262/blob/main/test/intl402/ListFormat/constructor/constructor/locales-valid.js

- Need to add the "de" locale to the test data in order to test.

https://github.com/tc39/test262/blob/main/test/intl402/ListFormat/prototype/resolvedOptions/type.js

- Need to add the "en-US" locale to the test data in order to test.


## Collator

https://github.com/tc39/test262/blob/main/test/intl402/Collator/prototype/compare/non-normative-sensitivity.js

- Missing locale data for "de" and "it".

https://github.com/tc39/test262/blob/main/test/intl402/Collator/usage-de.js

- No support for the `usage` option, specifically the `search` value.
