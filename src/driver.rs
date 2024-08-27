use std::net::TcpListener;
use std::process::Stdio;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
use tokio::time::{sleep, timeout, Duration};

use crate::args::Browser;

pub struct Driver {
    process: Option<Child>,
}

impl Drop for Driver {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.start_kill();
            let _ = process.try_wait();
        }
    }
}

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("Failed to start WebDriver process: {0}")]
    ProcessStartError(#[from] std::io::Error),
    #[error("Failed to connect to WebDriver: {0}")]
    WebDriverConnectionError(#[from] fantoccini::error::CmdError),
    #[error("Failed to build client {0}")]
    WebDriverClientError(#[from] fantoccini::error::NewSessionError),
    #[error("WebDriver failed to start within the timeout period")]
    WebDriverStartTimeout,
}

impl Driver {
    pub async fn start(browser: Browser, port: &mut u16) -> Result<Driver, DriverError> {
        *port = Driver::find_available_port(*port).await;
        println!("Using port: {}", *port);

        match browser {
            Browser::Chrome => {
                let process = Command::new("chromedriver")
                    .arg(format!("--port={}", port))
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .map_err(DriverError::ProcessStartError)?;

                Ok(Driver {
                    process: Some(process),
                })
            }
            Browser::Gecko => {
                let process = Command::new("geckodriver")
                    .arg(format!("--port={}", port))
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .map_err(DriverError::ProcessStartError)?;

                Ok(Driver {
                    process: Some(process),
                })
            }
            Browser::None => Ok(Driver { process: None }),
        }
    }

    async fn find_available_port(mut port: u16) -> u16 {
        loop {
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(listener) => {
                    drop(listener);
                    break port;
                }
                Err(_) => {
                    println!(
                        "Webdriver port {} is in use, trying next port: {}",
                        port,
                        port.wrapping_add(1)
                    );
                    port = port.wrapping_add(1);
                }
            }
        }
    }

    pub async fn wait_for_webdriver(
        &self,
        port: u16,
        timeout_duration: Duration,
    ) -> Result<(), DriverError> {
        let address = format!("127.0.0.1:{}", port);
        match timeout(timeout_duration, async {
            loop {
                if TcpStream::connect(&address).await.is_ok() {
                    println!("WebDriver is up on port: {}.", port);
                    return Ok(());
                }
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        {
            Ok(result) => result,
            Err(_) => Err(DriverError::WebDriverStartTimeout),
        }
    }
}