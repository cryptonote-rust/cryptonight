# Cryptonight Library For CryptoNote Based Crypto Currencies

[![](https://travis-ci.com/cryptonote-rust/cryptonight.svg?branch=master)](https://travis-ci.com/cryptonote-rust/cryptonight)
[![](https://img.shields.io/crates/v/cryptonote-cryptonight.svg)](https://crates.io/crates/cryptonote-cryptonight)
[![codecov](https://codecov.io/gh/cryptonote-rust/cryptonight/branch/master/graph/badge.svg)](https://codecov.io/gh/cryptonote-rust/cryptonight)


# Usage

## Cryptonight White Paper
```
    let aes = aes::new(AESSupport::HW);

    let input = byte_string::string_to_u8_array("");
    
    let cn_hash = hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6);
    // "eb14e8a833fac6fe9a43b57b336789c46ffe93f2868452240720607b14387e11"

    let input2 = b"This is a test";
    let cn_hash2 = hash::hash_alloc_scratchpad(&input2[0..], &aes, hash::HashVersion::Version6);
    // "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605"
```

## Cryptonight V7
```
    let aes = aes::new(AESSupport::HW);

    let input = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739430000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let cn_hash = hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version7);
    // "a01e369927b90e11d2159a85cedc0fed3f4cc9406b677fbe9c2afb810b20f231"

    let input = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739420000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let cn_hash = hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version7);
    // "ecdf8f5c8c0b399709a764257495382fb0230cf6fa55ee02e0667cf3d5be10d6"
```

