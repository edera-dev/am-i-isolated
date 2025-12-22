FROM docker.io/library/rust:1.92-alpine@sha256:f6c22e0a256c05d44fca23bf530120b5d4a6249a393734884281ca80782329bc AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/am-i-isolated /bin/am-i-isolated

FROM scratch AS final
ENTRYPOINT ["/bin/am-i-isolated"]
COPY --from=build /bin/am-i-isolated /bin/am-i-isolated
