FROM rust:1.64.0 as develop
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# Build
FROM develop as build
RUN cargo build --release

# Production
FROM rust:1.64.0-slim-stretch
COPY --from=build /app/target/release/api .
EXPOSE 8080
CMD ["/usr/local/bin/api"]
