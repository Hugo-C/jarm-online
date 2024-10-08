run_integration_tests:
	docker compose -f docker-compose.integration.yml up -d redis playwright_dummy_server
	cargo test -- --include-ignored
	docker compose -f docker-compose.integration.yml down

run_e2e_tests:
	docker compose -f docker-compose.playwright.yml build jarm_online_gui
	docker compose -f docker-compose.playwright.yml build jarm_online_api
	docker compose -f docker-compose.playwright.yml run pytest_runner --tracing=retain-on-failure --output=/code/tests/playwright-test-results/
	docker compose -f docker-compose.playwright.yml down

# e2e test with headed browser can't run in docker as a Xserver is required
run_e2e_tests_in_headed_mode:
	docker compose -f docker-compose.playwright.yml build jarm_online_gui
	docker compose -f docker-compose.playwright.yml build jarm_online_api
	docker compose -f docker-compose.playwright.yml run -p "80:80" -d jarm_online_gui
	PLAYWRIGHT_URL_UNDER_TEST="http://0.0.0.0:80/" pytest --headed --slowmo 1000
	docker compose -f docker-compose.playwright.yml down --remove-orphans