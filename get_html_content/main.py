from seleniumbase import SB

def main():
    query = "help+desk"
    location = "United+States"
    date_posted_in_days = 7

    us_indeed_domain = "https://www.indeed.com"
    url = f"{us_indeed_domain}/jobs?q={query}&l={location}&fromage={date_posted_in_days}&start=0"

    html, msg = get_indeed_jobs_html_page(url)
    print(msg)

    if msg == "Success":
        with open("html_content/index.html", "w") as f:
            f.write(html)

def get_indeed_jobs_html_page(url: str) -> tuple[str, str]:
    html: str = ""
    with SB(uc=True, test=True, locale="en", xvfb=True) as sb:
        sb.activate_cdp_mode(url)
        sb.sleep(2)
        sb.cdp.maximize()
        sb.uc_gui_click_captcha()
        sb.sleep(10)
        if sb.get_page_title() == "Just a moment...":
            return html, "Unable to load web page"

        html = sb.get_page_source()
    
    return html, "Success"

if __name__ == "__main__":
    main()