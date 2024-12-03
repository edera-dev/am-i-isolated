FROM docker.io/library/rust:1.83-alpine@sha256:838d384a1138fe1f2e448e3901bb3d23683570ba3dca581160880ffad760332b AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
