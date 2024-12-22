FROM rust:1.82.0 as builder

WORKDIR /workspace
COPY . .

WORKDIR /workspace/packages/cron_jobs
RUN cargo build --release

ENV PORT 8080

ENTRYPOINT ["target/release/cron_jobs"]
