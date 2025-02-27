FROM debian:bookworm-slim
LABEL maintainer="Antoine Popineau <antoine.popineau@checkmarble.com>"

WORKDIR /app
ENV PATH="/root/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
ENV GLINER_MODEL=urchade/gliner_medium-v2.1
EXPOSE 9000

RUN \
  apt update && \
  apt install -y pipx && \
  pipx install poetry && \
  rm -rf /var/cache/apt

COPY pyproject.toml poetry.lock /app/

RUN \
  poetry install && \
  poetry run python -c 'import os, gliner; gliner.GLiNER.from_pretrained(os.getenv("GLINER_MODEL"))' && \
  poetry cache clear -n --all '' && \
  rm -rf /root/.cache/pypoetry/artifacts

COPY . /app

ENTRYPOINT ["poetry", "run"]
CMD ["gunicorn", "--bind=0.0.0.0:9000", "--preload", "--workers=8", "--worker-class=uvicorn.workers.UvicornWorker", "main:app"]
