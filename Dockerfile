FROM docker.io/library/rust:1.83-alpine@sha256:9ab8f4eab808b1383c7e60a15fbf291e949fec85c3f98c34fb145b16c4ced0a1 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
