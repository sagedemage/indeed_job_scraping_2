from selenium import webdriver
from selenium.common.exceptions import NoSuchDriverException
import time
import os

def main():
    query = "help+desk"
    location = "United+States"
    date_posted_in_days = 7

    us_indeed_domain = "https://www.indeed.com"
    url = f"{us_indeed_domain}/jobs?q={query}&l={location}&fromage={date_posted_in_days}&start=0"

    version = "141.0.7390.54"
    chromedriver_binary_path = "chromedriver-binary/chromedriver"
    command = os.popen(f"./{chromedriver_binary_path} --version")

    if command.close() is None:
        out = command.read()
        output_list = out.split(" ")
        version = output_list[1]

    options = webdriver.ChromeOptions()
    options.add_argument("--disable-blink-features=AutomationControlled")
    user_agent = f"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{version} Safari/537.36"
    options.add_argument(f"--user-agent={user_agent}")
    options.add_experimental_option("excludeSwitches", ["enable-automation"])
    options.add_experimental_option("useAutomationExtension", False)
    html, msg = get_indeed_jobs_html_page(url, options, chromedriver_binary_path)
    print(msg)

    if msg == "Success":
        with open("html_content/index.html", "w") as f:
            f.write(html)

def get_indeed_jobs_html_page(url: str, options: webdriver.ChromeOptions, chromedriver_binary_path: str) -> tuple[str, str]:
    html: str = ""
    service = webdriver.ChromeService(executable_path=chromedriver_binary_path)

    try:
        driver = webdriver.Chrome(options=options, service=service)
        driver.get(url)
        time.sleep(10)
        html = driver.page_source
        if driver.title == "Just a moment...":
            return html, "Unable to load web page"
    except NoSuchDriverException:
        print(f"Unable to find the chromedriver binary at {chromedriver_binary_path}")
        exit()
    
    return html, "Success"

if __name__ == "__main__":
    main()

