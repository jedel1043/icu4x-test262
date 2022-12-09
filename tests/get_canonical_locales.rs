use icu_locid_transform::LocaleCanonicalizer;

// https://github.com/tc39/test262/blob/main/test/intl402/Intl/getCanonicalLocales/unicode-ext-canonicalize-yes-to-true.js
#[test]
fn unicode_ext_canonicalize_yes_to_true() {
    let data = icu_testdata::unstable();
    let canon = LocaleCanonicalizer::try_new_unstable(&data).unwrap();
    let mut tag = "und-u-kb-yes".parse().unwrap();
    canon.canonicalize(&mut tag);

    assert_eq!(tag, "und-u-kb".parse().unwrap());
}
