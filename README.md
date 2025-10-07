# indeed_job_scraping_2

## Setup

Download the ChromeDriver [here](https://developer.chrome.com/docs/chromedriver/downloads#chromedriver_1140573590).

Copy the chromedriver binary to the `get_html_content/chromedriver-binary/` directory.

Go to the `get_html_content/` directory
```
cd get_html_content/
```

Create the virtual environment
```
python3 -m venv .venv
```

Activate the virtual environment
```
source .venv/bin/activate
```

Install dependencies on Ubuntu
```
sudo apt install python3-tk python3-dev
```

**Note**: You must install tkinter on Linux to use MouseInfo.

Install pip dependencies
```
pip install -r requirements.txt
```

## Running the Two Programs

Run the get html content program
```
cd get_html_content/
python3 main.py
```

Run the job scraper program
```
cargo run
```

## Resources
- [Selenium headless: How to bypass Cloudflare detection using Selenium - Stack Overflow](https://stackoverflow.com/questions/68289474/selenium-headless-how-to-bypass-cloudflare-detection-using-selenium)
- [Change the Selenium User Agent: Steps & Best Practices](https://www.zenrows.com/blog/selenium-user-agent#set-custom-user-agent)
- [CDP Mode - SeleniumBase](https://seleniumbase.io/examples/cdp_mode/ReadMe/)
