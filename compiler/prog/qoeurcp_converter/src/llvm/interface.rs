use super::util::{cstring, cstring_mut, cstring_mut_mut};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::os::raw::{c_char, c_uint};
use std::ptr;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::*;

pub fn get_ty_kind(ty: LLVMTypeRef) -> LLVMTypeKind {
  unsafe { LLVMGetTypeKind(ty) }
}

pub fn host_cpu_name() -> *mut c_char {
  unsafe { LLVMGetHostCPUName() }
}

pub fn host_cpu_features() -> *mut c_char {
  unsafe { LLVMGetHostCPUFeatures() }
}

pub fn type_of(val: LLVMValueRef) -> LLVMTypeRef {
  unsafe { LLVMTypeOf(val) }
}

pub fn add_incoming(
  phi_node: LLVMValueRef,
  incoming_values: *mut LLVMValueRef,
  incoming_blocks: *mut LLVMBasicBlockRef,
  count: c_uint,
) {
  unsafe { LLVMAddIncoming(phi_node, incoming_values, incoming_blocks, count) }
}

pub fn dump_ty(value: LLVMTypeRef) {
  unsafe { LLVMDumpType(value) }
}

pub fn dump_value(value: LLVMValueRef) {
  unsafe { LLVMDumpValue(value) }
}

pub fn init_all_targets() {
  unsafe { LLVM_InitializeAllTargets() }
}

pub fn init_all_asm_printers() {
  unsafe { LLVM_InitializeAllAsmPrinters() }
}

pub fn move_basic_block_before(
  block: LLVMBasicBlockRef,
  move_pos: LLVMBasicBlockRef,
) {
  unsafe { LLVMMoveBasicBlockBefore(block, move_pos) }
}

pub fn move_basic_block_after(
  block: LLVMBasicBlockRef,
  move_pos: LLVMBasicBlockRef,
) {
  unsafe { LLVMMoveBasicBlockAfter(block, move_pos) }
}

pub fn make_fun_ty(
  ret_ty: LLVMTypeRef,
  param_ty: *mut LLVMTypeRef,
  param_count: c_uint,
  is_var_arg: LLVMBool,
) -> LLVMTypeRef {
  unsafe { LLVMFunctionType(ret_ty, param_ty, param_count, is_var_arg) }
}

pub fn make_pointer_ty(
  elmt_ty: LLVMTypeRef,
  addr_space: c_uint,
) -> LLVMTypeRef {
  unsafe { LLVMPointerType(elmt_ty, addr_space) }
}

pub fn position_at_end(builder: LLVMBuilderRef, block: LLVMBasicBlockRef) {
  unsafe { LLVMPositionBuilderAtEnd(builder, block) }
}

pub fn dispose_builder(builder: LLVMBuilderRef) {
  unsafe { LLVMDisposeBuilder(builder) }
}

pub fn dispose_module(module: LLVMModuleRef) {
  unsafe { LLVMDisposeModule(module) }
}

pub fn dispose_context(context: LLVMContextRef) {
  unsafe { LLVMContextDispose(context) }
}

pub fn dispose_message(msg: &str) {
  unsafe { LLVMDisposeMessage(cstring_mut!(msg)) }
}

pub fn print_file_to(
  module: LLVMModuleRef,
  name: &str,
  error: &str,
) -> LLVMBool {
  unsafe {
    LLVMPrintModuleToFile(module, cstring!(name), cstring_mut_mut!(error))
  }
}

pub fn add_fun(
  module: LLVMModuleRef,
  name: &str,
  ty: LLVMTypeRef,
) -> LLVMValueRef {
  unsafe { LLVMAddFunction(module, cstring!(name), ty) }
}

pub fn make_context_module_with_name(
  context: LLVMContextRef,
  name: &str,
) -> LLVMModuleRef {
  unsafe { LLVMModuleCreateWithNameInContext(cstring!(name), context) }
}

pub fn next_basic_block(block: LLVMBasicBlockRef) -> LLVMBasicBlockRef {
  unsafe { LLVMGetNextBasicBlock(block) }
}

pub fn last_basic_block(value: LLVMValueRef) -> LLVMBasicBlockRef {
  unsafe { LLVMGetLastBasicBlock(value) }
}

pub fn make_context_append_basic_block(
  context: LLVMContextRef,
  value: LLVMValueRef,
) -> LLVMBasicBlockRef {
  unsafe {
    LLVMAppendBasicBlockInContext(context, value, cstring!("appendbtmp"))
  }
}

pub fn make_context_insert_basic_block(
  context: LLVMContextRef,
  block: LLVMBasicBlockRef,
) -> LLVMBasicBlockRef {
  unsafe {
    LLVMInsertBasicBlockInContext(context, block, cstring!("insertbtmp"))
  }
}

pub fn dispose_target_machine(target_machine: LLVMTargetMachineRef) {
  unsafe { LLVMDisposeTargetMachine(target_machine) }
}

pub fn emit_target_machine_to_file(
  target_machine: LLVMTargetMachineRef,
  module: LLVMModuleRef,
  filename: &str,
  codegen: LLVMCodeGenFileType,
  error: &str,
) -> LLVMBool {
  unsafe {
    LLVMTargetMachineEmitToFile(
      target_machine,
      module,
      cstring_mut!(filename),
      codegen,
      cstring_mut_mut!(error),
    )
  }
}

pub fn find_target_from_triple(
  triple: &str,
  target: *mut LLVMTargetRef,
  error: &str,
) -> LLVMBool {
  unsafe {
    LLVMGetTargetFromTriple(cstring!(triple), target, cstring_mut_mut!(error))
  }
}

pub fn make_target_machine(
  target: LLVMTargetRef,
  triple: &str,
  cpu: &str,
  features: &str,
  level: LLVMCodeGenOptLevel,
  reloc: LLVMRelocMode,
  code_model: LLVMCodeModel,
) -> LLVMTargetMachineRef {
  unsafe {
    LLVMCreateTargetMachine(
      target,
      cstring!(triple),
      cstring!(cpu),
      cstring!(features),
      LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
      LLVMRelocMode::LLVMRelocDefault,
      LLVMCodeModel::LLVMCodeModelDefault,
    )
  }
}

pub fn make_build_alloca(
  builder: LLVMBuilderRef,
  ty: LLVMTypeRef,
  name: &str,
) -> LLVMValueRef {
  unsafe { LLVMBuildAlloca(builder, ty, cstring!(name)) }
}

pub fn make_build_block(
  builder: LLVMBuilderRef,
  dest: LLVMBasicBlockRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildBr(builder, dest) }
}

pub fn make_build_condition_block(
  builder: LLVMBuilderRef,
  condition: LLVMValueRef,
  consequence: LLVMBasicBlockRef,
  alternative: LLVMBasicBlockRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildCondBr(builder, condition, consequence, alternative) }
}

pub fn make_build_global_string(
  builder: LLVMBuilderRef,
  value: &str,
  name: &str,
) -> LLVMValueRef {
  unsafe { LLVMBuildGlobalString(builder, cstring!(value), cstring!(name)) }
}

pub fn make_build_lt_cmp(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildFCmp(
      builder,
      LLVMRealPredicate::LLVMRealULT,
      lhs,
      rhs,
      cstring!("lttmp"),
    )
  }
}

pub fn make_build_le_cmp(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildFCmp(
      builder,
      LLVMRealPredicate::LLVMRealULE,
      lhs,
      rhs,
      cstring!("letmp"),
    )
  }
}

pub fn make_build_gt_cmp(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildFCmp(
      builder,
      LLVMRealPredicate::LLVMRealUGT,
      lhs,
      rhs,
      cstring!("gttmp"),
    )
  }
}

pub fn make_build_ge_cmp(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildFCmp(
      builder,
      LLVMRealPredicate::LLVMRealUGE,
      lhs,
      rhs,
      cstring!("getmp"),
    )
  }
}

pub fn make_build_eq_cmp(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildFCmp(
      builder,
      LLVMRealPredicate::LLVMRealUEQ,
      lhs,
      rhs,
      cstring!("eqtmp"),
    )
  }
}

pub fn make_build_ne_cmp(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildFCmp(
      builder,
      LLVMRealPredicate::LLVMRealUNE,
      lhs,
      rhs,
      cstring!("netmp"),
    )
  }
}

pub fn make_build_phi_value(
  builder: LLVMBuilderRef,
  ty: LLVMTypeRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildPhi(builder, ty, cstring!("phitmp")) }
}

pub fn make_build_and(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildAnd(builder, lhs, rhs, cstring!("andtmp")) }
}

pub fn make_build_or(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildOr(builder, lhs, rhs, cstring!("ortmp")) }
}

pub fn make_build_call(
  builder: LLVMBuilderRef,
  llvm_fun: LLVMValueRef,
  param_tys: &mut Vec<LLVMValueRef>,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildCall(
      builder,
      llvm_fun,
      param_tys.as_mut_ptr(),
      param_tys.len() as u32,
      cstring!("calltmp"),
    )
  }
}

pub fn make_build_extract_elmt(
  builder: LLVMBuilderRef,
  values: LLVMValueRef,
  index: LLVMValueRef,
  name: &str,
) -> LLVMValueRef {
  unsafe { LLVMBuildExtractElement(builder, values, index, cstring!(name)) }
}

pub fn make_build_insert_elmt(
  builder: LLVMBuilderRef,
  arg_value: LLVMValueRef,
  elmt_value: LLVMValueRef,
  index: c_uint,
  name: &str,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildInsertValue(builder, arg_value, elmt_value, index, cstring!(name))
  }
}

pub fn make_build_load(
  builder: LLVMBuilderRef,
  pointer_value: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildLoad(builder, pointer_value, cstring!("loadtmp")) }
}

pub fn make_build_store(
  builder: LLVMBuilderRef,
  value: LLVMValueRef,
  pointer: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildStore(builder, value, pointer) }
}

pub fn make_build_binop_add_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildAdd(builder, lhs, rhs, cstring!("addtmp")) }
}

pub fn make_build_binop_div_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildUDiv(builder, lhs, rhs, cstring!("divtmp")) }
}

pub fn make_build_binop_mul_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildMul(builder, lhs, rhs, cstring!("multmp")) }
}

pub fn make_build_binop_rem_unsigned_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildSRem(builder, lhs, rhs, cstring!("sremtmp")) }
}

pub fn make_build_binop_rem_signed_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildURem(builder, lhs, rhs, cstring!("uremtmp")) }
}

pub fn make_build_binop_sub_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildSub(builder, lhs, rhs, cstring!("subtmp")) }
}

pub fn make_build_ret(
  builder: LLVMBuilderRef,
  value: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildRet(builder, value) }
}

pub fn make_build_ret_void(
  builder: LLVMBuilderRef,
  value: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildRetVoid(builder) }
}

pub fn make_const_int(ty: LLVMTypeRef, int: &i64) -> LLVMValueRef {
  unsafe { LLVMConstInt(ty, *int as u64, 0) }
}

pub fn make_const_int_value(
  context: LLVMContextRef,
  int: &i64,
) -> LLVMValueRef {
  unsafe {
    let int_ty = LLVMInt64TypeInContext(context);
    LLVMConstInt(int_ty, *int as u64, 0)
  }
}

pub fn make_const_real(ty: LLVMTypeRef, val: &f64) -> LLVMValueRef {
  unsafe { LLVMConstReal(ty, *val) }
}

pub fn make_const_real_value(
  context: LLVMContextRef,
  real: &f64,
) -> LLVMValueRef {
  unsafe {
    let real_ty = LLVMFloatTypeInContext(context);
    LLVMConstReal(real_ty, *real)
  }
}

pub fn make_context_double_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMDoubleTypeInContext(context) }
}

pub fn make_context_float_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMFloatTypeInContext(context) }
}

pub fn make_context_int1_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt1TypeInContext(context) }
}

pub fn make_context_int8_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt8TypeInContext(context) }
}

pub fn make_context_int16_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt16TypeInContext(context) }
}

pub fn make_context_int32_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt32TypeInContext(context) }
}

pub fn make_context_int64_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt64TypeInContext(context) }
}

pub fn make_context_struct_ty(
  context: LLVMContextRef,
  tys: &mut Vec<LLVMTypeRef>,
) -> LLVMTypeRef {
  unsafe {
    LLVMStructTypeInContext(context, tys.as_mut_ptr(), tys.len() as c_uint, 0)
  }
}

pub fn make_context_void_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMVoidTypeInContext(context) }
}
