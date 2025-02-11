use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::eforth;

/// * `CORE_SIZE` is the total number of cells addressable by the virtual machine
const CORE_SIZE: usize = 0x8000;
/// * `SP0` is the starting point of the variable stack
const SP0: u16 = 0x2200;
/// * `RP0` is the starting point of the return stack
const RP0: u16 = 0x7fff;

// Constants for CPU Instructions
const INSTRUCTION_LIT: u16 = 0x8000;
const INSTRUCTION_ALU: u16 = 0x6000;
const INSTRUCTION_CALL: u16 = 0x4000;
const INSTRUCTION_ZBRANCH: u16 = 0x2000;
const INSTRUCTION_BRANCH: u16 = 0x0000;

/// Mask to identify instruction bits
const MASK_INSTRUCTION: u16 = 0xe000;
/// Mask to identify ALU opCodes
const MASK_ALU_OPS: u16 = 0x1f00;
/// Mask to identify target address bits
const MASK_TARGET: u16 = 0x1fff;

// Constants for ALU opcodes
const OPCODE_NOP: u16 = 0x0000;
const OPCODE_T2N: u16 = 0x0100;
const OPCODE_R2T: u16 = 0x0200;
const OPCODE_M2T: u16 = 0x0300;
const OPCODE_N2T: u16 = 0x0400;
const OPCODE_T_PLUS_N: u16 = 0x0500;
const OPCODE_T_MUL_N: u16 = 0x0600;
const OPCODE_T_AND_N: u16 = 0x0700;
const OPCODE_T_OR_N: u16 = 0x0800;
const OPCODE_T_XOR_N: u16 = 0x0900;
const OPCODE_NOT_T: u16 = 0x0a00;
const OPCODE_SUB_T: u16 = 0x0b00;
const OPCODE_T_EQ_Z: u16 = 0x0c00;
const OPCODE_T_EQ_N: u16 = 0x0d00;
const OPCODE_N_ULT_T: u16 = 0x0e00;
const OPCODE_N_LT_T: u16 = 0x0f00;
const OPCODE_N_RSHIFT_T: u16 = 0x1000;
const OPCODE_N_LSHIFT_T: u16 = 0x1100;
const OPCODE_SP_DEPTH: u16 = 0x1200;
const OPCODE_RP_DEPTH: u16 = 0x1300;
const OPCODE_SP_SET: u16 = 0x1400;
const OPCODE_RP_SET: u16 = 0x1500;
const OPCODE_SAVE: u16 = 0x1600;
const OPCODE_TX: u16 = 0x1700;
const OPCODE_RX: u16 = 0x1800;
const OPCODE_UM_DIV_MOD: u16 = 0x1900;
const OPCODE_DIV_MOD: u16 = 0x1a00;
const OPCODE_BYE: u16 = 0x1b00;

/// `fputc` writes a single character of output to a file, and returns
/// all bits set on an error. It emulates the C function of the same name,
/// and is not a recommended way to output data in Rust, but is required for
/// the VM.
///
/// # Arguments
///
/// * `output`  - Output stream to write to
/// * `t`       - Single byte to write
///
/// # Returns
///
/// This function returns `t` on success and `0xffff` on error
///
fn fputc(output: &mut dyn Write, t: u8) -> u16 {
    let u: [u8; 1] = [t];
    if 1 == output.write(&u).unwrap() {
        t as u16
    } else {
        0xffff
    }
}

/// `fputc` gets a single character from an input stream, like the C function
/// with the same name, it returns all bits set (-1) on error. This is not a
/// very idiomatic way of doing things from a Rust point of view, but this
/// function is used by the virtual machine to get input, and it expects
/// errors to be signaled in the message.
///
/// # Arguments
///
/// `input` - Input stream to read from
///
/// # Returns
///
/// This function returns a single byte on success in the lower half a
/// 16-bit value, and all bits set (or `0xffff`) on failure.
fn fgetc(input: &mut dyn Read) -> u16 {
    let mut u: [u8; 1] = [0];
    if 1 == input.read(&mut u).unwrap() {
        u[0] as u16
    } else {
        0xffff
    }
}

/// # Embed Virtual Machine in Rust
///
/// * LICENSE:    MIT
/// * AUTHOR:     Richard James Howe
/// * COPYRIGHT:  Richard James Howe (2018, 2020)
/// * CONTACT:    <howe.r.j.89@gmail.com>
/// * REPOSITORY: <https://github.com/howerj/rust-embed-forth>
///
/// This project implements a 16-bit dual stack virtual machine (VM) tailored to
/// execute Forth, it should also come with an image which this VM can run,
/// which will be in a separate file. The VM is not that robust and incorrect
/// code that overflows the stack might cause a panic.
///
/// The original C VM is available at <https://github.com/howerj/embed>, along
/// with more up to date VM images (and perhaps even a more slightly up to date
/// virtual machine architecture).
///
/// * TODO: Implement Index trait for u16?
pub struct VM {
    /// `tracing` can be set true to enable logging, logging is very verbose
    tracing: bool,
    /// `count` is the number instructions executed so far, it is only updated
    /// if tracing is on.
    count: u64,
    /// `pc` Program counter.
    pc: u16,
    /// `rp`Return stack pointer.
    rp: u16,
    /// `sp` Data stack pointer.
    sp: u16,
    /// `t` Top of stack pointer.
    t: u16,
    /// `core` contains the program, data, and both stacks which index
    /// into `core` with `rp` and `sp`
    //#[derive(Copy, Clone)]
    core: [u16; CORE_SIZE],
}

#[allow(dead_code)]
impl VM {
    /// `new` constructs a new virtual machine image that can be passed to `run`
    /// straight away, as the program memory is copied from a default image
    /// that contains an eForth interpreter.
    pub fn new() -> Self {
        let mut r = VM {
            tracing: false,
            count: 0,
            pc: 0,
            rp: RP0,
            sp: SP0,
            t: 0,
            core: [0; CORE_SIZE],
        };

        for i in 0..eforth::EFORTH_CORE.len() {
            r.core[i] = eforth::EFORTH_CORE[i];
        }
        r
    }

    /// `reset` sets the VMs registers back to their defaults, it does not
    /// reset the program memory or the stack contents, but the stack pointers,
    /// top of stack register, and the program counter.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.t = 0;
        self.rp = RP0;
        self.sp = SP0;
    }

    /// Turns logging on/off, capturing each VM instructions execution
    ///
    /// # Arguments
    ///
    /// * `state` - Turn _very_ verbose tracing on/off, each instruction is logged to stderr in CSV format
    ///
    pub fn trace(&mut self, state: bool) {
        self.tracing = state;
    }

    /// `run` executes the virtual machine on the currently loaded program
    /// in `core`. The specification for the virtual machine is too long
    /// for this document, but visit <https://github.com/howerj/embed> for
    /// more documentation. This virtual machine and the image on it it likely
    /// to be out of date and incompatible with the version taken from original
    /// repository however.
    ///
    /// # Arguments
    ///
    /// * `input`  - Input file to read from
    /// * `output` - Output file to write to
    /// * `block`  - Optional name of file to write sections of memory to
    ///
    /// # Returns
    ///
    /// This function returns an error code suitable for use with
    /// `std::process:exit()`, negative values usually indicate failure, however
    /// any semantics attached to this number are entirely by convention only,
    /// the program running in the virtual machine can return any number it likes.
    ///
    /// # Example
    ///
    /// The following example loads code into a new VM instance from file
    /// *eforth.blk*, and sets input and output to the standard streams.
    /// The save opcode will save to the file *new.blk*. What the VM will
    /// do depends entirely on the code in the *eforth.blk* file.
    ///
    /// ```
    /// extern crate embed;
    /// use std::fs::File;
    /// use std::path::Path;
    ///
    /// let mut evm = embed::VM::new();
    /// let mut file = File::open(&Path::new("vm.blk")).unwrap();
    /// evm.load(&mut file);
    /// evm.run(Some("new.blk"), &mut std::io::stdin(), &mut std::io::stdout());
    /// ```
    ///
    pub fn run(
        &mut self,
        block: Option<&str>,
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> i32 {
        let (mut pc, mut rp, mut sp, mut t) = (self.pc, self.rp, self.sp, self.t);
        let mut d: u32;
        let mut m = self.core;

        self.header(&mut std::io::stderr());

        'eval: loop {
            let instruction = m[pc as usize];
            const DELTA: [u16; 4] = [0, 1, 0xfffe, 0xffff];

            VM::csv(self, &mut std::io::stderr(), pc, instruction, t, sp, rp);

            if INSTRUCTION_LIT & instruction == INSTRUCTION_LIT {
                /* literal */
                sp += 1;
                m[sp as usize] = t;
                t = instruction & 0x7fff;
                pc += 1;
            } else if MASK_INSTRUCTION & instruction == INSTRUCTION_ALU {
                /* ALU */
                let mut tp = t;
                let mut n = m[sp as usize];
                pc = if instruction & 0x10 == 0x10 {
                    m[rp as usize] >> 1
                } else {
                    pc + 1
                };

                let alu = instruction & MASK_ALU_OPS;
                match alu {
                    OPCODE_NOP => { /* tp = t */ }
                    OPCODE_T2N => tp = n,
                    OPCODE_R2T => tp = m[rp as usize],
                    OPCODE_M2T => tp = m[(t >> 1) as usize],
                    OPCODE_N2T => {
                        m[(t >> 1) as usize] = n;
                        sp -= 1;
                        tp = m[sp as usize]
                    }
                    OPCODE_T_PLUS_N => {
                        d = (t as u32) + (n as u32);
                        tp = (d >> 16) as u16;
                        m[sp as usize] = d as u16;
                        n = d as u16
                    }
                    OPCODE_T_MUL_N => {
                        d = (t as u32) * (n as u32);
                        tp = (d >> 16) as u16;
                        m[sp as usize] = d as u16;
                        n = d as u16
                    }
                    OPCODE_T_AND_N => tp &= n,
                    OPCODE_T_OR_N => tp |= n,
                    OPCODE_T_XOR_N => tp ^= n,
                    OPCODE_NOT_T => tp = !t,
                    OPCODE_SUB_T => tp = tp.wrapping_sub(1),
                    OPCODE_T_EQ_Z => tp = if t == 0 { 0xffff } else { 0 },
                    OPCODE_T_EQ_N => tp = if t == n { 0xffff } else { 0 },
                    OPCODE_N_ULT_T => tp = if n < t { 0xffff } else { 0 },
                    OPCODE_N_LT_T => tp = if (n as i16) < (t as i16) { 0xffff } else { 0 },
                    OPCODE_N_RSHIFT_T => tp = n >> t,
                    OPCODE_N_LSHIFT_T => tp = n << t,
                    OPCODE_SP_DEPTH => tp = sp << 1,
                    OPCODE_RP_DEPTH => tp = rp << 1,
                    OPCODE_SP_SET => sp = t >> 1,
                    OPCODE_RP_SET => {
                        rp = t >> 1;
                        tp = n
                    }
                    OPCODE_SAVE => {
                        tp = self.save_file(block, n >> 1, (((t as u32) + 1) >> 1) as u16)
                    }
                    OPCODE_TX => tp = fputc(output, t as u8),
                    OPCODE_RX => tp = fgetc(input),
                    OPCODE_UM_DIV_MOD => {
                        if t != 0 {
                            tp = n / t;
                            t = n % t;
                            n = t
                        } else {
                            pc = 1;
                            tp = 10
                        }
                    }
                    OPCODE_DIV_MOD => {
                        if t != 0 {
                            tp = ((n as i16) / (t as i16)) as u16;
                            t = ((n as i16) % (t as i16)) as u16;
                            n = t
                        } else {
                            pc = 1;
                            tp = 10
                        }
                    }
                    OPCODE_BYE => {
                        break 'eval;
                    }
                    _ => {
                        panic!("Invalid Opcode 0x{:4x},", alu)
                    }
                }

                sp = sp.wrapping_add(DELTA[(instruction & 0x3) as usize]);
                rp = rp.wrapping_sub(DELTA[((instruction >> 2) & 0x3) as usize]);
                if instruction & 0x20 == 0x20 {
                    tp = n;
                }
                if instruction & 0x40 == 0x40 {
                    m[rp as usize] = t
                }
                if instruction & 0x80 == 0x80 {
                    m[sp as usize] = t
                }
                t = tp;
            } else if MASK_INSTRUCTION & instruction == INSTRUCTION_CALL {
                /* call */
                rp -= 1;
                m[rp as usize] = (pc + 1) << 1;
                pc = instruction & MASK_TARGET;
            } else if MASK_INSTRUCTION & instruction == INSTRUCTION_ZBRANCH {
                /* 0branch */
                pc = if t == 0 {
                    instruction & MASK_TARGET
                } else {
                    pc + 1
                };
                t = m[sp as usize];
                sp -= 1;
            } else if MASK_INSTRUCTION & instruction == INSTRUCTION_BRANCH {
                /* branch */
                pc = instruction & MASK_TARGET;
            } else {
                panic!("Invalid instruction {}", instruction);
            }
        }

        self.pc = pc;
        self.rp = rp;
        self.sp = sp;
        self.t = t;

        (t as i16) as i32
    }

    /// Print a header for a CSV file trace, if tracing is enabled, the output should be consumable
    /// by the utility <https://github.com/carlos-jenkins/csv2vcd> which can turn a CSV file into
    /// a VCD (Value Change Dump) file. This file can be used with a suitable waveform viewer, such
    /// as GTKWave <http://gtkwave.sourceforge.net/> for debugging purposes.
    ///
    fn header(&self, output: &mut dyn Write) {
        if !self.tracing {
            return;
        }
        let _ignore = writeln!(
            output,
            "\"pc[15:0]\",\"instruction[15:0]\",\"t[15:0]\",\"sp[7:0]\",\"rp[7:0]\",\"TIME\""
        );
    }

    /// `csv` is used by `run` to output a CSV file with one line per instruction cycle,
    /// it is for internal use only. Tracing has to be enabled and is off by default as it
    /// produces a lot of output. The output should be compatible with the tool csv2vcd
    /// [csv2vcd](https://github.com/carlos-jenkins/csv2vcd) and which can be viewed with
    /// [GTKWave](http://gtkwave.sourceforge.net/), which should aid in analyzing the copious
    /// amounts of data produced.
    ///
    /// It should be noted that `csv` accepts the arguments it will print instead of printing
    /// out the values stored in `self`, as the value for the VM state such as the program
    /// counter and stack pointers are kept in locals until `run` returns, and only then are
    /// they updated.
    ///
    /// Arguments are logged in order, `pc` being the left most field in a record line and
    /// `rp` the rightmost (of the values passed in, the rightmost field is actually a "time"
    /// field, needed for the VCD format).
    ///
    /// # Arguments
    ///
    /// * `output`       - output stream to log to
    /// * `pc`           - the program counter
    /// * `instruction`  - the current instruction being executed, or `self->core[pc]`
    /// * `t`            - top of stack register
    /// * `sp`           - variable stack pointer, index into `core`
    /// * `rp`           - return stack pointer, index into `core`
    ///
    ///
    fn csv(&mut self, output: &mut dyn Write, pc: u16, instruction: u16, t: u16, sp: u16, rp: u16) {
        if !self.tracing {
            return;
        }
        let time = if self.count == 0 { "s" } else { "ns" };
        let _ignore = writeln!(
            output,
            "{}",
            format!(
                "{:04x},{:04x},{:04x},{:02x},{:02x},{}{}",
                pc, instruction, t, sp, rp, self.count, time
            )
        );
        self.count += 1;
    }

    /// `save_file` is for internal use only, as it converts any errors into results understandable
    /// by the virtual machine. Its purpose is to save optionally save
    fn save_file(&self, block: Option<&str>, start: u16, length: u16) -> u16 {
        let name = match block {
            None => return 0xffff,
            Some(name) => name,
        };

        let mut file = match File::create(Path::new(name)) {
            Err(r) => {
                println!("failed to create block \"{}\": {}", name, r);
                return 0xffff;
            }
            Ok(r) => r,
        };

        self.save_block(&mut file, start, length).unwrap_or(0xffff)
    }

    fn save_block(&self, block: &mut dyn Write, start: u16, length: u16) -> Option<u16> {
        if ((start as u32) + (length as u32)) > 0xffff {
            return None;
        }

        for i in start..length {
            let lo = self.core[i as usize] as u8;
            let hi = (self.core[i as usize] >> 8) as u8;
            let u: [u8; 2] = [lo, hi];
            if let Err(r) = block.write(&u) {
                let _ignore = r;
                return None;
            }
        }
        Some(0)
    }

    /// `save` the virtual machine to a sink, this saves the program/data
    /// core but none of the registers.
    ///
    /// # Arguments
    ///
    /// `output` - Output sink to write to, usually a file
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use std::path::Path;
    /// let mut vm = embed::VM::new();
    /// let mut output = File::create(&Path::new("vm.blk")).unwrap();
    /// vm.save(&mut output);
    /// ```
    ///
    /// TODO: Replace Option with proper Result return value
    pub fn save(&self, output: &mut dyn Write) -> Option<u16> {
        self.save_block(output, 0, CORE_SIZE as u16)
    }

    /// `load` the virtual machine from a source, this also reinitializes
    /// the VM registers to their default values
    ///
    /// # Arguments
    ///
    /// * `input` - Input source to read from containing core code
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use std::path::Path;
    /// let mut vm = embed::VM::new();
    /// let mut input = File::open(&Path::new("vm.blk")).unwrap();
    /// vm.load(&mut input);
    /// ```
    ///
    /// TODO: Replace Option with proper Result return value
    pub fn load(&mut self, input: &mut dyn Read) -> Option<u16> {
        let mut i = 0_u16;
        self.reset();
        while i < (CORE_SIZE as u16) {
            let lo = fgetc(input);
            let hi = fgetc(input);
            if lo == 0xffff || hi == 0xffff {
                return Some(i);
            }
            self.core[i as usize] = lo | (hi << 8);
            i += 1
        }
        Some(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    const BYE: u16 = 0x7b00;
    const ADD: u16 = 0x6523;
    const DEC: u16 = 0x6B00;

    fn literal(l: u16) -> u16 {
        if l & 0x8000 == 0x8000 {
            panic!("invalid literal {} > 0x7fff", format!("{}", l))
        };
        l | 0x8000
    }

    fn core(dst: &mut [u16], src: &[u16]) {
        let len = cmp::min(src.len(), dst.len());
        for i in 0..len {
            dst[i] = src[i]
        }
    }

    fn expect(vm: &mut VM, val: i32, program: &[u16]) {
        let (mut input, mut output) = (std::io::stdin(), std::io::stdout());
        core(&mut vm.core, program);
        assert_eq!(vm.run(None, &mut input, &mut output), val);
        vm.reset();
    }

    #[test]
    fn run() {
        let mut vm = VM::new();

        expect(&mut vm, 99, &[literal(99), BYE]);
        expect(&mut vm, 54, &[literal(55), DEC, BYE]);
        expect(&mut vm, 4, &[literal(2), literal(2), ADD, BYE]);
    }
}
