FROM rust:1.67 as builder
WORKDIR /usr/src/return-list-connects
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && 
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/return-list-connects /usr/local/bin/return-list-connects
EXPOSE 2000
CMD ["return-list-connects"]