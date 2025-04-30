# rust_stock_market_morning_routine

    - create a daily report about the market situation as fact sheet

## project init

touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show

## project environment

    - stable latest rust version
  
## used library / crates

    - [ib_async](https://crates.io/crates/ib_async)

## sources

### python

    - [ib-api-reloaded/ib_async](https://github.com/ib-api-reloaded/ib_async)

### rust

    - [wboayue /rust-ibapi ](https://github.com/wboayue/rust-ibapi)
      - 122 stars
      - high activity
  
    - [wvietor/ibkr_rust](https://github.com/wvietor/ibkr_rust)
    - [yahoo! finance API](https://github.com/xemwebe/yahoo_finance_api)

### License

    - TODO add [License](https://github.com/wvietor/ibkr_rust/blob/main/LICENSE)
    - TODO add [License](https://choosealicense.com/licenses/apache-2.0/)
  
### Test

- [Functional Testing Vs. Unit Testing Vs. Integration Testing](https://www.headspin.io/blog/unit-integration-and-functional-testing-4-main-points-of-difference#:~:text=Purpose%3A%20Unit%20testing%20checks%20the,it%20functions%20as%20a%20whole.)

- [Should unit tests really be put in the same file as the source?](https://users.rust-lang.org/t/should-unit-tests-really-be-put-in-the-same-file-as-the-source/62153/2)
  
  - **REASON** A nice benefit of that is that it speeds up compilation times, because the compiler doesn't have to parse the tests :smiley:
  
- [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

- [Complete Guide To Testing Code In Rust](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)

- [cargo-nextest HIGH ACTIVITY](https://crates.io/crates/cargo-nextest)
  
### TTD
