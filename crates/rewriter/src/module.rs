use wasm_encoder::{
    CodeSection, CustomSection, DataCountSection, DataSection, ElementSection, ExportSection,
    FunctionSection, GlobalSection, ImportSection, MemorySection, Module, StartSection,
    TableSection, TagSection, TypeSection,
};
use wasmparser::Parser;

#[derive(Clone, Debug)] // I did not add Default here, it may be used later
pub struct WasmModule<'a> {
    pub custom_sections: Vec<CustomSection<'a>>,
    pub type_section: TypeSection,
    pub import_section: ImportSection,
    pub function_section: FunctionSection,
    pub table_section: TableSection,
    pub memory_section: MemorySection,
    pub global_section: GlobalSection,
    pub export_section: ExportSection,
    pub start_section: StartSection,
    pub element_section: ElementSection,
    pub code_section: CodeSection,
    pub data_section: DataSection,
    pub data_count_section: DataCountSection,
    pub tag_section: TagSection,

    imported_functions_count: u32,
    imported_globals_count: u32,
    imported_memories_count: u32,
    imported_tables_count: u32,
    imported_tags_count: u32,
}

impl<'a> WasmModule<'a> {
    pub fn new(input_wasm_binary: &[u8]) -> Self {
        let mut parser = Parser::new(0);
        let mut wasm_module = WasmModule {
            custom_sections: Vec::new(),
            type_section: TypeSection::new(),
            import_section: ImportSection::new(),
            function_section: FunctionSection::new(),
            table_section: TableSection::new(),
            memory_section: MemorySection::new(),
            global_section: GlobalSection::new(),
            export_section: ExportSection::new(),
            start_section: StartSection { function_index: 0 },
            element_section: ElementSection::new(),
            code_section: CodeSection::new(),
            data_section: DataSection::new(),
            data_count_section: DataCountSection { count: 0 },
            tag_section: TagSection::new(),

            imported_functions_count: 0,
            imported_globals_count: 0,
            imported_memories_count: 0,
            imported_tables_count: 0,
            imported_tags_count: 0,
        };

        loop {
            let (payload, consumed) = match parser.parse(input_wasm_binary, true).unwrap() {
                wasmparser::Chunk::NeedMoreData(_) => break,
                wasmparser::Chunk::Parsed { payload, consumed } => (payload, consumed),
            };
        }

        wasm_module
    }
}
