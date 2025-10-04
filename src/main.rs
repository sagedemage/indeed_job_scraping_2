use scraper::Html;
use scraper::Selector;
use std::error::Error;
use std::thread;
use std::time;
use thirtyfour::prelude::*;
use csv::Writer;
use std::process::Command;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let port = "40000";
    let server_url = format!("http://localhost:{}", port);

    let output = Command::new("sh")
        .arg("-c")
        .arg("./chromedriver-binary/chromedriver --version")
        .output()?;

    let out: String = String::from_utf8(output.stdout)?;
    let output_list: Vec<&str> = out.split(" ").collect();
    let version = output_list[1];

    let mut caps = DesiredCapabilities::chrome();
    let user_agent = format!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36", version);
    //caps.add_arg("--headless")?;
    caps.add_arg(format!("--user-agent={}", user_agent).as_str())?;
    caps.add_arg("--disable-blink-features=AutomationControlled")?;
    caps.add_arg("--start-maximized")?;
    caps.add_experimental_option("excludeSwitches", ["enable-automation"])?;
    caps.add_experimental_option("useAutomationExtension", false)?;

    let driver = WebDriver::new(server_url.as_str(), caps).await?;

    let indeed_domain = "https://www.indeed.com";
    let website_url = format!("{indeed_domain}{}", "/jobs?q=help+desk&l=United+States");
    driver.goto(website_url).await?;

    let dur = time::Duration::from_secs(10);
    thread::sleep(dur);
    
    let title =  driver.title().await?;

    if title == "Just a moment..." {
        println!("Unable to load web page!");
        driver.quit().await?;
        exit(1);
    }

    let html = driver.source().await?;
    driver.quit().await?;

    let document = Html::parse_document(html.as_str());
    let job_card_selector = Selector::parse("div.cardOutline").unwrap();

    struct Job {
        title: String,
        link: String,
        company_name: String,
        location: String,
    }

    let mut jobs: Vec<Job> = Vec::new();
    for job_card_element in document.select(&job_card_selector) {
        let heading_selector = Selector::parse("h2.jobTitle").unwrap();
        let mut job = Job {
            title: String::new(),
            link: String::new(),
            company_name: String::new(),
            location: String::new(),
        };
        for heading_element in job_card_element.select(&heading_selector) {
            let link_selector = Selector::parse("a").unwrap();
            for link_element in heading_element.select(&link_selector) {
                let route = link_element.value().attr("href").unwrap();
                let link: String = format!("{indeed_domain}{}", route);
                let job_title = link_element.text().collect::<String>();
                
                job.title = job_title;
                job.link = link;
            }
        }
        let company_selector = Selector::parse("div.company_location").unwrap();
        for company_element in job_card_element.select(&company_selector) {
            let company_name_selector = Selector::parse("span").unwrap();
            for company_name_element in company_element.select(&company_name_selector) {
                if company_name_element.value().attr("data-testid").is_some() {
                    if company_name_element.value().attr("data-testid").unwrap() == "company-name" {
                        let company_name = company_name_element.text().collect::<String>();
                        job.company_name = company_name;
                    }
                }
            }

            let location_selector = Selector::parse("div").unwrap();
            for location_element in company_element.select(&location_selector) {
                if location_element.value().attr("data-testid").is_some() {
                    if location_element.value().attr("data-testid").unwrap() == "text-location" {
                        let location = location_element.text().collect::<String>();
                        job.location = location;
                    }
                }
            }
        }
        jobs.push(job);
    }

    let mut writer = Writer::from_path("indeed_job_data/jobs.csv")?;
    writer.write_record(&["Title", "Company_Name", "Location", "Link"])?;

    for job in jobs {
        println!("Job Title: {}", job.title);
        println!("company name: {}", job.company_name);
        println!("location: {}", job.location);
        println!("link: {}", job.link);
        println!("");

        writer.write_record(&[job.title, job.company_name, job.location, job.link])?;
    }
    writer.flush()?;
    Ok(())
}
