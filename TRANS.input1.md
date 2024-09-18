
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
typedef struct Pack {
  const uint32_t word;
  const unsigned int bytes_packed;
  const unsigned int bytes_unpacked;
  const unsigned int offsets[8];
  const int16_t _ALIGNED masks[8];
  const char header_mask;
  const char header;
} Pack;

#define PACK_COUNT 3
#define MAX_SUCCESSOR_N 7

static const Pack packs[PACK_COUNT] = {
  { 0x80000000, 1, 2, { 26, 24, 24, 24, 24, 24, 24, 24 }, { 15, 3, 0, 0, 0, 0, 0, 0 }, 0xc0, 0x80 },
  { 0xc0000000, 2, 4, { 25, 22, 19, 16, 16, 16, 16, 16 }, { 15, 7, 7, 7, 0, 0, 0, 0 }, 0xe0, 0xc0 },
  { 0xe0000000, 4, 8, { 23, 19, 15, 11, 8, 5, 2, 0 }, { 31, 15, 15, 15, 7, 7, 7, 3 }, 0xf0, 0xe0 }
};

static inline int decode_header(unsigned char val) {
  int i = -1;
  while ((signed char)val < 0) {
    val <<= 1;
    ++i;
  }
  return i;
}

union Code {
  uint32_t word;
  char bytes[4];
};

static inline int check_indices(const int16_t * shoco_restrict indices, int pack_n) {
  for (unsigned int i = 0; i < packs[pack_n].bytes_unpacked; ++i)
    if (indices[i] > packs[pack_n].masks[i])
      return 0;
  return 1;
}

static inline int find_best_encoding(const int16_t * shoco_restrict indices, unsigned int n_consecutive) {
  for (int p = PACK_COUNT - 1; p >= 0; --p)
    if ((n_consecutive >= packs[p].bytes_unpacked) && (check_indices(indices, p)))
      return p;
  return -1;
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
