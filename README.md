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
  