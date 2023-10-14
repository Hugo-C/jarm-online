FROM rust:1.73-bullseye as build-stage

ARG BUILD_TARGET="x86_64-unknown-linux-musl"
ARG BUILD_OPTIONS="--release --target $BUILD_TARGET"

RUN apt update && apt install -y build-essential musl-tools
RUN rustup set profile minimal && rustup default nightly && rustup target add $BUILD_TARGET

WORKDIR /app

# Create blank project
RUN USER=root cargo new project

# We want dependencies cached, so copy those first.
COPY Cargo.toml /app/project
COPY Cargo.lock /app/project

WORKDIR /app/project

# This is a dummy build to get the dependencies cached.
RUN cargo build $BUILD_OPTIONS

# Now copy in the rest of the sources
COPY . /app/project/

# This is the actual build, touch the main.rs to have newer timestamp
RUN touch /app/project/src/main.rs && cargo build $BUILD_OPTIONS -Z unstable-options --out-dir /app/dist

FROM alpine:3 as production-stage
RUN mkdir /app
COPY --from=build-stage /app/dist/jarm_online /app
COPY --from=build-stage /app/project/Rocket.toml /
RUN chown -R 1001:1001 /app
USER 1001
CMD /app/jarm_online