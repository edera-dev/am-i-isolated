FROM docker.io/library/rust:1.84-alpine@sha256:fdff417c3845c92360b439382f7d6dabca6c998f59c8dce6cd2a16a2e9e85498 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
