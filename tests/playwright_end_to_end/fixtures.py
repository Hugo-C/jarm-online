from time import sleep

import pytest
from playwright.async_api import Page, Error as PlayWrightError


@pytest.fixture
def home_page(page: Page) -> Page:
    try:
        page.goto("http://localhost:8080/")
    except PlayWrightError:
        sleep(2)  # try to wait for the page to be available
        page.goto("http://localhost:8080/")
    return page
