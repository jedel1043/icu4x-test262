use icu_collator::{
    provider::CollationMetadataV1Marker, CaseLevel, Collator, CollatorOptions, Strength,
};
use icu_locid::locale;
use icu_provider::{DataLocale, DataProvider, DataRequest, DataRequestMetadata};

// https://github.com/tc39/test262/blob/main/test/intl402/Collator/supportedLocalesOf/basic.js
#[test]
fn supported_locales_of_basic() {
    // https://github.com/unicode-org/cldr/blob/main/common/validity/language.xml
    let data = icu_testdata::unstable();

    let loc = locale!("zxx");

    let request = DataRequest {
        locale: &DataLocale::from(&loc),
        metadata: DataRequestMetadata::default(),
    };

    DataProvider::<CollationMetadataV1Marker>::load(&data, request).expect_err(
        r#"should reject the request, since the locale "zxx" has "no linguistic content""#,
    );
}

// https://github.com/tc39/test262/blob/main/test/intl402/Collator/prototype/compare/non-normative-sensitivity.js
// Fixed by https://github.com/unicode-org/icu4x/commit/856b3dcc07e34ef2935c256cd50265cbe04aa6b9
#[test]
fn func() {
    let data = icu_testdata::unstable();
    let collator = Collator::try_new_unstable(&data, &locale!("es").into(), {
        let mut opts = CollatorOptions::new();
        opts.strength = Some(Strength::Primary);
        opts.case_level = Some(CaseLevel::On);
        opts
    })
    .unwrap();
    let target = "Aa";
    let input = ["Aa", "aA", "áÁ", "Aã"];

    let matches = input
        .into_iter()
        .filter(|s| collator.compare(s, target).is_eq())
        .collect::<Vec<_>>();

    assert_eq!(&matches, &["Aa", "Aã"])
}
