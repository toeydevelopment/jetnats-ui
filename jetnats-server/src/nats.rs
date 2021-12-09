use nats::Connection;
use std::io;

pub trait INats {
    fn list_consumers(&self, s: &str) -> Result<Vec<String>, io::Error>;
    fn list_streams(&self) -> Result<Vec<String>, io::Error>;
}

pub struct Nats {
    nc: Connection,
}

impl Nats {
    pub fn new(nc: nats::Connection) -> Nats {
        Nats { nc }
    }
}

impl INats for Nats {
    fn list_consumers(&self, stream: &str) -> Result<Vec<String>, io::Error> {
        let consumers = self.nc.list_consumers(stream);
        match consumers {
            Ok(c) => Ok(c.map(|consumer| consumer.unwrap().name).collect()),
            Err(err) => return Err(err),
        }
    }

    fn list_streams(&self) -> Result<Vec<String>, io::Error> {
        let sx = self.nc.list_streams();

        Ok(sx.map(|s| s.unwrap().config.name).collect())
    }
}
