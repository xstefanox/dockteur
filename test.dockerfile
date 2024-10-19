FROM xstefanox/dockteur:latest AS dockteur-default

FROM xstefanox/dockteur:alpine AS dockteur-alpine

FROM python:slim AS default
ENV PYTHONUNBUFFERED=1
ENV DOCKTEUR_PORT=8080
COPY --from=dockteur-default /dockteur /usr/local/bin/dockteur
CMD ["python", "-m", "http.server", "8080"]
HEALTHCHECK --interval=1s --timeout=3s --retries=10 CMD ["dockteur"]

FROM python:alpine AS alpine
ENV PYTHONUNBUFFERED=1
ENV DOCKTEUR_PORT=8080
CMD ["python", "-m", "http.server", "8080"]
COPY --from=dockteur-alpine /dockteur /usr/local/bin/dockteur
HEALTHCHECK --interval=1s --timeout=3s --retries=10 CMD ["dockteur"]
