import os

from fastapi import FastAPI
from gliner import GLiNER
from pydantic import BaseModel

MODEL=os.getenv("GLINER_MODEL", "urchade/gliner_medium-v2.1")
LABELS=os.getenv("GLINER_LABELS", "Person,Company").split(",")

class Query(BaseModel):
  text: str

class Match(BaseModel):
  type: str
  text: str

model = GLiNER.from_pretrained(MODEL)
app = FastAPI()

@app.post("/detect")
async def detect(query: Query):
  labels = LABELS
  entities = model.predict_entities(query.text, labels, threshold=0.1)

  matches = []

  for entity in entities:
    match = Match(type=entity["label"], text=entity["text"])
    matches.append(match)

  return matches
