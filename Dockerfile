FROM rust:1.42.0-slim-buster
WORKDIR /usr/app
RUN cargo install cargo-watch
RUN cargo install systemfd
EXPOSE 3000
CMD systemfd --no-pid -s http::0.0.0.0:3000 -- cargo watch -x run
