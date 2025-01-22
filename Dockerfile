FROM rust:1.84.0
WORKDIR /server
COPY . .
ENV DATABASE_URL=postgres://postgres:linux@db:5432/mydatabase
RUN cargo build --release
RUN cargo install sqlx-cli --no-default-features --features postgres