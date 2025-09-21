FROM rust:1.85

# Create app directory
WORKDIR /app

# Copy your project files
COPY . .

# Build release binary
RUN cargo build --release

# Run the binary
CMD ["./target/release/messageReader"]
