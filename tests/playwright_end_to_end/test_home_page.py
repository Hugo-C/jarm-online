import re
from playwright.sync_api import Page, expect
from fixtures import home_page  # noqa required fixture


def test_scan_address(home_page: Page):
    expect(home_page).to_have_title(re.compile("Jarm online"))

    submit_scan_address_field = home_page.get_by_placeholder("Url or IP")
    submit_scan_address_field.fill("test.com")
    submit_scan_address_field.press("Enter")

    # TODO check result


def test_latest_urls(home_page: Page):
    latest_url_header = home_page.get_by_role("heading", name=re.compile("Latest urls .+"))
    expect(latest_url_header).to_be_visible()

    # Check details of each url in the list and collapse it afterwards
    for i in range(5):
        url_detail_button = home_page.get_by_role("button", name=re.compile(f"URL {i + 1} unfold_more"))
        url_detail = home_page.get_by_text(text=f"JARM and it's maliciousness about URL {i + 1}", exact=True)

        expect(url_detail).to_be_hidden()
        url_detail_button.click()
        expect(url_detail).not_to_be_hidden()
        url_detail_button.click()
        expect(url_detail).to_be_hidden()


def test_fork_me_link(home_page: Page):
    fork_link = home_page.get_by_role("link", name="Fork me")

    # Expect an attribute "to be strictly equal" to the value.
    expect(fork_link).to_have_attribute("href", "https://github.com/Hugo-C/jarm-online")
