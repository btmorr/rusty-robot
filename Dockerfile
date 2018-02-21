# FROM rust:1.22.1
FROM parity/rust-arm:gitlab-ci


COPY ./driver /
RUN cargo build

CMD ["./target/debug/driver", "--hostname", "0.0.0.0", "--port", "8080"]

# docker build -t driver:<tag> .
# docker run -p 8080:8080 -it driver:<tag>
