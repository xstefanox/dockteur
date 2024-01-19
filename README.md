# Dockteur

_A HTTP health checker for Docker containers_

# Description

Most of the Docker images present on Docker Hub do not implement a specific health check, even though Dockerfile
[officially supports this feature](https://docs.docker.com/engine/reference/builder/#healthcheck).

If your container exposes a HTTP service, the simplest solution to implement native Docker health check could be to
use a standard command line HTTP client like curl, wget, httpie, etc.
The problem with this approach is that these clients could require additional dependencies and this could increase the
size of the image and enlarge the attack surface of the image.

Dockteur is a very thin and simple HTTP client that has the sole purpose of making a HTTP call and adapt the result
into a standard Unix process return codes.
These are the main features:
- statically linked into a single executable: just copy one file into the final Docker image
- multi-arch: `x86` and `arm` are both supported
- configurable with environment variables
- specific log messages: you should be able to easily detect issue the Docker container healthcheck logs with the 
  `docker inspect <container>` command

# How to use

You can include Dockteur into you Docker image to enable the native healthcheck:

```dockerfile
# load the image into a multi-stage Dockerfile
FROM xstefanox/dockteur:1.2.0 AS dockteur

# start from your base image
FROM debian:stable AS production
# copy the Dockteur executable into your image
COPY --from=dockteur /dockteur /usr/local/bin/dockteur
# configure the healthcheck
HEALTHCHECK --interval=3s --timeout=3s --retries=10 CMD dockteur
```

# Configuration

You can configure the HTTP endpoint to invoke for the healthcheck defining the following environment variables

* `HEALTHCHECK_METHOD`: the HTTP method (default `GET`)
* `HEALTHCHECK_PORT`: the TCP port (default `80`)
* `HEALTHCHECK_PATH`: the HTTP path (default `/`)
* `HEALTHCHECK_TIMEOUT_MILLIS`: the request timeout in milliseconds (default `500`)
