use csv::Writer;
use scraper::Html;
use scraper::Selector;
use scraper::error::SelectorErrorKind;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() -> Result<(), SelectorErrorKind<'static>> {
    let indeed_domain = "https://www.indeed.com";

    let html_file_path = "get_html_content/html_content/index.html";
    let file_result = File::open(html_file_path);
    let mut html = String::new();
    if file_result.is_ok() {
        let mut file = file_result.unwrap();
        file.read_to_string(&mut html).unwrap();
    } else {
        println!("Unable to read the {} file", html_file_path);
        process::exit(0x0100);
    }

    let document = Html::parse_document(html.as_str());
    let job_card_selector = Selector::parse("div.cardOutline")?;

    struct Job {
        title: String,
        link: String,
        company_name: String,
        location: String,
    }

    let mut jobs: Vec<Job> = Vec::new();
    for job_card_element in document.select(&job_card_selector) {
        let heading_selector = Selector::parse("h2.jobTitle")?;
        let mut job = Job {
            title: String::new(),
            link: String::new(),
            company_name: String::new(),
            location: String::new(),
        };
        for heading_element in job_card_element.select(&heading_selector) {
            let link_selector = Selector::parse("a")?;
            for link_element in heading_element.select(&link_selector) {
                let route = link_element.value().attr("href").unwrap();
                let link: String = format!("{indeed_domain}{}", route);
                let job_title = link_element.text().collect::<String>();

                job.title = job_title;
                job.link = link;
            }
        }
        let company_selector = Selector::parse("div.company_location")?;
        for company_element in job_card_element.select(&company_selector) {
            let company_name_selector = Selector::parse("span")?;
            for company_name_element in company_element.select(&company_name_selector) {
                if company_name_element.value().attr("data-testid").is_some() {
                    if company_name_element.value().attr("data-testid").unwrap() == "company-name" {
                        let company_name = company_name_element.text().collect::<String>();
                        job.company_name = company_name;
                    }
                }
            }

            let location_selector = Selector::parse("div")?;
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

    let log_file_path = "logs/scrap_jobs.log";
    let result_logger = File::create(log_file_path);

    if result_logger.is_ok() {
        let mut logger = result_logger.unwrap();

        let csv_file_path = "data/indeed_jobs.csv";
        let writer_result = Writer::from_path(csv_file_path);
        if writer_result.is_ok() {
            let mut writer = writer_result.unwrap();
            writer
                .write_record(&["Title", "Company_Name", "Location", "Link"])
                .unwrap();

            for job in jobs {
                let job_title_info = format!("Job Title: {}\n", job.title);
                let company_name_info = format!("Company Name: {}\n", job.company_name);
                let location_info = format!("Location: {}\n", job.location);
                let link_info = format!("Link: {}\n\n", job.link);

                logger.write(job_title_info.as_bytes()).unwrap();
                logger.write(company_name_info.as_bytes()).unwrap();
                logger.write(location_info.as_bytes()).unwrap();
                logger.write(link_info.as_bytes()).unwrap();

                writer
                    .write_record(&[job.title, job.company_name, job.location, job.link])
                    .unwrap();
            }
            writer.flush().unwrap();
        } else {
            println!("Unable to open the {} file", csv_file_path);
            process::exit(0x0100);
        }
    } else {
        println!("Unable to open the {} file", log_file_path);
        process::exit(0x0100);
    }

    Ok(())
}
