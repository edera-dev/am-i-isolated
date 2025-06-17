FROM docker.io/library/rust:1.87-alpine@sha256:126df0f2a57e675f9306fe180b833982ffb996e90a92a793bb75253cfeed5475 AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
