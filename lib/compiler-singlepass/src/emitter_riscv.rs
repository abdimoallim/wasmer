//! RISC-V emitter scaffolding.

use crate::{
    codegen_error, common_decl::Size, location::Location as AbstractLocation,
    machine_riscv::AssemblerRiscv,
};
pub use crate::{
    location::Multiplier,
    machine::{Label, Offset},
    riscv_decl::{FPR, GPR},
};
use dynasm::dynasm;
use dynasmrt::{AssemblyOffset, DynamicLabel, DynasmApi, DynasmLabelApi};
use wasmer_types::{target::CpuFeature, CompileError};

/// Force `dynasm!` to use the correct arch (riscv64) when cross-compiling.
macro_rules! dynasm {
    ($a:expr ; $($tt:tt)*) => {
        dynasm::dynasm!(
            $a.inner
            ; .arch riscv64
            ; $($tt)*
        )
    };
}

/// Location abstraction specialized to RISC-V.
pub type Location = AbstractLocation<GPR, FPR>;

/// Branch conditions for RISC-V.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

/// Emitter trait for RISC-V.
#[allow(unused)]
pub trait EmitterRiscv {
    /// Returns the SIMD (FPU) feature if available.
    fn get_simd_arch(&self) -> Option<&CpuFeature>;
    /// Generates a new internal label.
    fn get_label(&mut self) -> Label;
    /// Gets the current code offset.
    fn get_offset(&self) -> Offset;
    /// Returns the size of a jump instruction in bytes.
    fn get_jmp_instr_size(&self) -> u8;

    /// Finalize the function, e.g., resolve labels.
    fn finalize_function(&mut self) -> Result<(), CompileError>;

    // @fix: revert to declaration when any instruction is implemented

    fn emit_add(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_addi(&mut self, rd: Location, rs1: Location, imm: Location) -> Result<(), CompileError>;
    fn emit_sub(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_mul(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_div(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_rem(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_sll(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_slli(&mut self, rd: Location, rs1: Location, shamt: Location) {}
    fn emit_srl(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_srli(&mut self, rd: Location, rs1: Location, shamt: Location) {}
    fn emit_sra(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_srai(&mut self, rd: Location, rs1: Location, shamt: Location) {}
    fn emit_and(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_andi(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_or(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_ori(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_xor(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_xori(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_lui(&mut self, rd: Location, imm: Location) {}
    fn emit_auipc(&mut self, rd: Location, imm: Location) {}
    fn emit_jal(&mut self, rd: Location, imm: Location) {}
    fn emit_jalr(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_beq(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_bne(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_blt(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_bge(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_bltu(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_bgeu(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_lb(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_lh(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_lw(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_ld(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_lbu(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_lhu(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_lwu(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_sb(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_sh(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_sw(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_sd(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_fadd_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fsub_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fmul_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fdiv_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fsqrt_s(&mut self, rd: Location, rs1: Location) {}
    fn emit_fmin_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fmax_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fmv_s(&mut self, rd: Location, rs1: Location) {}
    fn emit_feq_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_flt_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fle_s(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fcvt_w_s(&mut self, rd: Location, rs1: Location) {}
    fn emit_fcvt_s_w(&mut self, rd: Location, rs1: Location) {}
    fn emit_fld(&mut self, rd: Location, rs1: Location, imm: Location) {}
    fn emit_fsd(&mut self, rs1: Location, rs2: Location, imm: Location) {}
    fn emit_fadd_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fsub_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fmul_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fdiv_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fsqrt_d(&mut self, rd: Location, rs1: Location) {}
    fn emit_fmin_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fmax_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fmv_d(&mut self, rd: Location, rs1: Location) {}
    fn emit_feq_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_flt_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fle_d(&mut self, rd: Location, rs1: Location, rs2: Location) {}
    fn emit_fcvt_w_d(&mut self, rd: Location, rs1: Location) {}
    fn emit_fcvt_d_w(&mut self, rd: Location, rs1: Location) {}
    fn emit_ecall(&mut self) {}
    fn emit_ebreak(&mut self) {}
    fn emit_csrrw(&mut self, rd: Location, csr: Location, rs1: Location) {}
    fn emit_csrrs(&mut self, rd: Location, csr: Location, rs1: Location) {}
    fn emit_csrrc(&mut self, rd: Location, csr: Location, rs1: Location) {}
    fn emit_csrrwi(&mut self, rd: Location, csr: Location, imm: Location) {}
    fn emit_csrrsi(&mut self, rd: Location, csr: Location, imm: Location) {}
    fn emit_csrrci(&mut self, rd: Location, csr: Location, imm: Location) {}
}

impl EmitterRiscv for AssemblerRiscv {
    fn get_simd_arch(&self) -> Option<&CpuFeature> {
        self.simd_arch.as_ref()
    }

    fn get_label(&mut self) -> Label {
        self.new_dynamic_label()
    }

    fn get_offset(&self) -> AssemblyOffset {
        self.offset()
    }

    fn get_jmp_instr_size(&self) -> u8 {
        4 // jal/jalr
    }

    fn finalize_function(&mut self) -> Result<(), CompileError> {
        Ok(())
    }

    fn emit_addi(&mut self, rd: Location, rs1: Location, rs2: Location) -> Result<(), CompileError> {
        match (rd, rs1, rs2) {
            // reg-imm: addi rd, rs1, imm
            (Location::GPR(rd), Location::GPR(rs1), Location::Imm32(imm)) => {
                let rd = rd as u8;
                let rs1 = rs1 as u8;
                let imm = imm as i32;

                if imm >= -2048 && imm <= 2047 {
                    // addi for 12-bit signed immediates
                    dynasm!(self
                        ; .feature i
                        ; addi X(rd), X(rs1), imm
                    );

                    Ok(())
                } else {
                    // lui + addi + add sequence + rounding for large immediates
                    let upper = ((imm + 0x800) >> 12) as i32;
                    let lower = (imm & 0xFFF) as i32;
                    // sign-extend lower if negative
                    let lower = if lower > 2047 { lower - 4096 } else { lower };
                    let temp_reg = 31u8; // x31 as scratch register

                    dynasm!(self
                        ; .feature i
                        ; lui X(temp_reg), upper
                        ; addi X(temp_reg), X(temp_reg), lower
                        ; add X(rd), X(rs1), X(temp_reg)
                    );

                    Ok(())
                }
            }

            (_, _, Location::Imm64(_)) => codegen_error!("singlepass 64-bit immediate not supported in addi"),
            (Location::SIMD(_), _, _) => codegen_error!("singlepass no vector locations for addi"),
            (_, Location::SIMD(_), _) => codegen_error!("singlepass no vector locations for addi"),
            (_, _, Location::SIMD(_)) => codegen_error!("singlepass no vector locations for addi"),
            _ => codegen_error!("singlepass unreachable operand combination for addi"),
        }
    }
}
