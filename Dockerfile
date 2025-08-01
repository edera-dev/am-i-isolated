FROM docker.io/library/rust:1.88-alpine@sha256:9dfaae478ecd298b6b5a039e1f2cc4fc040fc818a2de9aa78fa714dea036574d AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
