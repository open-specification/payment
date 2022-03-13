# ![The Payments Module](https://github.com/open-specification/payment/blob/master/.github/cover.png?raw=true)

## Features

- **Verify Credit Card Number with Luhn Method**
    - The Luhn method, also known as the mod-ten method, is one of the most basic methods to verify a credit card number. Just because a credit card number may pass the luhn algorithm, does not mean that the credit card is valid.

- **Get Issuer from Credit Card Number**
    - One important piece of information that can be obtained from a credit card number is the issuer. Given the number, the module can identify who the credit card issuer is, or if the credit card issuer does not exist.

- **Performance**
    - The power of the Rust Programming Language enables this module to have sub-millisecond response times. Speed may vary depending on your network connection, but the code itself will never be the performance bottleneck.

## Licensing

Currently Unlicense; Full Copyright Owned by William Peter McGonagle. Future Code May Be Released Under Open-Source or Closed-Source Licenses.
