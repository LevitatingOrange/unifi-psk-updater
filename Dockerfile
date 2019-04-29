# select build image
FROM rust:1.34 as build

# create a new empty shell project
RUN USER=root cargo new --bin unifi-psk-updater
WORKDIR /unifi-psk-updater

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src
COPY ./static ./static
# build for release
#RUN rm ./target/release/deps/unifi-psk-updater*

RUN cargo build --release

#RUN ls -lisa target/release

# our final base
FROM rust:1.34

# copy the build artifact from the build stage
COPY --from=build /unifi-psk-updater/target/release/unifi-psk-updater .
COPY ./example_conf.toml /example_conf.toml
ENV CONFIG_PATH /example_conf.toml
# set the startup command to run your binary
#CMD ["ls", "-lisa"]
CMD ["sh", "-c", "/unifi-psk-updater ${CONFIG_PATH}"]
#CMD ["/unifi-psk-updater", "$CONFIG_PATH"]
