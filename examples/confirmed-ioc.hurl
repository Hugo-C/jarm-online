# Request with host and port
POST {{host}}/confirmed-ioc-scans
Authorization: Token {{auth_token}}
{
    "host": "some host",
    "port": "443",
    "jarm_hash": "27d40d40d29d40d1dc42d43d00041d132f09251ceeb363bb0349f742bf0947",
    "scan_timestamp": 1736201288,
    "threat_fox_first_seen": 1736201200,
    "threat_fox_confidence_level": 1,
    "threat_fox_malware": "some malware"
}

HTTP 201
[Asserts]
header "location" == "https://jarm.online/api/v1/confirmed-ioc-scans"

GET {{host}}/confirmed-ioc-scans

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.results" count > 0
jsonpath "$.next" == null  # no pagination yet
