use std::fmt::format;
use log::info;
use zkevm_opcode_defs::decoding::encoding_mode_production::EncodingModeProduction;
use zkevm_opcode_defs::decoding::{VariantMonotonicNumber, VmEncodingMode};
use zkevm_opcode_defs::{DecodedOpcode, OpcodeVariant, Operand};
use zkevm_opcode_defs::definitions::all::Opcode;
use zkevm_opcode_defs::imm_mem_modifiers::ImmMemHandlerFlags;
use zkevm_opcode_defs::definitions::uma::UMAOpcode;
use zkevm_opcode_defs::imm_mem_modifiers::RegOrImmFlags;
use zkevm_opcode_defs::definitions::shift::ShiftOpcode;

#[derive(Debug, Clone, Copy)]
pub struct Disassemble;

impl Disassemble {
    pub fn disassemble_hex_string(hex_string: &str) {
        let hex_string = if hex_string.starts_with("0x") {
            hex_string.trim_start_matches("0x")
        } else {
            hex_string
        };
        let code_bytes = hex_string.as_bytes();
        Self::disassemble_bytes(code_bytes);
    }

    pub fn disassemble_bytes(code_bytes: &[u8]) {
        let mut code_page: Vec<u64> = Vec::new();
        let mut opcodes: Vec<DecodedOpcode> = Vec::new();
        for chunk in code_bytes.chunks(16) {
            let hex_chunk = std::str::from_utf8(chunk).unwrap();
            let inst_val = match u64::from_str_radix(hex_chunk, 16) {
                Ok(val) => val,
                Err(_) => {
                    panic!("Failed to parse hex chunk: {}", hex_chunk)
                }
            };

            code_page.push(inst_val);
            let (decode_opcode, _) = EncodingModeProduction::parse_preliminary_variant_and_absolute_number(inst_val);
            opcodes.push(decode_opcode);
        }

        for code in opcodes {
            let asm = Self::decode_opcode_to_string(code, code_page.clone());
            println!("{}", asm);
        }
    }

    pub fn decode_opcode_to_string(code: DecodedOpcode, code_page: Vec<u64>) -> String {
        match code.variant.opcode{
            Opcode::Invalid(_) => "".to_string(),
            Opcode::Nop(_nop) => {
                format!("nop\tstack+=[{:?}]", code.imm_1)
            },
            Opcode::Add(_add) => {
                format!("add\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
            },
            _ => "".to_string(),
        }
    }

    fn get_src0(code: DecodedOpcode, code_page: Vec<u64>) -> String {
        match code.variant.src0_operand_type {
            Operand::RegOnly => {
                format!("r{:?}", code.src0_reg_idx)
            },
            Operand::RegOrImm(imm) => {
                match imm {
                    RegOrImmFlags::UseImm16Only=> {
                        format!("{:?}", code.imm_0)
                    },
                    RegOrImmFlags::UseRegOnly => {
                        format!("r{:?}", code.src0_reg_idx)
                    }
                }
            },
            Operand::Full(imm) => {
                match imm {
                    ImmMemHandlerFlags::UseRegOnly => {
                        format!("r{:?}", code.src0_reg_idx)
                    },
                    ImmMemHandlerFlags::UseStackWithPushPop => {
                        format!("stack+=[{:?}]", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseStackWithOffset => {
                        format!("stack+=[{:?}]", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseAbsoluteOnStack => {
                        format!("stack[{:?}]", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseImm16Only => {
                        format!("{:?}", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseCodePage => {
                        if code.imm_0 as usize > code_page.len() {
                            format!("0x{:x}", code.imm_0)
                        } else {
                            format!("0x{:x}", code_page[code.imm_0 as usize])
                        }
                    }
                }
            }
        }
    }

    fn get_dst0(code: DecodedOpcode, code_page: Vec<u64>) -> String {
        match code.variant.dst0_operand_type {
            Operand::RegOnly => {
                format!("r{:?}", code.dst0_reg_idx)
            },
            Operand::RegOrImm(imm) => {
                match imm {
                    RegOrImmFlags::UseImm16Only=> {
                        format!("{:?}", code.imm_0)
                    },
                    RegOrImmFlags::UseRegOnly => {
                        format!("r{:?}", code.dst0_reg_idx)
                    }
                }
            },
            Operand::Full(imm) => {
                match imm {
                    ImmMemHandlerFlags::UseRegOnly => {
                        format!("r{:?}", code.dst0_reg_idx)
                    },
                    ImmMemHandlerFlags::UseStackWithPushPop => {
                        format!("stack+=[{:?}]", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseStackWithOffset => {
                        format!("stack+=[{:?}]", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseAbsoluteOnStack => {
                        format!("stack[{:?}]", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseImm16Only => {
                        format!("{:?}", code.imm_0)
                    },
                    ImmMemHandlerFlags::UseCodePage => {
                        if code.imm_0 as usize > code_page.len() {
                            format!("0x{:x}", code.imm_0)
                        } else {
                            format!("0x{:x}", code_page[code.imm_0 as usize])
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_hex_string() {
        // cargo test test_disassemble_hex_string -- --nocapture
        let hex_string = "0x0000008003000039000000400030043f00000000030100190000006003300270000000100330019700000001022001900000001a0000c13d000000040230008c000000360000413d000000000201043b000000e002200270000000120420009c000000290000613d000000130420009c000000220000613d000000140220009c000000360000c13d0000000002000416000000000202004b000000360000c13d000000040230008a000000200220008c000000360000413d0000000401100370000000000101043b000000390000013d0000000001000416000000000101004b000000360000c13d00000020010000390000010000100443000001200000044300000011010000410000003d0001042e0000000001000416000000000101004b000000360000c13d000000000100041a000000800010043f00000017010000410000003d0001042e0000000001000416000000000101004b000000360000c13d000000000100041a000000010200008a000000000221004b000000380000c13d000000150100004100000000001004350000001101000039000000040010043f00000016010000410000003e0001043000000000010000190000003e000104300000000101100039000000000010041b00000000010000190000003d0001042e0000003c000004320000003d0001042e0000003e00010430000000000000000000000000000000000000000000000000000000000000000000000000ffffffff000000020000000000000000000000000000004000000100000000000000000000000000000000000000000000000000000000000000000000000000d09de08a000000000000000000000000000000000000000000000000000000008381f58a000000000000000000000000000000000000000000000000000000003fb5c1cb4e487b710000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000240000000000000000000000000000000000000000000000000000000000000020000000800000000000000000a9fec98f579a72aad563d3eabb878855a8e580e39796f9db7fa2e3dfaf43ee0d";
        Disassemble::disassemble_hex_string(hex_string);
    }
}