# Dockteur

_A health checker for Docker containers_

# Description

Most of the Docker images present on Docker Hub do not implement a specific health check, even though Dockerfile
[officially supports this feature](https://docs.docker.com/engine/reference/builder/#healthcheck).

The simplest solution to implement native Docker health check could be to use a standard command line client like
curl, wget, redis-cli, etc.
The problem with this approach is that these clients could require additional dependencies and this could increase the
size of the image and enlarge the attack surface of the image.

Dockteur is a very thin and simple health checker that has the sole purpose of verifying that a service is reachable
and adapt the result into a standard Unix process return codes.
These are the main features:
- statically linked into a single executable: just copy one file into the final Docker image
- multi-arch: `x86` and `arm` are both supported
- configurable with environment variables
- specific log messages: you should be able to easily detect issue the Docker container healthcheck logs with the
  `docker inspect <container>` command

# Supported protocols

## HTTP

Dockteur performs a HTTP request and checks the response status code.

## Redis

Dockteur sends a `PING` command and checks that the response is `PONG`.

# How to use

You can include Dockteur into your Docker image to enable the native healthcheck:

```dockerfile
# load the image into a multi-stage Dockerfile (hint: use a stable version, not latest)
FROM xstefanox/dockteur:latest AS dockteur

# start from your base image
FROM debian:stable AS production
# copy the Dockteur executable into your image
COPY --from=dockteur /dockteur /usr/local/bin/dockteur
# configure the healthcheck
HEALTHCHECK --interval=3s --timeout=3s --retries=10 CMD dockteur
```

# Configuration

## Common

* `DOCKTEUR_PROTOCOL`: the protocol to use for the healthcheck (`http` or `redis`, default `http`)
* `DOCKTEUR_PORT`: the TCP port (default `80` for HTTP, `6379` for Redis)
* `DOCKTEUR_TIMEOUT_MILLIS`: the request timeout in milliseconds (default `500`)

## HTTP

* `DOCKTEUR_METHOD`: the HTTP method (default `GET`)
* `DOCKTEUR_PATH`: the HTTP path (default `/`)
* `DOCKTEUR_STATUS_CODE`: the expected HTTP status code (default `200`)

# Development

1. Initialise your local repository checkout

   ```shell
   pre-commit install --hook-type commit-msg --hook-type pre-commit
   ```
