# Marble Name Entity Recognition API

This repository powers Marble's Name Entity Recognition (NER) used within the Sanctions Check feature.

## Build and run

```
$ docker build -t marble/marble-ner:latest -f Dockerfile.gpu . # Or Dockerfile.gpu
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

Before pulling the dependencies, you need to rename/symlink the proper `pyproject.toml` and `poetry.lock` for your platform, mostly whether or not you are running on a GPU. A Makefile is provided to help with this.

Note that the CPU built wheel of pytorch is not provided for macOS. If running that platform, you need to be using the `gpu` build. The Docker build requires a `TARGET=<cpu|gpu>` build argument to target the proper build (defaults to CPU).

```
$ make prepare TARGET=cpu # Or TARGET=gpu
$ poetry install
# Either run it directly with uvicorn
$ NER_API_KEY=apikey make run
```

In order to build the Docker image from macOS, you either have to use `--build-arg TARGET=gpu` or `--platform linux/x86_64`.

## Configure

You can configure which labels the Name Entity Recognition will try and detect from the input string. By default, it is set to `Person` and `Company`, but you can adjust it by setting a comma-separated list of labels in `GLINER_LABELS`. For example:

```sh
$ NER_API_KEY=apikey GLINER_LABELS='Person,Company,Country,Illegal Activity' make run
$ http POST :9000/detect text='Dinner with Martin Finnigan at Moneycorp, flying to Germany for some light money laundering and have a meeting with Bob Gloss' --bearer apikey
[
    {
        "type": "Person",
        "text": "Martin Finnigan"
    },
    {
        "type": "Company",
        "text": "Moneycorp"
    },
    {
        "type": "Country",
        "text": "Germany"
    },
    {
        "type": "Illegal Activity",
        "text": "money laundering"
    },
    {
        "type": "Person",
        "text": "Bob Gloss"
    }
]
```
