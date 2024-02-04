# Request with host and port
GET {{host}}/last-scans

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.last_scans" != null