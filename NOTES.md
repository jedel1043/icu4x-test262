# Notes

- Switched from `AnyProvider` to `BufferProvider`, because `icu_testdata@v1.0.0` for the any provider
has a fallback bug (already fixed).

- `icu_segmenter` is incompatible with `icu@v1.0.0`.

- Need to add the "de" locale to the test data in order to pass the test
https://github.com/tc39/test262/blob/main/test/intl402/ListFormat/constructor/constructor/locales-valid.js

- Need to add the "en-US" locale to the test data in order to pass the test
https://github.com/tc39/test262/blob/main/test/intl402/ListFormat/prototype/resolvedOptions/type.js