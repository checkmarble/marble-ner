import os

from typing import Annotated, Optional

from fastapi import Depends, FastAPI, Request, Response
from fastapi.security import OAuth2PasswordBearer
from gliner import GLiNER
from pydantic import BaseModel

if os.getenv('NER_API_KEY') is None:
    raise Exception('NER_API_KEY must be defined')

MODEL=os.getenv("GLINER_MODEL", "urchade/gliner_medium-v2.1")
LABELS=os.getenv("GLINER_LABELS", "Person,Company").split(",")

class Query(BaseModel):
  text: str

class Match(BaseModel):
  type: str
  text: str

model = GLiNER.from_pretrained(MODEL)
app = FastAPI()
auth = OAuth2PasswordBearer(tokenUrl='')

@app.get("/-/health")
def healthcheck():
  pass

@app.post("/detect")
def detect(token: Annotated[str, Depends(auth)], query: Query, response: Response):
  if token != os.environ['NER_API_KEY']:
    response.status_code = 401

    return {"detail": "Not authenticated"}

  labels = LABELS
  entities = model.predict_entities(query.text, labels, threshold=0.1)

  matches = []

  for entity in entities:
    match = Match(type=entity["label"], text=entity["text"])
    matches.append(match)

  return matches
