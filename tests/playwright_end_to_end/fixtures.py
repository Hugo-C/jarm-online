import os
from time import sleep

import pytest
from playwright.async_api import Page, Error as PlayWrightError


@pytest.fixture
def home_page(page: Page) -> Page:
    url = os.getenv("PLAYWRIGHT_URL_UNDER_TEST") or "http://localhost:8080/"
    try:
        page.goto(url)
    except PlayWrightError:
        sleep(2)  # try to wait for the page to be available
        page.goto(url)
    return page
