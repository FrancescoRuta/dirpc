pub trait TypeEncoding {
    type EncodedType;
    const NAME: &'static str;
}