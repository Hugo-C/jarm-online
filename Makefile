bootstrap_and_run_e2e_tests:
	./start_e2e_test.sh

run_e2e_tests_in_headed_mode:
	pytest --headed --slowmo 1000