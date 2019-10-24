/*
 * Copyright (c) 2019 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Denis Vasilìk <rust@denisvasilik.com>
 */

//! Current Program Status Register - CPSR

use register::cpu::{RegisterReadOnly};
use register::{register_bitfields};

register_bitfields! {u32,
    CPSR [
        N OFFSET(31) NUMBITS(1) [],
        Z OFFSET(30) NUMBITS(1) [],
        C OFFSET(29) NUMBITS(1) [],
        V OFFSET(28) NUMBITS(1) [],
        Q OFFSET(27) NUMBITS(1) [],
        IT1 OFFSET(25) NUMBITS(2) [],
        J OFFSET(24) NUMBITS(1) [],
        GE OFFSET(16) NUMBITS(19) [],
        IT0 OFFSET(10) NUMBITS(6) [],
        E OFFSET(9) NUMBITS(1) [],
        A OFFSET(8) NUMBITS(1) [],
        I OFFSET(7) NUMBITS(1) [],
        F OFFSET(6) NUMBITS(1) [],
        T OFFSET(5) NUMBITS(1) [],
        M OFFSET(0) NUMBITS (5) []
    ]
}

pub struct Reg;

impl RegisterReadOnly<u32, CPSR::Register> for Reg {
    sys_coproc_read_raw!(u32, "CPSR");
}

pub static CPSR: Reg = Reg {};
