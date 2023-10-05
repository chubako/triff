pub fn check_permissions(mode1: u32, mode2: u32) -> bool {
    let permissions1 = mode1 & !libc::S_IFMT;
    let permissions2 = mode2 & !libc::S_IFMT;
    permissions1==permissions2
}
