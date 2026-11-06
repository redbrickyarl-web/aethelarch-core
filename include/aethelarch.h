#ifndef AETHELARCH_H
#define AETHELARCH_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

int32_t aethelarch_matvec_ternary(
    const uint8_t* x_bits,
    const uint8_t* w_mask,
    const uint8_t* w_sign,
    size_t length
);

uint32_t sbe_attention_coincidence(uint64_t q_mask, uint64_t k_mask);

#ifdef __cplusplus
}
#endif

#endif /* AETHELARCH_H */
