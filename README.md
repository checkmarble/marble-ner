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
