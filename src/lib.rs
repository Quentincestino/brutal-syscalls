#![feature(type_ascription)]
#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "riscv")]
pub mod riscv;

#[repr(C)]
pub enum BrResult {
    Success,
    Interrupted,
    Timeout,
    AlreadyBinded,
    AlreadyAck,
    NotBinded,
    NotPermitted,
    BadAddress,
    BadArguments,
    BadHandle,
    BadId,
    BadSyscall,
    NotImplemented,
    OutOfMemory,
    ChannelFull,
    ChannelEmpty,
    WouldBlock,
    BrResultCount,
}

#[repr(C)]
pub enum BrSyscall {
    Log,
    Now,
    Map,
    Unmap,
    Create,
    Dup,
    Start,
    Exit,
    Ipc,
    Drop,
    Close,
    Bind,
    Unbind,
    Ack,
    Inspect,
    In,
    Out,
    BrSyscallCount,
}

pub type BrArg = u64;

fn br_syscall(
    syscall: BrSyscall,
    arg1: BrArg,
    arg2: BrArg,
    arg3: BrArg,
    arg4: BrArg,
    arg5: BrArg,
) -> BrResult {
    #[cfg(target_arch = "x86_64")]
    return crate::x86_64::syscall(syscall, arg1, arg2, arg3, arg4, arg5);

    #[cfg(target_arch = "riscv")]
    crate::riscv::syscall(syscall, arg1, arg2, arg3, arg4, arg5)
}

pub fn br_log(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Log, args, 0, 0, 0, 0)
}

pub fn br_now(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Now, args, 0, 0, 0, 0)
}

pub fn br_map(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Map, args, 0, 0, 0, 0)
}

pub fn br_unmap(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Unmap, args, 0, 0, 0, 0)
}

pub fn br_create(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Create, args, 0, 0, 0, 0)
}

pub fn br_dup(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Dup, args, 0, 0, 0, 0)
}

pub fn br_start(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Start, args, 0, 0, 0, 0)
}

pub fn br_exit(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Exit, args, 0, 0, 0, 0)
}

pub fn br_ipc(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Ipc, args, 0, 0, 0, 0)
}

pub fn br_drop(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Drop, args, 0, 0, 0, 0)
}

pub fn br_close(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Close, args, 0, 0, 0, 0)
}

pub fn br_bind(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Bind, args, 0, 0, 0, 0)
}

pub fn br_unbind(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Unbind, args, 0, 0, 0, 0)
}

pub fn br_ack(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Ack, args, 0, 0, 0, 0)
}

pub fn br_inspect(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Inspect, args, 0, 0, 0, 0)
}

pub fn br_in(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::In, args, 0, 0, 0, 0)
}

pub fn br_out(args: BrArg) -> BrResult {
    br_syscall(BrSyscall::Out, args, 0, 0, 0, 0)
}
