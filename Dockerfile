FROM docker.io/library/rust:1.84-alpine@sha256:0cfc78e96e5314e8279cec9faf3067744764ff58cc6a87a3f9c89217f8c4aa16 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
