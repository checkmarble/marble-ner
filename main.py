from dataclasses import dataclass, field
import os

import uvicorn
import torch

from typing import Annotated, List

from fastapi import Depends, FastAPI, Response
from fastapi.security import OAuth2PasswordBearer
from gliner import GLiNER
from pathlib import Path
from pydantic import BaseModel
from functools import lru_cache

print(f"Starting Marble NER - pytorch={torch.__version__} - gpu={torch.cuda.is_available()} - cuda={torch.version.cuda}")

if os.getenv('NER_API_KEY') is None:
    raise Exception('NER_API_KEY must be defined')

MODEL=os.getenv("GLINER_MODEL", "urchade/gliner_medium-v2.1")
LABELS=os.getenv("GLINER_LABELS", "Person,Company").split(",")

@dataclass
class Settings():
    labels: List[str] = field(default_factory=lambda: LABELS)

@lru_cache
def settings():
    return Settings()

class Query(BaseModel):
  text: str

class Match(BaseModel):
  type: str
  text: str

print("Loading model from {MODEL}")

model = GLiNER.from_pretrained(MODEL)
app = FastAPI()
auth = OAuth2PasswordBearer(tokenUrl='')

@app.get("/-/health")
def healthcheck():
  pass

@app.post("/detect")
def detect(settings: Annotated[Settings, Depends(settings)], token: Annotated[str, Depends(auth)], query: Query, response: Response):
  if token != os.environ['NER_API_KEY']:
    response.status_code = 401

    return {"detail": "Not authenticated"}

  entities = model.predict_entities(query.text, settings.labels, threshold=0.1)

  matches = []

  for entity in entities:
    match = Match(type=entity["label"], text=entity["text"])
    matches.append(match)

  return matches

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=int(os.getenv("PORT", "9000")))
