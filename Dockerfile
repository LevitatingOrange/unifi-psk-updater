# select build image
FROM rust:1.33 as build

# create a new empty shell project
RUN USER=root cargo new --bin wifi-pw-gen
WORKDIR /wifi-pw-gen

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
#RUN rm ./target/release/deps/wifi-pw-gen*
RUN cargo build --release

# our final base
FROM rust:1.23

# copy the build artifact from the build stage
COPY --from=build /wifi-pw-gen/target/release/wifi-pw-gen .

ENV CONFIG_PATH /wifi-config

# set the startup command to run your binary
CMD ["./wifi-pw-gen" "$CONFIG_PATH"]