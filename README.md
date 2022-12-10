# ICU4X + Test262 + Boa

This repository contains a minimized collection of tests from the [Test262](https://github.com/tc39/test262)
suite that fail when using the [ICU4X](https://github.com/unicode-org/icu4x) crate to implement
ECMAScript's [`Intl`](https://tc39.es/ecma402/#intl-object) builtin on the [Boa](https://github.com/boa-dev/boa) interpreter.

## Structure

Each file on the [tests](./tests) directory indicates the list of tests that failed while testing that service.
Every test has a comment, indicating the original test from which the minimized test was extracted.

## To Run

```bash
> cargo test
```

## Stats

| Component             | ✔️   	| ➖    | ❌    |           |
| -                  	| -     | -     | -     | -     	|
| Locale              	| 190 	| 68 	| 16 	| 69.34% 	|
| ListFormat          	| 158 	| 0  	| 4  	| 97.53% 	|
| getCanonicalLocales 	| 54  	| 0  	| 22 	| 71.05% 	|
| Collator              | 106   | 0   | 12  | 89.83%  |
