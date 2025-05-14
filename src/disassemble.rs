use std::fmt::format;
use alloy::hex;
use log::info;
use zkevm_opcode_defs::decoding::encoding_mode_production::EncodingModeProduction;
use zkevm_opcode_defs::decoding::{VariantMonotonicNumber, VmEncodingMode};
use zkevm_opcode_defs::{BinopOpcode, DecodedOpcode, OpcodeVariant, Operand, JumpOpcode};
use zkevm_opcode_defs::definitions::all::Opcode;
use zkevm_opcode_defs::imm_mem_modifiers::ImmMemHandlerFlags;
use zkevm_opcode_defs::definitions::uma::UMAOpcode;
use zkevm_opcode_defs::imm_mem_modifiers::RegOrImmFlags;
use zkevm_opcode_defs::definitions::shift::ShiftOpcode;
use alloy::primitives::U256;
#[derive(Debug, Clone, Copy)]
pub struct Disassemble;

impl Disassemble {
    pub fn disassemble_hex_string(hex_string: &str) {
        let hex_string = if hex_string.starts_with("0x") {
            hex_string.trim_start_matches("0x")
        } else {
            hex_string
        };
        let code_bytes = match  hex::decode(hex_string){
            Ok(bytes) => bytes,
            Err(e) => {
                panic!("Failed to decode hex string: {}", e)
            }
        };
        Self::disassemble_bytes(code_bytes);
    }

    pub fn disassemble_bytes(code_bytes:Vec<u8>) {
        let mut code_page: Vec<U256> = Vec::new();
        for chunk in code_bytes.chunks(32) {
            let item = U256::from_be_slice(chunk);
            code_page.push(item);
        }

        let mut opcodes: Vec<DecodedOpcode> = Vec::new();
        for chunk in code_bytes.chunks(8) {
            let mut padded_chunk = [0u8; 8];
            let len = chunk.len().min(8);
            padded_chunk[..len].copy_from_slice(&chunk[..len]);
            let inst_val = u64::from_be_bytes(padded_chunk);

            let (decode_opcode, _) = EncodingModeProduction::parse_preliminary_variant_and_absolute_number(inst_val);
            opcodes.push(decode_opcode);
        }

        for code in opcodes {
            let asm = Self::decode_opcode_to_string(code, code_page.clone());
            if asm != "" {
                println!("{}", asm);
            }

        }
    }

    pub fn decode_opcode_to_string(code: DecodedOpcode, code_page: Vec<U256>) -> String {
        println!("DecodedOpcode: {:?}", code);
        match code.variant.opcode{
            Opcode::Invalid(_) => "".to_string(),
            Opcode::Nop(_nop) => {
                format!("nop\tstack+=[{:?}]", code.imm_1)
            },
            Opcode::Add(_add) => {
                format!("add\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
            },
            Opcode::UMA(_uma) => {
                match _uma { 
                    UMAOpcode::AuxHeapRead => {
                        // TODO
                        format!("aux_heap_read\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    UMAOpcode::AuxHeapWrite => {
                        // TODO
                        format!("stm.ah\t{}, r{}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx)
                    },
                    UMAOpcode::FatPointerRead => {
                        // TODO
                        format!("fat_pointer_read\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    UMAOpcode::StaticMemoryRead => {
                        // TODO
                        format!("static_memory_read\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    UMAOpcode::StaticMemoryWrite => {
                        // TODO
                        format!("static_memory_write\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    UMAOpcode::HeapRead => {
                        // TODO
                        format!("aux_heap_read\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    UMAOpcode::HeapWrite => {
                        format!("stm.h\t{}, r{}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx)
                    }
                }
            },
            Opcode::Binop(_binop) => {
                match _binop { 
                    BinopOpcode::And => {
                        format!("and\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    BinopOpcode::Or => {
                        // TODO
                        format!("or\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                    BinopOpcode::Xor => {
                        // TODO
                        format!("xor\t{}, r{:?}, {}", Self::get_src0(code, code_page.clone()), code.src1_reg_idx, Self::get_dst0(code, code_page.clone()))
                    },
                }
            },
            Opcode::Jump(_jump) => {
                format!("jump\t{}", Self::get_src0(code, code_page.clone()))
            }
            _ => "".to_string(),
        }
    }

    fn get_src0(code: DecodedOpcode, code_page: Vec<U256>) -> String {
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

    fn get_dst0(code: DecodedOpcode, code_page: Vec<U256>) -> String {
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
        let hex_string = "0000008003000039000000400030043f00000001002001900000001a0000c13d00000060021002700000001202200197000000040020008c000000380000413d000000000301043b000000e003300270000000140030009c000000290000613d000000150030009c000000220000613d000000160030009c000000380000c13d000000240020008c000000380000413d0000000002000416000000000002004b000000380000c13d0000000401100370000000000101043b000000000010041b0000000001000019000000450001042e0000000001000416000000000001004b000000380000c13d0000002001000039000001000010044300000120000004430000001301000041000000450001042e0000000001000416000000000001004b000000380000c13d000000000100041a000000800010043f0000001c01000041000000450001042e0000000001000416000000000001004b000000380000c13d0000000001000411000000000001004b0000003a0000c13d000000000100041a000000010110003a000000170000c13d0000001a01000041000000000010043f0000001101000039000000040010043f0000001b010000410000004600010430000000000100001900000046000104300000001701000041000000800010043f0000002001000039000000840010043f0000000401000039000000a40010043f0000001801000041000000c40010043f000000190100004100000046000104300000004400000432000000450001042e0000004600010430000000000000000000000000000000000000000000000000000000000000000000000000ffffffff000000020000000000000000000000000000004000000100000000000000000000000000000000000000000000000000000000000000000000000000d09de08a000000000000000000000000000000000000000000000000000000008381f58a000000000000000000000000000000000000000000000000000000003fb5c1cb08c379a000000000000000000000000000000000000000000000000000000000787878780000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000008000000000000000004e487b7100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002400000000000000000000000000000000000000000000000000000000000000200000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a26469706673582212200c1222991df1b7c18bf0912541eb0940ac1710786f62097cec61a60770265bd364736f6c6378247a6b736f6c633a312e352e31333b736f6c633a302e382e32393b6c6c766d3a312e302e310055";
        Disassemble::disassemble_hex_string(hex_string);
    }
}