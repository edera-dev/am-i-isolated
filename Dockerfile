FROM docker.io/library/rust:1.85-alpine@sha256:64d3fbc2db18485ab3a5211d56fec25493c71595e820c486e9b8a8ac9edbc4e8 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
