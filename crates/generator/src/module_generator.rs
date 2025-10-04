
use common::opcodes::OpCode;
use crate::AST::{Node, InstructionType, Context};
use crate::instruction::{Instruction, InstructionArgs, BlockType, IfArgs, MemArg};



// fn json_to_ast_convert(json_ast: &JsonAst) -> Node {
//     let instr_data = &json_ast.instr;
    
//     // 转换操作码
//     let opcode = OpCode::from_u32(instr_data.opcode);
    
//     // 转换参数（根据操作码类型）
//     let args = match opcode {
//         OpCode::Block | OpCode::Loop => {
//             InstructionArgs::Block(parse_block_type(&instr_data.args))
//         }
//         OpCode::If => {
//             InstructionArgs::If(parse_if_args(&instr_data.args))
//         }
//         OpCode::I32Load | OpCode::I64Load => {
//             InstructionArgs::Mem(parse_mem_arg(&instr_data.args))
//         }
//         OpCode::Unknown(_) => {
//             // 对于未知操作码，尝试作为简单参数解析
//             InstructionArgs::Simple(instr_data.args.as_u32().unwrap_or(0))
//         }
//         _ => {
//             // 默认尝试作为简单参数
//             InstructionArgs::Simple(instr_data.args.as_u32().unwrap_or(0))
//         }
//     };
    
//     let instr = Instruction::new(opcode, args);
    
//     Node::new(
//         instr,
//         parse_instr_type(&json_ast.instr_type),
//         parse_context(&json_ast.context),
//     )
// }