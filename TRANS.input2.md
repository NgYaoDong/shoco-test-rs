
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c
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
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
