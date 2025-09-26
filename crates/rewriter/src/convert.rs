pub trait ParserToEncoderCompositeInnerType {
    fn convert(&self) -> wasm_encoder::CompositeInnerType;
}

pub trait ParserToEncoderValType {
    fn to_encoder_type(&self) -> wasm_encoder::ValType;
}

impl ParserToEncoderValType for wasmparser::ValType {
    fn to_encoder_type(&self) -> wasm_encoder::ValType {
        match self {
            wasmparser::ValType::I32 => wasm_encoder::ValType::I32,
            wasmparser::ValType::I64 => wasm_encoder::ValType::I64,
            wasmparser::ValType::F32 => wasm_encoder::ValType::F32,
            wasmparser::ValType::F64 => wasm_encoder::ValType::F64,
            wasmparser::ValType::V128 => wasm_encoder::ValType::V128,
            wasmparser::ValType::Ref(ref_ty) => {
                match *ref_ty {
                    wasmparser::RefType::FUNCREF => wasm_encoder::ValType::FUNCREF,
                    wasmparser::RefType::EXTERNREF => wasm_encoder::ValType::EXTERNREF,
                    wasmparser::RefType::EXNREF => wasm_encoder::ValType::EXNREF,
                    _ => panic!("Unsupported RefType"),
                    // It seems wasm_encoder has only three kinds of RefType
                }
            }
        }
    }
}

pub trait ParserToEncoderMemoryType {
    fn to_encoder_type(&self) -> wasm_encoder::MemoryType;
}

impl ParserToEncoderMemoryType for wasmparser::MemoryType {
    fn to_encoder_type(&self) -> wasm_encoder::MemoryType {
        wasm_encoder::MemoryType {
            minimum: self.initial,
            maximum: self.maximum,
            memory64: self.memory64,
            shared: self.shared,
            page_size_log2: self.page_size_log2,
        }
    }
}

pub trait ParserToEncoderGlobalType {
    fn to_encoder_type(&self) -> wasm_encoder::GlobalType;
}

impl ParserToEncoderGlobalType for wasmparser::GlobalType {
    fn to_encoder_type(&self) -> wasm_encoder::GlobalType {
        wasm_encoder::GlobalType {
            val_type: self.content_type.to_encoder_type(),
            mutable: self.mutable,
            shared: self.shared, // wasmparser::GlobalType does not have 'shared' field
        }
    }
}

pub trait ParserToEncoderTableType {
    fn to_encoder_type(&self) -> wasm_encoder::TableType;
}
impl ParserToEncoderTableType for wasmparser::TableType {
    fn to_encoder_type(&self) -> wasm_encoder::TableType {
        let element_type = match self.element_type {
            wasmparser::RefType::ANYREF => wasm_encoder::RefType::ANYREF,
            wasmparser::RefType::EQREF => wasm_encoder::RefType::EQREF,
            wasmparser::RefType::FUNCREF => wasm_encoder::RefType::FUNCREF,
            wasmparser::RefType::EXTERNREF => wasm_encoder::RefType::EXTERNREF,
            wasmparser::RefType::I31REF => wasm_encoder::RefType::I31REF,
            wasmparser::RefType::ARRAYREF => wasm_encoder::RefType::ARRAYREF,
            wasmparser::RefType::EXNREF => wasm_encoder::RefType::EXNREF,
            _ => panic!("Unsupported RefType in TableType"),
        };

        wasm_encoder::TableType {
            element_type: element_type,
            table64: self.table64,
            minimum: self.initial,
            maximum: self.maximum,
            shared: self.shared,
        }
    }
}

pub trait ParserToEncoderTagType {
    fn to_encoder_type(&self) -> wasm_encoder::TagType;
}

impl ParserToEncoderTagType for wasmparser::TagType {
    fn to_encoder_type(&self) -> wasm_encoder::TagType {
        let tag_kind = match self.kind {
            wasmparser::TagKind::Exception => wasm_encoder::TagKind::Exception,
            _ => panic!("Unsupported TagKind"),
        };
        wasm_encoder::TagType {
            kind: tag_kind,
            func_type_idx: self.func_type_idx,
        }
    }
}
