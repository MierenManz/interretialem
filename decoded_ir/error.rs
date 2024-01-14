pub(crate) enum DecodingError {
    UnknownValueType,
    ExpectedNonZero,
    MinBiggerThanMax,
}