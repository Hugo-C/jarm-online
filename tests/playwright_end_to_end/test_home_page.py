import re

from playwright.sync_api import Page, expect
from fixtures import home_page  # noqa required fixture

INPUT_PLACEHOLDER = "8.8.8.8 | host.com/path"
HOST_TO_SCAN = "playwright_dummy_server_container"
URL_TO_SCAN = f"https://{HOST_TO_SCAN}"
URL_EXPECTED_JARM_RESULT = "21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d"


def test_scan_address(home_page: Page):
    expect(home_page).to_have_title(re.compile("Jarm online"))

    submit_scan_address_field = home_page.get_by_placeholder(INPUT_PLACEHOLDER)
    submit_scan_address_field.fill(URL_TO_SCAN)
    submit_scan_address_field.press("Enter")

    # check result
    result = home_page.get_by_text("Jarm hash is:")
    expect(result).to_contain_text(URL_EXPECTED_JARM_RESULT)
    copy_button = home_page.get_by_role("button", name="COPY")
    expect(copy_button).to_be_visible()  # clipboard button is present

    # Tranco overlap is displayed
    assert home_page.get_by_text("fake_site_1.com").is_visible()
    assert home_page.get_by_text("11th Rank").is_visible()
    tranco_overlap_link = home_page.get_by_role("link", name="1 other matching domains")
    expect(tranco_overlap_link).to_have_attribute("href", f"/api/v1/tranco-overlap?jarm_hash={URL_EXPECTED_JARM_RESULT}")


def test_latest_urls(home_page: Page):
    latest_url_header = home_page.get_by_role("heading", name=re.compile("Latest urls .+"))
    expect(latest_url_header).to_be_visible()

    # Submit an url for it to appears in latest urls
    submit_scan_address_field = home_page.get_by_placeholder(INPUT_PLACEHOLDER)
    submit_scan_address_field.fill(URL_TO_SCAN)
    submit_scan_address_field.press("Enter")

    # Hide the result so it won't interfere with tests
    submit_scan_address_field.fill("")
    submit_scan_address_field.press("Enter")
    result = home_page.get_by_text("Jarm hash is:")
    expect(result).to_be_hidden()  # sanity check

    latest_url_title = home_page.get_by_text(HOST_TO_SCAN).first
    latest_url_result = home_page.get_by_text(URL_EXPECTED_JARM_RESULT).first
    expect(latest_url_title).to_be_visible()
    expect(latest_url_result).to_be_hidden()
    latest_url_title.click()
    expect(latest_url_result).to_be_visible()
    latest_url_port = home_page.get_by_text("443")  # Default port is shown
    expect(latest_url_port).to_be_visible()


def test_latest_urls_for_specific_port(home_page: Page):
    specific_port = "440"
    latest_url_header = home_page.get_by_role("heading", name=re.compile("Latest urls .+"))
    expect(latest_url_header).to_be_visible()

    # Submit urls for it to appears in latest urls
    submit_scan_address_field = home_page.get_by_placeholder(INPUT_PLACEHOLDER)
    url_with_unusual_port = f"{URL_TO_SCAN}:{specific_port}"
    submit_scan_address_field.fill(url_with_unusual_port)
    submit_scan_address_field.press("Enter")

    # Hide the result so it won't interfere with tests
    submit_scan_address_field.fill("")
    submit_scan_address_field.press("Enter")
    result = home_page.get_by_text("Jarm hash is:")
    expect(result).to_be_hidden()  # sanity check

    latest_url_title = home_page.get_by_text(HOST_TO_SCAN).first
    latest_url_result = home_page.get_by_text(URL_EXPECTED_JARM_RESULT).first
    expect(latest_url_title).to_be_visible()
    expect(latest_url_result).to_be_hidden()
    latest_url_title.click()
    expect(latest_url_result).to_be_visible()
    latest_url_port = home_page.get_by_text(specific_port)
    expect(latest_url_port).to_be_visible()


def test_scan_error_on_invalid_tld(home_page: Page):
    url_scanned = "https://test.invalid_tld"

    submit_scan_address_field = home_page.get_by_placeholder(INPUT_PLACEHOLDER)
    submit_scan_address_field.fill(url_scanned)
    submit_scan_address_field.press("Enter")

    # check error returned
    error_title = home_page.get_by_text("API returned an error")
    expect(error_title).to_be_visible()
    error_body = home_page.get_by_text("Dns resolve error")
    expect(error_body).to_be_visible()


def test_fork_me_link(home_page: Page):
    fork_link = home_page.get_by_role("link", name="Fork me")

    # Expect an attribute "to be strictly equal" to the value.
    expect(fork_link).to_have_attribute("href", "https://github.com/Hugo-C/jarm-online")
