FROM debian:bookworm-slim AS py
LABEL maintainer="Antoine Popineau <antoine.popineau@checkmarble.com>"

WORKDIR /app
ENV PATH="/root/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
ENV GLINER_MODEL=urchade/gliner_medium-v2.1

RUN \
    apt update && apt upgrade -y && \
    apt install -y --no-install-suggests --no-install-recommends pipx python3 python3-pip && \
    pipx install poetry && \
    pipx inject poetry poetry-plugin-export && \
    rm -rf /var/cache/apt

COPY pyproject.toml poetry.lock /app/

RUN \
    poetry export --format=requirements.txt --output requirements.txt && \
    python3 -m venv /venv && \
    /venv/bin/python -m pip install -r requirements.txt

RUN \
    /venv/bin/python -c 'import os, gliner; gliner.GLiNER.from_pretrained(os.getenv("GLINER_MODEL"))' && \
    chown -R 65532:65532 /root/.cache/huggingface

FROM al3xos/python-distroless:3.11.11-debian12
LABEL maintainer="Antoine Popineau <antoine.popineau@checkmarble.com>"

WORKDIR /app
ENV PYTHONPATH=/venv/lib/python3.11/site-packages
USER nonroot

COPY --from=py /venv /venv
COPY --from=py /root/.cache/huggingface /home/nonroot/.cache/huggingface
COPY . /app

EXPOSE 9000
ENTRYPOINT ["python"]
CMD ["/venv/bin/gunicorn", "--bind=0.0.0.0:9000", "--preload", "--workers=8", "--worker-class=uvicorn.workers.UvicornWorker", "main:app"]
