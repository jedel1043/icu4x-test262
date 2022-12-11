use icu_collator::provider::CollationMetadataV1Marker;
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
