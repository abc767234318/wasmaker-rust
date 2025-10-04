
use common::opcodes::OpCode;


#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub args: InstructionArgs,
}

impl Instruction {
    pub fn new(opcode: OpCode, args: InstructionArgs) -> Self {
        Instruction { opcode, args }
    }
    
    pub fn new_simple(opcode: OpCode, arg: u32) -> Self {
        Instruction { opcode, args: InstructionArgs::Simple(arg) }
    }

    pub fn new_block(opcode: OpCode, bt: BlockType) -> Self {
        Instruction { opcode, args: InstructionArgs::Block(bt) }
    }

    pub fn new_mem(opcode: OpCode, memarg:MemArg) -> Self {
        Instruction { opcode, args: InstructionArgs::Mem(memarg) }
    }

    pub fn new_br_table(opcode: OpCode, br_table_args: BrTableArgs) -> Self {
        Instruction { opcode, args: InstructionArgs::BrTable(br_table_args) }
    }

    pub fn new_if(opcode: OpCode, if_args: IfArgs) -> Self {
        Instruction { opcode, args: InstructionArgs::If(if_args) }
    }

    pub fn new_if_with_branches(opcode: OpCode, bt: BlockType, instrs1: Vec<Instruction>, instrs2: Vec<Instruction>) -> Self {
        let if_args = IfArgs { bt, instrs1, instrs2 };
        Instruction { opcode, args: InstructionArgs::If(if_args) }
    }

    pub fn new_none(opcode: OpCode) -> Self {
        Instruction { opcode, args: InstructionArgs::None }
    }

    pub fn new_loop(opcode: OpCode, bt: BlockType) -> Self {
        Instruction { opcode, args: InstructionArgs::Loop(bt) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfArgs {
    pub bt: BlockType,           // 块类型
    pub instrs1: Vec<Instruction>, // then 分支
    pub instrs2: Vec<Instruction>, // else 分支
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrTableArgs {
    pub labels: Vec<u32>,    // 向量类型
    pub default_label: u32,  // 默认标签
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionArgs {
    Block(BlockType),
    Loop(BlockType),
    If(IfArgs),
    Mem(MemArg),
    BrTable(BrTableArgs),
    Simple(u32),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    V128 = 0x7B,
    FuncRef = 0x70,
    ExternRef = 0x6F,
}