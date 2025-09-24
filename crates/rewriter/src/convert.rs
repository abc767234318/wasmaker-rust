pub trait ParserToEncoderType {
    fn to_encoder_type(&self) -> wasm_encoder::CompositeInnerType;
}
