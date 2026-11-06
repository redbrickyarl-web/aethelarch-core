#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[no_mangle]
pub unsafe extern "C" fn aethelarch_matvec_ternary(
    x_bits: *const u8,
    w_mask: *const u8,
    w_sign: *const u8,
    length: usize,
) -> i32 {
    if x_bits.is_null() || w_mask.is_null() || w_sign.is_null() || length == 0 {
        return 0;
    }

    let mut pos_count: u32 = 0;
    let mut neg_count: u32 = 0;
    let mut i = 0;

    #[cfg(target_arch = "aarch64")]
    {
        while i + 16 <= length {
            let vx = vld1q_u8(x_bits.add(i));
            let vm = vld1q_u8(w_mask.add(i));
            let vs = vld1q_u8(w_sign.add(i));

            let v_active = vandq_u8(vx, vm);
            let v_pos = vandq_u8(v_active, vmvnq_u8(vs));
            let v_neg = vandq_u8(v_active, vs);

            pos_count += vaddvq_u8(vcntq_u8(v_pos)) as u32;
            neg_count += vaddvq_u8(vcntq_u8(v_neg)) as u32;

            i += 16;
        }
    }

    while i < length {
        let x = *x_bits.add(i);
        let m = *w_mask.add(i);
        let s = *w_sign.add(i);
        let active = x & m;

        pos_count += (active & !s).count_ones();
        neg_count += (active & s).count_ones();
        i += 1;
    }

    (pos_count as i32) - (neg_count as i32)
}

#[no_mangle]
pub unsafe extern "C" fn sbe_attention_coincidence(q_mask: u64, k_mask: u64) -> u32 {
    if q_mask == 0 {
        return 0;
    }
    (q_mask & k_mask).count_ones()
}
