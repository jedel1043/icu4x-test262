use icu_locid::{extensions::unicode::Value, locale, Locale};
use icu_locid_transform::{LocaleCanonicalizer, LocaleExpander};

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/constructor-non-iana-canon.js
#[test]
fn constructor_non_iana_canon() {
    let data = icu_testdata::unstable();
    let canon = LocaleCanonicalizer::try_new_unstable(&data).unwrap();
    let expander = LocaleExpander::try_new_unstable(&data).unwrap();

    // Valid language tag per the https://unicode.org/reports/tr35/#unicode_language_subtag production
    "posix".parse::<Locale>().unwrap();

    // Should maximize to its canonical tag `hyw`
    let mut locale = "hy-arevmda".parse::<Locale>().unwrap();
    canon.canonicalize(&mut locale);
    expander.maximize(&mut locale);
    assert_eq!(locale, locale!("hyw"));
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/constructor-options-canonicalized.js
#[test]
fn constructor_options_canonicalized() {
    let data = icu_testdata::unstable();
    let canon = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    let locale = &mut "en-u-ca-islamicc".parse::<Locale>().unwrap();
    canon.canonicalize(locale);

    assert_eq!(locale, &"en-u-ca-islamic-civil".parse::<Locale>().unwrap());

    let locale = &mut "en-u-ca-ethiopic-amete-alem".parse::<Locale>().unwrap();
    canon.canonicalize(locale);
    assert_eq!(locale, &"en-u-ca-ethioaa".parse::<Locale>().unwrap());
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/constructor-options-numberingsystem-invalid.js
#[test]
fn constructor_options_numberingsystem_invalid() {
    for tag in ["-latn-", "latn-", "latn--"] {
        tag.parse::<Value>().expect_err("undetected invalid value");
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/constructor-unicode-ext-invalid.js
#[test]
fn constructor_unicode_ext_invalid() {
    "da-u-ca-gregory-u-ca-buddhist"
        .parse::<Locale>()
        .expect_err("undetected duplicate singleton subtag");
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/invalid-tag-throws.js
#[test]
fn invalid_tag_throws() {
    "pt-u-ca-gregory-u-nu-latn"
        .parse::<Locale>()
        .expect_err("undetected duplicate singleton subtag");

    "de-u-ca-"
        .parse::<Locale>()
        .expect_err("undetected incomplete Unicode extension sequences");

    "si-x"
        .parse::<Locale>()
        .expect_err("undetected incomplete private-use tags");

    for tag in [
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
    ] {
        tag.parse::<Locale>().expect_err(
            "ECMAScript uses BCP 47 locale identifiers, which reject using underscores",
        );
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/likely-subtags-grandfathered.js
#[test]
fn likely_subtags_grandfathered() {
    let data = icu_testdata::unstable();
    let expander = LocaleExpander::try_new_unstable(&data).unwrap();
    let canonicalizer = LocaleCanonicalizer::try_new_unstable(&data).unwrap();

    let mut orig = locale!("cel-gaulish");
    let canonical = locale!("xtg");

    canonicalizer.canonicalize(&mut orig);
    assert_eq!(orig, canonical);

    {
        let mut tag = orig.clone();
        expander.maximize(&mut tag);
        canonicalizer.canonicalize(&mut tag);
        assert_eq!(tag, canonical);
    }

    {
        let mut tag = orig.clone();
        expander.maximize(&mut tag);
        expander.maximize(&mut tag);
        canonicalizer.canonicalize(&mut tag);
        assert_eq!(tag, canonical);
    }

    {
        let mut tag = orig.clone();
        expander.minimize(&mut tag);
        canonicalizer.canonicalize(&mut tag);
        assert_eq!(tag, canonical);
    }

    {
        let mut tag = orig.clone();
        expander.minimize(&mut tag);
        expander.minimize(&mut tag);
        canonicalizer.canonicalize(&mut tag);
        assert_eq!(tag, canonical);
    }

    {
        let mut tag = orig.clone();
        expander.minimize(&mut tag);
        expander.maximize(&mut tag);
        assert_eq!(tag, canonical);
    }
    {
        let mut tag = orig.clone();
        expander.maximize(&mut tag);
        expander.minimize(&mut tag);
        assert_eq!(tag, canonical);
    }

    {
        let mut tag = orig.clone();
        expander.minimize(&mut tag);
        expander.maximize(&mut tag);
        assert_eq!(tag, canonical);
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/likely-subtags.js
#[test]
fn likely_subtags() {
    let data = icu_testdata::unstable();
    let expander = LocaleExpander::try_new_unstable(&data).unwrap();

    {
        let mut loc = locale!("und-150");
        expander.maximize(&mut loc);
        assert_eq!(loc, locale!("ru-Cyrl-RU"));
    }

    {
        let mut loc = locale!("und-Latn-AQ");
        expander.minimize(&mut loc);
        assert_eq!(loc, locale!("und-AQ"));
    }
}

// https://github.com/tc39/test262/blob/main/test/intl402/Locale/prototype/minimize/removing-likely-subtags-first-adds-likely-subtags.js
#[test]
fn removing_likely_subtags_first_adds_likely_subtags() {
    fn test_minimization(mut tag: Locale, mut minimal: Locale, expander: &LocaleExpander) {
        // Assert the |minimal| tag is indeed minimal.
        {
            let old_minimal = minimal.clone();
            expander.minimize(&mut minimal);
            assert_eq!(old_minimal, minimal, "tag should be minimal");
        }

        // Assert RemoveLikelySubtags(tag) returns |minimal|.
        expander.minimize(&mut tag);
        assert_eq!(tag, minimal);
    }

    let data = icu_testdata::unstable();
    let expander = LocaleExpander::try_new_unstable(&data).unwrap();

    test_minimization(locale!("und-150"), locale!("ru"), &expander);
    test_minimization(locale!("aae-Latn-IT"), locale!("aae-Latn-IT"), &expander);
}
