FROM docker.io/library/rust:1.85-alpine@sha256:bea885d2711087e67a9f7a7cd1a164976f4c35389478512af170730014d2452a AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
