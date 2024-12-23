CREATE TABLE confirmed_ioc_scan (
	id UUID4 PRIMARY KEY,
	host TEXT NOT NULL,
	port TEXT NOT NULL,
	jarm_hash TEXT NOT NULL,
    scan_timestamp INTEGER NOT NULL,
    threat_fox_first_seen INTEGER NOT NULL,
    threat_fox_confidence_level INTEGER NOT NULL,
    threat_fox_malware TEXT NOT NULL
);