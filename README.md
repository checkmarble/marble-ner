# Marble Name Entity Recognition API

This repository powers Marble's Name Entity Recognition (NER) used within the Sanctions Check feature.

## Build and run

```
$ docker build -t marble/marble-ner:latest .
$ docker run --rm -p 9000:9000 -e GLINER_LABELS=Person,Company,Country marble/marble-ner:latest
```

You can instruct the model to try and label specific names from your input by specifying a comma-separated list of labels in the `GLINER_LABELS` environment variable. By default, the used labels are `Person` and `Company`.

## Query

```
$ curl -XPOST 127.0.0.1:9000/detect -H content-type:application/json -d \
  '{"text":"flying to canada to have dinner at acme\'s headquarters with joe"}'
[
  {
    "type": "Country",
    "text": "canada"
  },
  {
    "type": "Company",
    "text": "acme"
  },
  {
    "type": "Person",
    "text": "joe"
  }
]
```

## Develop

```
$ poetry install
# Either run it directly with uvicorn
$ NER_API_KEY=apikey poetry run uvicorn --workers=1 main:main
# Or run as as production through gunicorn
$ NER_API_KEY=apikey poetry run gunicorn --bind=0.0.0.0:9000 --workers=1 --preload --worker-class=uvicorn.workers.UvicornWorker main:app
```
