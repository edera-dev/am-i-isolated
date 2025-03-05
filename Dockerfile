FROM docker.io/library/rust:1.85-alpine@sha256:1030547bd568497d69e41771ada279179f0613369dc54779e46a3f6f376b3020 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
