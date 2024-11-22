########################################################################################################################
# taskwarrior-simple-rest build stage
########################################################################################################################

FROM rust:1.82.0-slim AS build

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/home/taskapp/" \
    --shell "/sbin/nologin" \
    --uid 1004 \
    "taskwarrior-simple-rest"

RUN cargo build --target x86_64-unknown-linux-musl --release

# Add a shell step to inspect the built files
# RUN ls ./target/x86_64-unknown-linux-musl/release && sleep 1000

########################################################################################################################
# taskwarrior-simple-rest image
########################################################################################################################

# FROM scratch
FROM rust:1.82.0-slim

# Install the Taskwarrior deps
RUN apt-get update && apt-get install -y taskwarrior

RUN addgroup --system --gid 1004 taskwarrior 
RUN adduser --home /home/taskwarrior --system --uid 1004 taskwarrior 

COPY --from=build --chown=taskwarrior-simple-rest:taskwarrior-simple-rest ./target/x86_64-unknown-linux-musl/release/taskwarrior_simple_rest /app/taskwarrior_simple_rest

USER taskwarrior:taskwarrior

ENTRYPOINT ["./app/taskwarrior_simple_rest"]
