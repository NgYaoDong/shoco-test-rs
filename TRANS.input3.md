
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
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
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
