# noreplay

noreplay is a library used to detect if a number was used within a certain window.
This is useful for detecting and preventing packet replay attacks, and is used in
protocols like DTLS and SRTP. Anti replay safeguards are also mentioned in RFC's such as (RFC4303)[https://www.rfc-editor.org/rfc/rfc4303#section-3.4.3].

## Purpose

This rust library was written as a learning project to understand how a sliding window datastructure could be implemented using bitwise operations which in turn is used to efficiently detect replay attacks.  

I ran into some DTLS replay attack errors while doing some work on (un)muting tracks on webrtc, and found an 
implementation for a replay detector in the [pion/transport](https://github.com/pion/transport/tree/master/replaydetector) repo. 
The implementation in this repo is heavily inspired by the code in the linked repo above, however there are some differences
in how I modeled the bits in the vector of uints in the Bigint library.

Learning concepts
- [x] Rust error types
- [x] Writing rust tests
- [ ] Paramaterize test with rust macros
- [ ] Benchmarking different data structures (Bigint, dequeue, vec)

## Tests

```
cargo test
```
