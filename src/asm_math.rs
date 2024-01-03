#![feature(const_float_bits_conv)]
#![feature(asm_const)]

use core::arch::asm;
use cortex_m_semihosting::hprintln;

const ITERATIONS: usize = 10; // Max = 16
const TWO: f32 = 2.0;
const PI: f32 = f32::from_bits(0x40490FDB);
const K_N: [f32; 16] = [
    f32::from_bits(0x3f3504f3),
    f32::from_bits(0x3f21e89b),
    f32::from_bits(0x3f1d130e),
    f32::from_bits(0x3f1bdc8a),
    f32::from_bits(0x3f1b8ed6),
    f32::from_bits(0x3f1b7b68),
    f32::from_bits(0x3f1b768c),
    f32::from_bits(0x3f1b7555),
    f32::from_bits(0x3f1b7507),
    f32::from_bits(0x3f1b74f4),
    f32::from_bits(0x3f1b74ef),
    f32::from_bits(0x3f1b74ee),
    f32::from_bits(0x3f1b74ee),
    f32::from_bits(0x3f1b74ee),
    f32::from_bits(0x3f1b74ee),
    f32::from_bits(0x3f1b74ee),
];
const ANGLES: [f32; 16] = [
    f32::from_bits(0x3f49_0fdb),
    f32::from_bits(0x3eed_6338),
    f32::from_bits(0x3e7a_dbb0),
    f32::from_bits(0x3dfe_add5),
    f32::from_bits(0x3d7f_aade),
    f32::from_bits(0x3cff_eaae),
    f32::from_bits(0x3c7f_faab),
    f32::from_bits(0x3bff_feab),
    f32::from_bits(0x3b7f_ffab),
    f32::from_bits(0x3aff_ffeb),
    f32::from_bits(0x3a7f_fffb),
    f32::from_bits(0x39ff_ffff),
    f32::from_bits(0x3980_0000),
    f32::from_bits(0x3900_0000),
    f32::from_bits(0x3880_0000),
    f32::from_bits(0x3800_0000),
];

pub unsafe fn sincos(alpha: f32) -> (f32, f32) {
    // Move alpha to correct value range
    let mut alpha: f32 = alpha % (2.0 * PI);
    if alpha < -PI {
        alpha += 2.0 * PI
    } else if alpha > PI {
        alpha -= 2.0 * PI
    }

    let mut iter: u32 = 0;

    let mut x: f32 = 1.0;
    let mut y: f32 = 0.0;
    let mut angle: f32 = 0.0;
    let mut theta: f32 = 0.0;
    let mut dx: f32 = 0.0;
    let mut dy: f32 = 0.0;
    let mut p2i: f32 = 1.0;

    // CORDIC Armv7-M implementation
    asm!(
        "
    // cordic_iteration:
    91:
        // Calculate the next iteration
        CMP {iter}, #{ITERATIONS}                      // Check if all iterations are done
        BEQ 95f

        // Get angle from array
        VLDR.32 {angle}, [{angles_ptr}]             // Load angle from array
        ADD {angles_ptr}, {angles_ptr}, #4          // Increase pointer by 4B

        // Prepare needed values
        VMUL.F32 {dx}, {y}, {p2i}
        VMUL.F32 {dy}, {x}, {p2i}

        // Compare theta & alpha
        VCMP.F32 {theta}, {alpha}
        VMRS APSR_nzcv, FPSCR                       // move FP flags to ARM core flags [N,Z,C,V]
        BLT 93f                                     // Branch if theta >= alpha

    // cordic_rotate_pos:
    92:
        VSUB.F32 {theta}, {theta}, {angle}
        VADD.F32 {x}, {x}, {dx}
        VSUB.F32 {y}, {y}, {dy}

        B 94f

    // cordic_rotate_neg:
    93:
        VADD.F32 {theta}, {theta}, {angle}
        VSUB.F32 {x}, {x}, {dx}
        VADD.F32 {y}, {y}, {dy}

    // cordic_rotate_done:
    94:
        VDIV.F32 {p2i}, {p2i}, {TWO}
        ADD {iter}, {iter}, #1                     // Increment iteration counter

        B 91b

    // cordic_done:
    95:
        VMUL.F32 {x}, {x}, {k}
        VMUL.F32 {y}, {y}, {k}",
    ITERATIONS = const ITERATIONS,
    TWO = in(sreg) TWO,
    alpha = in(sreg) alpha,
    k = in(sreg) K_N[ITERATIONS - 1],
    angles_ptr = in(reg) &ANGLES as *const f32,
    iter = inout(reg) iter,
    x = inout(sreg) x,
    y = inout(sreg) y,
    angle = inout(sreg) angle,
    theta = inout(sreg) theta,
    dx = inout(sreg) dx,
    dy = inout(sreg) dy,
    p2i = inout(sreg) p2i,
    );

    return (x, y);
}
