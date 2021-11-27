# imagic-lib
Simple library for encrypting an image based on a given seed.

# Explanation
The "encryption" is simple pixel shuffling for now.

I am going to use this library for my new project, so I guess I'll add something more along with documentation.

It is also possible to insert a hidden message in an encrypted image that wouldn't be visible in the decrypted one, but it's stupid, so I won't show how to do it. 

If the encryption/decryption process takes too much time, <br> try running it as a release (`cargo run --release`)

# Example
```rust
let mut data: Vec<u8>;

data = Imagic::Encrypt.from_buffer(
    include_bytes!("input.jpg"),    // input bytes
    "secret_key".into(),            // seed
    90,                             // image quality
)?;
File::create("encrypted.jpg").unwrap().write_all(&data).ok();

data = Imagic::Decrypt.from_buffer(
    &data,                  // input bytes
    "secret_key".into(),    // seed
    50,                     // image quality
)?;
File::create("decrypted.jpg").unwrap().write_all(&data).ok();
```

<!-- https://imgur.com/a/WGACrJq --> 

| Input                                 | Encrypted                             | Decrypted                             |
|---------------------------------------|---------------------------------------|---------------------------------------|
| ![](https://i.imgur.com/8QejjtV.jpeg) | ![](https://i.imgur.com/Ur83ELy.jpeg) | ![](https://i.imgur.com/yR3Vdwm.jpeg) |


