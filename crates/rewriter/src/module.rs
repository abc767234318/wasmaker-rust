use core::panic;
use std::borrow::Cow;

use wasm_encoder::{
    CodeSection, CompositeInnerType, CustomSection, DataCountSection, DataSection, ElementSection,
    ExportSection, FunctionSection, GlobalSection, ImportSection, MemorySection, Module, SectionId,
    StartSection, TableSection, TagSection, TypeSection,
};
use wasmparser::{Parser, Payload, TypeRef};

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
                wasmparser::Chunk::NeedMoreData(hint) => {
                    panic!("Invalid wasm binary: {hint:?}");
                }
                wasmparser::Chunk::Parsed { payload, consumed } => (payload, consumed),
            };

            match payload {
                Payload::CustomSection(reader) => {
                    let custom_section = CustomSection {
                        name: Cow::Owned(reader.name().to_string()),
                        data: Cow::Owned(reader.data().to_vec()),
                    };
                    wasm_module.custom_sections.push(custom_section);
                }
                Payload::TypeSection(reader) => {
                    let mut type_section = TypeSection::new();
                    for ty_group_iter in reader.into_iter() {
                        let ty_iter = ty_group_iter.unwrap().into_types();
                        for ty in ty_iter {
                            let composite_type = ty.composite_type.inner;
                            match composite_type {
                                wasm_encoder::CompositeInnerType::Func(func_ty) => {
                                    let func_params = func_ty.params();
                                    let func_results = func_ty.results();
                                    type_section.function(
                                        func_params.iter().copied(),
                                        func_results.iter().copied(),
                                    );
                                }
                                // The following types are not supported yet.
                                wasm_encoder::CompositeInnerType::Array(array_type) => {}
                                wasm_encoder::CompositeInnerType::Struct(struct_type) => {}
                                wasm_encoder::CompositeInnerType::Cont(cont_type) => {}
                                _ => {
                                    panic!("Unsupported type: {:?}", composite_type);
                                }
                            }
                        }
                    }
                    wasm_module.type_section = type_section;
                }
                Payload::ImportSection(reader) => {
                    let mut import_section = ImportSection::new();
                    for import_item in reader {
                        let import_item = import_item.unwrap();
                        let module = import_item.module;
                        let name = import_item.name;
                        match import_item.ty {
                            TypeRef::Func(_) => wasm_module.imported_functions_count += 1,
                            TypeRef::Global(_) => wasm_module.imported_globals_count += 1,
                            TypeRef::Memory(_) => wasm_module.imported_memories_count += 1,
                            TypeRef::Table(_) => wasm_module.imported_tables_count += 1,
                            TypeRef::Tag(_) => wasm_module.imported_tags_count += 1,
                        }
                        import_section.import(module, name, import_item.ty);
                    }
                    wasm_module.import_section = import_section;
                }
                Payload::FunctionSection(reader) => {
                    let mut function_section = FunctionSection::new();
                    for func in reader {
                        let func = func.unwrap();
                        function_section.function(func);
                    }
                    wasm_module.function_section = function_section;
                }
                Payload::TableSection(reader) => {
                    let mut table_section = TableSection::new();
                    for table in reader {
                        let table = table.unwrap();
                        table_section.table(table);
                    }
                    wasm_module.table_section = table_section;
                }
                Payload::MemorySection(reader) => {
                    let mut memory_section = MemorySection::new();
                    for memory in reader {
                        let memory = memory.unwrap();
                        memory_section.memory(memory);
                    }
                    wasm_module.memory_section = memory_section;
                }
                Payload::GlobalSection(reader) => {
                    let mut global_section = GlobalSection::new();
                    for global in reader {
                        let global = global.unwrap();
                        global_section.global(
                            global.ty,
                            global
                                .init_expr
                                .get_operators_reader()
                                .into_iter()
                                .map(|op| op.unwrap()),
                        );
                    }
                    wasm_module.global_section = global_section;
                }
                _ => {}
            }
        }
        wasm_module
    }
}
