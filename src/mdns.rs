use anyhow::{Context, Result};
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{error, info};
use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceRegistration, ServiceType, TxtRecord};

#[derive(Default, Debug)]
pub struct ServiceContext {
    service_name: String,
}

pub struct MdnsServiceWrapper {
    port: u16,
}

impl MdnsServiceWrapper {
    pub fn new(port: u16, _host: &str) -> Result<Self> {
        info!("Creating mDNS service for _ota._tcp.local on port {}", port);

        Ok(Self { port })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting mDNS service advertisement");

        // Create service type for OTA updates
        let service_type =
            ServiceType::new("ota", "tcp").context("Failed to create service type")?;

        // Create the mDNS service
        let mut service = MdnsService::new(service_type, self.port);

        // Create TXT record with service information
        let mut txt_record = TxtRecord::new();
        txt_record
            .insert("version", "1.0")
            .context("Failed to insert version in TXT record")?;
        txt_record
            .insert("path", "/version")
            .context("Failed to insert path in TXT record")?;
        txt_record
            .insert("description", "OTA Update Server")
            .context("Failed to insert description in TXT record")?;

        // Set service properties
        service.set_name("OTA Server");
        service.set_registered_callback(Box::new(on_service_registered));
        service.set_txt_record(txt_record);

        // Set context for callback
        let context: Arc<Mutex<ServiceContext>> = Arc::default();
        service.set_context(Box::new(context));

        info!("Registering mDNS service: _ota._tcp.local");
        info!("Service name: OTA Server");
        info!("Port: {}", self.port);

        // Register the service and get the event loop
        let event_loop = service
            .register()
            .context("Failed to register mDNS service")?;

        // Start the event loop in a background task
        tokio::spawn(async move {
            loop {
                // Poll the event loop to keep the service alive
                if let Err(e) = event_loop.poll(Duration::from_millis(100)) {
                    error!("mDNS event loop error: {}", e);
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        info!("mDNS service registered successfully");
        Ok(())
    }

    pub fn _stop(&mut self) -> Result<()> {
        info!("Stopping mDNS service");
        // The service will be automatically unregistered when the event loop stops
        Ok(())
    }
}

fn on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    context: Option<Arc<dyn Any>>,
) {
    match result {
        Ok(service) => {
            info!("mDNS service registered successfully: {:?}", service);

            if let Some(context) = context {
                if let Some(context) = context.downcast_ref::<Arc<Mutex<ServiceContext>>>() {
                    if let Ok(mut ctx) = context.lock() {
                        ctx.service_name = service.name().clone();
                        info!("Service context updated: {:?}", ctx);
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to register mDNS service: {}", e);
        }
    }
}
