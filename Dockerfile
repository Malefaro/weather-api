  
FROM rust

RUN USER=root cargo new --bin weather
WORKDIR ./weather
COPY ./Cargo.toml ./Cargo.toml
RUN rustup component add rustfmt
RUN cargo build --release
RUN rm src/*.rs

COPY . .

# https://github.com/rust-lang/cargo/issues/7181#issuecomment-515260460
RUN touch src/main.rs 

# RUN rm ./target/release/deps/netology-test*
RUN cargo build --release
CMD ./target/release/netology-test