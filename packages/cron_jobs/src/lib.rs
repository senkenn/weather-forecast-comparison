pub mod application_business_rules {
    pub mod services_interface;
    pub mod usecase;
}

pub mod enterprise_business_rules {
    pub mod domain;
}

pub mod interface_adapters {
    pub mod csv_writer_interface;
    pub mod s3_service;
    pub mod scraper_interface;
}

pub mod frameworks_drivers {
    pub mod csv_writer;
    pub mod date;
    pub mod scraper;
}
