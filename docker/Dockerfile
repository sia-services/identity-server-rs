FROM debian:bullseye-slim

ARG APP=/usr/local/bin

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8443

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}/keystore

COPY ./target/release/identity-server-rs ${APP}
COPY ./keystore/internal-devel.crt ${APP}/keystore
COPY ./keystore/internal-devel.key ${APP}/keystore

RUN ls -la ${APP}

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["/usr/local/bin/identity-server-rs"]
