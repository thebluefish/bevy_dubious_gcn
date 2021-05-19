use rusb::{Direction, EndpointDescriptor};

#[derive(Debug)]
pub struct Endpoint {
    direction: Direction,
    address: u8,
    interval: u8,
}

impl<'a> From<EndpointDescriptor<'a>> for Endpoint {
    fn from(descriptor: EndpointDescriptor<'a>) -> Self {
        Endpoint {
            direction: descriptor.direction(),
            address: descriptor.number(),
            interval: descriptor.interval(),
        }
    }
}