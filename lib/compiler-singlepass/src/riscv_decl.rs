//! RISC-V structures.

use crate::{
    common_decl::{MachineState, MachineValue, RegisterIndex},
    location::{CombinedRegister, Reg as AbstractReg},
};
use std::{collections::BTreeMap, slice::Iter};
use wasmer_types::target::CallingConvention;
use wasmer_types::{CompileError, Type};

/// General-purpose registers.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum GPR {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30, X31,
}

/// Floating-point registers.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum FPR {
    F0, F1, F2, F3, F4, F5, F6, F7,
    F8, F9, F10, F11, F12, F13, F14, F15,
    F16, F17, F18, F19, F20, F21, F22, F23,
    F24, F25, F26, F27, F28, F29, F30, F31,
}

impl AbstractReg for GPR {
    #[rustfmt::skip]
    fn is_callee_save(self) -> bool {
        matches!(
            self, // s0 - s11, x8 - x9
            GPR::X8 | GPR::X9 | GPR::X18 | GPR::X19 | GPR::X20 | GPR::X21
            | GPR::X22 | GPR::X23 | GPR::X24 | GPR::X25 | GPR::X26 | GPR::X27
        )
    }

    #[rustfmt::skip]
    fn is_reserved(self) -> bool {
        matches!(
            self,
            GPR::X0 |    // zero
            GPR::X2 |    // sp
            GPR::X3 |    // gp
            GPR::X4      // tp
        )
    }

    fn into_index(self) -> usize {
        self as usize
    }

    fn from_index(n: usize) -> Result<GPR, ()> {
        if n <= GPR::X31 as usize {
            // SAFETY: We've checked the bounds and all variants are valid.
            Ok(unsafe { std::mem::transmute(n as u8) })
        } else {
            Err(())
        }
    }

    fn iterator() -> Iter<'static, GPR> {
        #[rustfmt::skip]
        static REGISTERS: [GPR; 32] = [
            GPR::X0, GPR::X1, GPR::X2, GPR::X3, GPR::X4, GPR::X5, GPR::X6, GPR::X7,
            GPR::X8, GPR::X9, GPR::X10, GPR::X11, GPR::X12, GPR::X13, GPR::X14, GPR::X15,
            GPR::X16, GPR::X17, GPR::X18, GPR::X19, GPR::X20, GPR::X21, GPR::X22, GPR::X23,
            GPR::X24, GPR::X25, GPR::X26, GPR::X27, GPR::X28, GPR::X29, GPR::X30, GPR::X31,
        ];

        REGISTERS.iter()
    }

    fn to_dwarf(self) -> u16 {
        // TODO: map register to DWARF register number.
        todo!()
    }
}

impl AbstractReg for FPR {
    #[rustfmt::skip]
    fn is_callee_save(self) -> bool {
        matches!(
            self, // fs0 - fs11, f8 - f9
            FPR::F8 | FPR::F9 | FPR::F18 | FPR::F19 | FPR::F20 | FPR::F21 | 
            FPR::F22 | FPR::F23 | FPR::F24 | FPR::F25 | FPR::F26 | FPR::F27
        )
    }

    fn is_reserved(self) -> bool {
        false
    }

    fn into_index(self) -> usize {
        self as usize
    }

    fn from_index(n: usize) -> Result<FPR, ()> {
        if n <= FPR::F31 as usize {
            Ok(unsafe { std::mem::transmute(n as u8) })
        } else {
            Err(())
        }
    }

    fn iterator() -> Iter<'static, FPR> {
        #[rustfmt::skip]
        static REGISTERS: [FPR; 32] = [
            FPR::F0, FPR::F1, FPR::F2, FPR::F3, FPR::F4, FPR::F5, FPR::F6, FPR::F7,
            FPR::F8, FPR::F9, FPR::F10, FPR::F11, FPR::F12, FPR::F13, FPR::F14, FPR::F15,
            FPR::F16, FPR::F17, FPR::F18, FPR::F19, FPR::F20, FPR::F21, FPR::F22, FPR::F23,
            FPR::F24, FPR::F25, FPR::F26, FPR::F27, FPR::F28, FPR::F29, FPR::F30, FPR::F31,
        ];

        REGISTERS.iter()
    }

    fn to_dwarf(self) -> u16 {
        // TODO: map FPR register to DWARF register number.
        todo!()
    }
}

/// A combined RISC-V register.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RiscvRegister {
    /// General-purpose register.
    GPR(GPR),
    /// Floating-point register.
    FPR(FPR),
}

impl CombinedRegister for RiscvRegister {
    fn to_index(&self) -> RegisterIndex {
        match *self {
            RiscvRegister::GPR(x) => RegisterIndex(x as usize),
            RiscvRegister::FPR(x) => RegisterIndex(x as usize + /* FPR offset */ 0),
        }
    }
    fn from_gpr(x: u16) -> Self {
        RiscvRegister::GPR(GPR::from_index(x as usize).unwrap())
    }
    fn from_simd(x: u16) -> Self {
        RiscvRegister::FPR(FPR::from_index(x as usize).unwrap())
    }
    fn _from_dwarf_regnum(x: u16) -> Option<Self> {
        // TODO: map DWARF register number to RiscvRegister
        None
    }
}

/// Allocator for function argument registers according to the RISC-V ABI.
#[derive(Default)]
pub struct ArgumentRegisterAllocator {
    next_gpr: usize,
    next_fpr: usize,
}

impl ArgumentRegisterAllocator {
    /// Integer argument registers in order (a0-a7).
    const INT_REGS: [GPR; 8] = [
        GPR::X10, GPR::X11, GPR::X12, GPR::X13,
        GPR::X14, GPR::X15, GPR::X16, GPR::X17,
    ];

    /// Floating-point argument registers in order (fa0-fa7).
    const FLOAT_REGS: [FPR; 8] = [
        FPR::F10, FPR::F11, FPR::F12, FPR::F13,
        FPR::F14, FPR::F15, FPR::F16, FPR::F17,
    ];

    /// Allocates a register for argument type `ty`. Returns `None` if no register is available.
    pub fn next(
        &mut self,
        ty: Type,
        calling_convention: CallingConvention,
    ) -> Result<Option<RiscvRegister>, CompileError> {
        match (ty, calling_convention) {
            (Type::I32 | Type::I64, _) => {
                if self.next_gpr < Self::INT_REGS.len() {
                    let reg = Self::INT_REGS[self.next_gpr];
                    self.next_gpr += 1;

                    Ok(Some(RiscvRegister::GPR(reg)))
                } else {
                    Ok(None)
                }
            }

            (Type::F32 | Type::F64, CallingConvention::WasmBasicCAbi) => {
                if self.next_fpr < Self::FLOAT_REGS.len() {
                    let reg = Self::FLOAT_REGS[self.next_fpr];
                    self.next_fpr += 1;

                    Ok(Some(RiscvRegister::FPR(reg)))
                } else {
                    Ok(None)
                }
            }

            // @todo: structs to registers (2*XLEN bits max)
            //        XLEN is 64 for riscv64

            _ => Err(CompileError::UnsupportedABI(ty.to_string(), calling_convention)),
        }
    }
}

/// Create a new `MachineState` with default values for RISC-V.
pub fn new_machine_state() -> MachineState {
    MachineState {
        stack_values: vec![],
        register_values: vec![MachineValue::Undefined; /* GPR+FPR count */ 0],
        prev_frame: BTreeMap::new(),
        wasm_stack: vec![],
        wasm_inst_offset: usize::MAX,
    }
}
