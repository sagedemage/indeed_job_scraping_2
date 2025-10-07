# indeed_job_scraping_2

## Setup

Download the ChromeDriver [here](https://developer.chrome.com/docs/chromedriver/downloads#chromedriver_1140573590).

Copy the chromedriver binary to the `get_html_content/chromedriver-binary/` directory.

Create virtual environment
```
python3 -m venv .venv
```

Activate the virtual environment
```
source .venv/bin/activate
```

Install dependencies
```
pip install -r requirements.txt
```

## Resources
- [Selenium headless: How to bypass Cloudflare detection using Selenium - Stack Overflow](https://stackoverflow.com/questions/68289474/selenium-headless-how-to-bypass-cloudflare-detection-using-selenium)
- [Change the Selenium User Agent: Steps & Best Practices](https://www.zenrows.com/blog/selenium-user-agent#set-custom-user-agent)
