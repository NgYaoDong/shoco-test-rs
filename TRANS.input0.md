
The task is to translate the following C program to **safe** Rust that has the same external behavior. 
There are multiple functions in the C program so let us do it piece by piece.
Let us first take a look of the complete program:

### Full Program

```c
#include "stdint.h"
#include "stdio.h"
#include "assert.h"
#include "string.h"

#include <stddef.h>

#if defined(_MSC_VER)
#define shoco_restrict __restrict
#elif __GNUC__
#define shoco_restrict __restrict__
#else
#define shoco_restrict restrict
#endif

size_t shoco_compress(const char * const shoco_restrict in, size_t len, char * const shoco_restrict out, size_t bufsize);
size_t shoco_decompress(const char * const shoco_restrict in, size_t len, char * const shoco_restrict out, size_t bufsize);

#include <stdint.h>

#if (defined (__BYTE_ORDER__) && (__BYTE_ORDER__ == __ORDER_BIG_ENDIAN__) || __BIG_ENDIAN__)
  #define swap(x) (x)
#else
  #if defined(_MSC_VER)
    #include <stdlib.h>
    #define swap(x) _byteswap_ulong(x)
  #elif defined (__GNUC__)
    #if defined(__builtin_bswap32)
      #define swap(x) __builtin_bswap32(x)
    #else
      #define swap(x) ((x<<24) + ((x&0x0000FF00)<<8) + ((x&0x00FF0000)>>8) + (x>>24))
    #endif
  #else
    #include <byteswap.h>
    #define swap(x) bswap_32(x)
  #endif
#endif

#if defined(_MSC_VER)
  #define _ALIGNED __declspec(align(16))
  #define inline __inline
#elif defined(__GNUC__)
  #define _ALIGNED __attribute__ ((aligned(16)))
#else
  #define _ALIGNED
#endif

#define MIN_CHR 39
#define MAX_CHR 122

static const char chrs_by_chr_id[32] = {
  'e', 'a', 'i', 'o', 't', 'h', 'n', 'r', 's', 'l', 'u', 'c', 'w', 'm', 'd', 'b', 'p', 'f', 'g', 'v', 'y', 'k', '-', 'H', 'M', 'T', '\'', 'B', 'x', 'I', 'W', 'L'
};

static const int8_t chr_ids_by_chr[256] = {
  -1, -1, -1, -1, -1, -1, ...
};

static const int8_t successor_ids_by_chr_id_and_chr_id[32][32] = {
  {7, 4, 12, -1, 6, -1, 1, 0, 3, 5, -1, 9, -1, 8, 2, -1, 15, 14, -1, 10, 11, -1, -1, -1, -1, -1, -1, -1, 13, -1, -1, -1},
  {-1, -1, 6, -1, 1, -1, 0, 3, 2, 4, 15, 11, -1, 9, 5, 10, 13, -1, 12, 8, 7, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1},
  ...
};

static const int8_t chrs_by_chr_and_successor_id[MAX_CHR - MIN_CHR][16] = {
  {'s', 't', 'c', 'l', 'm', 'a', 'd', 'r', 'v', 'T', 'A', 'L', 'e', 'M', 'Y', '-'},
  {'\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'},
  ...
};

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
size_t shoco_compress(const char * const shoco_restrict original, size_t strlen, char * const shoco_restrict out, size_t bufsize) {
  char *o = out;
  char * const out_end = out + bufsize;
  const char *in = original;
  int16_t _ALIGNED indices[MAX_SUCCESSOR_N + 1] = { 0 };
  int last_chr_index;
  int current_index;
  int successor_index;
  unsigned int n_consecutive;
  union Code code;
  int pack_n;
  unsigned int rest;
  const char * const in_end = original + strlen;

  while ((*in != '\0')) {
    if (strlen && (in == in_end))
      break;

    // find the longest string of known successors
    indices[0] = chr_ids_by_chr[(unsigned char)in[0]];
    last_chr_index = indices[0];
    if (last_chr_index < 0)
      goto last_resort;

    rest = in_end - in;
    for (n_consecutive = 1; n_consecutive <= MAX_SUCCESSOR_N; ++n_consecutive) {
      if (strlen && (n_consecutive == rest))
        break;

      current_index = chr_ids_by_chr[(unsigned char)in[n_consecutive]];
      if (current_index < 0)  // '\0' is always -1
        break;

      successor_index = successor_ids_by_chr_id_and_chr_id[last_chr_index][current_index];
      if (successor_index < 0)
        break;

      indices[n_consecutive] = (int16_t)successor_index;
      last_chr_index = current_index;
    }
    if (n_consecutive < 2)
      goto last_resort;

    pack_n = find_best_encoding(indices, n_consecutive);
    if (pack_n >= 0) {
      if (o + packs[pack_n].bytes_packed > out_end)
        return bufsize + 1;

      code.word = packs[pack_n].word;
      for (unsigned int i = 0; i < packs[pack_n].bytes_unpacked; ++i)
        code.word |= indices[i] << packs[pack_n].offsets[i];

      // In the little-endian world, we need to swap what's
      // in the register to match the memory representation.
      // On big-endian systems, this is a dummy.
      code.word = swap(code.word);

      // if we'd just copy the word, we might write over the end
      // of the output string
      for (unsigned int i = 0; i < packs[pack_n].bytes_packed; ++i)
        o[i] = code.bytes[i];

      o += packs[pack_n].bytes_packed;
      in += packs[pack_n].bytes_unpacked;
    } else {
last_resort:
      if (*in & 0x80) {
        // non-ascii case
        if (o + 2 > out_end)
          return bufsize + 1;
        // put in a sentinel byte
        *o++ = 0x00;
      } else {
        // an ascii byte
        if (o + 1 > out_end)
          return bufsize + 1;
      }
      *o++ = *in++;
    }
  }

  return o - out;
}
size_t shoco_decompress(const char * const shoco_restrict original, size_t complen, char * const shoco_restrict out, size_t bufsize) {
  char *o = out;
  char * const out_end = out + bufsize;
  const char *in = original;
  char last_chr;
  union Code code = { 0 };
  int offset;
  int mask;
  int mark;
  const char * const in_end = original + complen;

  while (in < in_end) {
    mark = decode_header(*in);
    if (mark < 0) {
      if (o >= out_end)
        return bufsize + 1;

      // ignore the sentinel value for non-ascii chars
      if (*in == 0x00) {
        if (++in >= in_end)
          return SIZE_MAX;
      }

      *o++ = *in++;
    } else {
      if (o + packs[mark].bytes_unpacked > out_end)
        return bufsize + 1;
      else if (in + packs[mark].bytes_packed > in_end)
        return SIZE_MAX;

      // This should be OK as well, but it fails with emscripten.
      // Test this with new versions of emcc.
      //code.word = swap(*(uint32_t *)in);
      for (unsigned int i = 0; i < packs[mark].bytes_packed; ++i)
        code.bytes[i] = in[i];
      code.word = swap(code.word);

      // unpack the leading char
      offset = packs[mark].offsets[0];
      mask = packs[mark].masks[0];
      last_chr = o[0] = chrs_by_chr_id[(code.word >> offset) & mask];

      // unpack the successor chars
      for (unsigned int i = 1; i < packs[mark].bytes_unpacked; ++i) {
        offset = packs[mark].offsets[i];
        mask = packs[mark].masks[i];
        last_chr = o[i] = chrs_by_chr_and_successor_id[(unsigned char)last_chr - MIN_CHR][(code.word >> offset) & mask];
      }

      o += packs[mark].bytes_unpacked;
      in += packs[mark].bytes_packed;
    }
  }

  // append a 0-terminator if it fits
  if (o < out_end)
    *o = '\0';

  return o - out;
}
static const char LARGE_STR[] = "This is a large string that won't possibly fit into a small buffer";
static const char NON_ASCII_STR[] = "Übergrößenträger";

int main() {
  char buf_1[1];
  char buf_2[2];
  char buf_4[4];
  char buf_large[4096];
  size_t ret;

  // test compression
  ret = shoco_compress(LARGE_STR, 0, buf_2, 2);
  assert(ret == 3); // bufsize + 1 if buffer too small

  ret = shoco_compress(LARGE_STR, 0, buf_large, 4096);
  assert(ret <= strlen(LARGE_STR));

  ret = shoco_compress("a", 0, buf_1, 1);
  assert(ret == 1); // bufsize if null byte didn't fit

  buf_2[1] = 'x';
  ret = shoco_compress("a", 0, buf_2, 2);
  assert(ret == 1); // compressed string length without null byte
  assert(buf_2[1] == 'x'); // Canary is still alive

  ret = shoco_compress("a", 0, buf_4, 4);
  assert(ret == 1);

  ret = shoco_compress("test", 0, buf_4, 4);
  assert(ret <= 4);

  buf_4[1] = 'x';
  ret = shoco_compress("test", 1, buf_4, 4); // buffer large enough, but strlen said "just compress first char"
  assert(ret == 1);
  assert(buf_4[1] == 'x');

  ret = shoco_compress("t\x80", 1, buf_4, 4);
  assert(ret == 1);
  assert(buf_4[1] == 'x');

  buf_4[1] = 'y';
  ret = shoco_compress("test", 1, buf_4, 1);
  assert(ret == 1);
  assert(buf_4[1] == 'y'); // no null byte written

  buf_4[1] = 'z';
  ret = shoco_compress("a", 1, buf_4, 4);
  assert(ret == 1);
  assert(buf_4[1] == 'z');

  buf_4[1] = 'b';
  ret = shoco_compress("a", 2, buf_4, 4);
  assert(ret == 1);
  assert(buf_4[1] == 'b');

  ret = shoco_compress("ä", 0, buf_1, 1); // this assumes that 'ä' is not in the frequent chars table
  assert(ret == 2);

  
  //test decompression
  char compressed_large[4096];
  int large_len = strlen(LARGE_STR);
  int comp_len;
  comp_len = shoco_compress(LARGE_STR, 0, compressed_large, 4096);

  buf_large[large_len] = 'x';
  ret = shoco_decompress(compressed_large, comp_len, buf_large, 4096);
  assert(ret == large_len);
  assert(strcmp(buf_large, LARGE_STR) == 0);
  assert(buf_large[large_len] == '\0'); // null byte written
  
  ret = shoco_decompress(compressed_large, comp_len, buf_2, 2);
  assert(ret == 3); // ret = bufsize + 1, because buffer too small

  buf_large[large_len] = 'x';
  ret = shoco_decompress(compressed_large, comp_len, buf_large, large_len);
  assert(ret == large_len);
  assert(buf_large[large_len] != '\0'); // no null byte written

  ret = shoco_decompress(compressed_large, 5, buf_large, 4096);
  assert((ret < large_len) || (ret == 4097)); // either fail (bufsize + 1) or it happened to work


  char compressed_non_ascii[256];
  int non_ascii_len = strlen(NON_ASCII_STR);
  comp_len = shoco_compress(NON_ASCII_STR, 0, compressed_non_ascii, 256);

  buf_large[non_ascii_len] = 'x';
  ret = shoco_decompress(compressed_non_ascii, comp_len, buf_large, 4096);
  assert(ret == non_ascii_len);
  assert(strcmp(buf_large, NON_ASCII_STR) == 0);
  assert(buf_large[non_ascii_len] == '\0'); // null byte written

  ret = shoco_decompress("\x00", 1, buf_large, 4096);
  assert(ret == SIZE_MAX);

  ret = shoco_decompress("\xe0""ab", 3, buf_large, 4096);
  assert(ret == SIZE_MAX);

  puts("All tests passed.");
  return 0;
}
```

---------

We are splitting the above program into multiple parts. You are expected to output the complete translation of each part I gave you and I will verify it before giving you the task for the next part. I know Rust and the Cargo build system, thus you don't need to explain how to build the program. I will concatenate your outputs to get the final program and build it myself.

Let us first look at Part 0 (a fraction of the full program above):

### Part 0

```c
#include "stdint.h"
#include "stdio.h"
#include "assert.h"
#include "string.h"

#include <stddef.h>

#if defined(_MSC_VER)
#define shoco_restrict __restrict
#elif __GNUC__
#define shoco_restrict __restrict__
#else
#define shoco_restrict restrict
#endif

size_t shoco_compress(const char * const shoco_restrict in, size_t len, char * const shoco_restrict out, size_t bufsize);
size_t shoco_decompress(const char * const shoco_restrict in, size_t len, char * const shoco_restrict out, size_t bufsize);

#include <stdint.h>

#if (defined (__BYTE_ORDER__) && (__BYTE_ORDER__ == __ORDER_BIG_ENDIAN__) || __BIG_ENDIAN__)
  #define swap(x) (x)
#else
  #if defined(_MSC_VER)
    #include <stdlib.h>
    #define swap(x) _byteswap_ulong(x)
  #elif defined (__GNUC__)
    #if defined(__builtin_bswap32)
      #define swap(x) __builtin_bswap32(x)
    #else
      #define swap(x) ((x<<24) + ((x&0x0000FF00)<<8) + ((x&0x00FF0000)>>8) + (x>>24))
    #endif
  #else
    #include <byteswap.h>
    #define swap(x) bswap_32(x)
  #endif
#endif

#if defined(_MSC_VER)
  #define _ALIGNED __declspec(align(16))
  #define inline __inline
#elif defined(__GNUC__)
  #define _ALIGNED __attribute__ ((aligned(16)))
#else
  #define _ALIGNED
#endif

#define MIN_CHR 39
#define MAX_CHR 122

static const char chrs_by_chr_id[32] = {
  'e', 'a', 'i', 'o', 't', 'h', 'n', 'r', 's', 'l', 'u', 'c', 'w', 'm', 'd', 'b', 'p', 'f', 'g', 'v', 'y', 'k', '-', 'H', 'M', 'T', '\'', 'B', 'x', 'I', 'W', 'L'
};

static const int8_t chr_ids_by_chr[256] = {
  -1, -1, -1, -1, -1, -1, ...
};

static const int8_t successor_ids_by_chr_id_and_chr_id[32][32] = {
  {7, 4, 12, -1, 6, -1, 1, 0, 3, 5, -1, 9, -1, 8, 2, -1, 15, 14, -1, 10, 11, -1, -1, -1, -1, -1, -1, -1, 13, -1, -1, -1},
  {-1, -1, 6, -1, 1, -1, 0, 3, 2, 4, 15, 11, -1, 9, 5, 10, 13, -1, 12, 8, 7, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1},
  ...
};

static const int8_t chrs_by_chr_and_successor_id[MAX_CHR - MIN_CHR][16] = {
  {'s', 't', 'c', 'l', 'm', 'a', 'd', 'r', 'v', 'T', 'A', 'L', 'e', 'M', 'Y', '-'},
  {'\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'},
  ...
};

```

Please first translate translate Part 0 of the C program (provided above) to Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
