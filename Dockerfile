FROM rust:1.82

WORKDIR /pokemon

# Install system dependencies for diesel_cli
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install diesel_cli with PostgreSQL support
RUN cargo install diesel_cli --no-default-features --features postgres
