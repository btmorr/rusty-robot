FROM rust:1.22.1

COPY ./driver /
RUN cargo build

CMD ["./target/debug/driver", "hello", "world"]

# docker build -t driver:<tag> .
# docker run -t driver:<tag>
