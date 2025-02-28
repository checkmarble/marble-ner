import os

os.environ['NER_API_KEY'] = 'randomapikey'

from fastapi.testclient import TestClient
from main import app


client = TestClient(app)

def test_healthcheck():
  response = client.get('/-/health')

  assert response.status_code == 200

def test_detection():
  response = client.post('/detect', headers={'authorization': 'Bearer randomapikey'}, json={"text": "dinner with joe finnigan"})

  assert response.status_code == 200

  json = response.json()

  assert len(json) == 1
  assert json[0]['type'] == "Person"
  assert json[0]['text'] == "joe finnigan"

def test_invalid_bearer_token():
  response = client.post('/detect', headers={'authorization': 'Bearer wrongapikey'}, json={"text": "dinner with joe finnigan"})

  assert response.status_code == 401

def test_no_bearer_token():
  response = client.post('/detect', json={"text": "dinner with joe finnigan"})

  assert response.status_code == 401
