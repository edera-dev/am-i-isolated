FROM docker.io/library/rust:1.82-alpine@sha256:00c2107fa0e7a3eecf1fb31c814cd11a450026fae3fe375a1eed141be5fe75bc AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
