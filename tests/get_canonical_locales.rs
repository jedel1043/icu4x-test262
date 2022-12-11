use icu_locid::Locale;
use icu_locid_transform::LocaleCanonicalizer;

fn check_canonicalization(tag: &str, canonical: &str, canonicalizer: &LocaleCanonicalizer) {
    let mut tag = tag.parse().unwrap();
    canonicalizer.canonicalize(&mut tag);
    assert_eq!(
        &tag.to_string(),
        canonical,
        "canonicalization didn't return the correct result"
    );
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/unicode-ext-canonicalize-yes-to-true.js
#[test]
fn unicode_ext_canonicalize_yes_to_true() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    check_canonicalization("und-u-kb-yes", "und-u-kb", &canonicalizer)
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/unicode-ext-canonicalize-calendar.js
#[test]
fn unicode_ext_canonicalize_calendar() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    check_canonicalization(
        "und-u-ca-ethiopic-amete-alem",
        "und-u-ca-ethioaa",
        &canonicalizer,
    );
    check_canonicalization(
        "und-u-ca-islamicc",
        "und-u-ca-islamic-civil",
        &canonicalizer,
    );
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/unicode-ext-canonicalize-timezone.js
#[test]
fn unicode_ext_canonicalize_timezone() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    for (tag, canonical) in [
        ("und-u-tz-cnckg", "und-u-tz-cnsha"),
        ("und-u-tz-eire", "und-u-tz-iedub"),
        ("und-u-tz-est", "und-u-tz-utcw05"),
        ("und-u-tz-gmt0", "und-u-tz-gmt"),
        ("und-u-tz-uct", "und-u-tz-utc"),
        ("und-u-tz-zulu", "und-u-tz-utc"),
    ] {
        check_canonicalization(tag, canonical, &canonicalizer);
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/transformed-ext-canonical.js
#[test]
fn transformed_ext_canonical() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    for (tag, canonical) in [
        ("sl-t-sl-rozaj-biske-1994", "sl-t-sl-1994-biske-rozaj"),
        ("DE-T-M0-DIN-K0-QWERTZ", "de-t-k0-qwertz-m0-din"),
        ("en-t-m0-true", "en-t-m0-tru"),
        ("en-t-iw", "en-t-he"),
        (
            "und-Latn-t-und-hani-m0-names",
            "und-Latn-t-und-hani-m0-prprname",
        ),
    ] {
        check_canonicalization(tag, canonical, &canonicalizer);
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/unicode-ext-canonicalize-col-strength.js
#[test]
fn unicode_ext_canonicalize_col_strength() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    check_canonicalization("und-u-ks-primary", "und-u-ks-level1", &canonicalizer);
    check_canonicalization("und-u-ks-tertiary", "und-u-ks-level3", &canonicalizer);
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/unicode-ext-canonicalize-measurement-system.js
#[test]
fn unicode_ext_canonicalize_measurement_system() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    check_canonicalization("und-u-ms-imperial", "und-u-ms-uksystem", &canonicalizer);
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/error-cases.js
#[test]
fn error_cases() {
    for tag in ["en-us-", "-en-us", "en-us-en-us", "--", "-", "", "-e-"] {
        tag.parse::<Locale>()
            .expect_err("should not parse invalid locale");
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/invalid-tags.js
#[test]
fn invalid_tags() {
    for tag in [
        "",                 // empty tag
        "i",                // singleton alone
        "x",                // private use without subtag
        "u",                // extension singleton in first place
        "419",              // region code in first place
        "u-nu-latn-cu-bob", // extension sequence without language
        "hans-cmn-cn",      // "hans" could theoretically be a 4-letter language code,
        // but those can't be followed by extlang codes.
        "cmn-hans-cn-u-u",           // duplicate singleton
        "cmn-hans-cn-t-u-ca-u",      // duplicate singleton
        "de-gregory-gregory",        // duplicate variant
        "*",                         // language range
        "de-*",                      // language range
        "中文",                    // non-ASCII letters
        "en-ß",                     // non-ASCII letters
        "ıd",                       // non-ASCII letters
        "es-Latn-latn",              // two scripts
        "pl-PL-pl",                  // two regions
        "u-ca-gregory",              // extension in first place
        "de-1996-1996",              // duplicate numeric variant
        "pt-u-ca-gregory-u-nu-latn", // duplicate singleton subtag
        // Invalid tags starting with: https://github.com/tc39/ecma402/pull/289
        "no-nyn",        // regular grandfathered in BCP47, but invalid in UTS35
        "i-klingon",     // irregular grandfathered in BCP47, but invalid in UTS35
        "zh-hak-CN",     // language with extlang in BCP47, but invalid in UTS35
        "sgn-ils",       // language with extlang in BCP47, but invalid in UTS35
        "x-foo",         // privateuse-only in BCP47, but invalid in UTS35
        "x-en-US-12345", // more privateuse-only variants.
        "x-12345-12345-en-US",
        "x-en-US-12345-12345",
        "x-en-u-foo",
        "x-en-u-foo-u-bar",
        "x-u-foo",
        // underscores in different parts of the language tag
        "de_DE",
        "DE_de",
        "cmn_Hans",
        "cmn-hans_cn",
        "es_419",
        "es-419-u-nu-latn-cu_bob",
        "i_klingon",
        "cmn-hans-cn-t-ca-u-ca-x_t-u",
        "enochian_enochian",
        "de-gregory_u-ca-gregory",
        "en\u{0000}", // null-terminator sequence
        " en",        // leading whitespace
        "en ",        // trailing whitespace
        "it-IT-Latn", // country before script tag
        "de-u",       // incomplete Unicode extension sequences
        "de-u-",
        "de-u-ca-",
        "de-u-ca-gregory-",
        "si-x", // incomplete private-use tags
        "x-",
        "x-y-",
    ] {
        tag.parse::<Locale>()
            .expect_err("should not parse invalid locale");
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/transformed-ext-valid.js
#[test]
fn transformed_ext_valid() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    let valid = [
        // tlang with unicode_language_subtag.
        "en-t-en",
        // tlang with unicode_script_subtag.
        "en-t-en-latn",
        // tlang with unicode_region_subtag.
        "en-t-en-ca",
        // tlang with unicode_script_subtag and unicode_region_subtag.
        "en-t-en-latn-ca",
        // tlang with unicode_variant_subtag.
        "en-t-en-emodeng",
        // tlang with unicode_script_subtag and unicode_variant_subtag.
        "en-t-en-latn-emodeng",
        // tlang with unicode_script_subtag and unicode_variant_subtag.
        "en-t-en-ca-emodeng",
        // tlang with unicode_script_subtag, unicode_region_subtag, and unicode_variant_subtag.
        "en-t-en-latn-ca-emodeng",
        // No tlang. (Must contain at least one tfield.)
        "en-t-d0-ascii",
    ];

    let fields = [
        // No extra tfield
        "",
        // tfield with a tvalue consisting of a single subtag.
        "-i0-handwrit",
        // tfield with a tvalue consisting of two subtags.
        "-s0-accents-publish",
    ];

    for tag in valid {
        for extra in fields {
            let s = tag.to_string() + extra;
            check_canonicalization(&s, &s, &canonicalizer);
        }
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/transformed-ext-invalid.js
#[test]
fn transformed_ext_invalid() {
    for tag in [
        // empty
        "en-t",
        "en-t-a",
        "en-t-x",
        "en-t-0",
        // incomplete
        "en-t-",
        "en-t-en-",
        "en-t-0x-",
        // tlang: unicode_language_subtag must be 2-3 or 5-8 characters and mustn't
        // contain extlang subtags.
        "en-t-root",
        "en-t-abcdefghi",
        "en-t-ar-aao",
        // tlang: unicode_script_subtag must be 4 alphabetical characters, can't
        // be repeated.
        "en-t-en-lat0",
        "en-t-en-latn-latn",
        // tlang: unicode_region_subtag must either be 2 alpha characters or a three
        // digit code.
        "en-t-en-0",
        "en-t-en-00",
        "en-t-en-0x",
        "en-t-en-x0",
        "en-t-en-latn-0",
        "en-t-en-latn-00",
        "en-t-en-latn-xyz",
        // tlang: unicode_variant_subtag is either 5-8 alphanum characters or 4
        // characters starting with a digit.
        "en-t-en-abcdefghi",
        "en-t-en-latn-gb-ab",
        "en-t-en-latn-gb-abc",
        "en-t-en-latn-gb-abcd",
        "en-t-en-latn-gb-abcdefghi",
        // tkey must be followed by tvalue.
        "en-t-d0",
        "en-t-d0-m0",
        "en-t-d0-x-private",
    ] {
        tag.parse::<Locale>()
            .expect_err("should not parse invalid locale");
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/non-iana-canon.js
#[test]
fn non_iana_canon() {
    let data = icu_testdata::unstable();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    for (tag, canonical) in [
        ("mo", "ro"),
        ("es-ES-preeuro", "es-ES-preeuro"),
        ("uz-UZ-cyrillic", "uz-UZ-cyrillic"),
        ("posix", "posix"),
        ("hi-direct", "hi-direct"),
        ("zh-pinyin", "zh-pinyin"),
        ("zh-stroke", "zh-stroke"),
        // "aar" should be canonicalized into "aa" because "aar" matches the type attribute of
        // a languageAlias element in
        // https://www.unicode.org/repos/cldr/trunk/common/supplemental/supplementalMetadata.xml
        ("aar-x-private", "aa-x-private"),
        // "heb" should be canonicalized into "he" because "heb" matches the type attribute of
        // a languageAlias element in
        // https://www.unicode.org/repos/cldr/trunk/common/supplemental/supplementalMetadata.xml
        ("heb-x-private", "he-x-private"),
        ("de-u-kf", "de-u-kf"),
        // "ces" should be canonicalized into "cs" because "ces" matches the type attribute of
        // a languageAlias element in
        // https://www.unicode.org/repos/cldr/trunk/common/supplemental/supplementalMetadata.xml
        ("ces", "cs"),
        ("hy-arevela", "hy"),
        ("hy-arevmda", "hyw"),
    ] {
        check_canonicalization(tag, canonical, &canonicalizer);
    }
}
