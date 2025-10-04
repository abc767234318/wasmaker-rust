use crate::instruction::{Instruction, ValType};


#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: Option<u64>,
    pub instr: Instruction,
    pub sub_instrs: Vec<Node>,
    pub instr_type: InstructionType,
    pub context: Option<Context>,
}

impl Node {
    pub fn new(instr: Instruction, instr_type: InstructionType, context: Option<Context>) -> Self {
        Node {
            id: None,
            instr,
            sub_instrs: Vec::new(),
            instr_type,
            context,
        }
    }
}




#[derive(Debug, Clone, PartialEq)]
pub struct InstructionType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub local_variable: Option<LocalVariableContext>,
    pub functype: Option<FunctionTypeContext>,
    pub global_variable: Option<GlobalVariableContext>,
    pub memory: Option<MemoryContext>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableContext {
    pub local_variable_type: ValType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionTypeContext {
    pub param_types: Vec<ValType>,
    pub result_types: Vec<ValType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableContext {
    pub global_variable_type: ValType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryContext {
    pub max: u32,
}