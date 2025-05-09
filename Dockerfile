FROM docker.io/library/rust:1.86-alpine@sha256:661d708cc863ce32007cf46807a72062a80d2944a6fae9e0d83742d2e04d5375 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
