use crate::*;

extern "C" {
    fn arch_syscall(
        syscall: BrSyscall,
        arg1: BrArg,
        arg2: BrArg,
        arg3: BrArg,
        arg4: BrArg,
        arg5: BrArg,
    ) -> BrResult;
}

pub(crate) extern "C" fn syscall(
    syscall: BrSyscall,
    arg1: BrArg,
    arg2: BrArg,
    arg3: BrArg,
    arg4: BrArg,
    arg5: BrArg,
) -> BrResult {
    unsafe { arch_syscall(syscall, arg1, arg2, arg3, arg4, arg5) }
}
