# Use the official Rust image as the base image
FROM rust:buster as builder

# Set the working directory
WORKDIR /usr/local/bin

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Install dependecies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/local/bin/target/release/mto-webserver .

# Change binary permissions
RUN chmod +x /usr/local/bin/mto-webserver

# Expose the port your application will run on
EXPOSE 8080

# Set the environment variable for the application
ENV RUST_LOG=info

# Run the application
CMD ["./mto-webserver"]
