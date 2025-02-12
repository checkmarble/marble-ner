FROM pytorch/pytorch:2.6.0-cuda12.4-cudnn9-runtime
LABEL maintainer="Antoine Popineau <antoine.popineau@checkmarble.com>"

WORKDIR /app
ENV GLINER_MODEL=urchade/gliner_medium-v2.1
EXPOSE 9000

COPY pyproject.toml poetry.lock /app/
RUN pip install poetry && poetry install && poetry run python -c 'import os, gliner; gliner.GLiNER.from_pretrained(os.getenv("GLINER_MODEL"))'
COPY . /app

ENTRYPOINT ["poetry", "run"]
CMD ["gunicorn", "--bind=0.0.0.0:9000", "--preload", "--workers=8", "--worker-class=uvicorn.workers.UvicornWorker", "main:app"]
