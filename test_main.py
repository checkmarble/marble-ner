import os

os.environ['NER_API_KEY'] = 'randomapikey'

from fastapi.testclient import TestClient
from main import Settings, app, settings


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

def test_detection_for_extra_type():
  def _settings():
      return Settings(labels=['Person', 'Country'])

  app.dependency_overrides[settings] = _settings

  response = client.post('/detect', headers={'authorization': 'Bearer randomapikey'}, json={"text": "dinner with joe finnigan at ACME Inc in Cyprus"})

  assert response.status_code == 200

  json = response.json()

  assert len(json) == 2
  assert json[0]['type'] == "Person"
  assert json[0]['text'] == "joe finnigan"
  assert json[1]['type'] == "Country"
  assert json[1]['text'] == "Cyprus"

  app.dependency_overrides = {}

def test_detection_for_defaults():
  response = client.post('/detect', headers={'authorization': 'Bearer randomapikey'}, json={"text": "dinner with joe finnigan at ACME Inc"})

  assert response.status_code == 200

  json = response.json()

  assert len(json) == 2
  assert json[0]['type'] == "Person"
  assert json[0]['text'] == "joe finnigan"
  assert json[1]['type'] == "Company"
  assert json[1]['text'] == "ACME Inc"

def test_invalid_bearer_token():
  response = client.post('/detect', headers={'authorization': 'Bearer wrongapikey'}, json={"text": "dinner with joe finnigan"})

  assert response.status_code == 401

def test_no_bearer_token():
  response = client.post('/detect', json={"text": "dinner with joe finnigan"})

  assert response.status_code == 401
