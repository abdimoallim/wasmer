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
    fn is_callee_save(self) -> bool {
        // TODO: implement callee-save registers for RISC-V.
        todo!()
    }
    fn is_reserved(self) -> bool {
        // TODO: implement reserved registers for RISC-V (e.g., X0 always zero, stack pointer).
        todo!()
    }
    fn into_index(self) -> usize {
        self as usize
    }
    fn from_index(n: usize) -> Result<GPR, ()> {
        // TODO: map index to GPR.
        todo!()
    }
    fn iterator() -> Iter<'static, GPR> {
        // TODO: return an iterator over all GPR variants.
        todo!()
    }
    fn to_dwarf(self) -> u16 {
        // TODO: map register to DWARF register number.
        todo!()
    }
}

impl AbstractReg for FPR {
    fn is_callee_save(self) -> bool {
        // TODO: implement callee-save registers for FPR.
        todo!()
    }
    fn is_reserved(self) -> bool {
        // TODO: implement reserved floating-point registers.
        todo!()
    }
    fn into_index(self) -> usize {
        self as usize
    }
    fn from_index(n: usize) -> Result<FPR, ()> {
        // TODO: map index to FPR.
        todo!()
    }
    fn iterator() -> Iter<'static, FPR> {
        // TODO: return an iterator over all FPR variants.
        todo!()
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
    // TODO: track next GPR/FPR for argument passing.
}

impl ArgumentRegisterAllocator {
    /// Allocates a register for argument type `ty`. Returns `None` if no register is available.
    pub fn next(
        &mut self,
        ty: Type,
        calling_convention: CallingConvention,
    ) -> Result<Option<RiscvRegister>, CompileError> {
        // TODO: implement RISC-V calling convention register allocation.
        todo!()
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
