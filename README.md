# AES-128 Encryption Implementation in Rust

This is a Rust implementation of the Advanced Encryption Standard (AES-128) based on the NIST FIPS 197 specification.

## Overview

AES (Advanced Encryption Standard) is a symmetric block cipher that processes data blocks of 128 bits using cipher keys of 128 bits. This implementation focuses on the AES-128 variant.

### Features

- Full AES-128 encryption and decryption
- Implementation of all standard AES transformations:
  - SubBytes (S-box substitution)
  - ShiftRows
  - MixColumns
  - AddRoundKey
- Key expansion for round key generation
- Support for multiple block encryption

### Implementation Details

The implementation follows the NIST FIPS 197 specification and includes:

- 10 rounds of transformation for encryption/decryption
- 128-bit block size
- 128-bit key length
- State-based processing using 4x4 byte matrices

### Reference

This implementation is based on:

- [NIST FIPS 197](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197-upd1.pdf) - Advanced Encryption Standard (AES)

## Usage

```rust
let plaintext = "6BC1BEE22E409F96E93D7E117393172A";
let key = "2B7E151628AED2A6ABF7158809CF4F3C";
let output = false; // Set to true for detailed transformation output

// Encrypt
let ciphertext = cipher(plaintext, key, output);

// Decrypt
let decrypted = inv_cipher(&ciphertext, key, output);
```
