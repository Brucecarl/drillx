#include <hashx.h>
#include <stdio.h>

extern "C" int test_hash() {
    uint8_t seed[40] = {255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255};
    uint64_t nonce=7019;
    memcpy(seed + 32, &nonce, 8);
    // char hash[HASHX_SIZE];
    hashx_ctx *ctx = hashx_alloc(HASHX_INTERPRETED);
    if (!hashx_make(ctx, seed, sizeof(seed))) {
        printf("make hashx error\n");
        return 1;
    }

    // hashx_exec(ctx, 123456789, hash);
    hashx_free(ctx);
    printf("make hash success\n");
    return 0;
}