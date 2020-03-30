FROM rust:1.42.0-slim-buster
RUN apt-get update && apt-get install -y \
    libpq-dev \
&& rm -rf /var/lib/apt/lists/*
WORKDIR /usr/app
RUN cargo install cargo-watch && \
	cargo install systemfd && \
	cargo install diesel_cli --no-default-features --features postgres
EXPOSE 3000
CMD systemfd --no-pid -s http::0.0.0.0:3000 -- cargo watch -x run
