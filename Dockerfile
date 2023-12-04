FROM rust:1.74.0-alpine3.18

ARG UID
ARG GID

RUN apk add --update --no-cache bash git

WORKDIR /mnt/app

RUN addgroup -g ${GID} code && adduser -D -G code -u ${UID} code

USER code

CMD while [ true ]; do sleep 3; done
