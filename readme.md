# ![The Payments Module](https://github.com/open-specification/payment/blob/master/.github/cover.png?raw=true)

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/open-specification/payment?color=teal)
![GitHub commit activity](https://img.shields.io/github/commit-activity/y/open-specification/payment?color=teal)
![GitHub top language](https://img.shields.io/github/languages/top/open-specification/payment?color=teal)
![GitHub contributors](https://img.shields.io/github/contributors/open-specification/payment?color=teal)

## Features

1. **Verify Credit Card Number with Luhn Method** - `/luhn/:number/`
    - The Luhn method, also known as the mod-ten method, is one of the most basic methods to verify a credit card number. Just because a credit card number may pass the luhn algorithm, does not mean that the credit card is valid.

1. **Get Issuer from Credit Card Number** - `/issuer/:number/`
    - One important piece of information that can be obtained from a credit card number is the issuer. Given the number, the module can identify who the credit card issuer is, or if the credit card issuer does not exist.

1. **Check if the Valid Through Date Works** - `/date/:month/:year/`
    - Blah.

1. **Find Detailed Information about Credit Number** - `/info/:number/`
    - Blah.

1. **Performance**
    - The power of the Rust Programming Language enables this module to have sub-millisecond response times. Speed may vary depending on your network connection, but the code itself will never be the performance bottleneck.

## Licensing

The code is under the Buisness Source License license. See the [licensing.md](./license.md) file for more information.
