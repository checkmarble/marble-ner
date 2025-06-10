import json
import logging

from datetime import datetime
from logging import Formatter
from starlette.middleware.base import BaseHTTPMiddleware

class JsonFormatter(Formatter):
    def __init__(self):
        super(JsonFormatter, self).__init__()

    def format(self, record):
        json_record = {}
        json_record["level"] = record.levelname
        json_record["message"] = record.getMessage()

        for k, v in record.__dict__.items():
            if k.startswith("ner."):
                json_record[k.removeprefix("ner.")] = v

        return json.dumps(json_record)

handler = logging.StreamHandler()
handler.setFormatter(JsonFormatter())

logging.getLogger("uvicorn.access").disabled = True

logger = logging.root
logger.handlers = [handler]
logger.setLevel(logging.INFO)

class LogMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request, call_next):
        then = datetime.now()
        resp = await call_next(request)
        latency = datetime.now() - then

        logger.info(
            f"{request.method} {request.url.path}",
            extra={
                "ner.method": request.method,
                "ner.client": request.client.host if request.client else None,
                "ner.useragent": request.headers.get("user-agent"),
                "ner.size": resp.headers.get("content-length"),
                "ner.latency": latency.total_seconds(),
                "ner.status": resp.status_code,
            },
        )
        return resp
