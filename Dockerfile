FROM rust:1.54

RUN rustup default nightly

WORKDIR /app

# Create blank project
RUN USER=root cargo new project

# We want dependencies cached, so copy those first.
COPY Cargo.toml /app/project
COPY Cargo.lock /app/project

WORKDIR /app/project

# This is a dummy build to get the dependencies cached.
RUN cargo build --release

# Now copy in the rest of the sources
COPY . /app/project/

# This is the actual build, touch the main.rs to have newer timestamp
RUN touch /app/project/src/main.rs
RUN cargo build --release

CMD ["cargo", "run", "--release"]