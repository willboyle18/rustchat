FROM messense/rust-musl-cross:x86_64-musl AS builder
ENV SQLX_OFFLINE=true
WORKDIR /rustchat
#Copy the source code
COPY . .
# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /rustchat/target/x86_64-unknown-linux-musl/release/rustchat /rustchat
ENTRYPOINT ["/rustchat"]
EXPOSE 3000
